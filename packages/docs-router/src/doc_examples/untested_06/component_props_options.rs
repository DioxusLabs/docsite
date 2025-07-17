#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! {
        // ANCHOR: OptionalProps_usage
        Title { title: "Some Title" }
        Title { title: "Some Title", subtitle: "Some Subtitle" }
        // Providing an Option explicitly won't compile though:
        // Title {
        //     title: "Some Title",
        //     subtitle: None,
        // },
        // ANCHOR_END: OptionalProps_usage

        // ANCHOR: ExplicitOption_usage
        ExplicitOption { title: "Some Title", subtitle: None }
        ExplicitOption { title: "Some Title", subtitle: Some("Some Title".to_string()) }
        // This won't compile:
        // ExplicitOption {
        //     title: "Some Title",
        // },
        // ANCHOR_END: ExplicitOption_usage

        // ANCHOR: DefaultComponent_usage
        DefaultComponent { number: 5 }
        DefaultComponent {}
        // ANCHOR_END: DefaultComponent_usage

        // ANCHOR: IntoComponent_usage
        IntoComponent { string: "some &str" }
        // ANCHOR_END: IntoComponent_usage
    }
}

// ANCHOR: OptionalProps
#[derive(PartialEq, Clone, Props)]
struct OptionalProps {
    title: String,
    subtitle: Option<String>,
}

fn Title(props: OptionalProps) -> Element {
    rsx! {
        h1 { "{props.title}: ", {props.subtitle.unwrap_or_else(|| "No subtitle provided".to_string())} }
    }
}
// ANCHOR_END: OptionalProps

// ANCHOR: ExplicitOption
#[derive(PartialEq, Clone, Props)]
struct ExplicitOptionProps {
    title: String,
    #[props(!optional)]
    subtitle: Option<String>,
}

fn ExplicitOption(props: ExplicitOptionProps) -> Element {
    rsx! {
        h1 { "{props.title}: ", {props.subtitle.unwrap_or_else(|| "No subtitle provided".to_string())} }
    }
}
// ANCHOR_END: ExplicitOption

// ANCHOR: DefaultComponent
#[derive(PartialEq, Props, Clone)]
struct DefaultProps {
    // default to 42 when not provided
    #[props(default = 42)]
    number: i64,
}

fn DefaultComponent(props: DefaultProps) -> Element {
    rsx! { h1 { "{props.number}" } }
}
// ANCHOR_END: DefaultComponent

// ANCHOR: IntoComponent
#[derive(PartialEq, Props, Clone)]
struct IntoProps {
    #[props(into)]
    string: String,
}

fn IntoComponent(props: IntoProps) -> Element {
    rsx! { h1 { "{props.string}" } }
}
// ANCHOR_END: IntoComponent
