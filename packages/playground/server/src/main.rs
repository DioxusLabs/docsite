use anyhow::{bail, Context, Result};
use axum::extract::{ws::WebSocket, WebSocketUpgrade};
use axum::Json;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use axum::{
    error_handling::HandleErrorLayer,
    extract::Request,
    middleware::{self, Next},
    response::{Redirect, Response},
    routing::{get, post},
    BoxError, Router,
};
use axum_client_ip::ClientIp;
use axum_client_ip::ClientIpSource;
use dioxus::logger::tracing;
use dioxus_dx_wire_format::StructuredOutput;
use dioxus_logger::tracing::{debug, error, info, trace, warn, Level};
use fs_extra::dir::CopyOptions;
use futures::{future::Either, pin_mut, SinkExt, StreamExt as _};
use model::api::{GetSharedProjectRes, ShareProjectReq, ShareProjectRes};
use model::AppError;
use model::{BuildStage, CargoDiagnostic};
use model::{Project, SocketMessage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::net::IpAddr;
use std::path::PathBuf;
use std::process::Stdio;
use std::{
    env,
    sync::{atomic::AtomicBool, Arc},
};
use std::{io, net::SocketAddr, sync::atomic::Ordering, time::Duration};
use tokio::fs;
use tokio::io::{AsyncBufReadExt as _, BufReader};
use tokio::process::Command;
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    Mutex,
};
use tokio::task::JoinHandle;
use tokio::{net::TcpListener, select, time::Instant};
use tokio_util::io::ReaderStream;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};
use uuid::Uuid;

const REQUESTS_PER_INTERVAL: u64 = 30;
const RATE_LIMIT_INTERVAL: Duration = Duration::from_secs(60);
const DEFAULT_PORT: u16 = 3000;
const DEFAULT_BUILD_TEMPLATE_PATH: &str = "./template";
const PRIMARY_GIST_FILE_NAME: &str = "dxp.rs";

const GISTS_URL_PREFIX: &str = "https://api.github.com/gists";
const GITHUB_USER_AGENT: &str = "Dioxus Playground";

#[tokio::main]
async fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let state = AppState::new().await;
    let app = Router::new()
        .route("/ws", get(AppState::ws_handler))
        .nest(
            "/built/:build_id",
            Router::new()
                .route("/", get(AppState::serve_built_index))
                .route("/*file_path", get(AppState::serve_other_built)),
        )
        .nest(
            "/shared",
            Router::new()
                .route("/", post(AppState::share_project))
                .route("/:id", get(AppState::get_shared_project)),
        )
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
                .layer(RateLimitLayer::new(
                    REQUESTS_PER_INTERVAL,
                    RATE_LIMIT_INTERVAL,
                ))
                .layer(
                    match state.production {
                        true => ClientIpSource::FlyClientIp,
                        false => ClientIpSource::ConnectInfo,
                    }
                    .into_extension(),
                ),
        )
        .with_state(state.clone());

    // Start the Axum server.
    let final_address = &format!("0.0.0.0:{port}", port = state.port);
    info!("listening on `{}`", final_address);
    axum::serve(
        TcpListener::bind(final_address).await.unwrap(),
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

/// The state of the server application.
#[derive(Clone)]
struct AppState {
    /// Is the server running in production?
    production: bool,

    /// The port the server will listen on.
    port: u16,

    /// The path to the build template.
    build_template_path: PathBuf,

    /// The path where built projects are temporarily stored.
    built_path: PathBuf,

    gist_auth_token: String,

    /// A list of connected sockets by ip. Used to disallow extra socket connections.
    _connected_sockets: Arc<Mutex<Vec<String>>>,

    reqwest_client: reqwest::Client,
}

impl AppState {
    /// Build the app state and initialize app services.
    async fn new() -> Self {
        let production = env::var("PRODUCTION")
            .ok()
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(false);

        let port = std::env::var("PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(DEFAULT_PORT);

        let build_template_path = PathBuf::from(
            env::var("BUILD_TEMPLATE_PATH")
                .unwrap_or_else(|_| DEFAULT_BUILD_TEMPLATE_PATH.to_string()),
        );

        let gist_auth_token = env::var("GIST_AUTH_TOKEN").ok().unwrap_or_default();

        let built_path = match production {
            true => PathBuf::from("/usr/src/app/temp/"),
            false => PathBuf::from("./temp/"),
        };

        Self {
            production,
            port,
            build_template_path,
            built_path,
            gist_auth_token,
            _connected_sockets: Arc::new(Mutex::new(Vec::new())),
            reqwest_client: reqwest::Client::new(),
        }
    }

    /// Handle any pre-websocket processing.
    async fn ws_handler(
        State(state): State<AppState>,
        ClientIp(ip): ClientIp,
        ws: WebSocketUpgrade,
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| async move {
            let (mut socket_tx, mut socket_rx) = socket.split();

            // Copy the template project over to a new tempdir.
            let users_project = tempdir::TempDir::new("user-project");

            // Ensure only one client per socket.
            // let mut connected_sockets = state.connected_sockets.lock().await;
            // if connected_sockets.contains(&ip) {
            //     // Client is already connected. Send error and close socket.
            //     let _ = socket_tx
            //         .send(SocketMessage::AlreadyConnected.into_axum())
            //         .await;
            //     let _ = socket_tx.close().await;
            //     return;
            // } else {
            // connected_sockets.push(ip.clone());
            // }
            // drop(connected_sockets);

            // // Handle incoming build commands.
            // Some(command) = rx.recv() => {
            //     match command {
            //         BuildCommand::Start { request } => builder.start_build(&mut pending_builds, request),
            //         BuildCommand::Stop { id } => builder.stop_build(&mut pending_builds, id),
            //     }
            // }
            // // Handle finished build or make progress on current build.
            // build_result = builder.finished() => builder.handle_finished_build(&mut pending_builds, build_result),

            // // Start our build loop.
            // let (build_tx, mut build_rx) = mpsc::unbounded_channel();
            // let mut current_build: Option<BuildRequest> = None;

            // loop {
            //     let srx = socket_rx.next();
            //     let brx = build_rx.recv();
            //     pin_mut!(srx, brx);
            //     let res = futures::future::select(srx, brx).await;

            //     match res {
            //         Either::Left((Some(Ok(socket_msg)), _)) => {
            //             // Try decoding the socket messages into our own type. If invalid, just ignore it.
            //             let Ok(socket_msg) = SocketMessage::try_from(socket_msg) else {
            //                 continue;
            //             };

            //             // Start a new build, stopping any existing ones.
            //             if let SocketMessage::BuildRequest(code) = socket_msg {
            //                 if let Some(ref request) = current_build {
            //                     // if result.is_err() {
            //                     //     error!(build_id = ?request.id, "failed to send build stop signal for new build request");
            //                     //     continue;
            //                     // }
            //                 }

            //                 let project = Project::new(code, None, None);
            //                 let request = BuildRequest {
            //                     id: project.id(),
            //                     project,
            //                     ws_msg_tx: build_tx.clone(),
            //                 };

            //                 current_build = Some(request);
            //             }
            //         }
            //         Either::Right((Some(build_msg), _)) => {
            //             // Send the build message.
            //             let socket_msg = match build_msg.clone() {
            //                 BuildMessage::Building(stage) => SocketMessage::BuildStage(stage),
            //                 BuildMessage::Finished(result) => SocketMessage::BuildFinished(result),
            //                 BuildMessage::QueuePosition(i) => SocketMessage::QueuePosition(i),
            //                 BuildMessage::CargoDiagnostic(diagnostic) => {
            //                     SocketMessage::BuildDiagnostic(diagnostic)
            //                 }
            //             };
            //             let socket_result = socket_tx.send(socket_msg.into_axum()).await;
            //             if socket_result.is_err() {
            //                 break;
            //             }

            //             // If the build finished, let's close this socket.
            //             if let BuildMessage::Finished(_) = build_msg {
            //                 current_build = None;
            //                 let _ = socket_tx.close().await;
            //                 break;
            //             }
            //         }
            //         _ => break,
            //     }
            // }

            // // The socket has closed. Make sure we cancel any active builds.
            // if let Some(request) = current_build {
            //     let result = state
            //         .build_queue_tx
            //         .send(BuildCommand::Stop { id: request.id });

            //     if result.is_err() {
            //         error!(build_id = ?request.id, "failed to send build stop signal for closed websocket");
            //     }
            // }

            // Drop the socket from our connected list.
            // TODO: Convert this to a drop guard.
            // let mut connected_sockets = state.connected_sockets.lock().await;
            // let index = connected_sockets.iter().position(|x| **x == ip);
            // if let Some(index) = index {
            //     connected_sockets.remove(index);
            // }
        })
    }

    /// Handle providing temporary built wasm assets.
    /// This should delete temporary projects after 30 seconds.
    async fn serve_built_index(
        State(state): State<AppState>,
        Path(build_id): Path<Uuid>,
    ) -> impl IntoResponse {
        let path = state.built_path.join(build_id.to_string());

        let index_path = path.join("index.html");
        let file = match tokio::fs::File::open(index_path.clone()).await {
            Ok(f) => f,
            Err(e) => {
                warn!(err = ?e, path = ?index_path, "failed to read built project:");
                return Err((StatusCode::NOT_FOUND, "not found"));
            }
        };

        Ok((
            [(header::CONTENT_TYPE, "text/html")],
            Body::from_stream(ReaderStream::new(file)),
        ))
    }

    async fn serve_other_built(
        State(state): State<AppState>,
        Path((build_id, file_path)): Path<(Uuid, PathBuf)>,
    ) -> impl IntoResponse {
        let path = state.built_path.join(build_id.to_string()).join(file_path);

        let file = match tokio::fs::File::open(path.clone()).await {
            Ok(f) => f,
            Err(e) => {
                warn!(err = ?e, path = ?path, "failed to read built project:");
                return Err((StatusCode::NOT_FOUND, "read failure"));
            }
        };

        let Some(file_ext) = path.extension() else {
            warn!(build_id = ?build_id, path = ?path, "failed to get file extension");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "read failure"));
        };

        let content_type = match file_ext.to_str() {
            Some("wasm") => "application/wasm",
            Some("js") => "application/javascript",
            Some(_) => {
                warn!(build_id = ?build_id, path = ?path, "project tried accessing denied file");
                return Err((StatusCode::NOT_FOUND, "not found"));
            }
            None => {
                warn!(build_id = ?build_id, path = ?path, "failed to get file extension");
                return Err((StatusCode::INTERNAL_SERVER_ERROR, "read failure"));
            }
        };

        Ok((
            [(header::CONTENT_TYPE, content_type)],
            Body::from_stream(ReaderStream::new(file)),
        ))
    }

    #[tracing::instrument(skip_all, level = "trace")]
    async fn get_shared_project(
        State(state): State<AppState>,
        Path(id): Path<String>,
    ) -> Result<Json<GetSharedProjectRes>, AppError> {
        let res = state
            .reqwest_client
            .get(format!("{GISTS_URL_PREFIX}/{id}"))
            .bearer_auth(&state.gist_auth_token)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header(header::USER_AGENT, GITHUB_USER_AGENT)
            .send()
            .await?;

        if res.status() == StatusCode::NOT_FOUND {
            return Err(AppError::ResourceNotFound);
        }

        let gist = res.json::<Gist>().await?;

        let Some(file) = gist.files.get(PRIMARY_GIST_FILE_NAME) else {
            return Err(AppError::ResourceNotFound);
        };

        Ok(Json(GetSharedProjectRes {
            id: gist.id,
            code: file.content.clone(),
        }))
    }

    #[tracing::instrument(skip_all, level = "trace")]
    async fn share_project(
        State(state): State<AppState>,
        Json(payload): Json<ShareProjectReq>,
    ) -> Result<Json<ShareProjectRes>, AppError> {
        let mut files = HashMap::new();

        files.insert(
            PRIMARY_GIST_FILE_NAME.to_string(),
            GistFile {
                content: payload.code,
            },
        );

        let new_gist = NewGist {
            description: "A user-saved Dioxus Playground snippet.".to_string(),
            public: true,
            files,
        };

        let new_gist = state
            .reqwest_client
            .post(GISTS_URL_PREFIX)
            .bearer_auth(&state.gist_auth_token)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header(header::USER_AGENT, GITHUB_USER_AGENT)
            .json(&new_gist)
            .send()
            .await?
            .json::<Gist>()
            .await?;

        Ok(Json(ShareProjectRes { id: new_gist.id }))
    }
}

// TODO: We need some way of cleaning up any stopped builds.
/// The builder provides a convenient interface for controlling builds running in another task.
pub struct Builder {
    template_path: PathBuf,
    built_path: PathBuf,
    is_building: Arc<AtomicBool>,
    current_build: Option<BuildRequest>,
    task: JoinHandle<Result<()>>,
}

impl Builder {
    fn new(
        build_template_path: PathBuf,
        built_path: PathBuf,
        is_building: Arc<AtomicBool>,
    ) -> Self {
        Self {
            template_path: build_template_path,
            built_path,
            is_building,
            current_build: None,
            task: tokio::spawn(std::future::pending()),
        }
    }

    /// Start a new build, cancelling any ongoing builds.
    fn start(&mut self, request: BuildRequest) {
        let _ = request.ws_msg_tx.send(BuildMessage::QueuePosition(0));

        self.stop_current();
        self.is_building.store(true, Ordering::SeqCst);
        self.current_build = Some(request.clone());
        self.task =
            tokio::spawn(request.build(self.template_path.clone(), self.built_path.clone()));
    }

    /// Stop the current build.
    fn stop_current(&mut self) {
        self.task.abort();
        self.task = tokio::spawn(std::future::pending());
        self.current_build = None;
        self.is_building.store(false, Ordering::SeqCst);
    }

    /// Wait for the current build to finish.
    async fn finished(&mut self) -> Result<BuildRequest> {
        // Ensure we don't poll a completed task.
        if self.task.is_finished() {
            self.stop_current();
        }

        // Make progress on the build task.
        let task = &mut self.task;
        task.await??;

        self.current_build.take().context("no current build")
    }

    /// Check if the builder has an ongoing build.
    fn has_build(&self) -> bool {
        self.current_build.is_some()
    }

    /// Get the current ongoing build if it exists.
    fn current_build(&self) -> Option<BuildRequest> {
        self.current_build.clone()
    }

    /// Start a build or add it to the queue.
    fn start_build(&mut self, pending_builds: &mut VecDeque<BuildRequest>, request: BuildRequest) {
        // If the builder has a build, add to queue, otherwise start the build.
        match self.has_build() {
            false => self.start(request),
            true => {
                let _ = request
                    .ws_msg_tx
                    .send(BuildMessage::QueuePosition(pending_builds.len() + 1));

                pending_builds.push_back(request);
            }
        };
    }

    /// Stop the current build by:
    /// - Checking if it's the current build and if so, stop it, update queue positions, and return early.
    /// - Iterate through queue looking for a matching id.
    ///   If matching id found, update queue positions *behind* matching queue and remove matched item.
    fn stop_build(&mut self, pending_builds: &mut VecDeque<BuildRequest>, id: Uuid) {
        // Check if the ongoing build is the cancelled build.
        let current_build_id = self.current_build().map(|b| b.id);
        if let Some(current_build_id) = current_build_id {
            if id == current_build_id {
                self.stop_current();

                // Start the next build request.
                let next_request = pending_builds.pop_front();
                if let Some(request) = next_request {
                    self.start(request);
                }

                Self::update_queue_positions(pending_builds);
                return;
            }
        }

        // Try finding the build in the queue
        let mut matching_id = None;

        for (i, build_request) in pending_builds.iter_mut().enumerate() {
            if build_request.id == id {
                matching_id = Some(i);
                continue;
            }

            // Tell any other requests behind the removed that they're moving up.
            if matching_id.is_some() {
                let _ = build_request
                    .ws_msg_tx
                    .send(BuildMessage::QueuePosition(i - 1));
            }
        }

        // Remove the stopped build.
        if let Some(id) = matching_id {
            pending_builds.remove(id);
        }
    }

    /// Handle a finished build by:
    /// - Finishing the current build, sending the BuildMessage::Finnished to the socket.
    /// - Start the next build.
    /// - Update queue positions.
    fn handle_finished_build(
        &mut self,
        pending_builds: &mut VecDeque<BuildRequest>,
        build_result: Result<BuildRequest>,
    ) {
        // Tell the socket the result of their build.
        let _ = match build_result {
            Ok(request) => {
                dioxus::logger::tracing::trace!(request = ?request, "build finished");
                request
                    .ws_msg_tx
                    .send(BuildMessage::Finished(Ok(request.id)))
            }
            Err(e) => {
                dioxus::logger::tracing::warn!(err = ?e, src = ?e.source(), "build failed");
                match self.current_build() {
                    Some(request) => request
                        .ws_msg_tx
                        .send(BuildMessage::Finished(Err(e.to_string()))),
                    None => Ok(()),
                }
            }
        };

        // Start the next build.
        let next_request = pending_builds.pop_front();
        if let Some(request) = next_request {
            self.start(request);
        }

        Self::update_queue_positions(pending_builds);
    }

    /// Iterate through the queue and alert each request with their current queue position.
    fn update_queue_positions(pending_builds: &mut VecDeque<BuildRequest>) {
        for (i, build_request) in pending_builds.iter_mut().enumerate() {
            let _ = build_request
                .ws_msg_tx
                .send(BuildMessage::QueuePosition(i + 1));
        }
    }
}

/// A build request which contains the id of the build, the code to be built, and a socket to send build updates.
#[derive(Debug, Clone)]
pub struct BuildRequest {
    pub id: Uuid,
    pub project: Project,
    pub ws_msg_tx: UnboundedSender<BuildMessage>,
}

impl BuildRequest {
    /// Run the steps to produce a build for a [`BuildRequest`]
    async fn build(self, template_path: PathBuf, built_path: PathBuf) -> Result<()> {
        // If the project already exists, don't build it again.
        if std::fs::exists(built_path.join(self.id.to_string())).unwrap_or_default() {
            tracing::trace!("Skipping build for {self:?} since it already exists");
            return Ok(());
        }

        self.setup_template(&template_path).await?;
        self.dx_build(&template_path).await?;
        self.move_to_built(&template_path, &built_path).await?;

        Ok(())
    }

    /// Resets the template with values for the next build.
    async fn setup_template(&self, template_path: &std::path::Path) -> Result<()> {
        const BUILD_ID_ID: &str = "{BUILD_ID}";

        let snippets_from_copy = [
            template_path.join("snippets/Cargo.toml"),
            template_path.join("snippets/Dioxus.toml"),
        ];

        // New locations
        let snippets_to_copy = [
            template_path.join("Cargo.toml"),
            template_path.join("Dioxus.toml"),
        ];

        // Enumerate over a list of paths to copy and copies them to the new location while modifying any template strings.
        for (i, path) in snippets_from_copy.iter().enumerate() {
            let new_path = &snippets_to_copy[i];
            let contents = fs::read_to_string(path).await?;
            let contents = contents.replace(BUILD_ID_ID, &self.id.to_string());
            fs::write(new_path, contents).await?;
        }

        // Write the user's code to main.rs
        fs::write(template_path.join("src/main.rs"), self.project.contents()).await?;

        Ok(())
    }

    /// Run the build command provided by the DX CLI.
    /// Returns if DX built the project successfully.
    async fn dx_build(&self, template_path: &PathBuf) -> Result<()> {
        let mut child = Command::new("dx")
            .arg("build")
            .arg("--platform")
            .arg("web")
            .arg("--json-output")
            .arg("--verbose")
            .arg("--trace")
            .current_dir(template_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().expect("dx stdout should exist");
        let mut stdout_reader = BufReader::new(stdout).lines();

        let mut logs = Vec::new();

        loop {
            select! {
                // Read stdout lines from DX.
                result = stdout_reader.next_line() => {
                    let Ok(Some(line)) = result else {
                        continue;
                    };

                    logs.push(line.clone());
                    self.process_dx_message(line);
                }
                // Wait for the DX process to exit.
                status = child.wait() => {
                    // Check if the build was successful.
                    let exit_code = status.map(|c| c.code());
                    if let Ok(Some(code)) = exit_code {
                        if code == 0 {
                            break;
                        } else {
                            // Dump logs in debug.
                            for log in logs {
                                debug!("{log}");
                            }

                            bail!("DX build failed: {code}");
                        }
                    }
                    bail!("DX build failed");
                }
            }
        }

        Ok(())
    }

    /// Process a JSON-formatted message from the DX CLI, returning nothing on error.
    ///
    /// We don't care if this errors as it is human-readable output which the playground doesn't depend on for build status.
    fn process_dx_message(&self, message: String) {
        /// The DX CLI serves parseable JSON output with the regular tracing message and a parseable "json" field.
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub struct CliMessage {
            json: Option<String>,
        }

        // We parse the tracing json log and if it contains a json field, we parse that as StructuredOutput.
        let result = serde_json::from_str::<CliMessage>(&message)
            .ok()
            .and_then(|v| v.json)
            .and_then(|json| serde_json::from_str::<StructuredOutput>(&json).ok());

        let Some(output) = result else {
            return;
        };

        let _ = match output {
            StructuredOutput::BuildUpdate { stage } => {
                let stage = BuildStage::from(stage);
                self.ws_msg_tx.send(BuildMessage::Building(stage))
            }
            StructuredOutput::CargoOutput { message } => {
                let Ok(diagnostic) = CargoDiagnostic::try_from(message) else {
                    return;
                };

                // Don't send any diagnostics for dependencies.
                if diagnostic.target_crate != format!("play-{}", self.id) {
                    return;
                }

                self.ws_msg_tx
                    .send(BuildMessage::CargoDiagnostic(diagnostic))
            }
            _ => Ok(()),
        };
    }

    /// Moves the project built by `dx` to the final location for serving.
    async fn move_to_built(
        &self,
        template_path: &std::path::Path,
        built_path: &std::path::Path,
    ) -> Result<()> {
        let id_string = self.id.to_string();

        // The path to the built project from DX
        let play_build_id = format!("play-{}", &id_string);
        let built_project_parent = template_path.join("target/dx").join(play_build_id);

        // The public folder of the built project (what we want).
        let debug_web = built_project_parent.join("debug/web/");
        let public_folder = debug_web.join("public");

        let built_path = built_path.to_path_buf();

        // Rename the built project public folder to the build id.
        // Move the built project to the built projects folder to be served.
        // Delete the built project in the target directory to prevent a storage leak.
        // We use `spawn_blocking` to batch call `std::fs` as recommended by Tokio.
        tokio::task::spawn_blocking::<_, Result<()>>(move || {
            // Rename to be the build id
            let built_project = debug_web.join(&id_string);
            std::fs::rename(&public_folder, &built_project)?;

            // Copy to the built project folder for serving.
            let options = CopyOptions::new().overwrite(true);
            fs_extra::dir::move_dir(&built_project, &built_path, &options)?;
            std::fs::remove_dir_all(&built_project_parent)?;
            Ok(())
        })
        .await??;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gist {
    pub id: String,
    pub files: HashMap<String, GistFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GistFile {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewGist {
    pub description: String,
    pub public: bool,
    pub files: HashMap<String, GistFile>,
}

/// A message from the playground build process.
#[derive(Debug, Clone, PartialEq)]
pub enum BuildMessage {
    Building(model::BuildStage),
    CargoDiagnostic(CargoDiagnostic),
    Finished(Result<Uuid, String>),
    QueuePosition(usize),
}
