use app::AppState;
use axum::{
    BoxError, Router,
    error_handling::HandleErrorLayer,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{Redirect, Response},
    routing::{get, post},
};
use axum_client_ip::ClientIpSource;
use dioxus_logger::tracing::{Level, error, info};
use share::{get_shared_project, share_project};
use std::{net::SocketAddr, sync::atomic::Ordering, time::Duration};
use tokio::{net::TcpListener, time::Instant};
use tower::{ServiceBuilder, buffer::BufferLayer, limit::RateLimitLayer};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

mod app;
mod build;
mod serve;
mod share;
mod ws;

/// Rate limiter configuration.
/// How many requests each user should get within a time period.
const REQUESTS_PER_INTERVAL: u64 = 60;
/// The period of time after the request limit resets.
const RATE_LIMIT_INTERVAL: Duration = Duration::from_secs(60);

#[tokio::main]
async fn main() {
    #[cfg(feature = "tracing")]
    {
        use tracing_subscriber::prelude::*;

        let console_layer = console_subscriber::spawn();
        use tracing_subscriber::prelude::*;
        use tracing_subscriber::{EnvFilter, fmt};

        let fmt_layer = fmt::layer();
        let filter_layer = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("info"))
            .unwrap();

        tracing_subscriber::registry()
            .with(console_layer)
            .with(filter_layer)
            .with(fmt_layer)
            .init();
    }
    _ = dioxus_logger::init(Level::INFO);

    let state = AppState::new().await;
    let port = state.env.port;

    let secure_ip_src = match state.env.production {
        true => ClientIpSource::FlyClientIp,
        false => ClientIpSource::ConnectInfo,
    };

    // Allow bursts with up to five requests per IP address
    // and replenishes one element every 30 seconds
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(60)
        .burst_size(120)
        .finish()
        .unwrap();

    // Build the routers.
    let built_router = Router::new()
        .route("/", get(serve::serve_built_index))
        .route("/{*file_path}", get(serve::serve_other_built));

    let shared_router = Router::new()
        .route("/", post(share_project))
        .route("/{:id}", get(get_shared_project));

    let app = Router::new()
        .nest("/ws", Router::new().route("/", get(ws::ws_handler)))
        .nest("/built/{:build_id}", built_router)
        .nest("/shared", shared_router)
        .route(
            "/",
            get(|| async { Redirect::permanent("https://dioxuslabs.com/play") }),
        )
        .route("/health", get(|| async { StatusCode::OK }))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    error!(?error, "unhandled server error");
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                }))
                .layer(CompressionLayer::new())
                .layer(CorsLayer::very_permissive())
                .layer(BufferLayer::new(1024))
                .layer(GovernorLayer::new(governor_conf))
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

/// Start misc services for maintaining the server's operation.
fn start_cleanup_services(state: AppState) {
    if let Some(shutdown_delay) = state.env.shutdown_delay {
        tokio::task::spawn(async move {
            loop {
                let now = Instant::now();
                let next_shutdown_check = now + shutdown_delay;

                // Check if server should shut down.
                tokio::time::sleep_until(next_shutdown_check).await;
                let should_shutdown = check_shutdown(&state, &shutdown_delay).await;
                if should_shutdown {
                    // TODO: We could be more graceful here.
                    std::process::exit(0);
                }
            }
        });
    }
}

/// Check if the server should shutdown.
async fn check_shutdown(state: &AppState, shutdown_delay: &Duration) -> bool {
    let now = Instant::now();
    let mut last_req_time = state.last_request_time.lock().await;

    // Reset timer when build is occuring.
    if state.is_building.load(Ordering::SeqCst) {
        *last_req_time = now;
        return false;
    }

    // Exit program if not building and duration exceeds shutdown time.
    let duration_since_req = now.duration_since(*last_req_time);
    if duration_since_req.as_secs() >= shutdown_delay.as_secs() {
        return true;
    }

    false
}

/// A middleware that counts the time since the last request for the shutdown watcher.
async fn request_counter(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let now = Instant::now();
    let mut lock = state.last_request_time.lock().await;
    *lock = now;
    drop(lock);
    next.run(req).await
}
