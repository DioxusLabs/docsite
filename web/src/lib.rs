use crate::components::Tab;
use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use error::AppError;

mod bindings;
mod components;
mod error;
mod examples;
mod ws;

const _: &str = asset!("/public/dxp.css");

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

    let mut build_signals = BuildSignals {
        is_compiling,
        built_page_id,
        compiler_messages,
        queue_position,
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

    // Logic for pane resizing
    let pane_left_width: Signal<Option<i32>> = use_signal(|| None);
    let pane_right_width: Signal<Option<i32>> = use_signal(|| None);

    rsx! {
        script { src: "./monaco-editor-0.52/vs/loader.js", onload: move |_| bindings::monaco::init("dxp-panes-left", examples::SNIPPETS[0].1) }

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
