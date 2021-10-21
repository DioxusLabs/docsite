use dioxus_docs_site::{App, AppProps};

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    dioxus::web::launch_with_props(App, AppProps { route: "home" }, |c| c)
}
