/*

Welcome to the Dioxus Playground!

Here's some info to get you started:
- The `main` function is already provided.
- The Dioxus prelude is already imported.
- There must be a component named `App` 
  this is the entrypoint into your Dioxus app. 
- Aside from first use, the `Run` button will 
  be disabled until hot reloading fails.
- Nothing is saved.

Have fun tinkering!

*/

#[component]
fn App() -> Element {
    rsx! {
        div {
            "Build cool stuff!"
        }
    }
}