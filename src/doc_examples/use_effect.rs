// ANCHOR: app
use dioxus::prelude::*;

#[component]
fn Profile(id: ReadOnlySignal<i32>) -> Element {
    // Only change the page title when the id changes
    use_effect(move || {
        // We read the id signal here, so it will automatically be added as a dependency for the effect
        eval(&format!("document.title = 'Profile #{}';", id));
    });

    // Because the dependencies are empty, this will only run once.
    // An empty tuple is always equal to an empty tuple.
    use_effect(|| {
        tracing::info!("Hello, World!");
    });

    rsx!(
        div {
            "Profile #{id}"
        }
    )
}

fn app() -> Element {
    rsx!(Profile { id: 0 })
}
// ANCHOR_END: app

struct User {
    name: String,
}
fn fetch_user(id: usize) -> User {
    // ...
    unimplemented!()
}
