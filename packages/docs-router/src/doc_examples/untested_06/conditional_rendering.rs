#![allow(unused)]
#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn App() -> Element {
    let mut is_logged_in = use_signal(|| false);

    rsx! {
        LogIn {
            is_logged_in: is_logged_in(),
            log_in: move |_| is_logged_in.set(true),
            log_out: move |_| is_logged_in.set(false),
        }
    }
}

#[component]
fn LogIn(is_logged_in: bool, log_in: EventHandler, log_out: EventHandler) -> Element {
    // ANCHOR: if_else
    if is_logged_in {
        rsx! {
            "Welcome!"
            button { onclick: move |_| log_out.call(()), "Log Out" }
        }
    } else {
        rsx! {
            button { onclick: move |_| log_in.call(()), "Log In" }
        }
    }
    // ANCHOR_END: if_else
}

pub fn LogInImprovedApp() -> Element {
    let mut is_logged_in = use_signal(|| false);

    rsx! {
        LogInImproved {
            is_logged_in: is_logged_in(),
            log_in: move |_| is_logged_in.set(true),
            log_out: move |_| is_logged_in.set(false),
        }
    }
}

#[component]
fn LogInImproved(is_logged_in: bool, log_in: EventHandler, log_out: EventHandler) -> Element {
    // ANCHOR: if_else_improved
    rsx! {
        // We only render the welcome message if we are logged in
        // You can use if statements in the middle of a render block to conditionally render elements
        if is_logged_in {
            // Notice the body of this if statement is rsx code, not an expression
            "Welcome!"
        }
        button {
            // depending on the value of `is_logged_in`, we will call a different event handler
            onclick: move |_| if is_logged_in { log_out.call(()) } else { log_in.call(()) },
            if is_logged_in {
                // if we are logged in, the button should say "Log Out"
                "Log Out"
            } else {
                // if we are not logged in, the button should say "Log In"
                "Log In"
            }
        }
    }
    // ANCHOR_END: if_else_improved
}

pub fn LogInWarningApp() -> Element {
    let mut is_logged_in = use_signal(|| false);

    rsx! {
        input {
            r#type: "checkbox",
            checked: is_logged_in(),
            oninput: move |event| is_logged_in.set(event.checked()),
            "Logged In"
        }
        LogInWarning { is_logged_in: is_logged_in() }
    }
}

#[component]
fn LogInWarning(is_logged_in: bool) -> Element {
    // ANCHOR: conditional_none
    if is_logged_in {
        return rsx!();
    }

    rsx! {
        p { "You must be logged in to comment" }
    }
    // ANCHOR_END: conditional_none
}
