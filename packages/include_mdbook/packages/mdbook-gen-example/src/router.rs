#[derive(
    Clone,
    Copy,
    dioxus_router::Routable,
    PartialEq,
    Eq,
    Hash,
    Debug,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum BookRoute {
    #[route("/./chapter_1#:section")]
    Chapter1 { section: Chapter1Section },
    #[route("/./chapter_2#:section")]
    Chapter2 { section: Chapter2Section },
    #[route("/./chapter_3#:section")]
    Chapter3 { section: Chapter3Section },
}
impl BookRoute {
    /// Get the markdown for a page by its ID
    pub const fn page_markdown(id: use_mdbook::mdbook_shared::PageId) -> &'static str {
        match id.0 {
            0usize => {
                "# Liveview\n\nLiveview allows apps to *run* on the server and *render* in the browser. It uses WebSockets to communicate between the server and the browser.\n\nExamples:\n\n* [Axum Example](https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/axum.rs)\n* [Salvo Example](https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/salvo.rs)\n* [Warp Example](https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/warp.rs)\n\n## Support\n\nLiveview is currently limited in capability when compared to the Web platform. Liveview apps run on the server in a native thread. This means that browser APIs are not available, so rendering WebGL, Canvas, etc is not as easy as the Web. However, native system APIs are accessible, so streaming, WebSockets, filesystem, etc are all viable APIs.\n\n## Setup\n\nFor this guide, we're going to show how to use Dioxus Liveview with [Axum](https://docs.rs/axum/latest/axum/).\n\nMake sure you have Rust and Cargo installed, and then create a new project:\n\n````shell\ncargo new --bin demo\ncd app\n````\n\nAdd Dioxus and the liveview renderer with the Axum feature as dependencies:\n\n````shell\ncargo add dioxus\ncargo add dioxus-liveview --features axum\n````\n\nNext, add all the Axum dependencies. This will be different if you're using a different Web Framework\n\n````\ncargo add tokio --features full\ncargo add axum\n````\n\nYour dependencies should look roughly like this:\n\n````toml\n[dependencies]\naxum = \"0.4.5\"\ndioxus = { version = \"*\" }\ndioxus-liveview = { version = \"*\", features = [\"axum\"] }\ntokio = { version = \"1.15.0\", features = [\"full\"] }\n````\n\n````rs@included_example.rs\nfn it_works() {}\n\n````\n\n````sh\n{\"timestamp\":\"   9.927s\",\"level\":\"INFO\",\"message\":\"Bundled app successfully!\",\"target\":\"dx::cli::bundle\"}\n{\"timestamp\":\"   9.927s\",\"level\":\"INFO\",\"message\":\"App produced 2 outputs:\",\"target\":\"dx::cli::bundle\"}\n{\"timestamp\":\"   9.927s\",\"level\":\"INFO\",\"message\":\"app - [target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app]\",\"target\":\"dx::cli::bundle\"}\n{\"timestamp\":\"   9.927s\",\"level\":\"INFO\",\"message\":\"dmg - [target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg]\",\"target\":\"dx::cli::bundle\"}\n{\"timestamp\":\"   9.927s\",\"level\":\"DEBUG\",\"json\":\"{\\\"BundleOutput\\\":{\\\"bundles\\\":[\\\"target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app\\\"]}}\"}\n````"
            }
            1usize => {
                "# Roadmap & Feature-set\n\nThis feature set and roadmap can help you decide if what Dioxus can do today works for you.\n\nIf a feature that you need doesn't exist or you want to contribute to projects on the roadmap, feel free to get involved by [joining the discord](https://discord.gg/XgGxMSkvUM).\n\nGenerally, here's the status of each platform:\n\n* **Web**: Dioxus is a great choice for pure web-apps – especially for CRUD/complex apps. However, it does lack the ecosystem of React, so you might be missing a component library or some useful hook.\n\n* **SSR**: Dioxus is a great choice for pre-rendering, hydration, and rendering HTML on a web endpoint. Be warned – the VirtualDom is not (currently) `Send + Sync`.\n\n* **Desktop**: You can build very competent single-window desktop apps right now. However, multi-window apps require support from Dioxus core and are not ready.\n\n* **Mobile**: Mobile support is very young. You'll be figuring things out as you go and there are not many support crates for peripherals.\n\n* **LiveView**: LiveView support is very young. You'll be figuring things out as you go. Thankfully, none of it is too hard and any work can be upstreamed into Dioxus.\n\n````rust\nfn main() {\n  dioxus_rocks;\n}\n````\n\n## Features\n\n---\n\n|Feature|Status|Description|\n|-------|------|-----------|\n|Conditional Rendering|✅|if/then to hide/show component|\n|Map, Iterator|✅|map/filter/reduce to produce rsx!|\n|Keyed Components|✅|advanced diffing with keys|\n|Web|✅|renderer for web browser|\n|Desktop (webview)|✅|renderer for desktop|\n|Shared State (Context)|✅|share state through the tree|\n|Hooks|✅|memory cells in components|\n|SSR|✅|render directly to string|\n|Component Children|✅|cx.children() as a list of nodes|\n|Headless components|✅|components that don't return real elements|\n|Fragments|✅|multiple elements without a real root|\n|Manual Props|✅|Manually pass in props with spread syntax|\n|Controlled Inputs|✅|stateful wrappers around inputs|\n|CSS/Inline Styles|✅|syntax for inline styles/attribute groups|\n|Custom elements|✅|Define new element primitives|\n|Suspense|✅|schedule future render from future/promise|\n|Integrated error handling|✅|Gracefully handle errors with ? syntax|\n|NodeRef|✅|gain direct access to nodes|\n|Re-hydration|✅|Pre-render to HTML to speed up first contentful paint|\n|Jank-Free Rendering|✅|Large diffs are segmented across frames for silky-smooth transitions|\n|Effects|✅|Run effects after a component has been committed to render|\n|Portals|🛠|Render nodes outside of the traditional tree structure|\n|Cooperative Scheduling|🛠|Prioritize important events over non-important events|\n|Server Components|🛠|Hybrid components for SPA and Server|\n|Bundle Splitting|👀|Efficiently and asynchronously load the app|\n|Lazy Components|👀|Dynamically load the new components as the page is loaded|\n|1st class global state|✅|redux/recoil/mobx on top of context|\n|Runs natively|✅|runs as a portable binary w/o a runtime (Node)|\n|Subtree Memoization|✅|skip diffing static element subtrees|\n|High-efficiency templates|✅|rsx! calls are translated to templates on the DOM's side|\n|Compile-time correct|✅|Throw errors on invalid template layouts|\n|Heuristic Engine|✅|track component memory usage to minimize future allocations|\n|Fine-grained reactivity|👀|Skip diffing for fine-grain updates|\n\n* ✅ = implemented and working\n* 🛠 = actively being worked on\n* 👀 = not yet implemented or being worked on\n\n## Roadmap\n\nThese Features are planned for the future of Dioxus:\n\n### Core\n\n* [x] Release of Dioxus Core\n* [x] Upgrade documentation to include more theory and be more comprehensive\n* [x] Support for HTML-side templates for lightning-fast dom manipulation\n* [ ] Support for multiple renderers for same virtualdom (subtrees)\n* [ ] Support for ThreadSafe (Send + Sync)\n* [ ] Support for Portals\n\n### SSR\n\n* [x] SSR Support + Hydration\n* [ ] Integrated suspense support for SSR\n\n### Desktop\n\n* [ ] Declarative window management\n* [ ] Templates for building/bundling\n* [ ] Access to Canvas/WebGL context natively\n\n### Mobile\n\n* [ ] Mobile standard library\n  * [ ] GPS\n  * [ ] Camera\n  * [ ] filesystem\n  * [ ] Biometrics\n  * [ ] WiFi\n  * [ ] Bluetooth\n  * [ ] Notifications\n  * [ ] Clipboard\n* [ ] Animations\n\n### Bundling (CLI)\n\n* [x] Translation from HTML into RSX\n* [x] Dev server\n* [x] Live reload\n* [x] Translation from JSX into RSX\n* [ ] Hot module replacement\n* [ ] Code splitting\n* [ ] Asset macros\n* [ ] Css pipeline\n* [ ] Image pipeline\n\n### Essential hooks\n\n* [x] Router\n* [x] Global state management\n* [ ] Resize observer\n\n## Work in Progress\n\n### Build Tool\n\nWe are currently working on our own build tool called [Dioxus CLI](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli) which will support:\n\n* an interactive TUI\n* on-the-fly reconfiguration\n* hot CSS reloading\n* two-way data binding between browser and source code\n* an interpreter for `rsx!`\n* ability to publish to github/netlify/vercel\n* bundling for iOS/Desktop/etc\n\n### Server Component Support\n\nWhile not currently fully implemented, the expectation is that LiveView apps can be a hybrid between Wasm and server-rendered where only portions of a page are \"live\" and the rest of the page is either server-rendered, statically generated, or handled by the host SPA.\n\n### Native rendering\n\nWe are currently working on a native renderer for Dioxus using WGPU called [Blitz](https://github.com/DioxusLabs/blitz/). This will allow you to build apps that are rendered natively for iOS, Android, and Desktop.\n\n## Internal Links\n\nInternal links like [this](./chapter_1.md) are typechecked and will fail to compile if the file is not found."
            }
            2usize => {
                "# Assets\n\nSome assets:\n![some_external](https://avatars.githubusercontent.com/u/79236386?s=200&v=4)\n![some_local](/example-book/assets/logo.png)\n![some_local1](/example-book/assets/logo1.png)\n![some_local2](/example-book/assets/logo2.png)"
            }
            _ => panic!("Invalid page ID:"),
        }
    }
    pub fn sections(&self) -> &'static [use_mdbook::mdbook_shared::Section] {
        &self.page().sections
    }
    pub fn page(&self) -> &'static use_mdbook::mdbook_shared::Page<Self> {
        LAZY_BOOK.get_page(self)
    }
    pub fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        match self {
            BookRoute::Chapter1 { .. } => use_mdbook::mdbook_shared::PageId(0usize),
            BookRoute::Chapter2 { .. } => use_mdbook::mdbook_shared::PageId(1usize),
            BookRoute::Chapter3 { .. } => use_mdbook::mdbook_shared::PageId(2usize),
        }
    }
}
impl Default for BookRoute {
    fn default() -> Self {
        BookRoute::Chapter1 {
            section: Chapter1Section::Empty,
        }
    }
}
pub static LAZY_BOOK: use_mdbook::Lazy<use_mdbook::mdbook_shared::MdBook<BookRoute>> =
    use_mdbook::Lazy::new(|| {
        let mut page_id_mapping = ::std::collections::HashMap::new();
        let mut pages = Vec::new();
        let __push_page_0: fn(_, _) =
            |_pages: &mut Vec<_>, _page_id_mapping: &mut std::collections::HashMap<_, _>| {
                _pages.push((0usize, {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Chapter 1".to_string(),
                        url: BookRoute::Chapter1 {
                            section: Chapter1Section::Empty,
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Liveview".to_string(),
                                id: "liveview".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Support".to_string(),
                                id: "support".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Setup".to_string(),
                                id: "setup".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(0usize),
                    }
                }));
                _page_id_mapping.insert(
                    BookRoute::Chapter1 {
                        section: Chapter1Section::Empty,
                    },
                    ::use_mdbook::mdbook_shared::PageId(0usize),
                );
            };
        __push_page_0(&mut pages, &mut page_id_mapping);
        let __push_page_1: fn(_, _) =
            |_pages: &mut Vec<_>, _page_id_mapping: &mut std::collections::HashMap<_, _>| {
                _pages.push((1usize, {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Chapter 2".to_string(),
                        url: BookRoute::Chapter2 {
                            section: Chapter2Section::Empty,
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Roadmap & Feature-set".to_string(),
                                id: "roadmap--feature-set".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Features".to_string(),
                                id: "features".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Roadmap".to_string(),
                                id: "roadmap".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Core".to_string(),
                                id: "core".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "SSR".to_string(),
                                id: "ssr".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Desktop".to_string(),
                                id: "desktop".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Mobile".to_string(),
                                id: "mobile".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Bundling (CLI)".to_string(),
                                id: "bundling-cli".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Essential hooks".to_string(),
                                id: "essential-hooks".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Work in Progress".to_string(),
                                id: "work-in-progress".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Build Tool".to_string(),
                                id: "build-tool".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Server Component Support".to_string(),
                                id: "server-component-support".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Native rendering".to_string(),
                                id: "native-rendering".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Internal Links".to_string(),
                                id: "internal-links".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(1usize),
                    }
                }));
                _page_id_mapping.insert(
                    BookRoute::Chapter2 {
                        section: Chapter2Section::Empty,
                    },
                    ::use_mdbook::mdbook_shared::PageId(1usize),
                );
            };
        __push_page_1(&mut pages, &mut page_id_mapping);
        let __push_page_2: fn(_, _) =
            |_pages: &mut Vec<_>, _page_id_mapping: &mut std::collections::HashMap<_, _>| {
                _pages.push((2usize, {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Chapter 3".to_string(),
                        url: BookRoute::Chapter3 {
                            section: Chapter3Section::Empty,
                        },
                        segments: vec![],
                        sections: vec![::use_mdbook::mdbook_shared::Section {
                            title: "Assets".to_string(),
                            id: "assets".to_string(),
                            level: 1usize,
                        }],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(2usize),
                    }
                }));
                _page_id_mapping.insert(
                    BookRoute::Chapter3 {
                        section: Chapter3Section::Empty,
                    },
                    ::use_mdbook::mdbook_shared::PageId(2usize),
                );
            };
        __push_page_2(&mut pages, &mut page_id_mapping);
        ::use_mdbook::mdbook_shared::MdBook {
            summary: ::use_mdbook::mdbook_shared::Summary {
                title: Some("Summary".to_string()),
                prefix_chapters: vec![],
                numbered_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(
                        ::use_mdbook::mdbook_shared::Link {
                            name: "Chapter 1".to_string(),
                            location: Some(BookRoute::Chapter1 {
                                section: Chapter1Section::Empty,
                            }),
                            number: Some(::use_mdbook::mdbook_shared::SectionNumber(vec![1u32])),
                            nested_items: vec![],
                        },
                    ),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(
                        ::use_mdbook::mdbook_shared::Link {
                            name: "Chapter 2".to_string(),
                            location: Some(BookRoute::Chapter2 {
                                section: Chapter2Section::Empty,
                            }),
                            number: Some(::use_mdbook::mdbook_shared::SectionNumber(vec![2u32])),
                            nested_items: vec![],
                        },
                    ),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(
                        ::use_mdbook::mdbook_shared::Link {
                            name: "Chapter 3".to_string(),
                            location: Some(BookRoute::Chapter3 {
                                section: Chapter3Section::Empty,
                            }),
                            number: Some(::use_mdbook::mdbook_shared::SectionNumber(vec![3u32])),
                            nested_items: vec![],
                        },
                    ),
                ],
                suffix_chapters: vec![],
            },
            pages: pages.into_iter().collect(),
            page_id_mapping,
        }
    });
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum Chapter1Section {
    #[default]
    Empty,
    Liveview,
    Support,
    Setup,
}
impl std::str::FromStr for Chapter1Section {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "liveview" => Ok(Self::Liveview),
            "support" => Ok(Self::Support),
            "setup" => Ok(Self::Setup),
            _ => {
                Err("Invalid section name. Expected one of Chapter1Sectionliveview, support, setup")
            }
        }
    }
}
impl std::fmt::Display for Chapter1Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Liveview => f.write_str("liveview"),
            Self::Support => f.write_str("support"),
            Self::Setup => f.write_str("setup"),
        }
    }
}
#[component(no_case_check)]
pub fn Chapter1(section: Chapter1Section) -> Element {
    rsx! {
        h1 { id : "liveview", Link { to : BookRoute::Chapter1 { section :
        Chapter1Section::Liveview, }, class : "header", "Liveview" } } p {
        "Liveview allows apps to " em { "run" } " on the server and " em { "render" }
        " in the browser. It uses WebSockets to communicate between the server and the browser."
        } p { "Examples:" } ul { li { Link { to :
        "https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/axum.rs",
        "Axum Example" } } li { Link { to :
        "https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/salvo.rs",
        "Salvo Example" } } li { Link { to :
        "https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/warp.rs",
        "Warp Example" } } } h2 { id : "support", Link { to : BookRoute::Chapter1 {
        section : Chapter1Section::Support, }, class : "header", "Support" } } p {
        "Liveview is currently limited in capability when compared to the Web platform. Liveview apps run on the server in a native thread. This means that browser APIs are not available, so rendering WebGL, Canvas, etc is not as easy as the Web. However, native system APIs are accessible, so streaming, WebSockets, filesystem, etc are all viable APIs."
        } h2 { id : "setup", Link { to : BookRoute::Chapter1 { section :
        Chapter1Section::Setup, }, class : "header", "Setup" } } p {
        "For this guide, we're going to show how to use Dioxus Liveview with " Link { to
        : "https://docs.rs/axum/latest/axum/", "Axum" } "." } p {
        "Make sure you have Rust and Cargo installed, and then create a new project:" }
        CodeBlock { source : { ::dioxus_code::code_str!("cargo new --bin demo\ncd app",
        ::dioxus_code::CodeOptions::builder()
        .with_language(::dioxus_code::Language::Bash)) }, } p {
        "Add Dioxus and the liveview renderer with the Axum feature as dependencies:" }
        CodeBlock { source : {
        ::dioxus_code::code_str!("cargo add dioxus\ncargo add dioxus-liveview --features axum",
        ::dioxus_code::CodeOptions::builder()
        .with_language(::dioxus_code::Language::Bash)) }, } p {
        "Next, add all the Axum dependencies. This will be different if you're using a different Web Framework"
        } CodeBlock { source : {
        ::dioxus_code::advanced::HighlightedSource::from_static_parts("cargo add tokio --features full\ncargo add axum",
        ::dioxus_code::Language::Rust, & [],) }, } p {
        "Your dependencies should look roughly like this:" } CodeBlock { source : {
        ::dioxus_code::code_str!("[dependencies]\naxum = \"0.4.5\"\ndioxus = { version = \"*\" }\ndioxus-liveview = { version = \"*\", features = [\"axum\"] }\ntokio = { version = \"1.15.0\", features = [\"full\"] }",
        ::dioxus_code::CodeOptions::builder()
        .with_language(::dioxus_code::Language::Toml)) }, } CodeBlock { source : {
        ::dioxus_code::code_str!("fn it_works() {}",
        ::dioxus_code::CodeOptions::builder()
        .with_language(::dioxus_code::Language::Rust)) }, name : "included_example.rs"
        .to_string(), } CodeBlock { source : {
        ::dioxus_code::code_str!("{\"timestamp\":\"   9.927s\",\"level\":\"INFO\",\"message\":\"Bundled app successfully!\",\"target\":\"dx::cli::bundle\"}\n{\"timestamp\":\"   9.927s\",\"level\":\"INFO\",\"message\":\"App produced 2 outputs:\",\"target\":\"dx::cli::bundle\"}\n{\"timestamp\":\"   9.927s\",\"level\":\"INFO\",\"message\":\"app - [target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app]\",\"target\":\"dx::cli::bundle\"}\n{\"timestamp\":\"   9.927s\",\"level\":\"INFO\",\"message\":\"dmg - [target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg]\",\"target\":\"dx::cli::bundle\"}\n{\"timestamp\":\"   9.927s\",\"level\":\"DEBUG\",\"json\":\"{\\\"BundleOutput\\\":{\\\"bundles\\\":[\\\"target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app\\\"]}}\"}",
        ::dioxus_code::CodeOptions::builder()
        .with_language(::dioxus_code::Language::Bash)) }, }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum Chapter2Section {
    #[default]
    Empty,
    RoadmapFeatureSet,
    Features,
    Roadmap,
    Core,
    Ssr,
    Desktop,
    Mobile,
    BundlingCli,
    EssentialHooks,
    WorkInProgress,
    BuildTool,
    ServerComponentSupport,
    NativeRendering,
    InternalLinks,
}
impl std::str::FromStr for Chapter2Section {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "roadmap--feature-set" => Ok(Self::RoadmapFeatureSet),
            "features" => Ok(Self::Features),
            "roadmap" => Ok(Self::Roadmap),
            "core" => Ok(Self::Core),
            "ssr" => Ok(Self::Ssr),
            "desktop" => Ok(Self::Desktop),
            "mobile" => Ok(Self::Mobile),
            "bundling-cli" => Ok(Self::BundlingCli),
            "essential-hooks" => Ok(Self::EssentialHooks),
            "work-in-progress" => Ok(Self::WorkInProgress),
            "build-tool" => Ok(Self::BuildTool),
            "server-component-support" => Ok(Self::ServerComponentSupport),
            "native-rendering" => Ok(Self::NativeRendering),
            "internal-links" => Ok(Self::InternalLinks),
            _ => {
                Err(
                    "Invalid section name. Expected one of Chapter2Sectionroadmap--feature-set, features, roadmap, core, ssr, desktop, mobile, bundling-cli, essential-hooks, work-in-progress, build-tool, server-component-support, native-rendering, internal-links",
                )
            }
        }
    }
}
impl std::fmt::Display for Chapter2Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::RoadmapFeatureSet => f.write_str("roadmap--feature-set"),
            Self::Features => f.write_str("features"),
            Self::Roadmap => f.write_str("roadmap"),
            Self::Core => f.write_str("core"),
            Self::Ssr => f.write_str("ssr"),
            Self::Desktop => f.write_str("desktop"),
            Self::Mobile => f.write_str("mobile"),
            Self::BundlingCli => f.write_str("bundling-cli"),
            Self::EssentialHooks => f.write_str("essential-hooks"),
            Self::WorkInProgress => f.write_str("work-in-progress"),
            Self::BuildTool => f.write_str("build-tool"),
            Self::ServerComponentSupport => f.write_str("server-component-support"),
            Self::NativeRendering => f.write_str("native-rendering"),
            Self::InternalLinks => f.write_str("internal-links"),
        }
    }
}
#[component(no_case_check)]
pub fn Chapter2(section: Chapter2Section) -> Element {
    rsx! {
        h1 { id : "roadmap--feature-set", Link { to : BookRoute::Chapter2 { section :
        Chapter2Section::RoadmapFeatureSet, }, class : "header", "Roadmap & Feature-set"
        } } p {
        "This feature set and roadmap can help you decide if what Dioxus can do today works for you."
        } p {
        "If a feature that you need doesn't exist or you want to contribute to projects on the roadmap, feel free to get involved by "
        Link { to : "https://discord.gg/XgGxMSkvUM", "joining the discord" } "." } p {
        "Generally, here's the status of each platform:" } ul { li { p { strong { "Web" }
        ": Dioxus is a great choice for pure web-apps – especially for CRUD/complex apps. However, it does lack the ecosystem of React, so you might be missing a component library or some useful hook."
        } } li { p { strong { "SSR" }
        ": Dioxus is a great choice for pre-rendering, hydration, and rendering HTML on a web endpoint. Be warned – the VirtualDom is not (currently) "
        code { "Send + Sync" } "." } } li { p { strong { "Desktop" }
        ": You can build very competent single-window desktop apps right now. However, multi-window apps require support from Dioxus core and are not ready."
        } } li { p { strong { "Mobile" }
        ": Mobile support is very young. You'll be figuring things out as you go and there are not many support crates for peripherals."
        } } li { p { strong { "LiveView" }
        ": LiveView support is very young. You'll be figuring things out as you go. Thankfully, none of it is too hard and any work can be upstreamed into Dioxus."
        } } } CodeBlock { source : {
        ::dioxus_code::code_str!("fn main() {\n  dioxus_rocks;\n}",
        ::dioxus_code::CodeOptions::builder()
        .with_language(::dioxus_code::Language::Rust)) }, } h2 { id : "features", Link {
        to : BookRoute::Chapter2 { section : Chapter2Section::Features, }, class :
        "header", "Features" } } hr {} table { thead { th { "Feature" } th { "Status" }
        th { "Description" } } tr { th { "Conditional Rendering" } th { "✅" } th {
        "if/then to hide/show component" } } tr { th { "Map, Iterator" } th { "✅" } th
        { "map/filter/reduce to produce rsx!" } } tr { th { "Keyed Components" } th {
        "✅" } th { "advanced diffing with keys" } } tr { th { "Web" } th { "✅" } th {
        "renderer for web browser" } } tr { th { "Desktop (webview)" } th { "✅" } th {
        "renderer for desktop" } } tr { th { "Shared State (Context)" } th { "✅" } th {
        "share state through the tree" } } tr { th { "Hooks" } th { "✅" } th {
        "memory cells in components" } } tr { th { "SSR" } th { "✅" } th {
        "render directly to string" } } tr { th { "Component Children" } th { "✅" } th
        { "cx.children() as a list of nodes" } } tr { th { "Headless components" } th {
        "✅" } th { "components that don't return real elements" } } tr { th {
        "Fragments" } th { "✅" } th { "multiple elements without a real root" } } tr {
        th { "Manual Props" } th { "✅" } th {
        "Manually pass in props with spread syntax" } } tr { th { "Controlled Inputs" }
        th { "✅" } th { "stateful wrappers around inputs" } } tr { th {
        "CSS/Inline Styles" } th { "✅" } th {
        "syntax for inline styles/attribute groups" } } tr { th { "Custom elements" } th
        { "✅" } th { "Define new element primitives" } } tr { th { "Suspense" } th {
        "✅" } th { "schedule future render from future/promise" } } tr { th {
        "Integrated error handling" } th { "✅" } th {
        "Gracefully handle errors with ? syntax" } } tr { th { "NodeRef" } th { "✅" }
        th { "gain direct access to nodes" } } tr { th { "Re-hydration" } th { "✅" } th
        { "Pre-render to HTML to speed up first contentful paint" } } tr { th {
        "Jank-Free Rendering" } th { "✅" } th {
        "Large diffs are segmented across frames for silky-smooth transitions" } } tr {
        th { "Effects" } th { "✅" } th {
        "Run effects after a component has been committed to render" } } tr { th {
        "Portals" } th { "🛠" } th {
        "Render nodes outside of the traditional tree structure" } } tr { th {
        "Cooperative Scheduling" } th { "🛠" } th {
        "Prioritize important events over non-important events" } } tr { th {
        "Server Components" } th { "🛠" } th { "Hybrid components for SPA and Server" }
        } tr { th { "Bundle Splitting" } th { "👀" } th {
        "Efficiently and asynchronously load the app" } } tr { th { "Lazy Components" }
        th { "👀" } th { "Dynamically load the new components as the page is loaded" }
        } tr { th { "1st class global state" } th { "✅" } th {
        "redux/recoil/mobx on top of context" } } tr { th { "Runs natively" } th { "✅"
        } th { "runs as a portable binary w/o a runtime (Node)" } } tr { th {
        "Subtree Memoization" } th { "✅" } th { "skip diffing static element subtrees"
        } } tr { th { "High-efficiency templates" } th { "✅" } th {
        "rsx! calls are translated to templates on the DOM's side" } } tr { th {
        "Compile-time correct" } th { "✅" } th {
        "Throw errors on invalid template layouts" } } tr { th { "Heuristic Engine" } th
        { "✅" } th { "track component memory usage to minimize future allocations" } }
        tr { th { "Fine-grained reactivity" } th { "👀" } th {
        "Skip diffing for fine-grain updates" } } } ul { li {
        "✅ = implemented and working" } li { "🛠 = actively being worked on" } li {
        "👀 = not yet implemented or being worked on" } } h2 { id : "roadmap", Link {
        to : BookRoute::Chapter2 { section : Chapter2Section::Roadmap, }, class :
        "header", "Roadmap" } } p {
        "These Features are planned for the future of Dioxus:" } h3 { id : "core", Link {
        to : BookRoute::Chapter2 { section : Chapter2Section::Core, }, class : "header",
        "Core" } } ul { li { input { r#type : "checkbox", readonly : true, class :
        "mdbook-checkbox", value : "true", } "Release of Dioxus Core" } li { input {
        r#type : "checkbox", readonly : true, class : "mdbook-checkbox", value : "true",
        } "Upgrade documentation to include more theory and be more comprehensive" } li {
        input { r#type : "checkbox", readonly : true, class : "mdbook-checkbox", value :
        "true", } "Support for HTML-side templates for lightning-fast dom manipulation" }
        li { input { r#type : "checkbox", readonly : true, class : "mdbook-checkbox",
        value : "false", }
        "Support for multiple renderers for same virtualdom (subtrees)" } li { input {
        r#type : "checkbox", readonly : true, class : "mdbook-checkbox", value : "false",
        } "Support for ThreadSafe (Send + Sync)" } li { input { r#type : "checkbox",
        readonly : true, class : "mdbook-checkbox", value : "false", }
        "Support for Portals" } } h3 { id : "ssr", Link { to : BookRoute::Chapter2 {
        section : Chapter2Section::Ssr, }, class : "header", "SSR" } } ul { li { input {
        r#type : "checkbox", readonly : true, class : "mdbook-checkbox", value : "true",
        } "SSR Support + Hydration" } li { input { r#type : "checkbox", readonly : true,
        class : "mdbook-checkbox", value : "false", }
        "Integrated suspense support for SSR" } } h3 { id : "desktop", Link { to :
        BookRoute::Chapter2 { section : Chapter2Section::Desktop, }, class : "header",
        "Desktop" } } ul { li { input { r#type : "checkbox", readonly : true, class :
        "mdbook-checkbox", value : "false", } "Declarative window management" } li {
        input { r#type : "checkbox", readonly : true, class : "mdbook-checkbox", value :
        "false", } "Templates for building/bundling" } li { input { r#type : "checkbox",
        readonly : true, class : "mdbook-checkbox", value : "false", }
        "Access to Canvas/WebGL context natively" } } h3 { id : "mobile", Link { to :
        BookRoute::Chapter2 { section : Chapter2Section::Mobile, }, class : "header",
        "Mobile" } } ul { li { input { r#type : "checkbox", readonly : true, class :
        "mdbook-checkbox", value : "false", } "Mobile standard library" ul { li { input {
        r#type : "checkbox", readonly : true, class : "mdbook-checkbox", value : "false",
        } "GPS" } li { input { r#type : "checkbox", readonly : true, class :
        "mdbook-checkbox", value : "false", } "Camera" } li { input { r#type :
        "checkbox", readonly : true, class : "mdbook-checkbox", value : "false", }
        "filesystem" } li { input { r#type : "checkbox", readonly : true, class :
        "mdbook-checkbox", value : "false", } "Biometrics" } li { input { r#type :
        "checkbox", readonly : true, class : "mdbook-checkbox", value : "false", } "WiFi"
        } li { input { r#type : "checkbox", readonly : true, class : "mdbook-checkbox",
        value : "false", } "Bluetooth" } li { input { r#type : "checkbox", readonly :
        true, class : "mdbook-checkbox", value : "false", } "Notifications" } li { input
        { r#type : "checkbox", readonly : true, class : "mdbook-checkbox", value :
        "false", } "Clipboard" } } } li { input { r#type : "checkbox", readonly : true,
        class : "mdbook-checkbox", value : "false", } "Animations" } } h3 { id :
        "bundling-cli", Link { to : BookRoute::Chapter2 { section :
        Chapter2Section::BundlingCli, }, class : "header", "Bundling (CLI)" } } ul { li {
        input { r#type : "checkbox", readonly : true, class : "mdbook-checkbox", value :
        "true", } "Translation from HTML into RSX" } li { input { r#type : "checkbox",
        readonly : true, class : "mdbook-checkbox", value : "true", } "Dev server" } li {
        input { r#type : "checkbox", readonly : true, class : "mdbook-checkbox", value :
        "true", } "Live reload" } li { input { r#type : "checkbox", readonly : true,
        class : "mdbook-checkbox", value : "true", } "Translation from JSX into RSX" } li
        { input { r#type : "checkbox", readonly : true, class : "mdbook-checkbox", value
        : "false", } "Hot module replacement" } li { input { r#type : "checkbox",
        readonly : true, class : "mdbook-checkbox", value : "false", } "Code splitting" }
        li { input { r#type : "checkbox", readonly : true, class : "mdbook-checkbox",
        value : "false", } "Asset macros" } li { input { r#type : "checkbox", readonly :
        true, class : "mdbook-checkbox", value : "false", } "Css pipeline" } li { input {
        r#type : "checkbox", readonly : true, class : "mdbook-checkbox", value : "false",
        } "Image pipeline" } } h3 { id : "essential-hooks", Link { to :
        BookRoute::Chapter2 { section : Chapter2Section::EssentialHooks, }, class :
        "header", "Essential hooks" } } ul { li { input { r#type : "checkbox", readonly :
        true, class : "mdbook-checkbox", value : "true", } "Router" } li { input { r#type
        : "checkbox", readonly : true, class : "mdbook-checkbox", value : "true", }
        "Global state management" } li { input { r#type : "checkbox", readonly : true,
        class : "mdbook-checkbox", value : "false", } "Resize observer" } } h2 { id :
        "work-in-progress", Link { to : BookRoute::Chapter2 { section :
        Chapter2Section::WorkInProgress, }, class : "header", "Work in Progress" } } h3 {
        id : "build-tool", Link { to : BookRoute::Chapter2 { section :
        Chapter2Section::BuildTool, }, class : "header", "Build Tool" } } p {
        "We are currently working on our own build tool called " Link { to :
        "https://github.com/DioxusLabs/dioxus/tree/master/packages/cli", "Dioxus CLI" }
        " which will support:" } ul { li { "an interactive TUI" } li {
        "on-the-fly reconfiguration" } li { "hot CSS reloading" } li {
        "two-way data binding between browser and source code" } li {
        "an interpreter for " code { "rsx!" } } li {
        "ability to publish to github/netlify/vercel" } li {
        "bundling for iOS/Desktop/etc" } } h3 { id : "server-component-support", Link {
        to : BookRoute::Chapter2 { section : Chapter2Section::ServerComponentSupport, },
        class : "header", "Server Component Support" } } p {
        "While not currently fully implemented, the expectation is that LiveView apps can be a hybrid between Wasm and server-rendered where only portions of a page are \"live\" and the rest of the page is either server-rendered, statically generated, or handled by the host SPA."
        } h3 { id : "native-rendering", Link { to : BookRoute::Chapter2 { section :
        Chapter2Section::NativeRendering, }, class : "header", "Native rendering" } } p {
        "We are currently working on a native renderer for Dioxus using WGPU called "
        Link { to : "https://github.com/DioxusLabs/blitz/", "Blitz" }
        ". This will allow you to build apps that are rendered natively for iOS, Android, and Desktop."
        } h2 { id : "internal-links", Link { to : BookRoute::Chapter2 { section :
        Chapter2Section::InternalLinks, }, class : "header", "Internal Links" } } p {
        "Internal links like " Link { to : BookRoute::Chapter1 { section :
        Chapter1Section::Empty, }, "this" }
        " are typechecked and will fail to compile if the file is not found." }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum Chapter3Section {
    #[default]
    Empty,
    Assets,
}
impl std::str::FromStr for Chapter3Section {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "assets" => Ok(Self::Assets),
            _ => Err("Invalid section name. Expected one of Chapter3Sectionassets"),
        }
    }
}
impl std::fmt::Display for Chapter3Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Assets => f.write_str("assets"),
        }
    }
}
#[component(no_case_check)]
pub fn Chapter3(section: Chapter3Section) -> Element {
    rsx! {
        h1 { id : "assets", Link { to : BookRoute::Chapter3 { section :
        Chapter3Section::Assets, }, class : "header", "Assets" } } p { "Some assets:" img
        { src : "https://avatars.githubusercontent.com/u/79236386?s=200&v=4", alt :
        "some_external", title : "", } img { src :
        asset!("/example-book/assets/logo.png", ImageAssetOptions::new().with_webp()),
        alt : "some_local", title : "", } img { src :
        asset!("/example-book/assets/logo1.png", ImageAssetOptions::new().with_webp()),
        alt : "some_local1", title : "", } img { src :
        asset!("/example-book/assets/logo2.png", ImageAssetOptions::new().with_webp()),
        alt : "some_local2", title : "", } }
    }
}

use super::*;
