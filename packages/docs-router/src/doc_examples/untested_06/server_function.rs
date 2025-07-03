#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    launch(App)
}

fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
        button {
            onclick: move |_| {
                async move {
                    if let Ok(new_count) = double_server(count()).await {
                        count.set(new_count);
                    }
                }
            },
            "Double"
        }
    }
}

#[server]
async fn double_server(number: i32) -> Result<i32, ServerFnError> {
    // Perform some expensive computation or access a database on the server
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let result = number * 2;
    println!("server calculated {result}");
    Ok(result)
}
