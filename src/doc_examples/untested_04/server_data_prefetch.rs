#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

fn main() {
    LaunchBuilder::new(app).launch();
}

fn app(cx: Scope) -> Element {
    let mut count = use_server_future(cx, (), |_| async { get_server_data().await })?;

    cx.render(rsx! {
        "server data is {count.value():?}"
    })
}

#[server]
async fn get_server_data() -> Result<String, ServerFnError> {
    // Access a database
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Ok("Hello from the server!".to_string())
}
