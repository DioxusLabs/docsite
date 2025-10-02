//! Initialization of the server application and environment configurations.

use crate::{
    build::{BuildCommand, BuildRequest, watcher::start_build_watcher},
    start_cleanup_services,
};
use dioxus_logger::tracing::{info, warn};
use governor::{
    Quota, RateLimiter,
    clock::{QuantaClock, QuantaInstant},
    middleware::NoOpMiddleware,
    state::keyed::DashMapStateStore,
};
use std::{
    env,
    net::IpAddr,
    num::NonZeroU32,
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};
use tokio::{
    sync::{Mutex, mpsc::UnboundedSender},
    time::Instant,
};
use uuid::Uuid;

const DEFAULT_PORT: u16 = 3000;

// Paths
const DEFAULT_BUILD_TEMPLATE_PATH: &str = "./template";

/// Max size of the built directory before old projects are removed.
const DEFAULT_BUILT_DIR_SIZE: u64 = 1 * 1024 * 1024 * 1024; // 1 GB
/// Max memory usage of dx during a build before it is killed.
const DEFAULT_DX_MEMORY_LIMIT: u64 = 5 * 1024 * 1024 * 1024; // 5 GB
/// Max seconds a dx build can take before it is killed.
const DEFAULT_DX_BUILD_TIMEOUT: u64 = 30; // 30 seconds
/// Max size of the target directory before it is cleaned.
const DEFAULT_TARGET_DIR_SIZE: u64 = 3 * 1024 * 1024 * 1024; // 3 GB

/// A group of environment configurations for the application.
#[derive(Clone)]
pub struct EnvVars {
    /// Is the server running in production?
    pub production: bool,

    /// The port the server will listen on.
    pub port: u16,

    /// The path to the build template.
    pub build_template_path: PathBuf,

    /// The path where built projects are temporarily stored.
    pub built_path: PathBuf,

    /// The max size of the built project directory before old projects are removed.
    pub max_built_dir_size: u64,

    /// The max size of the target directory before it is cleaned.
    pub max_target_dir_size: u64,

    /// The max memory limit for dx during a build.
    #[cfg_attr(not(target_os = "linux"), allow(unused))]
    pub dx_memory_limit: u64,

    /// The max seconds a dx build can take before it is killed.
    #[cfg_attr(not(target_os = "linux"), allow(unused))]
    pub dx_build_timeout: u64,

    /// The optional shutdown delay that specifies how many seconds after
    /// inactivity to shut down the server.
    pub shutdown_delay: Option<Duration>,

    pub gist_auth_token: String,
}

impl EnvVars {
    /// Get the environment configuration for the server.
    pub async fn new() -> Self {
        let production = Self::get_production_env();
        let port = Self::get_port_env();
        let build_template_path = Self::get_build_template_path();
        let shutdown_delay = Self::get_shutdown_delay();
        let gist_auth_token = Self::get_gist_auth_token();

        Self {
            production,
            port,
            build_template_path,
            built_path: if production {
                PathBuf::from("/usr/src/app/temp/")
            } else {
                PathBuf::from("./temp/")
            },
            shutdown_delay,
            max_built_dir_size: DEFAULT_BUILT_DIR_SIZE,
            max_target_dir_size: DEFAULT_TARGET_DIR_SIZE,
            dx_memory_limit: DEFAULT_DX_MEMORY_LIMIT,
            dx_build_timeout: DEFAULT_DX_BUILD_TIMEOUT,
            gist_auth_token: gist_auth_token.unwrap_or_default(),
        }
    }

    /// Get the path to the target dir
    pub fn target_dir(&self) -> PathBuf {
        self.build_template_path.join("target")
    }

    /// Get the path to the built template hot patch cache
    pub fn built_template_hotpatch_cache(&self, id: &Uuid) -> PathBuf {
        self.target_dir()
            .join("hotpatch_cache")
            .join(id.to_string())
    }

    /// Get the production environment variable.
    fn get_production_env() -> bool {
        let production = env::var("PRODUCTION")
            .ok()
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(false);

        info!("is the server is running in production? {production}");
        production
    }

    /// Get the serve port from environment or default.
    fn get_port_env() -> u16 {
        let mut port = DEFAULT_PORT;
        match env::var("PORT") {
            Ok(v) => {
                port = v
                    .parse()
                    .expect("the `PORT` environment variable should be a number")
            }
            Err(_) => info!(
                "`PORT` environment variable not set; defaulting to `{}`",
                port
            ),
        }

        port
    }

    /// Get the build template path from environment or default.
    fn get_build_template_path() -> PathBuf {
        let mut build_template_path = PathBuf::from(DEFAULT_BUILD_TEMPLATE_PATH);
        match env::var("BUILD_TEMPLATE_PATH") {
            Ok(v) => build_template_path = PathBuf::from(v),
            Err(_) => info!(
                "`BUILD_TEMPLATE_PATH` environment variable is not set; defaulting to `{:?}`",
                build_template_path
            ),
        }

        build_template_path
    }

    /// Get the server shutdown delay from the environment.
    fn get_shutdown_delay() -> Option<Duration> {
        let shutdown_delay = env::var("SHUTDOWN_DELAY")
            .ok()
            .and_then(|v| v.parse().ok())
            .map(Duration::from_secs);

        if shutdown_delay.is_none() {
            warn!("`SHUTDOWN_DELAY` environment variable is not set; the server will not turn off")
        }

        shutdown_delay
    }

    /// Get the GitHub Gists authentication token from the environment.
    fn get_gist_auth_token() -> Option<String> {
        let gist_auth_token = env::var("GIST_AUTH_TOKEN").ok();

        if gist_auth_token.is_none() {
            warn!("`GIST_AUTH_TOKEN` environment variable is not set")
        }

        gist_auth_token
    }
}

/// The state of the server application.
#[derive(Clone)]
pub struct AppState {
    /// The environment configuration.
    pub env: EnvVars,

    /// The time instante since the last request.
    pub last_request_time: Arc<Mutex<Instant>>,

    /// The build command channel.
    pub build_queue_tx: UnboundedSender<BuildCommand>,

    /// Prevents the server from shutting down during an active build.
    pub is_building: Arc<AtomicBool>,

    pub reqwest_client: reqwest::Client,

    pub build_govener: Arc<
        RateLimiter<IpAddr, DashMapStateStore<IpAddr>, QuantaClock, NoOpMiddleware<QuantaInstant>>,
    >,
}

impl AppState {
    /// Build the app state and initialize app services.
    pub async fn new() -> Self {
        let mut env = EnvVars::new().await;

        // Build the app state
        let is_building = Arc::new(AtomicBool::new(false));
        let build_queue_tx = start_build_watcher(env.clone(), is_building.clone());

        // Get prebuild arg
        let prebuild = std::env::args()
            .collect::<Vec<String>>()
            .get(1)
            .map(|x| x == "--prebuild")
            .unwrap_or(false);

        if prebuild {
            info!("server is prebuilding");
            env.shutdown_delay = Some(Duration::from_secs(1));
        }

        let build_govener = Arc::new(RateLimiter::keyed(
            Quota::with_period(Duration::from_secs(5))
                .unwrap()
                .allow_burst(NonZeroU32::new(2).unwrap()),
        ));

        let state = Self {
            env,
            build_queue_tx,
            last_request_time: Arc::new(Mutex::new(Instant::now())),
            is_building,
            reqwest_client: reqwest::Client::new(),
            build_govener,
        };

        // Queue the examples to be built on startup.
        // This ensures the cache is hot before users try to use it, meaning the examples will be ready to go.
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        std::mem::forget(rx);
        for project in example_projects::get_example_projects() {
            dioxus::logger::tracing::trace!(example = ?project, "queueing example project");
            let _ = state.build_queue_tx.send(BuildCommand::Start {
                request: BuildRequest {
                    id: project.id(),
                    previous_build_id: None,
                    project: project.clone(),
                    ws_msg_tx: tx.clone(),
                },
            });
        }

        // Start the app services
        start_cleanup_services(state.clone());

        state
    }
}
