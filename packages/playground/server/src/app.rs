//! Initialization of the server application and environment configurations.

use crate::{
    build::{watcher::start_build_watcher, BuildCommand, BuildRequest},
    start_cleanup_services,
};
use dioxus_logger::tracing::{info, warn};
use std::{
    env,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};
use tokio::{
    sync::{mpsc::UnboundedSender, Mutex},
    time::Instant,
};

const DEFAULT_PORT: u16 = 3000;

// Paths
const DEFAULT_BUILD_TEMPLATE_PATH: &str = "./template";

// Duration after built projects are created to be removed.
const DEFAULT_BUILT_CLEANUP_DELAY: Duration = Duration::from_secs(20);

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

    /// The time after creation each built project should be removed.
    pub built_cleanup_delay: Duration,

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
            built_cleanup_delay: DEFAULT_BUILT_CLEANUP_DELAY,
            gist_auth_token: gist_auth_token.unwrap_or_default(),
        }
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

    /// A list of connected sockets by ip. Used to disallow extra socket connections.
    pub _connected_sockets: Arc<Mutex<Vec<String>>>,

    pub reqwest_client: reqwest::Client,
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

        let state = Self {
            env,
            build_queue_tx,
            last_request_time: Arc::new(Mutex::new(Instant::now())),
            is_building,
            _connected_sockets: Arc::new(Mutex::new(Vec::new())),
            reqwest_client: reqwest::Client::new(),
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
