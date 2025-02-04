mod dog_app_component_props {
    use dioxus::prelude::*;

    // ANCHOR: dog_app_component_props
    #[derive(Props, PartialEq, Clone)]
    struct DogAppProps {
        breed: String,
    }
    // ANCHOR_END: dog_app_component_props

    // ANCHOR: dog_app_component_props_fn
    fn DogApp(props: DogAppProps) -> Element {
        todo!()
    }
    // ANCHOR_END: dog_app_component_props_fn
}

mod dog_app_component_macro {
    use dioxus::prelude::*;

    // ANCHOR: dog_app_component_macro
    #[component]
    fn DogApp(breed: String) -> Element {
        todo!()
    }
    // ANCHOR_END: dog_app_component_macro
}

mod dog_app_component_props_clone {
    use dioxus::prelude::*;

    // ANCHOR: dog_app_component_props_clone
    #[component]
    fn DogApp(breed: String) -> Element {
        tracing::info!("Rendered with breed: {breed}");

        todo!()
    }
    // ANCHOR_END: dog_app_component_props_clone
}

mod dog_app_component_run {
    use dioxus::prelude::*;

    // ANCHOR: dog_app_component_run
    #[component]
    fn DogApp(breed: String) -> Element {
        rsx! {
            "Breed: {breed}"
        }
    }
    // ANCHOR_END: dog_app_component_run
}

mod dog_app_component_compose {
    use dioxus::prelude::*;

    #[component]
    fn Header() -> Element {
        todo!()
    }

    #[component]
    fn Footer() -> Element {
        todo!()
    }

    #[component]
    fn DogApp(breed: String) -> Element {
        todo!()
    }

    // ANCHOR: dog_app_component_compose
    #[component]
    fn App() -> Element {
        rsx! {
            Header {}
            DogApp { breed: "corgi" }
            Footer {}
        }
    }
    // ANCHOR_END: dog_app_component_compose
}
