pub fn generate_search_index() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::{static_dir, Route};
        use std::sync::atomic::{AtomicBool, Ordering};

        static INDEX_FRESH: AtomicBool = AtomicBool::new(false);
        let index_fresh = INDEX_FRESH.swap(true, Ordering::Relaxed);
        if !index_fresh {
            std::env::set_var("CARGO_MANIFEST_DIR", static_dir().join("assets"));
            dioxus_search::SearchIndex::<Route>::create(
                "searchable",
                dioxus_search::BaseDirectoryMapping::new(static_dir()),
            );
        }
    }
}
