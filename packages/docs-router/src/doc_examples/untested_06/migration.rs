mod element_fixed {
    // ANCHOR: element_fixed
    use dioxus::prelude::*;

    fn app() -> Element {
        let number = use_signal(|| -1);

        if number() < 0 {
            // ✅ You can return VNode::empty() instead
            return VNode::empty();
        }
        if number() < 0 {
            // ✅ Or an empty rsx! macro
            return rsx! {};
        }

        rsx! {
            "Positive number: {number}"
        }
    }
    // ANCHOR_END: element_fixed
}

#[allow(deprecated)]
mod prevent_default_old {
    // ANCHOR: prevent_default_old
    use dioxus::prelude::*;

    fn app() -> Element {
        rsx! {
            a {
                href: "https://dioxuslabs.com",
                // ❌ The prevent default attribute is deprecated in dioxus 0.6
                prevent_default: "onclick",
                "Don't navigate to dioxuslabs.com"
            }
        }
    }
    // ANCHOR_END: prevent_default_old
}

mod prevent_default_new {
    // ANCHOR: prevent_default_new
    use dioxus::prelude::*;

    fn app() -> Element {
        rsx! {
            a {
                href: "https://dioxuslabs.com",
                // ✅ Instead, you can call event.prevent_default() inside the event handler
                onclick: move |event| event.prevent_default(),
                "Don't navigate to dioxuslabs.com"
            }
        }
    }
    // ANCHOR_END: prevent_default_new
}

mod prevent_default_new_liveview {
    // ANCHOR: prevent_default_new_liveview
    use dioxus::prelude::*;

    fn app() -> Element {
        rsx! {
            a {
                href: "https://dioxuslabs.com",
                // ✅ In liveview, you can use javascript to prevent default behavior
                "onclick": "event.preventDefault()",
                "Don't navigate to dioxuslabs.com"
            }
        }
    }
    // ANCHOR_END: prevent_default_new_liveview
}

mod assets_new {
    // ANCHOR: assets_new
    use dioxus::prelude::*;

    fn app() -> Element {
        rsx! {
            img {
                src: asset!("/assets/static/bundle.png", ImageAssetOptions::new().with_size(ImageSize::Manual { width: 100, height: 100 }))
            }
        }
    }
    // ANCHOR_END: assets_new
}
