#![allow(non_snake_case)]
use dioxus::prelude::*;

use super::{log, ComponentWithLogs};
pub use drop::DropDemo;
pub use effect::EffectDemo;
pub use rerenders::RerenderDemo;
pub use use_hook::UseHookDemo;

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

mod rerenders {
    use super::*;

    // ANCHOR: rerenders
    fn Rerenders() -> Element {
        let mut count = use_signal(|| 0);

        log!("Rerendering parent component with {}", *count.peek());

        rsx! {
            button { onclick: move |_| count += 1, "Increment" }
            // Since we read count here, the component will rerender when count changes
            Count { current_count: count() }
        }
    }

    // If the count prop changes, the component will rerender
    #[component]
    fn Count(current_count: i32) -> Element {
        log!("Rerendering child component with {current_count}");

        rsx! {
            div { "The count is {current_count}" }
        }
    }
    // ANCHOR_END: rerenders

    // ANCHOR: dont_mutate
    fn Bad() -> Element {
        let mut count = use_signal(|| 0);

        // ❌ Don't mutate state in the body of the component.
        // It can easily cause an infinite loop!
        count += 1;

        rsx! {
            // If you both read and write to state in the body of the component,
            // you can cause an infinite loop!
            "{count}"
        }
    }
    // ANCHOR_END: dont_mutate

    pub fn RerenderDemo() -> Element {
        rsx! {
            ComponentWithLogs {
                Rerenders {}
            }
        }
    }
}

mod effect {
    use super::*;

    // ANCHOR: effect
    fn Effect() -> Element {
        // Effects run after the component is rendered
        // You can use them to read or modify the rendered component
        use_effect(|| {
            log!("Effect ran");
            eval(&format!(
                "document.getElementById('effect-output').innerText = 'Effect ran'"
            ));
        });

        rsx! {
            div {
                id: "effect-output",
                "This will be changed by the effect"
            }
        }
    }
    // ANCHOR_END: effect

    pub fn EffectDemo() -> Element {
        rsx! {
            ComponentWithLogs {
                Effect {}
            }
        }
    }
}

mod drop {
    use super::*;

    // ANCHOR: drop
    fn TogglesChild() -> Element {
        let mut show = use_signal(|| true);

        rsx! {
            button { onclick: move |_| show.toggle(), "Toggle" }
            if show() {
                Child {}
            }
        }
    }

    fn Child() -> Element {
        // You can use the use_drop hook to clean up any resources
        use_drop(|| {
            log!("Child dropped");
        });

        rsx! {
            div { "Child" }
        }
    }
    // ANCHOR_END: drop

    pub fn DropDemo() -> Element {
        rsx! {
            ComponentWithLogs {
                TogglesChild {}
            }
        }
    }
}
