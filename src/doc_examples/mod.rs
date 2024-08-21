#![allow(unused)]

use dioxus::prelude::*;

#[component]
pub fn TwoPanelComponent(left: Element, right: Element) -> Element {
    rsx! {
        div { class: "w-full h-40 overflow-y-hidden flex flex-row justify-between",
            div { class: "w-1/2 h-full", {left} }
            div { class: "w-1/2 h-full text-sm", {right} }
        }
    }
}

#[derive(Default)]
struct LogState {
    logs: Vec<String>,
}

fn use_provide_log_state() -> Signal<LogState> {
    use_context_provider(|| Signal::new(LogState::default()))
}

pub fn log(message: impl ToString) {
    consume_context::<Signal<LogState>>()
        .write()
        .logs
        .insert(0, message.to_string());
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        log(format!($($arg)*))
    }
}

#[component]
pub fn ComponentWithLogs(children: Element) -> Element {
    let logs = use_provide_log_state();

    rsx! {
        TwoPanelComponent {
            left: children,
            right: rsx! {
                div { class: "p-2 text-center border-gray-200 dark:border-gray-800",
                    "Logs"
                }
                for log in logs.read().logs.iter() {
                    div { class: "p-2 border-b border-gray-200 dark:border-gray-800",
                        "{log}"
                    }
                }
            }
        }
    }
}

// Include any examples we compile into the docsite
#[cfg(not(feature = "doc_test"))]
pub mod boolean_attribute;
#[cfg(not(feature = "doc_test"))]
pub mod breaking_out;
#[cfg(not(feature = "doc_test"))]
pub mod building_uis_with_rsx;
#[cfg(not(feature = "doc_test"))]
pub mod component_children;
#[cfg(not(feature = "doc_test"))]
pub mod component_lifecycle;
#[cfg(not(feature = "doc_test"))]
pub mod component_owned_props;
#[cfg(not(feature = "doc_test"))]
pub mod components;
#[cfg(not(feature = "doc_test"))]
pub mod conditional_rendering;
#[cfg(not(feature = "doc_test"))]
pub mod dangerous_inner_html;
#[cfg(not(feature = "doc_test"))]
pub mod event_click;
#[cfg(not(feature = "doc_test"))]
pub mod event_prevent_default;
#[cfg(not(feature = "doc_test"))]
pub mod full_example;
#[cfg(not(feature = "doc_test"))]
pub mod hackernews_async;
#[cfg(not(feature = "doc_test"))]
pub mod hackernews_complete;
#[cfg(not(feature = "doc_test"))]
pub mod hackernews_post;
#[cfg(not(feature = "doc_test"))]
pub mod hackernews_state;
#[cfg(not(feature = "doc_test"))]
pub mod hello_world;
#[cfg(not(feature = "doc_test"))]
pub mod hooks_counter;
#[cfg(not(feature = "doc_test"))]
pub mod hooks_counter_two_state;
#[cfg(not(feature = "doc_test"))]
pub mod hooks_use_signal;
#[cfg(not(feature = "doc_test"))]
pub mod input_controlled;
#[cfg(not(feature = "doc_test"))]
pub mod input_uncontrolled;
#[cfg(not(feature = "doc_test"))]
pub mod moving_state_around;
#[cfg(not(feature = "doc_test"))]
pub mod reactivity;
#[cfg(not(feature = "doc_test"))]
pub mod readme;
#[cfg(not(feature = "doc_test"))]
pub mod rendering_lists;
#[cfg(not(feature = "doc_test"))]
pub mod rsx_overview;
#[cfg(not(feature = "doc_test"))]
pub mod spawn;
#[cfg(not(feature = "doc_test"))]
pub mod use_resource;

// Check any examples we don't compile into the docs
#[cfg(feature = "doc_test")]
automod::dir!(pub "src/doc_examples");
