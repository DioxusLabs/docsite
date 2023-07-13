use crate::*;
use dioxus::prelude::*;

#[derive(PartialEq, Eq)]
pub struct BlogPost {
	pub category: &'static str,
	pub date: &'static str,
	pub title: &'static str,
	pub description: &'static str,
	pub link: Route,
	pub content: &'static str,
}

#[inline_props]
pub fn BlogList(cx: Scope) -> Element {
	cx.render(rsx!(
		section { class: "text-gray-600 body-font overflow-hidden dark:bg-ideblack",
			div { class: "container max-w-screen-lg pt-12 pb-12 mx-auto",
				div { class: "-my-8 px-8 pb-12",
					// Header
					BlogHeader {}
					section { class: "body-font overflow-hidden dark:bg-ideblack",
						div { class: "container px-6 mx-auto",
							div { class: "-my-8 divide-y-2 divide-gray-100",
								BlogRoute::static_routes().into_iter().map(|route| rsx! { BlogPostItem { route: route } })
							}
						}
					}
				}
			}
		}
	))
}

#[inline_props]
pub fn BlogPost(cx: Scope) -> Element {
	cx.render(rsx! {
		div { class: "w-full pt-12 text-sm dark:bg-ideblack", min_height: "100vh",
			div { class: "max-w-screen-2xl flex flex-row justify-between mx-auto dark:text-white",
				section { class: "text-gray-600 body-font overflow-hidden dark:bg-ideblack mx-auto container pt-12 pb-12 max-w-screen-md",
					div { class: "-my-8",
						div { class: "flex w-full mb-20 flex-wrap list-none",
							style {
								".markdown-body ul {{ list-style: disc; }}"
								".markdown-body li {{ display: list-item; }}"
							}
							article { class: "markdown-body pt-1",
								Outlet {}
							}
						}
					}
				}
			}
		}
	})
}

fn BlogHeader(cx: Scope) -> Element {
	cx.render(rsx!(
		section { class: "py-20",
			div { class: "container px-4 mx-auto dark:text-white",
				h2 { class: "text-5xl lg:text-6xl font-semibold font-heading text-center border-b border-gray-200 dark:border-gray-800 pb-8 max-w-screen-md mx-auto",
					"Technical Blog"
				}
			}
		}
	))
}

#[inline_props]
fn BlogPostItem(cx: Scope, route: BlogRoute) -> Element {
	let page = route.page();
	let title = &page.title;
	let link = route.clone();

	render! {
		div { class: "py-8 flex flex-wrap md:flex-nowrap",
			// div { class: "md:w-32 md:mb-0 mb-6 flex-shrink-0 flex flex-col",
			//	 span { class: "mt-1 text-gray-500 text-sm", "{date}" }
			// }
			div { class: "md:flex-grow",
				h2 { class: "text-2xl font-medium text-gray-900 title-font mb-2 dark:text-white",
					"{title}"
				}
				Link { class: "text-indigo-500 inline-flex items-center mt-2", target: Route::Blog { child: link.clone() },
					"Read more"
					icons::ArrowRight {}
				}
			}
		}
	}
}
