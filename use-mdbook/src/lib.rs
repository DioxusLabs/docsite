use std::{collections::HashMap, path::PathBuf};

pub use mdbook_macro::include_mdbook;
use once_cell::sync::Lazy;

pub type LazyMdbook = Lazy<MdBook>;

pub struct MdBook {
    pub summary: mdbook_shared::Summary,

    // rendered pages to HTML
    pub pages: HashMap<PathBuf, String>,
}

pub struct Page {
    pub title: String,

    // rendered as HTML
    pub content: String,
}

pub fn static_load(summary: &'static str) -> MdBook {
    let summary = serde_json::from_str::<mdbook_shared::Summary>(summary).unwrap();

    MdBook {
        summary,
        pages: HashMap::new(),
    }
}
