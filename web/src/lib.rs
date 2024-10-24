use std::time::Duration;

use crate::components::Tab;
use base64::{prelude::BASE64_URL_SAFE, Engine};
use bindings::monaco;
use components::material_icons::Warning;
use dioxus::prelude::*;
use dioxus_document::{eval, Link};
use dioxus_logger::tracing::{error, info};
use dioxus_sdk::{
    theme::{use_system_theme, SystemTheme},
    utils::timing::use_debounce,
};
use error::AppError;
use hotreload::HotReload;

mod bindings;
mod components;
mod error;
mod examples;
mod hotreload;
mod ws;

const DXP_CSS: &str = asset!("/assets/dxp.css");
const MONACO_FOLDER: &str = "/monaco-editor-0.52"; //asset!(folder("/assets/monaco-editor-0.52"));

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
    let mut is_compiling = use_signal(|| false);
    let queue_position = use_signal(|| None);
    let built_page_id = use_signal(|| None);
    let mut compiler_messages = use_signal(Vec::<String>::new);
    let mut current_tab = use_signal(|| Tab::Page);
    let mut show_share_warning = use_signal(|| false);

    let mut build_signals = BuildSignals {
        is_compiling,
        built_page_id,
        compiler_messages,
        queue_position,
    };

    let mut hot_reload = use_signal(|| HotReload::new());
    let on_model_changed = use_debounce(Duration::from_millis(150), move |new_code: String| {
        let result = hot_reload.write().process_file_change(new_code);
        info!("hr result: {:?}", result);
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

    // Change tab automatically
    use_memo(move || {
        if built_page_id().is_none() {
            current_tab.set(Tab::Logs);
        } else {
            current_tab.set(Tab::Page);
        }
    });

    // Send a request to compile code.
    let on_run = move |_| {
        if is_compiling() {
            return;
        }

        reset_signals(&mut build_signals);
        is_compiling.set(true);
        compiler_messages.push("Starting build...".to_string());

        let socket_url = urls.socket.to_string();

        spawn(async move {
            if let Err(e) = start_build(&mut build_signals, socket_url).await {
                error!(error = ?e, "failed to build project");
                reset_signals(&mut build_signals);
                compiler_messages.push("Failed to build project.".to_string());
            }
        });
    };

    // Build full url to built page.
    let built_page_url =
        use_memo(move || built_page_id().map(|id| format!("{}{}", urls.built, id)));

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
        }
    }
}

/// A helper struct for passing around common build signals.
#[derive(Clone, Copy)]
pub(crate) struct BuildSignals {
    pub is_compiling: Signal<bool>,
    pub built_page_id: Signal<Option<String>>,
    pub compiler_messages: Signal<Vec<String>>,
    pub queue_position: Signal<Option<u32>>,
}

/// Start a build and handle updating the build signals according to socket messages.
async fn start_build(signals: &mut BuildSignals, socket_url: String) -> Result<(), AppError> {
    let val = bindings::monaco::get_current_model_value();

    // Send socket compile request
    let mut socket = ws::Socket::new(&socket_url)?;
    socket.compile(val).await?;

    // Handle socket messages
    loop {
        let msg = socket.next().await;
        match msg {
            Some(Ok(msg)) => {
                let is_done = ws::handle_message(signals, msg);
                if is_done {
                    break;
                }
            }
            Some(Err(e)) => return Err(e),
            None => {}
        }
    }
    socket.close().await;
    Ok(())
}

/// Reset build signals.
fn reset_signals(signals: &mut BuildSignals) {
    signals.is_compiling.set(false);
    signals.queue_position.set(None);
    signals.built_page_id.set(None);
    signals.compiler_messages.clear();
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
fn decode_share_link(share_code: &str) -> Result<String, AppError> {
    let bytes = BASE64_URL_SAFE
        .decode(share_code)
        .map_err(|_| AppError::ShareCodeDecoding)?;
    let decoded_bytes =
        miniz_oxide::inflate::decompress_to_vec(&bytes).map_err(|_| AppError::ShareCodeDecoding)?;
    let decoded = String::from_utf8(decoded_bytes).map_err(|_| AppError::ShareCodeDecoding)?;
    Ok(decoded)
}
