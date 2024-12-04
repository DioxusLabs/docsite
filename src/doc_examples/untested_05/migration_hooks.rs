use dioxus::prelude::*;

// ANCHOR: use_resource
fn MyComponent() -> Element {
    let state = use_signal(|| 0);
    // No need to manually set the dependencies, the use_resource hook will automatically detect signal dependencies
    let my_resource = use_resource(move || async move {
        // start a request that depends on the state
        // Because we read from the state signal, this future will be re-run whenever the state changes
        println!("{state}");
    });
    rsx! {"{state}"}
}
// ANCHOR_END: use_resource

// ANCHOR: dependencies
fn HasDependencies() -> Element {
    let state = use_signal(|| 0);
    // No need to manually set the dependencies, the use_resource hook will automatically detect signal dependencies
    let my_resource = use_resource(move || async move {
        // Because we read from the state signal, this future will be re-run whenever the state changes
        println!("{state}");
    });
    let state_plus_one = use_memo(move || {
        // Because we read from the state signal, this future will be re-run whenever the state changes
        state() + 1
    });
    rsx! {"{state_plus_one}"}
}
// ANCHOR_END: dependencies
