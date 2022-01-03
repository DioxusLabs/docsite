use dioxus_docs_site::App;

fn main() {
    console_error_panic_hook::set_once();
    dioxus::web::launch(App)
}
