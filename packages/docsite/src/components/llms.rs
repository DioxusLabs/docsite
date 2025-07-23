use dioxus::logger::tracing;

pub fn generate_llms_txt() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::static_dir;
        use crate::Route;
        use dioxus::prelude::Routable;
        use std::{
            fs::File,
            io::Write,
            path::{Path, PathBuf},
        };

        fn llms_full_txt(path: &Path, route: &str) -> std::io::Result<File> {
            let llms_full_txt = path.join("llms-full.txt");
            _ = std::fs::create_dir_all(path);
            let new_file = !std::fs::exists(&llms_full_txt)?;
            let mut file = File::options()
                .append(true)
                .create(true)
                .open(&llms_full_txt)?;

            if new_file {
                writeln!(file, "<SYSTEM>This is the developer documentation for Dioxus at {route} and all its subroutes.</SYSTEM>\n")?;
            }

            Ok(file)
        }

        fn route_to_path(route: &str) -> PathBuf {
            let route = route.trim_matches('/');
            let (route, _) = route.split_once('#').unwrap_or((&route, ""));
            let (route, _) = route.split_once('?').unwrap_or((&route, ""));
            static_dir().join(route)
        }

        fn write_content_to_llm_txt(doc_route: Route, content: &str) -> std::io::Result<()> {
            let route = doc_route.to_string();
            let path = route_to_path(&route);
            _ = std::fs::create_dir_all(&path);
            let path = path.join("llms.txt");
            let file = std::fs::File::options()
                .create(true)
                .write(true)
                .open(&path)?;
            let mut file = std::io::BufWriter::new(file);
            writeln!(file, "<SYSTEM>This is the developer documentation for Dioxus from {route}.</SYSTEM>\n{content}")?;
            file.write_all(content.as_bytes())?;

            // Move up the tree of routes and add the content to each parent route's llms-full.txt
            let mut current_doc_route = doc_route;
            while let Some(doc_route) = current_doc_route.parent() {
                // Only write the latest docs to the root llms-full.txt
                if (doc_route == Route::Homepage {}) && doc_route.is_latest_docs() {
                    break;
                }
                // Make sure we don't fall into a loop
                if current_doc_route == doc_route {
                    tracing::error!("Loop detected in route: {doc_route}");
                    break;
                }
                let path = route_to_path(&doc_route.to_string());
                let file = llms_full_txt(&path, &route)?;
                let mut file = std::io::BufWriter::new(file);
                file.write_all(content.as_bytes())?;
                file.write_all(b"\n")?;
                current_doc_route = doc_route;
            }

            Ok(())
        }

        fn write_route(route: Route) -> std::io::Result<()> {
            match route {
                Route::Docs03 { child } => {
                    let id = child.page_id();
                    let content = crate::docs::router_03::BookRoute::page_markdown(id);
                    write_content_to_llm_txt(route, content)?;
                }
                Route::Docs04 { child } => {
                    let id = child.page_id();
                    let content = crate::docs::router_04::BookRoute::page_markdown(id);
                    write_content_to_llm_txt(route, content)?;
                }
                Route::Docs05 { child } => {
                    let id = child.page_id();
                    let content = crate::docs::router_05::BookRoute::page_markdown(id);
                    write_content_to_llm_txt(route, content)?;
                }
                Route::Docs06 { child } => {
                    let id = child.page_id();
                    let content = crate::docs::router_06::BookRoute::page_markdown(id);
                    write_content_to_llm_txt(route, content)?;
                }
                Route::Docs07 { child } => {
                    let id = child.page_id();
                    let content = crate::docs::router_07::BookRoute::page_markdown(id);
                    write_content_to_llm_txt(route, content)?;
                }
                _ => {}
            }

            Ok(())
        }

        for route in crate::Route::static_routes() {
            write_route(route.clone()).expect(format!("Failed to write route: {route}").as_str());
        }
    }
}
