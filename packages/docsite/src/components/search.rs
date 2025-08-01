pub fn generate_search_index() {
    use crate::{static_dir, Route};

    let out_dir = static_dir().join("assets");
    // Build the docsrs search index
    docsrs_search::build_into(&out_dir);

    std::env::set_var("CARGO_MANIFEST_DIR", &out_dir);
    let version_filter: &[(&str, fn(&Route) -> bool)] = &[
        ("0_3", |route| matches!(route, Route::Docs03 { .. })),
        ("0_4", |route| matches!(route, Route::Docs04 { .. })),
        ("0_5", |route| matches!(route, Route::Docs05 { .. })),
        ("0_6", |route| matches!(route, Route::Docs06 { .. })),
        ("0_7", |route| matches!(route, Route::Docs07 { .. })),
    ];
    for (version, filter) in version_filter.iter().copied() {
        dioxus_search::SearchIndex::<Route>::create(
            format!("searchable_{version}"),
            dioxus_search::BaseDirectoryMapping::new(static_dir()).map(|route| {
                filter(&route).then(|| {
                    let route = route.to_string();
                    println!("route: {route}");
                    let (route, _) = route.split_once('#').unwrap_or((&route, ""));
                    let (route, _) = route.split_once('?').unwrap_or((&route, ""));
                    std::path::PathBuf::from(route).join("index.html")
                })
            }),
        );
    }
}
