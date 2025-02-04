pub fn generate_search_index() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::{static_dir, Route};
        use std::sync::atomic::{AtomicBool, Ordering};

        std::env::set_var("CARGO_MANIFEST_DIR", static_dir().join("assets"));
        dioxus_search::SearchIndex::<Route>::create(
            "searchable",
            dioxus_search::BaseDirectoryMapping::new(static_dir()),
        );
    }
}
