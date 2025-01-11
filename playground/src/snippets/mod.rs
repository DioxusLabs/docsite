use crate::{
    build::{BuildStage, BuildState},
    hotreload::HotReload,
};
use dioxus::{
    hooks::{use_context_provider, use_effect},
    prelude::Element,
    signals::Signal,
};
use std::sync::LazyLock;

mod counter;
mod welcome;

pub use counter::App as CounterExample;
pub use welcome::App as WelcomeExample;

static WELCOME_SNIPPET: LazyLock<String> =
    LazyLock::new(|| remove_common_code(include_str!("welcome.rs")));

static COUNTER_SNIPPET: LazyLock<String> =
    LazyLock::new(|| remove_common_code(include_str!("counter.rs")));

type ExampleComponent = fn() -> Element;
pub static EXAMPLES: [(&str, &LazyLock<String>, ExampleComponent); 2] = [
    ("Welcome", &WELCOME_SNIPPET, WelcomeExample),
    ("Counter", &COUNTER_SNIPPET, CounterExample),
];

#[derive(Debug, Clone, Copy)]
pub struct SelectedExample(pub Option<usize>);

/// Provide a selected example from context and set needs rebuild when changed.
pub fn use_provide_selected_example(
    mut build: BuildState,
    mut hot_reload: HotReload,
) -> Signal<SelectedExample> {
    let selected_example = use_context_provider(|| Signal::new(SelectedExample(None)));

    // Set needs rebuild whenever an example is used.
    use_effect(move || {
        let example = selected_example().0;
        if example.is_some() {
            build.set_stage(BuildStage::NotStarted);
            hot_reload.set_needs_rebuild(true);
        }
    });

    selected_example
}

/// Remove any unnecessary code for the example to run in the playground.
/// E.g. the dioxus prelude which is already included but nescessary to compile the example.
fn remove_common_code(snippet: &str) -> String {
    snippet
        .replace("use dioxus::prelude::*;", "")
        .trim()
        .to_string()
}
