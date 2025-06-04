//! Server functions let you write server-side code that can be called from the client as a normal async function.
use dioxus::prelude::*;

#[component]
fn Server() -> Element {
    let future = use_server_future(get_server_data)?;

    match future() {
        Some(Ok(data)) => rsx! { pre { "The server says: {data}" } },
        Some(Err(e)) => rsx! {"An error occurred: {e:?}"},
        None => rsx! {"Loading..."},
    }
}

#[server]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
