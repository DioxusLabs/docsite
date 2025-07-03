use dioxus::prelude::*;

pub fn Text() -> Element {
    // ANCHOR: text
    rsx! {
        "Hello world"
    }
    // ANCHOR_END: text
}

pub fn FormattedText() -> Element {
    struct User {
        name: String,
    }
    // ANCHOR: formatted_text
    let user = use_signal(|| User {
        name: "Dioxus".to_string(),
    });
    rsx! {
        // Unlike the format macro, you can include many expressions inline in the formatted text
        "Hello {user.read().name}"
    }
    // ANCHOR_END: formatted_text
}

pub fn Input() -> Element {
    // ANCHOR: input
    rsx! {
        input {}
    }
    // ANCHOR_END: input
}

pub fn WebComponent() -> Element {
    // ANCHOR: web_component
    rsx! {
        my-web-component {}
    }
    // ANCHOR_END: web_component
}

pub fn InputType() -> Element {
    // ANCHOR: input_type
    rsx! {
        input { type: "number" }
    }
    // ANCHOR_END: input_type
}

pub fn InputValue() -> Element {
    // ANCHOR: input_value
    let mut value = use_signal(|| "Hello world".to_string());
    rsx! {
        input { value: "{value}" }
    }
    // ANCHOR_END: input_value
}

pub fn InputDisabled() -> Element {
    // ANCHOR: input_disabled
    let number_type = use_signal(|| false);
    rsx! {
        input { type: if number_type() { "number" } }
    }
    // ANCHOR_END: input_disabled
}

pub fn OnInput() -> Element {
    // ANCHOR: on_input
    let mut value = use_signal(|| "Hello world".to_string());
    rsx! {
        input {
            oninput: move |event| value.set(event.value()),
            value: "{value}"
        }
    }
    // ANCHOR_END: on_input
}

pub fn InputChildren() -> Element {
    // ANCHOR: input_children
    rsx! {
        div {
            // display sets the layout mode of the element
            display: "flex",
            // justify-content centers the element horizontally
            justify_content: "center",
            input {
                type: "number"
            }
        }
    }
    // ANCHOR_END: input_children
}

pub fn ForLoop() -> Element {
    // ANCHOR: for_loop
    let mut items = use_signal(|| vec!["Hello", "Dioxus"]);

    rsx! {
        ul {
            for item in items.iter() {
                li { "{item}" }
            }
        }
    }
    // ANCHOR_END: for_loop
}

pub fn KeyedForLoop() -> Element {
    // ANCHOR: keyed_for_loop
    let mut items = use_signal(|| vec!["Hello", "Dioxus"]);

    rsx! {
        ul {
            for item in items.iter() {
                li { key: "{item}", "{item}" }
            }
        }
    }
    // ANCHOR_END: keyed_for_loop
}

pub fn IfStatement() -> Element {
    // ANCHOR: if_statement
    let logged_in = use_signal(|| false);

    rsx! {
        div {
            if logged_in() {
                "You are logged in"
            } else {
                "You are not logged in"
            }
        }
    }
    // ANCHOR_END: if_statement
}
