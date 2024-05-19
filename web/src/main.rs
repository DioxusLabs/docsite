use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use futures::StreamExt as _;
use model::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{js_sys, MessageEvent, WebSocket};

const BUILT_URI: &str = "built/";
const SOCKET_URI: &str = "wss://localhost:3000/ws";
const SNIPPET_WELCOME: &str = include_str!("snippets/welcome.rs");

fn main() {
    dioxus_logger::init(Level::WARN).expect("failed to start logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut compiling = use_signal(|| false);
    let mut iframe_src = use_signal(String::new);

    let socket_tx = use_coroutine(move |mut rx: UnboundedReceiver<SocketMessage>| async move {
        let ws = WebSocket::new(SOCKET_URI).unwrap();

        // Setup socket callback
        let cl = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                let text: String = text.into();

                // Parse the message and determine what it is.
                if let Ok(msg) = SocketMessage::try_from(text) {
                    match msg {
                        SocketMessage::CompileFinished(id) => {
                            compiling.set(false);
                            iframe_src.set(format!("{}/{}", BUILT_URI, id));
                        },
                        SocketMessage::SystemError(_) => todo!(),
                        _ => {},
                    }
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        ws.set_onmessage(Some(cl.as_ref().unchecked_ref()));
        cl.forget();

        // Handle sending messages to socket.
        while let Some(msg) = rx.next().await {
            let parsed = msg.to_string();

            // TODO: Error handling
            ws.send_with_str(parsed.as_str()).unwrap();
        }
    });

    // Once the element has mounted, startup `ace` editor.
    let on_editor_mount = move |_| async move {
        let code = format!(
            r#"
            let editor = ace.edit("editor");
            editor.setTheme("ace/theme/github");

            let RustMode = ace.require("ace/mode/rust").Mode;
            editor.session.setMode(new RustMode());

            editor.setValue(`{SNIPPET_WELCOME}`, -1);
            "#
        );
        eval(&code);
    };

    // Send a request to compile code.
    let on_run = move |_| async move {
        if compiling() {
            return;
        }
        compiling.set(true);

        // `editor` is global variable for the `ace` editor.
        let mut eval = eval(
            r#"
            let text = editor.getValue();
            dioxus.send(text);
            "#,
        );

        let val = eval.recv().await.unwrap().to_string();
        socket_tx.send(SocketMessage::CompileRequest(val));
    };

    let on_clear = move |_| {
        eval("editor.setValue(\"\");");
    };

    rsx! {
        div {
            id: "pane-container",
            div {
                id: "left-pane",
                div {
                    id: "header",
                    button {
                        id: "run-button",
                        class: if compiling() { "disabled" },
                        onclick: on_run,
                        "Run",
                    }

                    button {
                        id: "clear-button",
                        onclick: on_clear,
                        "Clear",
                    }
                }
                div {
                    id: "editor",
                    onmounted: on_editor_mount,
                }
            }

            div {
                id: "right-pane",
                iframe {
                    src: "",
                }
            }
        }

        script {
            src: "ace/ace.js",
            r#type: "text/javascript",
        }
        script {
            r#"
            
            var editor = ace.edit("editor");
            editor.setTheme("ace/mode/rust");
            "#
        }
    }
}
