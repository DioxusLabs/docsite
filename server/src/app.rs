//! Initialization of the server application and environment configurations.

use crate::{
    build::{watcher::start_build_watcher, BuildCommand},
    start_shutdown_watcher,
};
use dioxus_logger::tracing::warn;
use std::{
    env, io,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};
use tokio::{
    fs,
    sync::{mpsc::UnboundedSender, Mutex},
    time::Instant,
};

const DEFAULT_PORT: u16 = 3000;

// Paths
const DEFAULT_BUILT_PATH: &str = "../temp/";
const DEFAULT_BUILD_TEMPLATE_PATH: &str = "./template";

// Duration after built projects are created to be removed.
const REMOVAL_DELAY: Duration = Duration::from_secs(20);

/// A group of environment configurations for the application.
#[derive(Clone)]
pub struct EnvVars {
    /// The port the server will listen on.
    pub port: u16,

    /// The path to the build template.
    pub build_template_path: PathBuf,

    /// The path where built projects are temporarily stored.
    pub built_path: PathBuf,

    /// The optional shutdown delay that specifies how many seconds after
    /// inactivity to shut down the server.
    pub shutdown_delay: Option<Duration>,
}

impl EnvVars {
    /// Get the environment configuration for the server.
    pub async fn new() -> Self {
        let port = Self::get_port_env();
        let build_template_path = Self::get_build_template_path();
        let shutdown_delay = Self::get_shutdown_delay();

        Self {
            port,
            build_template_path,
            built_path: PathBuf::from(DEFAULT_BUILT_PATH),
            shutdown_delay,
        }
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
            Err(_) => warn!(
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
            Err(_) => warn!(
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
}

impl AppState {
    /// Build the app state and initialize app services.
    pub async fn new() -> Self {
        let env = EnvVars::new().await;

        // Build the app state
        let is_building = Arc::new(AtomicBool::new(false));
        let build_queue_tx = start_build_watcher(env.clone(), is_building.clone());

        let state = Self {
            env,
            build_queue_tx,
            last_request_time: Arc::new(Mutex::new(Instant::now())),
            is_building,
        };

        // Reset the built dir
        let result = state.reset_built_dir().await;
        if let Err(e) = result {
            warn!("failed to reset built dir: {}", e);
        }

        // Start the shutdown watcher if the env was provided.
        if let Some(shutdown_delay) = state.env.shutdown_delay {
            start_shutdown_watcher(state.clone(), shutdown_delay);
        }

        state
    }

    /// Remove and recreate the built directory to clear any stale built projects.
    async fn reset_built_dir(&self) -> Result<(), io::Error> {
        let _ = fs::remove_dir_all(&self.env.built_path).await;
        fs::create_dir(&self.env.built_path).await?;
        Ok(())
    }
}
