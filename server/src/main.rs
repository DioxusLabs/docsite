use axum::{routing::get, Router};
use build::QueueType;
use dioxus_logger::tracing::{info, Level};
use tokio::{net::TcpListener, sync::mpsc};
use tower_http::services::ServeDir;

mod build;
mod serve;
mod ws;

const LISTEN_ADDR: &str = "0.0.0.0:3000";
const SERVE_PATH: &str = "../dist";
const TEMP_PATH: &str = "../temp/";
const BUILD_TEMPLATE_PATH: &str = "./template";
const REMOVAL_DELAY: u64 = 30000;
const BANNED_WORDS: &'static [&str] = &["eval","web_sys","bindgen","document","window"];

#[derive(Clone)]
struct AppState {
    build_queue_tx: mpsc::UnboundedSender<QueueType>,
}

#[tokio::main]
async fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let build_queue_tx = build::start_build_watcher().await;

    let state = AppState { build_queue_tx };

    // Build routers
    let serve = ServeDir::new(SERVE_PATH);
    let built_router = Router::new()
        .route("/", get(serve::serve_built_index))
        .route("/*file_path", get(serve::serve_other_built));

    let app = Router::new()
        .nest_service("/", serve)
        .route("/ws", get(ws::ws_handler))
        .nest("/built/:build_id", built_router)
        .with_state(state);

    // Start axum
    let listener = TcpListener::bind(LISTEN_ADDR).await.unwrap();

    info!("listening on {}", LISTEN_ADDR);
    axum::serve(listener, app).await.unwrap();
}
