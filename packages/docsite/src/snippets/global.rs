//! Use Signal::global to easily manage global state with a simple Atom-based API
use dioxus::prelude::*;

// Define an atom of state
static COUNT: GlobalSignal<i32> = Signal::global(|| 0);

// Read it anywhere
fn Read() -> Element {
    rsx! {"Count: {COUNT}"}
}

// Or write to it from anywhere
fn Increment() -> Element {
    rsx! { button { onclick: move |_| *COUNT.write() += 1, "Increment" } }
}

fn App() -> Element {
    rsx! {
        Read {}
        Increment {}
    }
}
