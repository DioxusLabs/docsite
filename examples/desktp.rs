use dioxus_docs_site::App;

fn main() {
    dioxus::desktop::launch_with_props(App, (), |c| c)
}
