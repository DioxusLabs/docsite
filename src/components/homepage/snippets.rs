use super::super::snippets::*;
use dioxus::prelude::*;

pub static Snippets: Component<()> = |cx| {
    let (snippets, _) = use_state(&cx, build_snippets);
    let (selected_snippet, set_selected_snippet) = use_state(&cx, || 0);

    cx.render(rsx! {
        section { class: "text-gray-500 bg-white body-font mx-auto px-6 lg:px-24 xl:px-48 pt-12",
            div { class: "container flex flex-col md:flex-row w:2/3 px-6 py-10 mx-auto",
                div { class: "h-full resize-none flex-shrink-0 mb-6 pr-0 w-full md:w-auto md:text-left text-center rounded-lg shadow",
                    ul { class: "divide-y-2 divide-gray-100",
                        snippets.iter().enumerate().map(|(id, s)| {
                            let is_selected = if *selected_snippet == id {
                                "bg-blue-600 text-blue-200"
                            } else {
                                ""
                            };
                            rsx!(li {
                                key: "{s.title}",
                                cursor: "pointer",
                                class: "p-3 pr-8 hover:bg-blue-500 hover:text-blue-100 {is_selected}",
                                onclick: move |_| set_selected_snippet(id),
                                "{s.title}"
                            })
                        })
                    }
                }
                div { class: "flex flex-col md:pr-10 md:mb-0 mb-6 pr-0 w-full md:w-auto md:text-left text-center w:1/2 text-gray-800",
                    snippets.iter().enumerate().map(|(id, f)| {
                        let show = if id == *selected_snippet {"block"} else {"none"};
                        rsx!(div { style: "display: {show};", snippet( snippet: f ) })
                    })
                }
            }
        }
    })
};

#[derive(PartialEq, Props)]
pub struct SnippetProps<'a> {
    snippet: &'a Snippet,
}
fn snippet<'a>(cx: Scope<'a, SnippetProps<'a>>) -> Element {
    let Snippet {
        title,
        body,
        code,
        caller_id: _,
    } = &cx.props.snippet;

    cx.render(rsx! {
        section { class: "text-gray-600 body-font",
            div { class: "container px-5 mx-auto",
                div { class: "md:pr-12 md:mb-0 mb-10",
                    h1 { class: "sm:text-3xl text-2xl font-medium title-font mb-2 text-black",
                        "{title}"
                    }
                    body
                        .split('\n')
                        .map(|paragraph| rsx!( p{ class: "leading-relaxed text-base pb-4", "{paragraph}"} ))
                        .take(3)
                }
                div { class: "flex flex-col",
                    pre {
                        code {
                            class: "language-rust line-numbers",
                            "{code}"
                        }
                    }
                }
            }
        }
    })
}
