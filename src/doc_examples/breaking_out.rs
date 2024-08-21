use dioxus::prelude::*;
pub use downcast::Downcast;
pub use eval::Eval;
pub use onmounted::OnMounted;
pub use use_effect::Canvas;
pub use web_sys::WebSys;

mod eval {
    use super::*;

    // ANCHOR: eval
    pub fn Eval() -> Element {
        let mut domain = use_signal(String::new);
        rsx! {
            button {
                // When you click the button, some javascript will run in the browser
                // to read the domain and set the signal
                onclick: move |_| async move {
                    domain.set(eval("return document.domain").await.unwrap().to_string());
                },
                "Read Domain"
            }
            "Current domain: {domain}"
        }
    }
    // ANCHOR_END: eval
}

mod downcast {
    use super::*;

    // ANCHOR: downcast
    use dioxus::web::WebEventExt;
    pub fn Downcast() -> Element {
        let mut event_text = use_signal(|| 0);

        rsx! {
            div {
                onmousemove: move |event| {
                    event_text.set(event.as_web_event().movement_x());
                },
                "movement_x was {event_text}"
            }
        }
    }
    // ANCHOR_END: downcast
}

mod web_sys {
    use super::*;

    // ANCHOR: web_sys
    use ::web_sys::window;
    use wasm_bindgen::JsCast;
    pub fn WebSys() -> Element {
        let mut domain = use_signal(String::new);
        rsx! {
            button {
                // When you click the button, we use web-sys to read the domain and a signal
                onclick: move |_| {
                    domain.set(
                        window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .dyn_into::<::web_sys::HtmlDocument>()
                            .unwrap()
                            .domain()
                    );
                },
                "Read Domain"
            }
            "Current domain: {domain}"
        }
    }
    // ANCHOR_END: web_sys
}

mod use_effect {
    use super::*;

    // ANCHOR: use_effect
    pub fn Canvas() -> Element {
        let mut count = use_signal(|| 0);

        use_effect(move || {
            // Effects are reactive like memos, and resources. If you read a value inside the effect, the effect will rerun when that value changes
            let count = count.read();

            // You can use the count value to update the DOM manually
            eval(&format!(
                r#"var c = document.getElementById("dioxus-canvas");
    var ctx = c.getContext("2d");
    ctx.clearRect(0, 0, c.width, c.height);
    ctx.font = "30px Arial";
    ctx.fillText("{count}", 10, 50);"#
            ));
        });

        rsx! {
            button {
                // When you click the button, count will be incremented and the effect will rerun
                onclick: move |_| count += 1,
                "Increment"
            }
            canvas {
                id: "dioxus-canvas",
            }
        }
    }
    // ANCHOR_END: use_effect
}

mod onmounted {
    use super::*;

    // ANCHOR: onmounted
    pub fn OnMounted() -> Element {
        let mut input_element = use_signal(|| None);

        rsx! {
            div {
                height: "100px",
                button {
                    class: "focus:outline-2 focus:outline-blue-600 focus:outline-dashed",
                    // The onmounted event will run the first time the button element is mounted
                    onmounted: move |element| input_element.set(Some(element.data())),
                    "First button"
                }

                button {
                    // When you click the button, if the button element has been mounted, we focus to that element
                    onclick: move |_| async move {
                        if let Some(header) = input_element() {
                            let _ = header.set_focus(true).await;
                        }
                    },
                    "Focus first button"
                }
            }
        }
    }
    // ANCHOR_END: onmounted
}
