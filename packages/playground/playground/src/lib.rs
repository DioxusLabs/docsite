use build::{start_build, BuildState};
use components::icons::Warning;
use dioxus::logger::tracing::error;
use dioxus::prelude::*;
use dioxus_document::Link;
use dioxus_sdk::time::use_debounce;
use editor::monaco::{self, monaco_loader_src, set_monaco_markers};
use hotreload::{attempt_hot_reload, HotReload};
use model::{api::ApiClient, AppError, Project, SocketError};
use std::time::Duration;

use dioxus_sdk::window::theme::{use_system_theme, Theme};

use crate::{
    build::{BuildStateStoreExt, BuildStateStoreImplExt},
    hotreload::HotReloadStoreImplExt,
};

mod build;
mod components;
mod dx_components;
mod editor;
mod hotreload;
mod share_code;
mod ws;

const DXP_CSS: Asset = asset!("/assets/dxp.css");
const MONACO_FOLDER: Asset = asset!("/assets/monaco-editor-0.52.2");
const DX_COMPONENTS_CSS: Asset = asset!("/assets/dx-components-theme.css");

/// The URLS that the playground should use for locating resources and services.
#[derive(Debug, Clone, PartialEq)]
pub struct PlaygroundUrls {
    /// The URL to the websocket server.
    pub socket: &'static str,
    /// The URL to the built project files from the server.
    pub server: &'static str,
    /// The url location of the playground UI: e.g. `https://dioxuslabs.com/play`
    pub location: &'static str,
}

#[component]
pub fn Playground(
    urls: PlaygroundUrls,
    share_code: ReadSignal<Option<String>>,
    class: Option<String>,
) -> Element {
    let mut hot_reload = use_context_provider(|| Store::new(HotReload::new()));
    let api_client = use_context_provider(|| Signal::new(ApiClient::new(urls.server)));
    let mut errors = use_context_provider(|| Store::new(Errors::new()));

    let monaco_ready = use_signal(|| false);
    let mut show_share_warning = use_signal(|| false);

    // Default to the welcome project.
    // Project dirty determines whether the Rust-project is synced with the project in the editor.
    let mut project = use_context_provider(|| Signal::new(example_projects::get_welcome_project()));
    let mut build = use_context_provider(|| Store::new(BuildState::new(&project.read())));
    let mut project_dirty = use_signal(|| false);
    use_effect(move || {
        if project_dirty() && monaco_ready() {
            let project = project.read();
            monaco::set_current_model_value(&project.contents());
            project_dirty.set(false);
        }
    });

    // Get the shared project if a share code was provided.
    use_effect(move || {
        if let Some(share_code) = share_code() {
            spawn(async move {
                let api_client = api_client();
                let shared_project = Project::from_share_code(&api_client, share_code).await;
                if let Ok(shared_project) = shared_project {
                    show_share_warning.set(true);
                    project_dirty.set(true);
                    project.set(shared_project);
                    build.reset();
                }
            });
        }
    });

    // Handle events when code changes.
    let mut on_model_changed = use_debounce(Duration::from_millis(250), move |new_code: String| {
        // Update the project
        project.write().set_contents(new_code.clone());
        spawn(async move {
            editor::monaco::set_markers(&[]);

            if build.get_stage().is_finished() {
                attempt_hot_reload(hot_reload, &new_code);
            }
        });
    });

    let on_model_changed = use_callback(move |args| on_model_changed.action(args));

    // Handle setting diagnostics based on build state.
    use_effect(move || set_monaco_markers(build.diagnostics()));

    // Themes
    let system_theme = use_system_theme();
    use_effect(move || {
        editor::monaco::set_theme(system_theme().unwrap_or(Theme::Light));
    });

    // Handle starting a build.
    let on_rebuild = use_callback(move |_| {
        spawn(async move {
            if build.get_stage().is_running() || !monaco_ready() {
                return;
            }

            // Update hot reload
            let code = editor::monaco::get_current_model_value();

            let socket_url = urls.socket.to_string();
            match start_build(build, socket_url, code.clone()).await {
                Ok(success) => {
                    if success {
                        hot_reload.set_starting_code(&code);
                    }
                    hot_reload.set_needs_rebuild(!success)
                }
                Err(error) => errors.push_from_app_error(error),
            }
        });
    });

    // Construct the full URL to the built project.
    let built_page_url = use_memo(move || {
        let project = project.read();
        let prebuilt_id = project.prebuilt.then_some(project.id());
        let local_id = build.get_stage().finished_id();
        let id = local_id.or(prebuilt_id)?;
        Some(format!("{}/built/{}", urls.server, id))
    });

    // State for pane resizing, shared by headers and panes.
    // The actual logic is in the panes component.
    let mut pane_left_width: Signal<Option<i32>> = use_signal(|| None);
    let mut pane_right_width: Signal<Option<i32>> = use_signal(|| None);

    // Show the example list
    let show_examples = use_signal(|| true);
    use_effect(move || {
        let _show_examples = show_examples();
        pane_left_width.set(None);
        pane_right_width.set(None);
    });

    rsx! {
        div { class, id: "dxp-playground-root",
            // Head elements
            Link { rel: "stylesheet", href: DXP_CSS }
            Link { rel: "stylesheet", href: DX_COMPONENTS_CSS }

            // Monaco script
            script {
                src: monaco_loader_src(MONACO_FOLDER),
                onload: move |_| {
                    monaco::on_monaco_load(
                        MONACO_FOLDER,
                        system_theme().unwrap_or(Theme::Light),
                        &project.read().contents(),
                        hot_reload,
                        monaco_ready,
                        on_model_changed,
                        on_rebuild
                    );
                },
            }

            // Share warning
            components::Modal {
                on_ok: move |_| show_share_warning.set(false),
                open: show_share_warning(),
                if show_share_warning() {
                    components::ModalContent {
                        icon: rsx! {
                            Warning {}
                        },
                        title: "Do you trust this code?",
                        text: "Anyone can share their project. Verify that nothing malicious has been included before running this project.",
                        ok_text: "I understand",
                    }
                }
            }

            // Show errors one at a time.
            components::Modal {
                on_ok: move |_| {
                    errors.pop();
                },
                open: !errors.errors().is_empty(),
                if let Some((title, text)) = errors.last() {
                    components::ModalContent {
                        icon: rsx! {
                            Warning {}
                        },
                        title,
                        text,
                    }
                },
            }

            // Playground UI
            components::Header {
                urls,
                on_rebuild,
                show_examples,
                pane_left_width,
                pane_right_width,
                file_name: project.read().path.clone(),
            }
            div { id: "dxp-lower-half",
                components::Panes {
                    pane_left_width,
                    pane_right_width,
                    built_page_url,
                }
            }

        }
    }
}

/// A helper type for gracefully handling app errors and logging them.
#[derive(Clone, Store)]
pub struct Errors {
    errors: Vec<(String, String)>,
}

impl Errors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
}

#[store(pub)]
impl Store<Errors> {
    fn push_error(&mut self, error: (impl ToString, impl ToString)) {
        let error = (error.0.to_string(), error.1.to_string());
        error!(?error, "an error occured and was handled gracefully");
        self.errors().push(error);
    }

    fn push_from_app_error(&mut self, app_error: AppError) {
        let error = match app_error {
            AppError::Parse(error) => ("Parse Error", error.to_string()),
            AppError::Request(error) => ("Request Error", error.to_string()),
            AppError::ResourceNotFound => (
                "Resource Not Found",
                "A requested resource was not found.".to_string(),
            ),
            AppError::Socket(error) => (
                "Socket Error",
                match error {
                    SocketError::ParseJson(error) => error.to_string(),
                    SocketError::Utf8Decode(_) => "UTF-8 decode failed".to_string(),
                    SocketError::Gloo(web_socket_error) => web_socket_error.to_string(),
                    e => e.to_string(),
                },
            ),
            AppError::Js(error) => ("JS Error", error.to_string()),
            _ => return,
        };

        self.push_error(error);
    }

    fn first(&self) -> Option<(String, String)> {
        self.errors().first().map(|x| x.clone())
    }

    fn last(&self) -> Option<(String, String)> {
        self.errors().last().map(|x| x.clone())
    }

    fn pop(&mut self) -> Option<(String, String)> {
        self.errors().pop()
    }
}

impl Default for Errors {
    fn default() -> Self {
        Self::new()
    }
}
