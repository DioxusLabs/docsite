use dioxus_docs_site::{App, AppProps};

fn main() {
    dioxus::desktop::launch_with_props(App, AppProps { route: "home" }, |c| c)
}
