use dioxus::prelude::*;

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
                "Run",
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
