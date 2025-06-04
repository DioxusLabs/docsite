// ANCHOR: app
use dioxus::prelude::*;

#[component]
fn Profile(id: ReadOnlySignal<i32>) -> Element {
    // Only change the page title when the id changes
    use_effect(move || {
        // We read the id signal here, so it will automatically be added as a dependency for the effect
        document::eval(&format!("document.title = 'Profile #{}';", id));
    });

    // Because there are no dependencies, this effect will only run once.
    use_effect(|| {
        tracing::info!("Hello, World!");
    });

    // You can also add non-reactive state to the effect hook with the use_reactive macro
    let non_reactive_state = id();
    use_effect(use_reactive!(|(non_reactive_state,)| {
        tracing::info!("Profile #{}", non_reactive_state);
    }));

    rsx! {
        div { "Profile #{id}" }
    }
}

fn app() -> Element {
    rsx! {
        Profile { id: 0 }
    }
}
// ANCHOR_END: app

struct User {
    name: String,
}
fn fetch_user(id: usize) -> User {
    // ...
    unimplemented!()
}
