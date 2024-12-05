use crate::components::Tab;
use base64::{prelude::BASE64_URL_SAFE, Engine};
use bindings::monaco;
use build::{start_build, BuildStage, BuildState};
use components::material_icons::Warning;
use dioxus::prelude::*;
use dioxus_devtools::HotReloadMsg;
use dioxus_document::{eval, Link};
use dioxus_logger::tracing::error;
use dioxus_sdk::{
    theme::{use_system_theme, SystemTheme},
    utils::timing::use_debounce,
};
use error::ShareError;
use hotreload::{HotReload, HotReloadError};
use std::time::Duration;

mod bindings;
mod build;
mod components;
mod error;
mod examples;
mod hotreload;
mod ws;

const DXP_CSS: Asset = asset!("/assets/dxp.css");
const MONACO_FOLDER: Asset = asset!("/assets/monaco-editor-0.52");

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
    let mut build = provide_context(BuildState::new());
    let mut current_tab = use_signal(|| Tab::Page);
    let mut show_share_warning = use_signal(|| false);

    let mut hot_reload = use_signal(|| HotReload::new());
    let on_model_changed = use_debounce(Duration::from_millis(150), move |new_code: String| {
        if !build.stage().is_finished() {
            return;
        }

        // Process any potential hot -eloadable changes and send them to the iframe web client.
        let result = hot_reload.write().process_file_change(new_code);
        match result {
            Ok(templates) => {
                let hr_msg = HotReloadMsg {
                    templates,
                    assets: Vec::new(),
                    unknown_files: Vec::new(),
                };

                let e = eval(
                    r#"
                    const hrMsg = await dioxus.recv();
                    const iframeElem = document.getElementById("dxp-viewport");
                    const hrMsgJson = JSON.stringify(hrMsg);
                    
                    if (iframeElem) {
                        iframeElem.contentWindow.postMessage(hrMsgJson, "*");
                    }
                    "#,
                );
                _ = e.send(hr_msg);
            }
            Err(HotReloadError::NeedsRebuild) => error!("todo"),
            e => error!("hot reload error occured: {:?}", e),
        }
    });

    // We store the shared code in state as the editor may not be initialized yet.
    let shared_code = use_memo(use_reactive((&share_code,), move |(share_code,)| {
        let share_code = share_code?;
        let decoded = decode_share_link(&share_code).ok()?;

        show_share_warning.set(true);

        // If monaco is initialized, set it now. Otherwise save it for monaco onload code.
        if monaco::is_ready() {
            monaco::set_current_model_value(&decoded);
            hot_reload.write().set_starting_code(&decoded);
            return None;
        }

        Some(decoded)
    }));

    let monaco_vs_prefix = format!("{}/vs", MONACO_FOLDER);
    let monaco_vs_prefix_c = monaco_vs_prefix.clone();

    let system_theme = use_system_theme();
    use_effect(move || {
        let theme = system_theme().unwrap_or(SystemTheme::Light);
        bindings::monaco::set_theme(theme);
    });

    // Load either the shared code or the first snippet once monaco script is ready.
    let on_monaco_load = move |_| {
        let system_theme = system_theme().unwrap_or(SystemTheme::Light);
        let snippet = match shared_code() {
            Some(c) => c,
            None => examples::SNIPPETS[0].1.to_string(),
        };

        bindings::monaco::init(
            &monaco_vs_prefix_c,
            "dxp-panes-left",
            system_theme,
            &snippet,
        );
        hot_reload.write().set_starting_code(&snippet);
        bindings::monaco::register_model_change(on_model_changed);
    };

    // Send a request to compile code.
    let on_run = move |_| {
        if build.stage().is_running() {
            return;
        }
        
        build.reset();
        build.set_stage(BuildStage::Starting);

        let socket_url = urls.socket.to_string();
        spawn(async move {
            if let Err(e) = start_build(&mut build, socket_url).await {
                error!(error = ?e, "failed to build project");
            }
        });
    };

    // Build full url to built page.
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
        script { src: "{monaco_vs_prefix}/loader.js", onload: on_monaco_load }

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
            pane_left_width,
            pane_right_width,
            urls,
            on_run,
        }
        components::Panes {
            pane_left_width,
            pane_right_width,
            built_page_url,
        }
    }
}

/// Copy a share link to the clipboard.
///
/// This will:
/// 1. Get the current code from the editor.
/// 2. Compress it using `miniz_oxide`.
/// 3. Encodes it in url-safe base64.
/// 4. Formats the code with the provided `location` url prefix.
/// 5. Copies the link to the clipboard.
///
/// This allows users to have primitve serverless sharing.
/// Links will be large and ugly but it works.
fn copy_share_link(location: &str) {
    let code = monaco::get_current_model_value();
    let compressed = miniz_oxide::deflate::compress_to_vec(code.as_bytes(), 10);

    let mut encoded = String::new();
    BASE64_URL_SAFE.encode_string(compressed, &mut encoded);

    let formatted = format!("{}/{}", location, encoded);
    let e = eval(
        r#"
        const data = await dioxus.recv();
        navigator.clipboard.writeText(data);
        "#,
    );

    let _ = e.send(formatted);
}

/// Decode the share code into code.
fn decode_share_link(share_code: &str) -> Result<String, ShareError> {
    let bytes = BASE64_URL_SAFE.decode(share_code)?;
    let decoded_bytes = miniz_oxide::inflate::decompress_to_vec(&bytes)?;
    let decoded = String::from_utf8(decoded_bytes)?;
    Ok(decoded)
}
