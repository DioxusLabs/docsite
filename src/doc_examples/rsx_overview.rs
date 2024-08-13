#![allow(non_snake_case)]
#![allow(unused)]

use dioxus::prelude::*;

pub fn App() -> Element {
    rsx! {
        Button {}
        Children {}
        Fragments {}
        Attributes {}
        VariableAttributes {}
        CustomAttributes {}
        Formatting {}
        Expression {}
    }
}

pub fn Button() -> Element {
    // ANCHOR: button
    rsx! {
        button {
            // attributes / listeners
            // children
            "Hello, World!"
        }
    }
    // ANCHOR_END: button
}

pub fn AttributesType() -> Element {
    // ANCHOR: attributes_type
    rsx! {
        input { r#type: "text", color: "red" }
    }
    // ANCHOR_END: attributes_type
}

pub fn Children() -> Element {
    // ANCHOR: children
    rsx! {
        ol {
            li { "First Item" }
            li { "Second Item" }
            li { "Third Item" }
        }
    }
    // ANCHOR_END: children
}

pub fn Fragments() -> Element {
    // ANCHOR: fragments
    rsx! {
        p { "First Item" }
        p { "Second Item" }
        Fragment {
            span { "a group" }
            span { "of three" }
            span { "items" }
        }
    }
    // ANCHOR_END: fragments
}

pub fn ManyRoots() -> Element {
    // ANCHOR: manyroots
    rsx! {
        p { "First Item" }
        p { "Second Item" }
    }
    // ANCHOR_END: manyroots
}

pub fn Attributes() -> Element {
    // ANCHOR: attributes
    rsx! {
        img {
            src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
            class: "primary_button",
            width: "10px",
        }
    }
    // ANCHOR_END: attributes
}

pub fn VariableAttributes() -> Element {
    // ANCHOR: variable_attributes
    let written_in_rust = true;
    let button_type = "button";
    rsx! {
        button { disabled: "{written_in_rust}", class: "{button_type}", "Rewrite it in rust" }
    }
    // ANCHOR_END: variable_attributes
}

pub fn CustomAttributes() -> Element {
    // ANCHOR: custom_attributes
    rsx! {
        div { "style": "width: 20px; height: 20px; background-color: red;" }
    }
    // ANCHOR_END: custom_attributes
}

pub fn ConditionalAttributes() -> Element {
    // ANCHOR: conditional_attributes
    let large_font = true;
    rsx! {
        div { class: if large_font { "text-xl" }, "Hello, World!" }
    }
    // ANCHOR_END: conditional_attributes
}

pub fn Formatting() -> Element {
    // ANCHOR: formatting
    let coordinates = (42, 0);
    let country = "es";
    rsx! {
        div {
            class: "country-{country}",
            left: "{coordinates.0:?}",
            top: "{coordinates.1:?}",
            // arbitrary expressions are allowed,
            // as long as they don't contain `{}`
            div { "{country.to_uppercase()}" }
            div { "{7*6}" }
            // {} can be escaped with {{}}
            div { "{{}}" }
        }
    }
    // ANCHOR_END: formatting
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

pub fn Loops() -> Element {
    // ANCHOR: loops
    rsx! {
        // use a for loop where the body itself is RSX
        div {
            // create a list of text from 0 to 9
            for i in 0..3 {
                // NOTE: the body of the loop is RSX not a rust statement
                div { "{i}" }
            }
        }
        // iterator equivalent
        div {
            {(0..3).map(|i| rsx! {
                div { "{i}" }
            })}
        }
    }
    // ANCHOR_END: loops
}

pub fn IfStatements() -> Element {
    // ANCHOR: ifstatements
    rsx! {
        // use if statements without an else
        if true {
            div { "true" }
        }
    }
    // ANCHOR_END: ifstatements
}
