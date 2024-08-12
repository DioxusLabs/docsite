use dioxus::prelude::*;

const SPINNER: &str = manganis::mg!(file("public/spinner.svg"));

#[component]
pub fn Header(is_compiling: bool, queue_position: Option<u32>, on_run: EventHandler) -> Element {
    let on_clear = move |_| {
        eval("window.editorGlobal.setValue(\"\");");
    };

    rsx! {
        div {
            id: "dxp-header",

            button {
                id: "dxp-run-button",
                class: if is_compiling { "disabled" },

                onclick: move |_| on_run.call(()),
                if is_compiling {
                    img {
                        class: "dxp-spinner",
                        src: "{SPINNER}",
                    }

                    if let Some(pos) = queue_position {
                        "#{pos}"
                    }

                } else {
                    "Run"
                }
            }

            h1 {
                id: "dxp-title",

                "Dioxus Playground"
            }

            button {
                id: "dxp-clear-button",

                onclick: on_clear,
                "Clear"
            }
        }
    }
}
