#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

fn main() {
	launch!(@([127, 0, 0, 1], 8080), app, {});
}

fn app(cx: Scope) -> Element {
	let mut count = use_future(cx, (), |_| async { get_server_data().await });

	cx.render(rsx! {
		"server data is {count.value():?}"
	})
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
	// Access a database
	tokio::time::sleep(std::time::Duration::from_millis(100)).await;
	Ok("Hello from the server!".to_string())
}