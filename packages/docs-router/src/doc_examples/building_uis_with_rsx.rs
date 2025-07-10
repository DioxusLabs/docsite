use dioxus::prelude::*;

pub fn Text() -> Element {
    // ANCHOR: text
    rsx! { "Hello world" }
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
    rsx! { "Hello {user.read().name}" }
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
        input { r#type: "number" }
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
        input { r#type: if number_type() { "number" } }
    }
    // ANCHOR_END: input_disabled
}

pub fn OnInput() -> Element {
    // ANCHOR: on_input
    let mut value = use_signal(|| "Hello world".to_string());
    rsx! {
        input { oninput: move |event| value.set(event.value()), value: "{value}" }
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
            input { r#type: "number" }
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

pub fn Expression() -> Element {
    // ANCHOR: expression
    let text = "Dioxus";
    rsx! {
        span {
            {text.to_uppercase()}
            // create a list of text from 0 to 9
            {(0..10).map(|i| rsx! {
            "{i}"
            })}
        }
    }
    // ANCHOR_END: expression
}

pub fn CustomAttributes() -> Element {
    // ANCHOR: custom_attributes
    rsx! {
        div { "style": "width: 20px; height: 20px; background-color: red;" }
    }
    // ANCHOR_END: custom_attributes
}

pub fn StyleAttributes() -> Element {
    // ANCHOR: style_attributes
    rsx! {
        div {
            style: "width: 20px; height: 20px; background-color: red; margin: 10px;",
        }
        div {
            width: "20px",
            height: "20px",
            background_color: "red",
            margin: "10px",
        }
    }
    // ANCHOR_END: style_attributes
}

pub fn ClassAttribute() -> Element {
    let red = true;
    let blue_border = false;
    // ANCHOR: class_attribute
    rsx! {
        span {
            class: if red { "bg-red-500" },
            class: if blue_border { "border border-blue-500" },
            class: "w-4 h-4 block"
        }
    }
    // ANCHOR_END: class_attribute
}
