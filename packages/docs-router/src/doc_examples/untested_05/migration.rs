mod scope {
    // ANCHOR: scope
    use dioxus::prelude::*;

    // In dioxus 0.5, the scope is no longer passed as an argument to the function
    fn app() -> Element {
        // Hooks, context, and spawn are now called directly
        use_hook(|| { /*...*/ });
        provide_context({ /*...*/ });
        spawn(async move { /*...*/ });
        rsx! {
            /*...*/
        }
    }
    // ANCHOR_END: scope
}

mod props {
    // ANCHOR: props
    use dioxus::prelude::*;

    // In dioxus 0.5, props are always owned. You pass in owned props and you get owned props in the body of the component
    #[component]
    fn Comp(name: String) -> Element {
        // Name is owned here already (name is the type String inside the function)
        let owned_name: String = name;

        rsx! {
            "Hello {owned_name}"
            BorrowedComp {
                name: "other name"
            }
            ManualPropsComponent {
                name: "other name 2"
            }
        }
    }

    // Borrowed props are removed in dioxus 0.5. Mapped signals can act similarly to borrowed props if your props are borrowed from state
    // ReadOnlySignal is a copy wrapper over a state that will be automatically converted to
    #[component]
    fn BorrowedComp(name: ReadOnlySignal<String>) -> Element {
        rsx! {
            "Hello {name}"
        }
    }

    // In dioxus 0.5, props need to implement Props, Clone, and PartialEq
    #[derive(Props, Clone, PartialEq)]
    struct ManualProps {
        name: String,
    }

    // Functions accept the props directly instead of the scope
    fn ManualPropsComponent(props: ManualProps) -> Element {
        rsx! {
            "Hello {props.name}"
        }
    }
    // ANCHOR_END: props
}

mod futures {
    use dioxus::prelude::*;

    fn app() {
        // ANCHOR: futures
        // dependency1 and dependency2 must be Signal-like types like Signal, ReadOnlySignal, GlobalSignal, or another Resource
        use_resource(|| async move { /*use dependency1 and dependency2*/ });

        let non_reactive_state = 0;
        // You can also add non-reactive state to the resource hook with the use_reactive macro
        use_resource(use_reactive!(|(non_reactive_state,)| async move {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            non_reactive_state + 1
        }));
        // ANCHOR_END: futures
    }
}

mod state {
    use dioxus::prelude::*;

    fn app() -> Element {
        // ANCHOR: state
        // You can now use signals for local copy state, local clone state, and shared state with the same API
        let mut copy_state = use_signal(|| 0);
        let mut clone_shared_state = use_context_provider(|| Signal::new(String::from("Hello")));
        let mut clone_local_state = use_signal(|| String::from("Hello"));

        // Call the signal like a function to clone the current value
        let copy_state_value = copy_state();
        // Or use the read method to borrow the current value
        let clone_local_state_value = clone_local_state.read();
        let clone_shared_state_value = clone_shared_state.read();

        rsx! {
            "{copy_state_value}"
            "{clone_shared_state_value}"
            "{clone_local_state_value}"
            button {
                onclick: move |_| {
                    // All three states have the same API for updating the state
                    copy_state.set(1);
                    clone_shared_state.set("World".to_string());
                    clone_local_state.set("World".to_string());
                },
                "Set State"
            }
        }
        // ANCHOR_END: state
    }
}

mod fermi {
    // ANCHOR: fermi
    use dioxus::prelude::*;

    // Atoms and AtomRefs have been replaced with GlobalSignals
    static NAME: GlobalSignal<String> = Signal::global(|| "world".to_string());

    fn app() -> Element {
        rsx! {
            // You can use global state directly without the use_read or use_set hooks
            div { "hello {NAME}!" }
            Child {}
            ChildWithRef {}
        }
    }

    fn Child() -> Element {
        rsx! {
            button {
                onclick: move |_| *NAME.write() = "dioxus".to_string(),
                "reset name"
            }
        }
    }

    // Atoms and AtomRefs have been replaced with GlobalSignals
    static NAMES: GlobalSignal<Vec<String>> = Signal::global(|| vec!["world".to_string()]);

    fn ChildWithRef() -> Element {
        rsx! {
            div {
                ul {
                    for name in NAMES.read().iter() {
                        li { "hello: {name}" }
                    }
                }
                button {
                    onclick: move |_| {
                        // No need to clone the signal into futures, you can use it directly
                        async move {
                            NAMES.write().push("asd".to_string());
                        }
                    },
                    "Add name"
                }
            }
        }
    }
    // ANCHOR_END: fermi
}
