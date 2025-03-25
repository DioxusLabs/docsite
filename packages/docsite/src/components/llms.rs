pub fn generate_llms_txt() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::static_dir;
        use crate::Route;
        use dioxus::prelude::Routable;
        use std::fmt::Display;

        fn write_content_to_llm_txt(route: impl Display, content: &str) {
            let route = route.to_string();
            let route = route.trim_matches('/');
            let (route, _) = route.split_once('#').unwrap_or((&route, ""));
            let (route, _) = route.split_once('?').unwrap_or((&route, ""));
            let path = static_dir().join(route).join("index").join("llms.txt");
            _ = std::fs::create_dir_all(path.parent().unwrap());
            std::fs::write(&path, content).unwrap_or_else(|err| {
                panic!("Failed to write llms.txt to {}: {}", path.display(), err)
            });
        }
        for route in crate::Route::static_routes() {
            match route {
                Route::Docs03 { child } => {
                    let id = child.page_id();
                    let content = dioxus_docs_router::docs::router_03::BookRoute::page_markdown(id);
                    write_content_to_llm_txt(route, &content);
                }
                Route::Docs04 { child } => {
                    let id = child.page_id();
                    let content = dioxus_docs_router::docs::router_04::BookRoute::page_markdown(id);
                    write_content_to_llm_txt(route, &content);
                }
                Route::Docs05 { child } => {
                    let id = child.page_id();
                    let content = dioxus_docs_router::docs::router_05::BookRoute::page_markdown(id);
                    write_content_to_llm_txt(route, &content);
                }
                Route::Docs06 { child } => {
                    let id = child.page_id();
                    let content = dioxus_docs_router::docs::router_06::BookRoute::page_markdown(id);
                    write_content_to_llm_txt(route, &content);
                }
                _ => {}
            }
        }
    }
}
