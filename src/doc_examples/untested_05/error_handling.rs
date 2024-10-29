use dioxus::prelude::*;

mod handle_none {
    use super::*;
    // ANCHOR: none
    fn App() -> Element {
        None
    }
    // ANCHOR_END: none
}

mod try_hook {
    use super::*;
    // ANCHOR: try_hook
    fn App() -> Element {
        // immediately return "None"
        let name = use_hook(|| Some("hi"))?;

        todo!()
    }
    // ANCHOR_END: try_hook
}

mod try_result_hook {
    use super::*;
    // ANCHOR: try_result_hook
    fn App() -> Element {
        // Convert Result to Option
        let name: i32 = use_hook(|| "1.234").parse().ok()?;

        // Early return
        let count = use_hook(|| "1.234");
        let val: i32 = match count.parse() {
            Ok(val) => val,
            Err(err) => return rsx! {"Parsing failed"},
        };

        todo!()
    }
    // ANCHOR_END: try_result_hook
}

mod match_error {
    use super::*;
    // ANCHOR: match_error
    fn Commandline() -> Element {
        // ANCHOR: use_error
        let mut error = use_signal(|| None);
        // ANCHOR_END: use_error

        match error() {
            Some(error) => rsx! { h1 { "An error occurred" } },
            None => rsx! { input { oninput: move |_| error.set(Some("bad thing happened!")) } },
        }
    }
    // ANCHOR_END: match_error
}

mod match_error_children {
    use super::*;
    // ANCHOR: match_error_children
    fn Commandline() -> Element {
        let error = use_signal(|| None);

        if let Some(error) = error() {
            return rsx! {"An error occurred"};
        }

        rsx! {
            Child { error }
            Child { error }
            Child { error }
            Child { error }
        }
    }

    #[component]
    fn Child(error: Signal<Option<&'static str>>) -> Element {
        rsx! { input { oninput: move |_| error.set(Some("bad thing happened!")) } }
    }
    // ANCHOR_END: match_error_children
}

mod throw_error {
    use super::*;
    // ANCHOR: throw_error
    fn Parent() -> Element {
        rsx! {
            ErrorBoundary {
                handle_error: |error| {
                    rsx! {
                        "Oops, we encountered an error. Please report {error} to the developer of this application"
                    }
                },
                ThrowsError {}
            }
        }
    }

    fn ThrowsError() -> Element {
        let name: i32 = use_hook(|| "1.234").parse().throw()?;

        todo!()
    }
    // ANCHOR_END: throw_error
}
mod nested_throw {
    use super::*;
    // ANCHOR: nested_throw
    fn App() -> Element {
        rsx! {
            ErrorBoundary {
                handle_error: |error| {
                    rsx! {
                        "Hmm, something went wrong. Please report {error} to the developer of this application"
                    }
                },
                Parent {}
            }
        }
    }

    fn Parent() -> Element {
        rsx! {
            ErrorBoundary {
                handle_error: |error| {
                    rsx! {
                        "The child component encountered an error: {error}"
                    }
                },
                ThrowsError {}
            }
        }
    }

    fn ThrowsError() -> Element {
        let name: i32 = use_hook(|| "1.234").parse().throw()?;

        todo!()
    }
    // ANCHOR_END: nested_throw
}
