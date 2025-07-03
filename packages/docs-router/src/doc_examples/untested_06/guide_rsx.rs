use dioxus::prelude::*;

fn RsxIsHtml() -> Element {
    // ANCHOR: rsx_is_html
    rsx! {
        div {
            class: "bg-red-100"
        }
    }
    // ANCHOR_END: rsx_is_html
}

fn ChildRsx() -> Element {
    use tracing::info;

    // ANCHOR: child_rsx
    rsx! {
        div { class: "bg-red-100",
            button {
                onclick: move |_| info!("Clicked"),
                "Click me!"
            }
        }
    }
    // ANCHOR_END: child_rsx
}

fn RsxFormat() -> Element {
    let breed = "Pitbull";

    // ANCHOR: rsx_format
    rsx! {
        div { "Breed: {breed}" }
    }
    // ANCHOR_END: rsx_format
}

fn RsxExpression() -> Element {
    let show_title = false;
    // ANCHOR: rsx_expression
    rsx! {
        // Anything that's `Display`
        {"Something"}

        // Optionals
        {show_title.then(|| rsx! { "title!" } )}

        // And iterators
        ul {
            {(0..5).map(|i| rsx! { "{i}" })}
        }
    }
    // ANCHOR_END: rsx_expression
}

fn RsxLoop() -> Element {
    let show_title = false;
    // ANCHOR: rsx_loop
    rsx! {
        if show_title {
            "title!"
        }

        ul {
            for item in 0..5 {
                "{item}"
            }
        }
    }
    // ANCHOR_END: rsx_loop
}

fn RsxKeyed() -> Element {
    struct User {
        id: u32,
        name: String,
    }

    let users = [User {
        id: 1,
        name: "Dioxus".to_string(),
    }];

    // ANCHOR: rsx_keyed
    rsx! {
        for user in users {
            div {
                key: "{user.id}",
                "{user.name}"
            }
        }
    }
    // ANCHOR_END: rsx_keyed
}

// ANCHOR: adding_ui
#[component]
fn App() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
        div { id: "dogview",
            img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
        }
        div { id: "buttons",
            button { id: "skip", "skip" }
            button { id: "save", "save!" }
        }
    }
}
// ANCHOR_END: adding_ui
