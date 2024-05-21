use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use model::*;

use crate::components::{Header, RightPane};

mod components;
mod ws;

const BUILT_URI: &str = "built/";
const SOCKET_URI: &str = "ws://localhost:3000/ws";
const SNIPPET_WELCOME: &str = include_str!("snippets/welcome.rs");

fn main() {
    dioxus_logger::init(Level::WARN).expect("failed to start logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut is_compiling = use_signal(|| false);
    let built_src = use_signal(|| None);

    let socket_tx = ws::start_socket(is_compiling, built_src);

    // Once the element has mounted, startup `ace` editor.
    let on_editor_mount = move |_| async move {
        let code = format!(
            r#"
            let editor = ace.edit("editor");
            editor.setTheme("ace/theme/github");

            let RustMode = ace.require("ace/mode/rust").Mode;
            editor.session.setMode(new RustMode());

            editor.setValue(`{SNIPPET_WELCOME}`, -1);

            // Set a global so other evals can acces it.
            window.editorGlobal = editor;
            "#
        );
        eval(&code);
    };

    // Send a request to compile code.
    let on_run = move |_| {
        spawn(async move {
            if is_compiling() {
                return;
            }
            is_compiling.set(true);

            let mut eval = eval(
                r#"
                let text = window.editorGlobal.getValue();
                dioxus.send(text);
                "#,
            );

            // TODO: Error Handling
            let val = eval.recv().await.unwrap().as_str().unwrap().to_string();
            socket_tx.send(SocketMessage::CompileRequest(val));
        });
    };

    rsx! {
        div {
            id: "pane-container",
            div {
                id: "left-pane",
                Header {
                    is_compiling: is_compiling(),
                    on_run,
                }
                div {
                    id: "editor",
                    onmounted: on_editor_mount,
                }
            }

            RightPane {
                show_page_uri: built_src(),
            }
        }
    }
}
