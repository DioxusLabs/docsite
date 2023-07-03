use dioxus::prelude::*;
use dioxus_router::Link;
use wasm_bindgen::prelude::wasm_bindgen;

const STAR_CACHE_NAME: &str = "STARS-";

#[derive(Props, PartialEq, PartialOrd, Clone)]
struct Item {
    name: String,
    description: String,
    category: String,
    
    /// Option GitHub Information
    /// Items won't display stars without this.
    github: Option<GithubInfo>,

    /// Optional external link
    /// Replaces the auto-generated github link with an external link.
    link: Option<String>,
}

#[derive(Default, PartialEq, PartialOrd, Clone)]
struct GithubInfo {
    username: String,
    repo: String,
}

#[derive(serde::Deserialize)]
struct StarsResponse {
    stargazers_count: u64,
}

pub fn Awesome(cx: Scope) -> Element {
    let mut ITEMS = vec![
        Item {
            name: "dioxus".to_string(),
            description: "React-like GUI library for desktop, web, mobile, TUI, and more.".to_string(),
            category: "util".to_string(),
            github: Some(GithubInfo { 
                username: "DioxusLabs".to_string(),
                repo: "dioxus".to_string(),
            }),
            link: None,
        },
        Item {
            name: "dioxus-std".to_string(),
            description: "A library to provide abstractions to access common utilities when developing Dioxus applications.".to_string(),
            category: "util".to_string(),
            github: Some(GithubInfo { 
                username: "DioxusLabs".to_string(),
                repo: "dioxus-std".to_string(),
            }),
            link: None,
        },
        Item {
            name: "dioxus-logger".to_string(),
            description: "A logging utility to provide a standard interface whether you're targeting web, desktop, or mobile.".to_string(),
            category: "logging".to_string(),
            github: Some(GithubInfo { 
                username: "DogeDark".to_string(),
                repo: "dioxus-logger".to_string(),
            }),
            link: None,
        },
        Item {
            name: "dioxus-sortable".to_string(),
            description: "Sortable tables for Dioxus.".to_string(),
            category: "components".to_string(),
            github: Some(GithubInfo { 
                username: "feral-dot-io".to_string(),
                repo: "dioxus-sortable".to_string(),
            }),
            link: None,
        },
        Item {
            name: "youtube".to_string(),
            description: "youtube cool".to_string(),
            category: "misc".to_string(),
            github: None,
            link: Some("https://youtube.com".to_string()),
        },
        Item {
            name: "nowhere".to_string(),
            description: "this goes nowhere".to_string(),
            category: "misc".to_string(),
            github: None,
            link: None,
        }
    ];

    let search = use_state(cx, || "".to_string());

    ITEMS = ITEMS.into_iter().filter(|i| i.name.to_lowercase().contains(&search.get().to_lowercase())).collect();

    cx.render(rsx!(
        section {
            class: "dark:bg-ideblack w-full pt-24 pb-10",
            div {
                class: "container mx-auto max-w-screen-1g text-center",
                p {
                    class: "text-[3.3em] font-bold tracking-tight dark:text-white text-ghdarkmetal mb-2 px-2",
                    "Awesome stuff for Dioxus"
                }
                p {
                    class: "mx-auto text-xl text-gray-600 dark:text-gray-400 pb-10 px-2 max-w-screen-sm",
                    "Everything you'll need to build awesome Dioxus apps."
                }
            }
            div {
                class: "container mx-auto",
                div {
                    class: "mx-2 rounded-lg lg:w-2/5 lg:mx-auto",
                    background_color: "#24292f",
                    input {
                        class: "w-full text-center p-4 rounded-lg text-gray-300",
                        background_color: "#24292f",
                        placeholder: "Looking for something specific?",
                        value: "{search}",
                        oninput: move |evt| search.set(evt.value.clone()),
                    }
                }
            }
        }

        section {
            class: "dark:bg-ideblack w-full pb-96",
            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 container mx-auto px-2 max-w-screen-1g",
                ITEMS.iter().map(|item| rsx!(AwesomeItem { item: item.clone() }))
            }
        }
    ))
}

#[inline_props]
fn AwesomeItem(cx: Scope, item: Item) -> Element {

    let is_github = item.github.is_some();
    let username = item.github.clone().unwrap_or(GithubInfo::default()).username;
    let repo = item.github.clone().unwrap_or(GithubInfo::default()).repo;

    let stars = use_future(cx, (), |_| async move {
        if is_github {
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
        }

        None
    });

    // Format stars text
    let stars = match stars.value() {
        Some(Some(v)) => format!("{} ⭐", v),
        _ => "N/A ⭐".to_string(),
    };

    // Figure out what link to use
    let link = match &item.link {
        Some(link) => link.clone(),
        None => {
            if let Some(github) = &item.github {
                format!("https://github.com/{}/{}", github.username, github.repo)
            } else {
                "https://dioxuslabs.com/404".to_string()
            }
        }
    };

    // Get the category to display
    let display_category = category_to_display(item.category.clone());

    cx.render(rsx!(
        Link {
            to: "{link}",
            div {
                class: "flex flex-col h-full p-3 rounded hover:-translate-y-2 transition-transform duration-300",
                background_color: "#24292f",
                div {
                    p {
                        class: "text-xl text-gray-100 font-bold",
                        "{item.name}"
                    }
                    p {
                        class: "text-base pt-2 text-gray-300",
                        "{item.description}"
                    }
                }
                div {
                    class: "mt-auto pt-4 flex",
                    p {
                        class: "text-gray-300 font-bold",
                        "{display_category}"
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

fn category_to_display(category: String) -> String {
    let result = match category.as_str() {
        "util" => "🧰 Util",
        "logging" => "📡 Logging",
        "components" => "📦 Components",
        "example" => "📝 Example",
        "styling" => "🎨 Styling",
        "ci/cd" => "🔁 CI/CD",
        _ => "📎 Misc"
    };
    result.to_string()
}

#[wasm_bindgen(module = "/src/components/awesome/storage.js")]
extern "C" {
    pub fn get_stars(name: String) -> Option<usize>;
    pub fn set_stars(name: String, stars: usize);
}
