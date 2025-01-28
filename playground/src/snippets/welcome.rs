use dioxus::prelude::*;

/*

Welcome to the Dioxus Playground!

Here's some info to get you started:
- The `main` function is already provided.
- The Dioxus prelude is already imported.
- There must be a component named `App`
- Nothing is saved.

Have fun tinkering!

*/

#[component]
pub fn App() -> Element {
    rsx! {
        div {
            "Build cool stuff!"
        }
    }
}
