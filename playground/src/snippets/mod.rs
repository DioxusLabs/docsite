use crate::{
    build::{BuildStage, BuildState},
    hotreload::HotReload,
};
use dioxus::{
    hooks::{use_context_provider, use_effect},
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

pub static EXAMPLES: [(&str, &LazyLock<String>, SelectedExample); 2] = [
    ("Welcome", &WELCOME_SNIPPET, SelectedExample::Welcome),
    ("Counter", &COUNTER_SNIPPET, SelectedExample::Counter),
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectedExample {
    None,
    Welcome,
    Counter,
}

impl SelectedExample {
    pub fn is_some(&self) -> bool {
        *self != Self::None
    }
}

/// Provide a selected example from context and set needs rebuild when changed.
pub fn use_provide_selected_example(
    mut build: BuildState,
    mut hot_reload: HotReload,
) -> Signal<SelectedExample> {
    let selected_example = use_context_provider(|| Signal::new(SelectedExample::None));

    // Set needs rebuild whenever an example is used.
    use_effect(move || {
        let example = selected_example();
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
