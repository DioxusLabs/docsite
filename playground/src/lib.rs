use build::{start_build, BuildStage, BuildState};
use components::icons::Warning;
use dioxus::logger::tracing::error;
use dioxus::prelude::*;
use dioxus_document::Link;
use dioxus_sdk::{
    theme::{use_system_theme, SystemTheme},
    utils::timing::use_debounce,
};
use editor::monaco::{self, monaco_loader_src, set_monaco_markers};
use example_projects::ExampleProject;
use hotreload::{attempt_hot_reload, HotReload};
// use snippets::use_provide_selected_example;
use std::time::Duration;

mod build;
mod components;
mod editor;
mod error;
mod hotreload;
mod share_code;
mod ws;

const DXP_CSS: Asset = asset!("/assets/dxp.css");
const MONACO_FOLDER: Asset = asset!("/assets/monaco-editor-0.52.2");

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
pub fn Playground(
    urls: PlaygroundUrls,
    share_code: ReadOnlySignal<Option<String>>,
    class: Option<String>,
) -> Element {
    let mut project = use_signal(|| {
        if let Some(code) = share_code() {
            ExampleProject::from_compressed_base64(code)
                .unwrap_or_else(|_| example_projects::get_welcome_project())
        } else {
            example_projects::get_welcome_project()
        }
    });

    let mut build = use_context_provider(|| BuildState::new(project));
    let mut hot_reload = use_context_provider(HotReload::new);

    // We store the shared code in state as the editor may not be initialized yet.
    let mut show_share_warning = use_signal(|| false);

    // Handle events when code changes.
    let on_model_changed = use_debounce(Duration::from_millis(250), move |new_code: String| {
        editor::monaco::set_markers(&[]);

        if build.stage().is_finished() {
            attempt_hot_reload(hot_reload, &new_code);
        }
    });

    // Handle setting diagnostics based on build state.
    use_effect(move || set_monaco_markers(build.diagnostics()));

    // Themes
    let system_theme = use_system_theme();
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        editor::monaco::set_theme(system_theme().unwrap_or(SystemTheme::Light));
    });

    // The current contents of the editor.
    let mut custom_contents = use_signal(|| None);
    let contents = use_memo(move || {
        let default = project().contents;
        custom_contents().unwrap_or(default)
    });

    // Handle starting a build.
    let on_rebuild = move |_| async move {
        if build.stage().is_running() {
            return;
        }
        hot_reload.set_needs_rebuild(false);

        // Update hot reload
        let code = editor::monaco::get_current_model_value();
        custom_contents.set(Some(code.clone()));

        let socket_url = urls.socket.to_string();
        match start_build(build, socket_url, code).await {
            Ok(success) => hot_reload.set_needs_rebuild(!success),
            Err(e) => error!(error = ?e, "failed to build project"),
        }
    };

    // Construct the full URL to the built project.
    let built_page_url = use_memo(move || {
        let prebuilt_id = project.read().prebuilt.then_some(project.read().id());
        let local_id = build.stage().finished_id();
        let id = local_id.or(prebuilt_id)?;
        Some(format!("{}{}", urls.built, id))
    });

    // State for pane resizing, shared by headers and panes.
    // The actual logic is in the panes component.
    let pane_left_width: Signal<Option<i32>> = use_signal(|| None);
    let pane_right_width: Signal<Option<i32>> = use_signal(|| None);

    // Show the example list
    let show_examples = use_signal(|| true);

    rsx! {
        div { class, id: "dxp-playground-root",
            // Head elements
            Link { rel: "stylesheet", href: DXP_CSS }
            // Monaco script
            script {
                src: monaco_loader_src(MONACO_FOLDER),
                onload: move |_| {
                    #[cfg(target_arch = "wasm32")]
                    monaco::on_monaco_load(
                        MONACO_FOLDER,
                        system_theme().unwrap_or(SystemTheme::Light),
                        &contents(),
                        hot_reload,
                        on_model_changed,
                    );
                },
            }
            // Share warning
            if show_share_warning() {
                components::Modal {
                    icon: rsx! {
                        Warning {}
                    },
                    title: "Do you trust this code?",
                    text: "Anyone can share their project. Verify that nothing malicious has been included before running this project.",
                    on_ok: move |_| show_share_warning.set(false),
                }
            }
            // Playground UI
            components::Header {
                urls,
                on_rebuild,
                show_examples,
                pane_left_width,
                pane_right_width,
                example_name: project.read().path.clone(),
            }
            div { id: "dxp-lower-half",
                div {
                    id: "dxp-examples-list",
                    class: if show_examples() { "dxp-open" } else { "" },
                    for example in example_projects::get_example_projects().iter() {
                        button {
                            class: "dxp-example-project",
                            onclick: move |_| {
                                custom_contents.set(None);
                                build.set_stage(BuildStage::Finished(Ok(example.id())));
                                project.set(example.clone());
                                monaco::set_current_model_value(&example.contents);
                                hot_reload.set_starting_code(&example.contents);
                            },
                            h3 { {example.path.clone()} }
                            p { {example.description.clone()} }
                        }
                    }
                }
                components::Panes {
                    pane_left_width,
                    pane_right_width,
                    built_page_url,
                }
            }
        }
    }
}
