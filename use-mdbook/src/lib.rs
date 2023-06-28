use std::path::PathBuf;

use mdbook_shared::MdBook;
pub use mdbook_shared;

pub use mdbook_macro::*;
pub use once_cell::sync::Lazy;

pub type LazyMdbook = Lazy<MdBook<PathBuf>>;

pub fn static_load(contents: &'static str) -> MdBook<PathBuf> {
    serde_json::from_str(contents).unwrap()
}
