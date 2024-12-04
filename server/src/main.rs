use app::AppState;
use axum::{
    error_handling::HandleErrorLayer,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{Redirect, Response},
    routing::get,
    BoxError, Router,
};
use axum_client_ip::SecureClientIpSource;
use dioxus_logger::tracing::{info, Level};
use std::{net::SocketAddr, sync::atomic::Ordering, time::Duration};
use tokio::{net::TcpListener, time::Instant};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};

mod app;
mod build;
mod serve;
mod ws;

/// Rate limiter configuration.
/// How many requests each user should get within a time period.
const REQUESTS_PER_INTERVAL: u64 = 30;
/// The period of time after the request limit resets.
const RATE_LIMIT_INTERVAL: Duration = Duration::from_secs(60);

#[tokio::main]
async fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let state = AppState::new().await;
    let port = state.env.port;

    let secure_ip_src = match state.env.production {
        true => SecureClientIpSource::FlyClientIp,
        false => SecureClientIpSource::ConnectInfo,
    };

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
        .route("/health", get(|| async { StatusCode::OK }))
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
                .layer(secure_ip_src.into_extension())
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    request_counter,
                )),
        )
        .with_state(state);

    // Start the Axum server.
    let final_address = &format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(final_address).await.unwrap();

    info!("listening on `{}`", final_address);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
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
