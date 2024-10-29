#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

fn main() {
    let config = LaunchBuilder::new(App);
    #[cfg(feature = "ssr")]
    let config = config.incremental(
        IncrementalRendererConfig::default().invalidate_after(std::time::Duration::from_secs(120)),
    );

    config.launch();
}

fn App(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
        button {
            onclick: move |_| {
                to_owned![count];
                async move {
                    // Call the server function just like a local async function
                    if let Ok(new_count) = double_server(*count.current()).await {
                        count.set(new_count);
                    }
                }
            },
            "Double"
        }
    })
}

#[server]
async fn double_server(number: i32) -> Result<i32, ServerFnError> {
    // Perform some expensive computation or access a database on the server
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let result = number * 2;
    println!("server calculated {result}");
    Ok(result)
}
