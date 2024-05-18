use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {

    let on_run = move |_| async move {

    };

    rsx! {
        div {
            div {
                div {
                    button {
                        onclick: on_run,
                        "Run",
                    }
                }
            }

            div {
                iframe {
                    src: "",
                }
            }
        }
    }
}
