use std::{hash::Hash, path::PathBuf};

pub use mdbook_shared;
use mdbook_shared::MdBook;

pub use mdbook_macro::*;
pub use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;

pub type LazyMdbook = Lazy<MdBook<PathBuf>>;

pub fn static_load<L: DeserializeOwned + Hash + Eq>(contents: &[u8]) -> MdBook<L> {
    postcard::from_bytes(contents).unwrap()
}
