#![allow(non_snake_case)]
use dioxus::prelude::*;

pub use use_hook::UseHookDemo;
use super::{log, ComponentWithLogs};

mod use_hook {
    use super::*;

    // ANCHOR: use_hook
    fn UseHook() -> Element {
        // The closure that is passed to use_hook will be called once the first time the component is rendered
        let random_number = use_hook(|| {
            let new_random_number = js_sys::Math::random();

            log!("{new_random_number}");

            new_random_number
        });

        rsx! {
            div { "Random {random_number}" }
        }
    }
    // ANCHOR_END: use_hook

    fn FakePage() -> Element {
        let mut uuid = use_signal(|| 0);
        rsx! {
            button {
                onclick: move |_| uuid += 1,
                "🔄"
            }
            {std::iter::once(rsx!{
                UseHook {
                    key: "{uuid}"
                }
            })}
        }
    }

    pub fn UseHookDemo() -> Element {
        rsx! {
            ComponentWithLogs {
                FakePage {}
            }
        }
    }
}
