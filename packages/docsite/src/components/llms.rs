pub fn generate_llms_txt() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::Route;
        use std::fmt::Display;
        use dioxus::prelude::Routable;

        fn write_content_to_llm_txt(route: impl Display, content: &str) {
            let route = route.to_string();
            let (route, _) = route.split_once('#').unwrap_or((&route, ""));
            let (route, _) = route.split_once('?').unwrap_or((&route, ""));
            let path = std::path::PathBuf::from(route)
                .join("index")
                .join("llms.txt");
            _ = std::fs::create_dir_all(path.parent().unwrap());
            std::fs::write(path.to_str().unwrap(), content).expect("Failed to write llms.txt");
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
