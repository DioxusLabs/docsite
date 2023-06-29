mod docs {
    use dioxus::prelude::*;
    use mdbook_macro::mdbook_router;

    mdbook_router! {"../example-book"}
}

fn main() {}
