// use app::AppState;
// use axum::{
//     error_handling::HandleErrorLayer,
//     extract::{Request, State},
//     http::StatusCode,
//     middleware::{self, Next},
//     response::{Redirect, Response},
//     routing::{get, post},
//     BoxError, Router,
// };
// use axum_client_ip::SecureClientIpSource;
// use dioxus_logger::tracing::{error, info, warn, Level};
// use share::{get_shared_project, share_project};
// use std::{io, net::SocketAddr, sync::atomic::Ordering, time::Duration};
// use tokio::{net::TcpListener, select, time::Instant};
// use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
// use tower_http::{compression::CompressionLayer, cors::CorsLayer};

// mod app;
// mod build;
// mod serve;
// mod share;
// mod ws;

// /// Rate limiter configuration.
// /// How many requests each user should get within a time period.
// const REQUESTS_PER_INTERVAL: u64 = 30;
// /// The period of time after the request limit resets.
// const RATE_LIMIT_INTERVAL: Duration = Duration::from_secs(60);

// #[tokio::main]
// async fn main() {
//     dioxus_logger::init(Level::INFO).expect("failed to init logger");

//     let state = AppState::new().await;
//     let port = state.env.port;

//     let secure_ip_src = match state.env.production {
//         true => SecureClientIpSource::FlyClientIp,
//         false => SecureClientIpSource::ConnectInfo,
//     };

//     // Build the routers.
//     let built_router = Router::new()
//         .route("/", get(serve::serve_built_index))
//         .route("/*file_path", get(serve::serve_other_built));

//     let shared_router = Router::new()
//         .route("/", post(share_project))
//         .route("/:id", get(get_shared_project));

//     let app = Router::new()
//         .route("/ws", get(ws::ws_handler))
//         .nest("/built/:build_id", built_router)
//         .nest("/shared", shared_router)
//         .route(
//             "/",
//             get(|| async { Redirect::permanent("https://dioxuslabs.com/play") }),
//         )
//         .route("/health", get(|| async { StatusCode::OK }))
//         .layer(
//             ServiceBuilder::new()
//                 .layer(HandleErrorLayer::new(|error: BoxError| async move {
//                     error!(?error, "unhandled server error");
//                     (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
//                 }))
//                 .layer(CompressionLayer::new())
//                 .layer(CorsLayer::very_permissive())
//                 .layer(BufferLayer::new(1024))
//                 .layer(RateLimitLayer::new(
//                     REQUESTS_PER_INTERVAL,
//                     RATE_LIMIT_INTERVAL,
//                 ))
//                 .layer(secure_ip_src.into_extension())
//                 .layer(middleware::from_fn_with_state(
//                     state.clone(),
//                     request_counter,
//                 )),
//         )
//         .with_state(state);

//     // Start the Axum server.
//     let final_address = &format!("0.0.0.0:{port}");
//     let listener = TcpListener::bind(final_address).await.unwrap();

//     info!("listening on `{}`", final_address);
//     axum::serve(
//         listener,
//         app.into_make_service_with_connect_info::<SocketAddr>(),
//     )
//     .await
//     .unwrap();
// }

// /// Start misc services for maintaining the server's operation.
// fn start_cleanup_services(state: AppState) {
//     tokio::task::spawn(async move {
//         let cleanup_delay = state.env.built_cleanup_delay;
//         let shutdown_delay = state
//             .env
//             .shutdown_delay
//             .unwrap_or(Duration::from_secs(99999999));

//         loop {
//             let now = Instant::now();
//             let next_shutdown_check = now + shutdown_delay;
//             let next_cleanup_check = now + cleanup_delay;

//             select! {
//                 // Perform the next built project cleanup.
//                 _ = tokio::time::sleep_until(next_cleanup_check) => {
//                     if let Err(e) = check_cleanup(state.clone()).await {
//                         warn!("failed to clean built projects: {e}");
//                     }
//                 }

//                 // Check if server should shut down.
//                 _ = tokio::time::sleep_until(next_shutdown_check), if state.env.shutdown_delay.is_some() => {
//                     let should_shutdown = check_shutdown(&state, &shutdown_delay).await;
//                     if should_shutdown {
//                         // TODO: We could be more graceful here.
//                         std::process::exit(0);
//                     }
//                 }
//             }
//         }
//     });
// }

// /// Check and cleanup any expired built projects.
// async fn check_cleanup(state: AppState) -> Result<(), io::Error> {
//     let task = tokio::task::spawn_blocking(move || {
//         let dir = std::fs::read_dir(state.env.built_path)?;

//         for item in dir {
//             let item = item?;
//             let path = item.path();
//             let pathname = path.file_name().unwrap().to_string_lossy();

//             // Always cache the examples - don't remove those.
//             if example_projects::get_example_projects()
//                 .iter()
//                 .any(|p| p.id().to_string() == pathname)
//             {
//                 continue;
//             }

//             let time_elapsed = item
//                 .metadata()
//                 .and_then(|m| m.created())
//                 .and_then(|c| c.elapsed().map_err(io::Error::other))?;

//             if time_elapsed >= state.env.built_cleanup_delay {
//                 std::fs::remove_dir_all(path)?;
//             }
//         }

//         Ok(())
//     });

//     task.await.expect("task should not panic or abort")
// }

// /// Check if the server should shutdown.
// async fn check_shutdown(state: &AppState, shutdown_delay: &Duration) -> bool {
//     let now = Instant::now();
//     let mut last_req_time = state.last_request_time.lock().await;

//     // Reset timer when build is occuring.
//     if state.is_building.load(Ordering::SeqCst) {
//         *last_req_time = now;
//         return false;
//     }

//     // Exit program if not building and duration exceeds shutdown time.
//     let duration_since_req = now.duration_since(*last_req_time);
//     if duration_since_req.as_secs() >= shutdown_delay.as_secs() {
//         return true;
//     }

//     false
// }

// /// A middleware that counts the time since the last request for the shutdown watcher.
// async fn request_counter(State(state): State<AppState>, req: Request, next: Next) -> Response {
//     let now = Instant::now();
//     let mut lock = state.last_request_time.lock().await;
//     *lock = now;
//     drop(lock);
//     next.run(req).await
// }

fn main() {}
