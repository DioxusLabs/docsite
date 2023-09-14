use dioxus::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::*;

const ITEM_LIST_LINK: &str = "https://raw.githubusercontent.com/DioxusLabs/awesome-dioxus/master/awesome.json";
const STAR_CACHE_NAME: &str = "STARS-";

#[derive(Props, Clone, serde::Deserialize, PartialEq)]
struct Item {
    name: String,
    description: String,
    category: Category,
    
    /// Option GitHub Information
    /// Items won't display stars without this.
    github: Option<GithubInfo>,

    /// Optional external link
    /// Replaces the auto-generated github link with an external link.
    link: Option<String>,
}

#[derive(Default, Clone, serde::Deserialize, PartialEq)]
struct GithubInfo {
    username: String,
    repo: String,
}

#[derive(Clone, serde::Deserialize, PartialEq)]
enum Category {
    Util,
    Logging,
    Components,
    Example,
    Styling,
    Deployment,
    Renderer,
    Misc,
}

impl ToString for Category {
    fn to_string(&self) -> String {
        let result = match self{
            Self::Util => "🧰 Util",
            Self::Logging => "📡 Logging",
            Self::Components => "📦 Components",
            Self::Example => "📝 Example",
            Self::Styling => "🎨 Styling",
            Self::Deployment => "⚙️ Deployment",
            Self::Renderer => "🎥 Renderer",
            Self::Misc => "📎 Misc"
        };
        result.to_string()
    }
}

#[derive(serde::Deserialize)]
struct StarsResponse {
    stargazers_count: u64,
}

#[component]
pub fn Awesome(cx: Scope) -> Element {
    let items = use_future(cx, (), |_| async move {
        let req = match reqwest::get(ITEM_LIST_LINK).await {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };

        let items = match req.json::<Vec<Item>>().await {
            Ok(i) => i,
            Err(e) => return Err(e.to_string()),
        };

        Ok(items)
    });

    let search = use_state(cx, || "".to_string());

    match items.value() {
        Some(Ok(items)) => {
            to_owned![items];
            items.sort_by(|a, b| b.category.to_string().to_lowercase().cmp(&a.category.to_string().to_lowercase()));
            let items: Vec<Item> = items.into_iter().filter(|i| i.name.to_lowercase().contains(&search.get().to_lowercase())).collect();

            cx.render(rsx!(
                section {
                    class: "dark:bg-ideblack w-full pt-24 pb-10",
                    div {
                        class: "container mx-auto max-w-screen-1g text-center",
                        h1 {
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
                        items.iter().map(|item| rsx!(AwesomeItem { key: "{item.name}", item: item.clone() }))
                    }
                }
            ))
        }
        Some(Err(e)) => {
            cx.render(rsx!(
                section {
                    class: "dark:bg-ideblack w-full pt-24 pb-96",
                    div {
                        class: "container mx-auto max-w-screen-1g text-center animate-fadein-medium",
                        p {
                            class: "text-[3.3em] font-bold tracking-tight dark:text-white text-ghdarkmetal mb-2 px-2",
                            "It seems a not-so-awesome error occurred. 🙁"
                        }
                        p {
                            class: "mx-auto text-sm dark:text-gray-500 pb-10 px-2",
                            "{e}"
                        }
                    }
                }
            ))
        }
        None => {
            cx.render(rsx!(
                section {
                    class: "dark:bg-ideblack w-full pt-24 pb-96",
                    div {
                        class: "container mx-auto max-w-screen-1g text-center animate-fadein-medium",
                        p {
                            class: "text-[3.3em] font-bold tracking-tight dark:text-white text-ghdarkmetal mb-2 px-2",
                            "That's weird. There isn't anything awesome to show. 🙁"
                        }
                    }
                }
            ))
        }
    }
}

#[component]
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
    let display_category = item.category.to_string();
    cx.render(rsx!(
        Link {
            to: NavigationTarget::<Route>::External(link),
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

#[wasm_bindgen(module = "/src/components/awesome/storage.js")]
extern "C" {
    pub fn get_stars(name: String) -> Option<usize>;
    pub fn set_stars(name: String, stars: usize);
}
