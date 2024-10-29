use dioxus::prelude::*;

use super::{log, ComponentWithLogs, LogState, TwoPanelComponent};
pub use component::ComponentDemo;
pub use effect::EffectDemo;
pub use memo::MemoDemo;
pub use non_reactive_state::MakingPropsReactiveDemo;
pub use non_reactive_state::NonReactiveDemo;
pub use non_reactive_state::UseReactiveDemo;
pub use resource::ResourceDemo;

mod signal {
    use super::*;

    fn app() -> Element {
        // ANCHOR: signal
        let mut signal = use_signal(|| 0);
        // ANCHOR_END: signal

        {
            // ANCHOR: signal_read
            // Call the signal like a function to clone the current value
            let value: i32 = signal();
            // get a reference to the inner value with the .read() method
            let value: &i32 = &signal.read();
            // or use one of the traits implemented for Signal like Display
            log!("{signal}");
            // ANCHOR_END: signal_read
        }

        {
            // ANCHOR: signal_write
            // Set the value from the signal
            signal.set(1);
            // get a mutable reference to the inner value with the .write() method
            let mut value: &mut i32 = &mut signal.write();
            *value += 1;
            // ANCHOR_END: signal_write
        }

        rsx! {}
    }
}

mod effect {
    use super::*;

    // ANCHOR: effect
    fn Effect() -> Element {
        // use_signal creates a tracked value called count
        let mut count = use_signal(|| 0);

        use_effect(move || {
            // When we read count, it becomes a dependency of the effect
            let current_count = count();
            // Whenever count changes, the effect will rerun
            log!("{current_count}");
        });

        rsx! {
            button { onclick: move |_| count += 1, "Increment" }

            div { "Count is {count}" }
        }
    }
    // ANCHOR_END: effect

    pub fn EffectDemo() -> Element {
        rsx! {
            ComponentWithLogs { Effect {} }
        }
    }
}

mod memo {
    use super::*;

    // ANCHOR: memo
    fn Memo() -> Element {
        let mut count = use_signal(|| 0);

        // use_memo creates a tracked value that is derived from count
        // Since we read count inside the closure, it becomes a dependency of the memo
        // Whenever count changes, the memo will rerun
        let half_count = use_memo(move || count() / 2);

        use_effect(move || {
            // half_count is itself a tracked value
            // When we read half_count, it becomes a dependency of the effect
            // and the effect will rerun when half_count changes
            log!("{half_count}");
        });

        rsx! {
            button { onclick: move |_| count += 1, "Increment" }

            div { "Count is {count}" }
            div { "Half count is {half_count}" }
        }
    }
    // ANCHOR_END: memo

    pub fn MemoDemo() -> Element {
        rsx! {
            ComponentWithLogs { Memo {} }
        }
    }
}

mod resource {
    use super::*;

    // ANCHOR: resource
    fn Resource() -> Element {
        let mut count = use_signal(|| 0);

        // use_resource creates a tracked value that is derived from count
        // Since we read count inside the closure, it becomes a dependency of the resource
        // Whenever count changes, the resource will rerun
        let half_count = use_resource(move || async move {
            // You can do async work inside resources
            gloo_timers::future::TimeoutFuture::new(100).await;
            count() / 2
        });

        use_effect(move || {
            // half_count is itself a tracked value
            // When we read half_count, it becomes a dependency of the effect
            // and the effect will rerun when half_count changes
            log!("{:?}", half_count());
        });

        rsx! {
            button { onclick: move |_| count += 1, "Change Signal" }

            div { "Count is {count}" }
            div { "Half count is {half_count():?}" }
        }
    }
    // ANCHOR_END: resource

    pub fn ResourceDemo() -> Element {
        rsx! {
            ComponentWithLogs { Resource {} }
        }
    }
}

mod component {
    use super::*;

    // ANCHOR: component
    fn Component() -> Element {
        let mut count = use_signal(|| 0);

        rsx! {
            button { onclick: move |_| count += 1, "Change Signal" }

            // Since we read count inside Component, it becomes a dependency of Component
            // Whenever count changes, Component will rerun
            Count { count: count() }
        }
    }

    // Components automatically memorize their props. If the props change, Count will rerun
    #[component]
    fn Count(count: i32) -> Element {
        rsx! {
            div { "Count: {count}" }
        }
    }
    // ANCHOR_END: component

    pub fn ComponentDemo() -> Element {
        let mut count = use_signal(|| 0);

        rsx! {
            TwoPanelComponent {
                left: rsx! {
                    button { onclick: move |_| count += 1, "Change Signal" }

                    div { "Count: {count}" }
                },
                right: rsx! {
                    div { class: "p-2 text-center border-gray-200 dark:border-gray-800",
                        "UI"
                    }
                    for count in (0..=count()).rev() {
                        div {
                            class: "p-2 border-b border-gray-200 dark:border-gray-800",
                            div { "Count: {count}" }
                        }
                    }
                }
            }
        }
    }
}

mod non_reactive_state {
    use super::*;
    pub use making_props_reactive::*;
    pub use non_reactive::*;
    pub use use_reactive::*;

    mod non_reactive {
        use super::*;
        // ANCHOR: non_reactive
        fn Component() -> Element {
            let mut count = use_signal(|| 0);

            rsx! {
                button { onclick: move |_| count += 1, "Change Signal" }

                Count { count: count() }
            }
        }

        // The count reruns the component when it changes, but it is not a tracked value
        #[component]
        fn Count(count: i32) -> Element {
            // When you read count inside the memo, it does not subscribe to the count signal
            // because the value is not reactive
            let double_count = use_memo(move || count * 2);

            rsx! {
                div { "Double count: {double_count}" }
            }
        }
        // ANCHOR_END: non_reactive

        pub fn NonReactiveDemo() -> Element {
            let mut count = use_signal(|| 0);

            rsx! {
                TwoPanelComponent {
                    left: rsx! {
                        button { onclick: move |_| count += 1, "Change Signal" }

                        Count { count: count() }
                    },
                    right: rsx! {
                        div { class: "p-2 text-center border-gray-200 dark:border-gray-800",
                            "UI"
                        }
                        for count in (0..=count()).rev() {
                            div {
                                class: "p-2 border-b border-gray-200 dark:border-gray-800",
                                Count { count }
                            }
                        }
                    }
                }
            }
        }
    }

    mod use_reactive {
        use super::*;

        // ANCHOR: use_reactive
        #[component]
        fn Count(count: i32) -> Element {
            // You can manually track a non-reactive value with the use_reactive hook
            let double_count = use_memo(
                // Use reactive takes a tuple of dependencies and returns a reactive closure
                use_reactive!(|(count,)| count * 2),
            );

            rsx! {
                div { "Double count: {double_count}" }
            }
        }
        // ANCHOR_END: use_reactive

        pub fn UseReactiveDemo() -> Element {
            let mut count = use_signal(|| 0);

            rsx! {
                TwoPanelComponent {
                    left: rsx! {
                        button { onclick: move |_| count += 1, "Change Signal" }

                        Count { count: count() }
                    },
                    right: rsx! {
                        div { class: "p-2 text-center border-gray-200 dark:border-gray-800",
                            "UI"
                        }
                        for count in (0..=count()).rev() {
                            div {
                                class: "p-2 border-b border-gray-200 dark:border-gray-800",
                                Count { count }
                            }
                        }
                    }
                }
            }
        }
    }

    mod making_props_reactive {
        use super::*;

        // ANCHOR: making_props_reactive
        // You can track props by wrapping the type in a ReadOnlySignal
        // Dioxus will automatically convert T into ReadOnlySignal<T> when you pass
        // props to the component
        #[component]
        fn Count(count: ReadOnlySignal<i32>) -> Element {
            // Then when you read count inside the memo, it subscribes to the count signal
            let double_count = use_memo(move || count() * 2);

            rsx! {
                div { "Double count: {double_count}" }
            }
        }
        // ANCHOR_END: making_props_reactive

        pub fn MakingPropsReactiveDemo() -> Element {
            let mut count = use_signal(|| 0);

            rsx! {
                TwoPanelComponent {
                    left: rsx! {
                        button { onclick: move |_| count += 1, "Change Signal" }

                        Count { count: count() }
                    },
                    right: rsx! {
                        div { class: "p-2 text-center border-gray-200 dark:border-gray-800",
                            "UI"
                        }
                        for count in (0..=count()).rev() {
                            div {
                                class: "p-2 border-b border-gray-200 dark:border-gray-800",
                                Count { count }
                            }
                        }
                    }
                }
            }
        }
    }
}
