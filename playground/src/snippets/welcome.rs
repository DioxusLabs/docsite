/*

Welcome to the Dioxus Playground!

Here's some info to get you started:
- The `main` function is already provided.
- There must be a component named `App`
  this is the entrypoint into your Dioxus app. 
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