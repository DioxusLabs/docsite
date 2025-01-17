use build::{start_build, BuildState};
use components::icons::Warning;
use dioxus::logger::tracing::error;
use dioxus::prelude::*;
use dioxus_document::Link;
use dioxus_sdk::{
    theme::{use_system_theme, SystemTheme},
    utils::timing::use_debounce,
};
use editor::monaco::{monaco_loader_src, on_monaco_load, use_monaco_markers};
use hotreload::{attempt_hot_reload, HotReload};
use share_code::use_share_code;
use snippets::{use_provide_selected_example, SelectedExample};
use std::time::Duration;

mod build;
mod components;
mod editor;
mod error;
mod hotreload;
mod share_code;
mod snippets;
mod ws;

const DXP_CSS: Asset = asset!("/assets/dxp.css");
const MONACO_FOLDER: Asset = asset!("/assets/monaco-editor-0.52.2");

/// Include the template main.rs to get the extra lines for things like hot reload.
const TEMPLATE_MAIN_RS: &str = include_str!("../../server/template/snippets/main.rs");
const EXTRA_LINE_COUNT: GlobalSignal<usize> =
    GlobalSignal::new(|| TEMPLATE_MAIN_RS.lines().count() - 1);

/// The URLS that the playground should use for locating resources and services.
#[derive(Debug, Clone, PartialEq)]
pub struct PlaygroundUrls {
    /// The URL to the websocket server.
    pub socket: &'static str,
    /// The URL to the built project files from the server.
    pub built: &'static str,
    /// The url location of the playground UI: e.g. `https://dioxuslabs.com/play`
    pub location: &'static str,
}

#[component]
pub fn Playground(urls: PlaygroundUrls, share_code: Option<String>) -> Element {
    let build = use_context_provider(BuildState::new);
    let mut hot_reload = use_context_provider(HotReload::new);
    let mut selected_example = use_provide_selected_example(build, hot_reload);

    // We store the shared code in state as the editor may not be initialized yet.
    let mut show_share_warning = use_signal(|| false);
    let shared_code = use_share_code(share_code, show_share_warning, hot_reload);

    // Handle events when code changes.
    let on_model_changed = use_debounce(Duration::from_millis(250), move |new_code: String| {
        editor::monaco::set_markers(&[]);

        if build.stage().is_finished() && !selected_example().is_some() {
            attempt_hot_reload(hot_reload, &new_code);
        }
    });

    // Handle setting diagnostics based on build state.
    use_monaco_markers(build.diagnostics());

    // Themes
    let system_theme = use_system_theme();
    use_effect(move || {
        let theme = system_theme().unwrap_or(SystemTheme::Light);
        editor::monaco::set_theme(theme);
    });

    // Handle starting a build.
    let on_run = move |_| {
        if build.stage().is_running() || !hot_reload.needs_rebuild() {
            return;
        }
        hot_reload.set_needs_rebuild(false);
        selected_example.set(SelectedExample::None);

        // Update hot reload
        let code = editor::monaco::get_current_model_value();
        hot_reload.set_starting_code(&code);

        let socket_url = urls.socket.to_string();
        spawn(async move {
            match start_build(build, socket_url, code).await {
                Ok(success) => hot_reload.set_needs_rebuild(!success),
                Err(e) => error!(error = ?e, "failed to build project"),
            }
        });
    };

    // Construct the full URL to the built project.
    let built_page_url = use_memo(move || {
        let id = build.stage().finished_id();
        id.map(|id| format!("{}{}", urls.built, id))
    });

    // State for pane resizing, shared by headers and panes.
    // The actual logic is in the panes component.
    let pane_left_width: Signal<Option<i32>> = use_signal(|| None);
    let pane_right_width: Signal<Option<i32>> = use_signal(|| None);

    rsx! {
        // Head elements
        Link { rel: "stylesheet", href: DXP_CSS }

        // Monaco script
        script {
            src: monaco_loader_src(MONACO_FOLDER),
            onload: move |_| {
                on_monaco_load(
                    MONACO_FOLDER,
                    system_theme().unwrap_or(SystemTheme::Light),
                    shared_code(),
                    selected_example,
                    hot_reload,
                    on_model_changed
                );
            }
        }

        // Share warning
        if show_share_warning() {
            components::Modal {
                icon: rsx! { Warning {} },
                title: "Do you trust this code?",
                text: "Anyone can share their project. Verify that nothing malicious has been included before running this project.",
                on_ok: move |_| show_share_warning.set(false),
            }
        }

        // Playground UI
        components::Header {
            urls,
            on_run,
            pane_left_width,
            pane_right_width,
        }
        components::Panes {
            pane_left_width,
            pane_right_width,
            built_page_url,
        }
    }
}
