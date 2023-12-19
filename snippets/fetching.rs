//! Powerful data fetching library using macros
//!
//! Automatically tracks dependencies and caches results

#[component]
fn Tasks(cx: Scope, id: Uuid) -> Element {
    let content = use_fetch!(cx, "https://my.app.com/item/{id}");

    cx.render(match content.value() {
        Some(Ok(data)) => rsx! { pre { "Your data: {data}" } },
        Some(Err(e)) => rsx! {"An error loading the resource occurred."},
        None => rsx! { pre { "Loading..." } },
    })
}
