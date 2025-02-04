use dioxus::prelude::*;
use syntect_html::syntect_html_fs;

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct Snippet {
    pub(crate) title: &'static str,
    pub(crate) filename: &'static str,
    pub(crate) html: &'static str,
}

pub(crate) static SNIPPETS: &[Snippet] = &[
    Snippet {
        title: "Hello world",
        filename: "readme.rs",
        html: syntect_html_fs!("./src/snippets/readme.rs"),
    },
    Snippet {
        title: "Components",
        filename: "components.rs",
        html: syntect_html_fs!("./src/snippets/components.rs"),
    },
    Snippet {
        title: "Async",
        filename: "async.rs",
        html: syntect_html_fs!("./src/snippets/async_.rs"),
    },
    Snippet {
        title: "Server",
        filename: "server.rs",
        html: syntect_html_fs!("./src/snippets/server.rs"),
    },
    Snippet {
        title: "Global State",
        filename: "global.rs",
        html: syntect_html_fs!("./src/snippets/global.rs"),
    },
];

pub(crate) fn Snippets() -> Element {
    let mut selected_snippet = use_signal(|| 0);

    rsx! {
        section { class: "dark:text-white mt-4 -mx-4 sm:mx-0 lg:mt-0 lg:col-span-7 xl:col-span-6",
            div { class: "relative overflow-hidden min-h-0 flex-auto flex-col flex bg-ghmetal max-h-[60vh] sm:max-h-[none] sm:rounded-xl dark:backdrop-blur border border-neutral-500/30 shadow-cutesy",
                div { class: "flex-none overflow-auto whitespace-nowrap flex relative min-w-full bg-ghdarkmetal pt-3 px-3",
                    ul { class: "flex text-sm leading-6 text-gray-100",
                        for (id , snippet) in SNIPPETS.iter().enumerate() {
                            li { class: "flex-none",
                                button {
                                    class: "relative py-2 px-4 rounded-t-md",
                                    class: if selected_snippet() == id { "bg-ghmetal border-neutral-500/30 border text-white border-b-0" } else { "bg-ghdarkmetal" },
                                    r#type: "button",
                                    onclick: move |_| selected_snippet.set(id),
                                    "{snippet.filename}"
                                    if selected_snippet() == id {
                                        span { class: "absolute z-10 bottom-0 inset-x-0 h-2 bg-ghmetal" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "absolute bottom-0 inset-x-0 h-px bg-neutral-500/30" }
                }

                div {
                    for (id , snippet) in SNIPPETS.iter().enumerate() {
                        div {
                            key: "{snippet.title}",
                            class: "w-full min-h-0 p-4",
                            // Instead of hiding/showing, we just render all the code blocks at once and hide them with css instead
                            class: if selected_snippet() == id { "block" } else { "hidden" },
                            background_color: "#2b303b",
                            dangerous_inner_html: "{snippet.html}",
                        }
                    }
                }
            }
        }
    }
}

// div { class: "relative overflow-hidden flex bg-neutral-800 max-h-[60vh] sm:max-h-[none] sm:rounded-xl dark:bg-neutral-900/70 dark:backdrop-blur dark:ring-1 dark:ring-inset dark:ring-white/10 shadow-3xl",
// div { class: "relative overflow-hidden flex bg-neutral-800 h-[31.625rem] max-h-[60vh] sm:max-h-[none] sm:rounded-xl lg:h-[34.6875rem] xl:h-[31.625rem] dark:bg-neutral-900/70 dark:backdrop-blur dark:ring-1 dark:ring-inset dark:ring-white/10 shadow-3xl",
// div { class: "relative w-full flex flex-col",
// div { class: "flex-none",
//     div { class: "flex items-center h-8 space-x-1.5 px-3",
//         div { class: "w-2.5 h-2.5 bg-red-600 rounded-full" }
//         div { class: "w-2.5 h-2.5 bg-yellow-600 rounded-full" }
//         div { class: "w-2.5 h-2.5 bg-green-600 rounded-full" }
//     }
// }
