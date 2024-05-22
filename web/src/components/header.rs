use dioxus::prelude::*;

const SPINNER: &str = manganis::mg!(file("public/spinner.svg"));

#[component]
pub fn Header(is_compiling: bool, on_run: EventHandler) -> Element {
    let on_clear = move |_| {
        eval("window.editorGlobal.setValue(\"\");");
    };

    rsx! {
        div {
            id: "header",
            button {
                id: "run-button",
                class: if is_compiling { "disabled" },
                onclick: move |_| on_run.call(()),
                if is_compiling {
                    img {
                        class: "spinner",
                        src: "{SPINNER}",
                    }
                } else {
                    "Run"
                }
            }

            h1 {
                id: "title",
                "Dioxus Playground",
            }

            button {
                id: "clear-button",
                onclick: on_clear,
                "Clear",
            }
        }
    }
}
