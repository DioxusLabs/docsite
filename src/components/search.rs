use dioxus::prelude::*;

/// we use a component at the end of the router to generate the search index once the rest
/// of the pages have been ssr-ed during SSG.
#[component]
pub fn Search() -> Element {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::{static_dir, Route};
        use once_cell::sync::Lazy;

        static _INDEX: Lazy<bool> = Lazy::new(|| {
            std::env::set_var("CARGO_MANIFEST_DIR", static_dir().join("assets"));
            dioxus_search::SearchIndex::<Route>::create(
                "searchable",
                dioxus_search::BaseDirectoryMapping::new(static_dir()),
            );

            true
        });

        assert!(*_INDEX == true);
    }

    rsx! {}
}
