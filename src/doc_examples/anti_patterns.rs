#![allow(non_snake_case, unused)]

//! This example shows what *not* to do

use std::collections::HashMap;

use dioxus::prelude::*;

fn main() {}

fn AntipatternNestedFragments() -> Element {
    // ANCHOR: nested_fragments
    // ❌ Don't unnecessarily nest fragments
    let _ = rsx!(
        Fragment {
            Fragment {
                Fragment {
                    Fragment {
                        Fragment {
                            div { "Finally have a real node!" }
                        }
                    }
                }
            }
        }
    );

    // ✅ Render shallow structures
    rsx!(
        div { "Finally have a real node!" }
    )
    // ANCHOR_END: nested_fragments
}

#[derive(PartialEq, Props)]
struct NoKeysProps {
    data: HashMap<u32, String>,
}

fn AntipatternNoKeys(props: NoKeysProps) -> Element {
    // ANCHOR: iter_keys
    let data: &HashMap<_, _> = &props.data;

    // ❌ No keys
    rsx! {
        ul {
            data.values().map(|value| rsx!(
                li { "List item: {value}" }
            ))
        }
    };

    // ❌ Using index as keys
    rsx! {
        ul {
            cx.props.data.values().enumerate().map(|(index, value)| rsx!(
                li { key: "{index}", "List item: {value}" }
            ))
        }
    };

    // ✅ Using unique IDs as keys:
    rsx! {
        ul {
            cx.props.data.iter().map(|(key, value)| rsx!(
                li { key: "{key}", "List item: {value}" }
            ))
        }
    }
    // ANCHOR_END: iter_keys
}
