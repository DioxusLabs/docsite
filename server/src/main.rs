use std::{env, sync::Arc, time::Duration};

use axum::{
    extract::{Request, State},
    middleware::{self, Next},
    response::{Redirect, Response},
    routing::get,
    Router,
};
use build::QueueType;
use dioxus_logger::tracing::{info, warn, Level};
use tokio::{
    fs,
    net::TcpListener,
    sync::{mpsc, Mutex},
    time::Instant,
};

mod build;
mod serve;
mod ws;

const LISTEN_IP: &str = "0.0.0.0";

/// Duration after building to delete.
/// 10 seconds *should* be good for most clients.
const REMOVAL_DELAY: u64 = 10000;

// Paths
//const SERVE_PATH: &str = "../dist";
const TEMP_PATH: &str = "../temp/";
const BUILD_TEMPLATE_PATH: &str = "./template";

/// A list of words that if found anywhere within submitted code, will be rejected.
/// This isn't really nescessary as the code runs locally on the client that submitted it but
/// it is still an extra layer of security.
const BANNED_WORDS: &'static [&str] = &["eval", "web_sys", "bindgen", "document", "window"];

#[derive(Clone)]
struct AppState {
    last_request_time: Arc<Mutex<Instant>>,
    build_queue_tx: mpsc::UnboundedSender<QueueType>,
}

#[tokio::main]
async fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    // Get environment variables
    let mut port: u16 = 3000;
    if let Ok(v) = env::var("PORT") {
        port = v
            .parse()
            .expect("the `PORT` environment variable is not a number");
    } else {
        warn!(
            "`PORT` environment variable not set; defaulting to `{}`",
            port
        );
    }

    let mut build_template_path = String::from(BUILD_TEMPLATE_PATH);
    if let Ok(v) = env::var("BUILD_TEMPLATE_PATH") {
        build_template_path = v;
    } else {
        warn!(
            "`BUILD_TEMPLATE_PATH` environment variable is not set; defaulting to `{}`",
            build_template_path
        );
    }

    // Remove the temp directory if it exists then re-create it.
    fs::remove_dir_all(TEMP_PATH).await.ok();
    fs::create_dir(TEMP_PATH)
        .await
        .expect("failed to create temp directory");

    // Build app
    let build_queue_tx = build::start_build_watcher(build_template_path).await;
    let state = AppState {
        build_queue_tx,
        last_request_time: Arc::new(Mutex::new(Instant::now())),
    };

    if let Ok(v) = env::var("SHUTDOWN_DELAY") {
        let shutdown_time_ms = v
            .parse()
            .expect("the `SHUTDOWN_DELAY` environment variable is not a number");

        start_shutdown_watcher(state.clone(), shutdown_time_ms);
    } else {
        warn!("`SHUTDOWN_DELAY` environment variable not set; the server will not turn off");
    }

    // Build routers
    //let serve = ServeDir::new(SERVE_PATH);
    let built_router = Router::new()
        .route("/", get(serve::serve_built_index))
        .route("/*file_path", get(serve::serve_other_built));

    let app = Router::new()
        //.nest_service("/", serve)
        .route(
            "/",
            get(|| async { Redirect::permanent("https://dioxuslabs.com/play") }),
        )
        .route("/ws", get(ws::ws_handler))
        .nest("/built/:build_id", built_router)
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            request_counter,
        ))
        .with_state(state);

    // Start axum
    let final_address = &format!("{LISTEN_IP}:{port}");
    let listener = TcpListener::bind(final_address).await.unwrap();

    info!("listening on `{}`", final_address);
    axum::serve(listener, app).await.unwrap();
}

/// Watches time since last request and shuts down the server
/// if there have been no requests in x time.
fn start_shutdown_watcher(state: AppState, shutdown_time_ms: u64) {
    tokio::task::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let now = Instant::now();

            let last_req_time = state.last_request_time.lock().await;
            let duration_since_req = now.duration_since(*last_req_time);

            if duration_since_req.as_millis() >= shutdown_time_ms as u128 {
                break;
            }
        }

        // Exit the app with code 0 to tell Fly.io to scale to zero.
        std::process::exit(0);
    });
}

/// Updates the time since last request for the shutdown watcher.
async fn request_counter(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let now = Instant::now();
    let mut lock = state.last_request_time.lock().await;
    *lock = now;
    drop(lock);
    next.run(req).await
}
