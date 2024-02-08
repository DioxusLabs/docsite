use crate::dioxus_core::DynamicNode;
use crate::dioxus_core::VComponent;
use crate::icons;
use dioxus::prelude::*;

static IconsSet: &[Component<()>; 8] = &[
    icons::Icon1,
    icons::Icon2,
    icons::Icon3,
    icons::Icon4,
    icons::Icon5,
    icons::Icon6,
    icons::Icon1,
    icons::Icon2,
];

static FEATURES: &[(&str, &str)] = &[
    ("Declarative", "Easily describe the layout of your application with HTML or RSX syntax."),
    ("Component-Based", "Build encapsulated components that manage their own state, then compose them to make complex UIs."),
    ("Learn Once, Write Anywhere", "Components and hooks can be reused to render on the web, desktop, mobile, server, and more!"),
    ("Concurrent and Async", "1st class support for asynchronous tasks, suspense for datafetching, and pauseable coroutines."),
    ("Static Types Everywhere", "Eliminate a whole class of bugs at compile time with static typing for every library."),
    ("If it compiles, it works", "Fearlessly refactor even the largest of apps with powerful compile-time guarantees."),
    ("First-class error handling", "No more uncaught exceptions. Components can easily abort rendering without crashing the entire app."),
    ("Incredible inline documentation", "Comprehensive doc comments provide MDN hints and guides right under your finger tips."),
];

pub fn ValueAdd() -> Element {
    rsx! {
        section { class: "text-gray-600 body-font",
            div { class: "container mx-auto py-12 px-6 lg:px-40",
                div { class: "flex flex-wrap sm:-m-4 -mx-4 -mb-10 -mt-4 md:space-y-0 space-y-6 ",
                    for (idx , (title , content)) in FEATURES.iter().enumerate() {
                        div { key: "{title}", class: "p-4 md:w-1/4 flex",
                            div { class: "flex-grow lg:pl-6",
                                {DynamicNode::Component(VComponent::new(IconsSet[idx], (), title))},
                                a { href: "#",
                                    h2 { class: "dark:text-white text-gray-800 text-lg title-font font-medium mb-2",
                                        "{title}"
                                    }
                                }
                                {content.split('\n').map(|line| rsx!{p { key: "{line}", class: "leading-relaxed text-gray-700 text-base pb-4 dark:text-white", "{line}" }})}
                            }
                        }
                    }
                }
            }
        }
    }
}
