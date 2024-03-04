use dioxus::prelude::*;

mod use_signal {
    use dioxus::prelude::*;
    // ANCHOR: use_signal
    fn Parent() -> Element {
        let state = use_signal(|| 0);

        rsx! { Child { state } }
    }

    #[component]
    fn Child(state: Signal<i32>) -> Element {
        rsx! {"{state}"}
    }
    // ANCHOR_END: use_signal
}

mod context_signals {
    use dioxus::prelude::*;
    // ANCHOR: context_signals
    fn Parent() -> Element {
        // Create a new signal and provide it to the context API
        let state = use_context_provider(|| Signal::new(0));

        rsx! { Child {} }
    }

    fn Child() -> Element {
        // Get the state from the context API
        let state = use_context::<Signal<i32>>();

        rsx! {"{state}"}
    }
    // ANCHOR_END: context_signals
}

mod peek {
    use dioxus::prelude::*;
    // ANCHOR: peek
    fn Parent() -> Element {
        let state = use_signal(|| 0);

        // Even though we are reading the state, we don't need to subscribe to it
        let read_without_subscribing = state.peek();
        println!("{}", state.peek());

        rsx! { Child { state } }
    }

    #[component]
    fn Child(state: Signal<i32>) -> Element {
        rsx! {
            button { onclick: move |_| {
                    state += 1;
                }, "count is {state}" }
        }
    }
    // ANCHOR_END: peek
}

mod global_signals {
    use dioxus::prelude::*;
    // ANCHOR: global_signals
    static COUNT: GlobalSignal<i32> = Signal::global(|| 0);

    fn Parent() -> Element {
        rsx! {
            div { "{COUNT}" }
            button {
                onclick: move |_| {
                    *COUNT.write() += 1;
                },
                "Increment"
            }
        }
    }
    // ANCHOR_END: global_signals
}
