use crate::components::{Header, RightPane, Tab};
use dioxus::prelude::*;

mod components;
mod error;
mod ws;

const _: &str = manganis::mg!(file("public/dxp.css"));
const SNIPPET_WELCOME: &str = include_str!("snippets/welcome.rs");

// Ace editor
//const _: &str = manganis::mg!(file("public/ace/ace.js"));
//const _: &str = manganis::mg!(file("public/ace/mode-rust.js"));
//const _: &str = manganis::mg!(file("public/ace/theme-github.js"));

#[component]
pub fn Playground(socket_url: String, built_url: String) -> Element {
    let mut is_compiling = use_signal(|| false);
    let mut built_page_id = use_signal(|| None);
    let mut compiler_messages = use_signal(Vec::<String>::new);
    let mut current_tab = use_signal(|| Tab::Page);

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
            let editor = ace.edit("dxp-editor");

            let RustMode = ace.require("ace/mode/rust").Mode;
            editor.session.setMode(new RustMode());

            editor.setValue(`{SNIPPET_WELCOME}`, -1);

            // Set a global so other evals can acces it.
            window.editorGlobal = editor;

            let remove = null;
            const updateTheme = () => {{
                if (remove != null) {{
                    remove();
                }}
                const media = window.matchMedia("(prefers-color-scheme: dark");
                media.addEventListener("change", updateTheme);
                remove = () => {{
                    media.removeEventListener("change", updateTheme);
                }};

                if (media.matches) {{
                    window.editorGlobal.setTheme("ace/theme/github_dark");
                }} else {{
                    window.editorGlobal.setTheme("ace/theme/github");
                }}
            }};

            updateTheme();
            "#
        );
        eval(&code);
    };

    // Send a request to compile code.
    let on_run = move |_| {
        if is_compiling() {
            return;
        }
        is_compiling.set(true);
        built_page_id.set(None);
        compiler_messages.clear();
        compiler_messages.push("Starting build...".to_string());

        let socket_url = socket_url.clone();

        spawn(async move {
            let mut eval = eval(
                r#"
                let text = window.editorGlobal.getValue();
                dioxus.send(text);
                "#,
            );
            let val = eval.recv().await.unwrap().as_str().unwrap().to_string();

            let mut socket = ws::Socket::new(&socket_url).unwrap();
            socket.compile(val).await.unwrap();

            while let Some(msg) = socket.next().await {
                let is_done = ws::handle_message(is_compiling, built_page_id, compiler_messages, msg);
                if is_done {
                    break;
                }
            }
            socket.close().await;
        });
    };

    // Build full url to built page.
    let built_page_url = use_memo(move || built_page_id().map(|id| format!("{}{}", built_url, id)));

    rsx! {
        div {
            id: "dxp-pane-container",
            div {
                id: "dxp-left-pane",
                Header {
                    is_compiling: is_compiling(),
                    on_run,
                }
                div {
                    id: "dxp-editor",
                    onmounted: on_editor_mount,
                }
            }

            RightPane {
                current_tab,
                compiler_messages,
                built_page_url: built_page_url(),
            }
        }
    }
}
