use dioxus::prelude::*;

pub use component::ComponentDemo;
pub use effect::EffectDemo;
pub use memo::MemoDemo;
pub use resource::ResourceDemo;

#[derive(Default)]
struct LogState {
    logs: Vec<String>,
}

fn use_provide_log_state() -> Signal<LogState> {
    use_context_provider(|| Signal::new(LogState::default()))
}

fn log(message: impl ToString) {
    consume_context::<Signal<LogState>>()
        .write()
        .logs
        .insert(0, message.to_string());
}

macro_rules! log {
    ($($arg:tt)*) => {
        log(format!($($arg)*))
    }
}

#[component]
fn ComponentWithLogs(children: Element) -> Element {
    let logs = use_provide_log_state();

    rsx! {
        TwoPanelComponent {
            left: children,
            right: rsx! {
                div { class: "p-2 text-center border-gray-200 dark:border-gray-800",
                    "Logs"
                }
                for log in logs.read().logs.iter() {
                    div { class: "p-2 border-b border-gray-200 dark:border-gray-800",
                        "{log}"
                    }
                }
            }
        }
    }
}

#[component]
fn TwoPanelComponent(left: Element, right: Element) -> Element {
    rsx! {
        div { class: "w-full h-40 overflow-y-hidden flex flex-row justify-between",
            div { class: "w-1/2 h-full", {left} }
            div { class: "w-1/2 h-full", {right} }
        }
    }
}

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
