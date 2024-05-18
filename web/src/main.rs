use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let on_run = move |_| async move {
        let mut eval = eval(
            r#"
            let text = editor.getValue();
            dioxus.send(text);
            "#,
        );

        let val = eval.recv().await.unwrap().to_string();
        panic!("{}", val);
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
                        id: "clear-button",
                        onclick: on_clear,
                        "Clear",
                    }

                    button {
                        id: "run-button",
                        onclick: on_run,
                        "Run",
                    }
                }
                div {
                    id: "editor",
                }
                // textarea {
                //     id: "editor",
                //     placeholder: "Build cool stuff!",
                //     value: editor_content,
                //     prevent_default: "onkeydown",
                //     onkeydown: on_editor_key_down,
                // }
            }

            div {
                id: "right-pane",
                iframe {
                    src: "https://dioxuslabs.com",
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
