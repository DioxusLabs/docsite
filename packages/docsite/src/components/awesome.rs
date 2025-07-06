use crate::*;
use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::wasm_bindgen;

const ITEM_LIST_LINK: &str =
    "https://raw.githubusercontent.com/DioxusLabs/awesome-dioxus/master/awesome.json";
const STAR_CACHE_NAME: &str = "STARS-";

#[derive(Props, Clone, serde::Deserialize, PartialEq)]
struct Item {
    name: String,
    description: String,
    r#type: AwesomeType,
    category: Category,

    /// Option GitHub Information
    /// Items won't display stars without this.
    github: Option<GithubInfo>,

    /// Optional external link
    /// Replaces the auto-generated github link with an external link.
    link: Option<String>,
}

#[derive(Clone, serde::Deserialize, PartialEq)]
enum AwesomeType {
    Awesome,
    MadeWith,
}

#[derive(Default, Clone, serde::Deserialize, PartialEq)]
struct GithubInfo {
    username: String,
    repo: String,
}

#[derive(Clone, Copy, serde::Deserialize, PartialEq)]
enum Category {
    Misc,
    Util,
    Logging,
    Components,
    Example,
    Styling,
    Deployment,
    Renderer,
    App,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let converted = match self {
            Self::Misc => "📎 Misc",
            Self::Util => "🧰 Util",
            Self::Logging => "📡 Logging",
            Self::Components => "📦 Components",
            Self::Example => "📝 Example",
            Self::Styling => "🎨 Styling",
            Self::Deployment => "⚙️ Deployment",
            Self::Renderer => "🎥 Renderer",
            Self::App => "🚀 App",
        };

        write!(f, "{converted}")
    }
}

#[derive(serde::Deserialize)]
pub struct StarsResponse {
    pub stargazers_count: u64,
}

#[component]
pub(crate) fn Awesome() -> Element {
    let items = use_resource(move || async move {
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

    let content = match &*items.read_unchecked() {
        Some(Ok(items)) => {
            to_owned![items];
            items.sort_by(|a, b| {
                b.category
                    .to_string()
                    .to_lowercase()
                    .cmp(&a.category.to_string().to_lowercase())
            });

            rsx! {
                section { class: "w-full pt-4 md:pt-24 pb-10",
                    div { class: "mx-auto max-w-screen-1g text-center",
                        h1 { class: "text-[1.5em] md:text-5xl font-bold tracking-tight dark:text-white text-ghdarkmetal mb-2 px-2 ",
                            "Awesome stuff for Dioxus"
                        }
                        p { class: "mx-auto text-md lg:text-lg text-gray-600 dark:text-gray-400 pb-10 px-2 max-w-screen-md",
                            div {
                                "Everything you'll need to build awesome Dioxus apps. Also check out "
                                b {
                                    Link {
                                        class: "hover:text-sky-500 dark:hover:text-sky-400",
                                        to: "#made-with-dioxus",
                                        "Made with Dioxus"
                                    }
                                }
                                "!"
                            }
                            div { class: "pt-2",
                                "To submit your project, make a pull request in the "
                                b {
                                    Link {
                                        class: "hover:text-sky-500 dark:hover:text-sky-400",
                                        to: "https://github.com/DioxusLabs/awesome-dioxus",
                                        "awesome-dioxus"
                                    }
                                }
                                " repo."
                            }
                        }
                    }
                }
                section { class: "w-full pb-24",
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 container mx-auto px-2 max-w-screen-1g",
                        for item in items.iter() {
                            if let AwesomeType::Awesome = item.r#type {
                                AwesomeItem { key: "{item.name}", item: item.clone() }
                            }
                        }
                    }
                }
                section { class: " w-full pb-2 md:pb-10",
                    div { class: "container mx-auto max-w-screen-1g text-center",
                        h1 {
                            class: "text-[1.5em] md:text-[3.3em] font-bold tracking-tight dark:text-white text-ghdarkmetal mb-2 px-2",
                            id: "made-with-dioxus",
                            "Made with Dioxus"
                        }
                        p { class: "mx-auto text-xl text-gray-600 dark:text-gray-400 pb-10 px-2 max-w-screen-sm",
                            "Real world uses of Dioxus."
                        }
                    }
                }
                section { class: "w-full pb-24",
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 container mx-auto px-2 max-w-screen-1g",
                        for item in items.iter() {
                            if let AwesomeType::MadeWith = item.r#type {
                                AwesomeItem { key: "{item.name}", item: item.clone() }
                            }
                        }
                    }
                }
            }
        }
        Some(Err(e)) => {
            rsx! {
                section { class: "w-full pt-24 pb-96",
                    div { class: "container mx-auto max-w-screen-1g text-center animate-fadein-medium",
                        p { class: "text-[3.3em] font-bold tracking-tight dark:text-white text-ghdarkmetal mb-2 px-2",
                            "It seems a not-so-awesome error occurred. 🙁"
                        }
                        p { class: "mx-auto text-sm dark:text-gray-500 pb-10 px-2",
                            "{e}"
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                section { class: "w-full pt-24 pb-96",
                    div { class: "container mx-auto max-w-screen-1g text-center animate-fadein-medium",
                        p { class: "text-[3.3em] font-bold tracking-tight dark:text-white text-ghdarkmetal mb-2 px-2",
                            "Loading..."
                        }
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "mx-auto max-w-screen-lg", {content} }
    }
}

#[component]
fn AwesomeItem(item: ReadOnlySignal<Item>) -> Element {
    let stars = use_resource(move || async move {
        let item = item.read();
        let is_github = item.github.is_some();
        let username = item
            .github
            .clone()
            .unwrap_or(GithubInfo::default())
            .username;

        let repo = item.github.clone().unwrap_or(GithubInfo::default()).repo;

        if !is_github {
            return None;
        }

        // Check cache
        if let Some(stars) = get_stars(format!("{}{}/{}", STAR_CACHE_NAME, username, repo)) {
            return Some(stars);
        }

        // Not in cache or expired, lets get from github
        if let Ok(req) =
            reqwest::get(format!("https://api.github.com/repos/{username}/{repo}")).await
        {
            if let Ok(res) = req.json::<StarsResponse>().await {
                // Add to cache
                set_stars(
                    format!("{}{}/{}", STAR_CACHE_NAME, username, repo),
                    res.stargazers_count as usize,
                );
                return Some(res.stargazers_count as usize);
            }
        }

        None
    });

    // Format stars text
    let stars = match &*stars.value().read() {
        Some(Some(v)) => format!("{} ⭐", v),
        _ => "N/A ⭐".to_string(),
    };

    let item = item.read();

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

    rsx! {
        Link { to: NavigationTarget::<Route>::External(link), new_tab: true,
            div { class: "flex flex-col h-full p-3 rounded hover:-translate-y-2 transition-transform duration-300 shadow border border-gray-200",
                div {
                    p { class: "text-xl text-gray-800 dark:text-gray-100 font-bold",
                        "{item.name}"
                    }
                    p { class: "text-base pt-2 text-gray-700 dark:text-gray-400",
                        "{item.description}"
                    }
                }
                div { class: "mt-auto pt-4 flex",
                    if Category::App != item.category {
                        p { class: "text-gray-500 font-bold dark:text-gray-300", "{item.category}" }
                    }
                    p { class: "ml-auto text-gray-500 font-bold dark:text-gray-300",
                        "{stars}"
                    }
                }
            }
        }
    }
}

#[wasm_bindgen(module = "/src/components/storage.js")]
extern "C" {
    pub(crate) fn get_stars(name: String) -> Option<usize>;
    pub(crate) fn set_stars(name: String, stars: usize);
}
