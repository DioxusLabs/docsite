use dioxus_docs_site::app;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    dioxus::web::launch_with_props(app, (), |c| c)
}
