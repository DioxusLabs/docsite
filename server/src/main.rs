use axum::{
    error_handling::HandleErrorLayer,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{Redirect, Response},
    routing::get,
    BoxError, Router,
};
use build::watcher::{start_build_watcher, BuildCommand};
use dioxus_logger::tracing::{info, warn, Level};
use std::{
    env,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    fs,
    net::TcpListener,
    sync::{mpsc::UnboundedSender, Mutex},
    time::Instant,
};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};

mod build;
mod serve;
mod ws;

const LISTEN_IP: &str = "0.0.0.0";

/// Duration after building to delete.
/// 20 seconds should be plenty.
const REMOVAL_DELAY: Duration = Duration::from_secs(20);

/// Rate limiter configuration.
/// How many requests each user should get within a time period.
const REQUESTS_PER_INTERVAL: u64 = 30;
/// The period of time after the request limit resets.
const RATE_LIMIT_INTERVAL: Duration = Duration::from_secs(60);

// Paths
const TEMP_PATH: &str = "../temp/";
const BUILD_TEMPLATE_PATH: &str = "./template";

#[derive(Clone)]
struct AppState {
    last_request_time: Arc<Mutex<Instant>>,
    build_queue_tx: UnboundedSender<BuildCommand>,
    is_building: Arc<AtomicBool>,
}

#[tokio::main]
async fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    // Get the port to run on.
    let mut port: u16 = 3000;
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

    // Get the path to the build template.
    let mut build_template_path = PathBuf::from(BUILD_TEMPLATE_PATH);
    match env::var("BUILD_TEMPLATE_PATH") {
        Ok(v) => build_template_path = PathBuf::from(v),
        Err(_) => warn!(
            "`BUILD_TEMPLATE_PATH` environment variable is not set; defaulting to `{:?}`",
            build_template_path
        ),
    }

    // Remove any stale built projects in the temp folder and then recreate the folder.
    let _ = fs::remove_dir_all(TEMP_PATH).await;
    fs::create_dir(TEMP_PATH)
        .await
        .expect("failed to create temp directory");

    // Build the app state
    let is_building = Arc::new(AtomicBool::new(false));
    let build_queue_tx = start_build_watcher(is_building.clone(), build_template_path).await;
    let state = AppState {
        build_queue_tx,
        last_request_time: Arc::new(Mutex::new(Instant::now())),
        is_building,
    };

    // Start the shutdown watcher is requested.
    match env::var("SHUTDOWN_DELAY") {
        Ok(v) => {
            let shutdown_delay = v
                .parse()
                .expect("the `SHUTDOWN_DELAY` environment variable should be a number");

            start_shutdown_watcher(state.clone(), Duration::from_secs(shutdown_delay));
        }
        Err(_) => {
            warn!("`SHUTDOWN_DELAY` environment variable is not set; the server will not turn off")
        }
    }

    // Build the routers.
    let built_router = Router::new()
        .route("/", get(serve::serve_built_index))
        .route("/*file_path", get(serve::serve_other_built));

    let app = Router::new()
        .route("/ws", get(ws::ws_handler))
        .nest("/built/:build_id", built_router)
        .route(
            "/",
            get(|| async { Redirect::permanent("https://dioxuslabs.com/play") }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(
                    REQUESTS_PER_INTERVAL,
                    RATE_LIMIT_INTERVAL,
                ))
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    request_counter,
                )),
        )
        .with_state(state);

    // Start the Axum server.
    let final_address = &format!("{LISTEN_IP}:{port}");
    let listener = TcpListener::bind(final_address).await.unwrap();

    info!("listening on `{}`", final_address);
    axum::serve(listener, app).await.unwrap();
}

/// Checks if the server can shutdown.
fn start_shutdown_watcher(state: AppState, shutdown_delay: Duration) {
    tokio::task::spawn(async move {
        loop {
            tokio::time::sleep(shutdown_delay).await;

            let now = Instant::now();
            let mut last_req_time = state.last_request_time.lock().await;

            // Reset timer when build is occuring.
            if state.is_building.load(Ordering::SeqCst) {
                *last_req_time = now;
                continue;
            }

            // Exit program if not building and duration exceeds shutdown time.
            let duration_since_req = now.duration_since(*last_req_time);
            if duration_since_req.as_secs() >= shutdown_delay.as_secs() {
                break;
            }
        }

        // Exit the app with code 0 to tell Fly.io to scale to zero.
        std::process::exit(0);
    });
}

/// A middleware that counts the time since the last request for the shutdown watcher.
async fn request_counter(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let now = Instant::now();
    let mut lock = state.last_request_time.lock().await;
    *lock = now;
    drop(lock);
    next.run(req).await
}
