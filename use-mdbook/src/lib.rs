use mdbook_shared::{MdBook, Summary};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

pub use mdbook_macro::include_mdbook;
use once_cell::sync::Lazy;

pub type LazyMdbook = Lazy<MdBook>;

pub fn static_load(contents: &'static str) -> MdBook {
    serde_json::from_str(contents).unwrap()
}
