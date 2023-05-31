pub use mdbook_macro::include_mdbook;
use once_cell::sync::Lazy;

pub type LazyMdbook = Lazy<MdBook>;

pub struct MdBook {
    pub summary: mdbook_shared::Summary,
}

impl MdBook {
    pub fn load(summary: &'static str) -> Self {
        let summary = serde_json::from_str::<mdbook_shared::Summary>(summary).unwrap();

        Self { summary }
    }
}
