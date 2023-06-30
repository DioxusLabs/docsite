use dioxus::prelude::*;
use mdbook_macro::mdbook_router;

mdbook_router! {"../example-book"}

fn main() {
    for _ in 0.. 10000 {
        LAZY_BOOK.search_index.as_ref().unwrap().search("axum").unwrap();
    }
    let result = LAZY_BOOK.search_index.as_ref().unwrap().search("axum");
        println!("{:#?}", result);
}
