mod new_app {
    fn App() -> Element {
        todo!()
    }

    // ANCHOR: new_app
    use dioxus::prelude::*;

    fn main() {
        dioxus::launch(App);
    }
    // ANCHOR_END: new_app
}

mod new_app_full {
    // ANCHOR: new_app_full
    use dioxus::prelude::*;

    fn main() {
        dioxus::launch(App);
    }

    #[component]
    fn App() -> Element {
        rsx! { "HotDog!" }
    }
    // ANCHOR_END: new_app_full
}
