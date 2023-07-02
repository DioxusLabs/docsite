use dioxus::prelude::*;
use dioxus_router::Link;
use wasm_bindgen::prelude::wasm_bindgen;

const STAR_CACHE_NAME: &str = "STARS-";

#[derive(Props)]
struct Item<'a> {
    name: &'a str,
    description: &'a str,
    github_username: &'a str,
    github_repo: &'a str,
    category: &'a str,
}

const ITEMS: &[Item] = &[
    Item {
        name: "dioxus",
        description: "React-like GUI library for desktop, web, mobile, TUI, and more.",
        github_username: "DioxusLabs",
        github_repo: "dioxus",
        category: "🧰 Util",
    },
    Item {
        name: "dioxus-std",
        description: "A library to provide abstractions to access common utilities when developing Dioxus applications.",
        github_username: "DioxusLabs",
        github_repo: "dioxus-std",
        category: "🧰 Util",
    },
    Item {
        name: "dioxus-logger",
        description: "A logging utility to provide a standard interface whether you're targeting web, desktop, or mobile.",
        github_username: "DogeDark",
        github_repo: "dioxus-logger",
        category: "📡 Logging"
    },
    Item {
        name: "dioxus-sortable",
        description: "Sortable tables for Dioxus.",
        github_username: "feral-dot-io",
        github_repo: "dioxus-sortable",
        category: "📦 Components"
    }
];

#[derive(serde::Deserialize)]
struct StarsResponse {
    stargazers_count: u64,
}

pub fn Awesome(cx: Scope) -> Element {
    cx.render(rsx!(
        section {
            class: "dark:bg-ideblack w-full pt-24 pb-10",
            div {
                class: "container mx-auto w-1/3 rounded-lg",
                background_color: "#24292f",
                input {
                    class: "w-full text-center p-4 rounded-lg text-gray-300",
                    background_color: "#24292f",
                    placeholder: "Looking for something specific?"
                }
            }
        }

        section {
            class: "dark:bg-ideblack w-full pb-36",
            div {
                class: "grid grid-cols-3 gap-6 container mx-auto max-w-screen-1g",
                ITEMS.iter().map(|item| rsx!(AwesomeItem {
                    name: item.name,
                    description: item.description,
                    github_username: item.github_username,
                    github_repo: item.github_repo,
                    category: item.category,
                }))
            }
        }
    ))
}

fn AwesomeItem<'a>(cx: Scope<'a, Item<'a>>) -> Element {
    let username = cx.props.github_username.to_string();
    let repo = cx.props.github_repo.to_string();

    let stars = use_future(cx, (), |_| async move {
        // Check cache
        if let Some(stars) = get_stars(format!("{}{}/{}", STAR_CACHE_NAME, username, repo)) {
            return Some(stars);
        }

        // Not in cache or expired, lets get from github
        if let Ok(req) = reqwest::get(format!("https://api.github.com/repos/{username}/{repo}")).await {
            if let Ok(res) = req.json::<StarsResponse>().await {
                // Add to cache
                
                set_stars(format!("{}{}/{}", STAR_CACHE_NAME, username, repo), res.stargazers_count as usize);
                return Some(res.stargazers_count as usize);
            }
        }
        None
    });

    let stars = match stars.value() {
        Some(Some(v)) => format!("{} ⭐", v),
        _ => "N/A ⭐".to_string(),
    };

    cx.render(rsx!(
        Link {
            to: "https://github.com/{cx.props.github_username}/{cx.props.github_repo}",
            div {
                class: "flex flex-col h-full p-3 rounded hover:-translate-y-4 transition-transform duration-300",
                background_color: "#24292f",
                div {
                    p {
                        class: "text-xl text-gray-100 font-bold",
                        "{cx.props.name}"
                    }
                    p {
                        class: "text-base pt-2 text-gray-300",
                        "{cx.props.description}"
                    }
                }
                div {
                    class: "mt-auto pt-4 flex",
                    p {
                        class: "text-gray-300 font-bold",
                        "{cx.props.category}"
                    }
                    p {
                        class: "ml-auto text-gray-300 font-bold",
                        "{stars}"
                    }
                }
                
            }
        }
    ))
}

#[wasm_bindgen(module = "/src/components/awesome/storage.js")]
extern "C" {
    pub fn get_stars(name: String) -> Option<usize>;
    pub fn set_stars(name: String, stars: usize);
}
