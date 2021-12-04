use dioxus_docs_site::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    dioxus::web::launch_with_props(App, (), |c| c)
}
