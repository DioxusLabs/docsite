use std::env;

use axum::{response::Redirect, routing::get, Router};
use build::QueueType;
use dioxus_logger::tracing::{info, warn, Level};
use tokio::{fs, net::TcpListener, sync::mpsc};

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

    // Remove the temp directory if it exists then re-create it.
    fs::remove_dir_all(TEMP_PATH).await.ok();
    fs::create_dir(TEMP_PATH)
        .await
        .expect("failed to create temp directory");

    // Build app
    let build_queue_tx = build::start_build_watcher().await;
    let state = AppState { build_queue_tx };

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
        .with_state(state);

    // Start axum
    let final_address = &format!("{LISTEN_IP}:{port}");
    let listener = TcpListener::bind(final_address).await.unwrap();

    info!("listening on `{}`", final_address);
    axum::serve(listener, app).await.unwrap();
}
