use dioxus::prelude::*;

pub use effect::EffectDemo;
pub use memo::MemoDemo;
pub use resource::ResourceDemo;
pub use component::ComponentDemo;

#[derive(Default)]
struct LogState {
    logs: Vec<String>,
}

fn use_provide_log_state() -> Signal<LogState> {
    use_context_provider(|| Signal::new(LogState::default()))
}

fn log(message: impl ToString) {
    consume_context::<Signal<LogState>>().write().logs.push(message.to_string());
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
        div {
            class: "w-full flex flex-row justify-between",
            div { class: "w-1/2 h-full",
                {children}
            }
            div { class: "w-1/2 h-full",
                div {
                    class: "p-2 text-center border-b-2 border-gray-200 dark:border-gray-800",
                    "Logs"
                }
                for (i, log) in logs.read().logs.iter().enumerate() {
                    div { class: "p-2 border-b border-gray-200 dark:border-gray-800",
                        "{i}: {log}"
                    }
                }
            }
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
            button {
                onclick: move |_| count += 1,
                "Increment"
            }

            "Count is {count}"
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
            button {
                onclick: move |_| count += 1,
                "Increment"
            }

            "Count is {count}"
            "Half count is {half_count}"
        }
    }
    // ANCHOR_END: memo

    pub fn MemoDemo() -> Element {
        rsx! {
            ComponentWithLogs {
                Memo {}
            }
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
            log!("{half_count:?}");
        });
        
        rsx! {
            button {
                onclick: move |_| count += 1,
                "Change Signal"
            }

            "Count is {count}"
            "Half count is {half_count}"
        }
    }
    // ANCHOR_END: resource

    pub fn ResourceDemo() -> Element {
        rsx! {
            ComponentWithLogs {
                Resource {}
            }
        }
    }
}

mod component {
    use super::*;
    
    // ANCHOR: component
    fn Component() -> Element {
        let mut count = use_signal(|| 0);
        
        rsx! {
            button {
                onclick: move |_| count += 1,
                "Change Signal"
            }

            // Since we read count inside the component, it becomes a dependency of the component
            // Whenever count changes, the component will rerun
            "Count: {count}"
        }
    }
    // ANCHOR_END: component

    pub fn ComponentDemo() -> Element {
        rsx! {
            ComponentWithLogs {
                Component {}
            }
        }
    }
}
