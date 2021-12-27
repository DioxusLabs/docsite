use dioxus_docs_site::App;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    dioxus::web::launch_with_props(App, (), |c| c)
}
