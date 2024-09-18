use crate::components::{Header, RightPane, Tab};
use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use error::AppError;

mod components;
mod error;
mod ws;

const _: &str = asset!("/public/dxp.css");
const SNIPPET_WELCOME: &str = include_str!("snippets/welcome.rs");

#[component]
pub fn Playground(socket_url: String, built_url: String) -> Element {
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

    // Once the element has mounted, startup `ace` editor.
    let on_editor_mount = move |_| {
        let code = format!(
            r#"
            require.config({{ paths: {{ vs: './monaco-editor/vs' }} }});

			require(['vs/editor/editor.main'], function () {{
                const media = window.matchMedia("(prefers-color-scheme: dark");
                let currentTheme = "vs";
                if (media.matches) {{
                    currentTheme = "vs-dark";
                }}

				var editor = monaco.editor.create(document.getElementById('dxp-editor'), {{
					value: `{SNIPPET_WELCOME}`,
					language: 'rust',
                    automaticLayout: true,
                    theme: currentTheme,
                }});


                // Add theme logic
                media.addEventListener("change", () => {{
                    if (media.matches) {{
                        monaco.editor.setTheme("vs-dark");
                    }} else {{
                        monaco.editor.setTheme("vs");
                    }}
                }});
            }});
            "#
        );
        eval(&code);
    };

    // Send a request to compile code.
    let on_run = move |_| {
        if is_compiling() {
            return;
        }

        reset_signals(&mut build_signals);
        is_compiling.set(true);
        compiler_messages.push("Starting build...".to_string());

        let socket_url = socket_url.clone();

        spawn(async move {
            if let Err(e) = start_build(&mut build_signals, socket_url).await {
                error!(error = ?e, "failed to build project");
                reset_signals(&mut build_signals);
                compiler_messages.push("Failed to build project.".to_string());
            }
        });
    };

    // Build full url to built page.
    let built_page_url = use_memo(move || built_page_id().map(|id| format!("{}{}", built_url, id)));

    rsx! {
        script { src: "./monaco-editor/vs/loader.js", onload: on_editor_mount, }
        div { id: "dxp-pane-container",
            div { id: "dxp-left-pane",
                Header {
                    is_compiling: is_compiling(),
                    queue_position: queue_position(),
                    on_run,
                }
                div { id: "dxp-editor" }
            }

            RightPane {
                current_tab,
                compiler_messages,
                built_page_url: built_page_url(),
            }
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
    let mut eval = eval(
        r#"
        let text = window.editorGlobal.getValue();
        dioxus.send(text);
        "#,
    );

    // Decode eval
    let val = eval.recv().await?;
    let val = val
        .as_str()
        .ok_or(AppError::JsError("eval didn't provide str".into()))?
        .to_string();

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
