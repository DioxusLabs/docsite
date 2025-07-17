use dioxus::prelude::*;

fn App() -> Element {
    todo!()
}

// ANCHOR: set_server_url
fn main() {
    #[cfg(not(feature = "server"))]
    server_fn::client::set_server_url("https://hot-dog.fly.dev");

    dioxus::launch(App);
}
// ANCHOR_END: set_server_url

// ANCHOR: stable_server_endpoints
#[server(endpoint = "list_dogs")]
pub async fn list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    todo!()
}

#[server(endpoint = "remove_dog")]
pub async fn remove_dog(id: usize) -> Result<(), ServerFnError> {
    todo!()
}

#[server(endpoint = "save_dog")]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    todo!()
}
// ANCHOR_END: stable_server_endpoints
