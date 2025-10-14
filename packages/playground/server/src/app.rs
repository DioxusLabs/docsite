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
    fmt::Display,
    net::IpAddr,
    num::NonZeroU32,
    path::PathBuf,
    str::FromStr,
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
const DEFAULT_BUILT_DIR_SIZE: u64 = 1024 * 1024 * 1024; // 1 GB
/// Max memory usage of dx during a build before it is killed.
const DEFAULT_DX_MEMORY_LIMIT: u64 = 5 * 1024 * 1024 * 1024; // 5 GB
/// Max seconds a dx build can take before it is killed.
const DEFAULT_DX_BUILD_TIMEOUT: u64 = 5 * 60; // 5 minutes
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
        let max_built_dir_size = Self::get_max_built_dir_size();
        let max_target_dir_size = Self::get_max_target_dir_size();
        let dx_memory_limit = Self::get_dx_memory_limit();
        let dx_build_timeout = Self::get_dx_build_timeout();

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
            max_built_dir_size,
            max_target_dir_size,
            dx_memory_limit,
            dx_build_timeout,
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
        get_env_or("PRODUCTION", false)
    }

    /// Get the serve port from environment or default.
    fn get_port_env() -> u16 {
        get_env_or("PORT", DEFAULT_PORT)
    }

    /// Get the build template path from environment or default.
    fn get_build_template_path() -> PathBuf {
        get_env_parsed("BUILD_TEMPLATE_PATH").unwrap_or_else(|| DEFAULT_BUILD_TEMPLATE_PATH.into())
    }

    /// Get the server shutdown delay from the environment.
    fn get_shutdown_delay() -> Option<Duration> {
        let shutdown_delay = get_env_parsed::<u64>("SHUTDOWN_DELAY").map(Duration::from_secs);

        if shutdown_delay.is_none() {
            warn!("`SHUTDOWN_DELAY` environment variable is not set; the server will not turn off")
        }

        shutdown_delay
    }

    /// Get the GitHub Gists authentication token from the environment.
    fn get_gist_auth_token() -> Option<String> {
        get_env_parsed("GIST_AUTH_TOKEN")
    }

    /// Get the max size of the built directory from the environment or default.
    fn get_max_built_dir_size() -> u64 {
        get_env_or("MAX_BUILT_DIR_SIZE", DEFAULT_BUILT_DIR_SIZE)
    }

    /// Get the max size of the target directory from the environment or default.
    fn get_max_target_dir_size() -> u64 {
        get_env_or("MAX_TARGET_DIR_SIZE", DEFAULT_TARGET_DIR_SIZE)
    }

    /// Get the max memory limit for dx during a build from the environment or default.
    fn get_dx_memory_limit() -> u64 {
        get_env_or("DX_MEMORY_LIMIT", DEFAULT_DX_MEMORY_LIMIT)
    }

    /// Get the max seconds a dx build can take before it is killed from the environment or default.
    fn get_dx_build_timeout() -> u64 {
        get_env_or("DX_BUILD_TIMEOUT", DEFAULT_DX_BUILD_TIMEOUT)
    }
}

fn get_env_parsed<T: FromStr>(key: &str) -> Option<T> {
    env::var(key).ok().and_then(|v| v.parse().ok())
}

fn get_env_or<F: FromStr + Display>(key: &str, default: F) -> F {
    get_env_parsed(key).unwrap_or_else(|| {
        info!("`{key}` environment variable not set; defaulting to `{default}`");
        default
    })
}

/// The state of the server application.
#[derive(Clone)]
pub struct AppState {
    /// The environment configuration.
    pub env: Arc<EnvVars>,

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

        // Get prebuild arg
        let prebuild = std::env::args().any(|x| x == "--prebuild");

        if prebuild {
            info!("server is prebuilding");
            env.shutdown_delay = Some(Duration::from_secs(1));
        }

        let env = Arc::new(env);

        // Build the app state
        let is_building = Arc::new(AtomicBool::new(false));
        let build_queue_tx = start_build_watcher(env.clone(), is_building.clone());

        let build_govener = Arc::new(RateLimiter::keyed(
            Quota::with_period(Duration::from_secs(5))
                .expect("period is non-zero")
                .allow_burst(const { NonZeroU32::new(2).unwrap() }),
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
