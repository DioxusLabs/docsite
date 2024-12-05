#[derive(
    Clone,
    Copy,
    dioxus_router::prelude::Routable,
    PartialEq,
    Eq,
    Hash,
    Debug,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum BookRoute {
    #[route("/./chapter_1")]
    Chapter1 {},
    #[route("/./chapter_2")]
    Chapter2 {},
    #[route("/./chapter_3")]
    Chapter3 {},
}
impl BookRoute {
    pub fn sections(&self) -> &'static [use_mdbook::mdbook_shared::Section] {
        &self.page().sections
    }
    pub fn page(&self) -> &'static use_mdbook::mdbook_shared::Page<Self> {
        LAZY_BOOK.get_page(self)
    }
    pub fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        match self {
            BookRoute::Chapter1 {} => use_mdbook::mdbook_shared::PageId(0usize),
            BookRoute::Chapter2 {} => use_mdbook::mdbook_shared::PageId(1usize),
            BookRoute::Chapter3 {} => use_mdbook::mdbook_shared::PageId(2usize),
        }
    }
}
impl Default for BookRoute {
    fn default() -> Self {
        BookRoute::Chapter1 {}
    }
}
pub static LAZY_BOOK: use_mdbook::Lazy<use_mdbook::mdbook_shared::MdBook<BookRoute>> =
    use_mdbook::Lazy::new(|| {
        let mut page_id_mapping = ::std::collections::HashMap::new();
        let mut pages = Vec::new();
        pages.push((0usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Chapter 1".to_string(),
                url: BookRoute::Chapter1 {},
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
        page_id_mapping.insert(
            BookRoute::Chapter1 {},
            ::use_mdbook::mdbook_shared::PageId(0usize),
        );
        pages.push((1usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Chapter 2".to_string(),
                url: BookRoute::Chapter2 {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Roadmap & Feature-set".to_string(),
                        id: "roadmap-&-feature-set".to_string(),
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
                        id: "bundling-(cli)".to_string(),
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
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(1usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::Chapter2 {},
            ::use_mdbook::mdbook_shared::PageId(1usize),
        );
        pages.push((2usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Chapter 3".to_string(),
                url: BookRoute::Chapter3 {},
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
        page_id_mapping.insert(
            BookRoute::Chapter3 {},
            ::use_mdbook::mdbook_shared::PageId(2usize),
        );
        ::use_mdbook::mdbook_shared::MdBook {
            summary: ::use_mdbook::mdbook_shared::Summary {
                title: Some("Summary".to_string()),
                prefix_chapters: vec![],
                numbered_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(
                        ::use_mdbook::mdbook_shared::Link {
                            name: "Chapter 1".to_string(),
                            location: Some(BookRoute::Chapter1 {}),
                            number: Some(::use_mdbook::mdbook_shared::SectionNumber(vec![1u32])),
                            nested_items: vec![],
                        },
                    ),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(
                        ::use_mdbook::mdbook_shared::Link {
                            name: "Chapter 2".to_string(),
                            location: Some(BookRoute::Chapter2 {}),
                            number: Some(::use_mdbook::mdbook_shared::SectionNumber(vec![2u32])),
                            nested_items: vec![],
                        },
                    ),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(
                        ::use_mdbook::mdbook_shared::Link {
                            name: "Chapter 3".to_string(),
                            location: Some(BookRoute::Chapter3 {}),
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
::core::compile_error!(
    "Failed to read file example-book/book.toml: No such file or directory (os error 2)",
);
#[component(no_case_check)]
pub fn Chapter2() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "roadmap--feature-set",
            a { href: "#roadmap--feature-set", class: "header", "Roadmap & Feature-set" }
        }
        p {
            "This feature set and roadmap can help you decide if what Dioxus can do today works for you."
        }
        p {
            "If a feature that you need doesn't exist or you want to contribute to projects on the roadmap, feel free to get involved by "
            a { href: "https://discord.gg/XgGxMSkvUM", "joining the discord" }
            "."
        }
        p { "Generally, here's the status of each platform:" }
        ul {
            li {
                p {
                    strong { "Web" }
                    ": Dioxus is a great choice for pure web-apps – especially for CRUD/complex apps. However, it does lack the ecosystem of React, so you might be missing a component library or some useful hook."
                }
            }
            li {
                p {
                    strong { "SSR" }
                    ": Dioxus is a great choice for pre-rendering, hydration, and rendering HTML on a web endpoint. Be warned – the VirtualDom is not (currently) "
                    code { "Send + Sync" }
                    "."
                }
            }
            li {
                p {
                    strong { "Desktop" }
                    ": You can build very competent single-window desktop apps right now. However, multi-window apps require support from Dioxus core and are not ready."
                }
            }
            li {
                p {
                    strong { "Mobile" }
                    ": Mobile support is very young. You'll be figuring things out as you go and there are not many support crates for peripherals."
                }
            }
            li {
                p {
                    strong { "LiveView" }
                    ": LiveView support is very young. You'll be figuring things out as you go. Thankfully, none of it is too hard and any work can be upstreamed into Dioxus."
                }
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">  dioxus_rocks;\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        h2 { id: "features",
            a { href: "#features", class: "header", "Features" }
        }
        hr {}
        table {
            thead {
                th { "Feature" }
                th { "Status" }
                th { "Description" }
            }
            tr {
                th { "Conditional Rendering" }
                th { "✅" }
                th { "if/then to hide/show component" }
            }
            tr {
                th { "Map, Iterator" }
                th { "✅" }
                th { "map/filter/reduce to produce rsx!" }
            }
            tr {
                th { "Keyed Components" }
                th { "✅" }
                th { "advanced diffing with keys" }
            }
            tr {
                th { "Web" }
                th { "✅" }
                th { "renderer for web browser" }
            }
            tr {
                th { "Desktop (webview)" }
                th { "✅" }
                th { "renderer for desktop" }
            }
            tr {
                th { "Shared State (Context)" }
                th { "✅" }
                th { "share state through the tree" }
            }
            tr {
                th { "Hooks" }
                th { "✅" }
                th { "memory cells in components" }
            }
            tr {
                th { "SSR" }
                th { "✅" }
                th { "render directly to string" }
            }
            tr {
                th { "Component Children" }
                th { "✅" }
                th { "cx.children() as a list of nodes" }
            }
            tr {
                th { "Headless components" }
                th { "✅" }
                th { "components that don't return real elements" }
            }
            tr {
                th { "Fragments" }
                th { "✅" }
                th { "multiple elements without a real root" }
            }
            tr {
                th { "Manual Props" }
                th { "✅" }
                th { "Manually pass in props with spread syntax" }
            }
            tr {
                th { "Controlled Inputs" }
                th { "✅" }
                th { "stateful wrappers around inputs" }
            }
            tr {
                th { "CSS/Inline Styles" }
                th { "✅" }
                th { "syntax for inline styles/attribute groups" }
            }
            tr {
                th { "Custom elements" }
                th { "✅" }
                th { "Define new element primitives" }
            }
            tr {
                th { "Suspense" }
                th { "✅" }
                th { "schedule future render from future/promise" }
            }
            tr {
                th { "Integrated error handling" }
                th { "✅" }
                th { "Gracefully handle errors with ? syntax" }
            }
            tr {
                th { "NodeRef" }
                th { "✅" }
                th { "gain direct access to nodes" }
            }
            tr {
                th { "Re-hydration" }
                th { "✅" }
                th { "Pre-render to HTML to speed up first contentful paint" }
            }
            tr {
                th { "Jank-Free Rendering" }
                th { "✅" }
                th { "Large diffs are segmented across frames for silky-smooth transitions" }
            }
            tr {
                th { "Effects" }
                th { "✅" }
                th { "Run effects after a component has been committed to render" }
            }
            tr {
                th { "Portals" }
                th { "🛠" }
                th { "Render nodes outside of the traditional tree structure" }
            }
            tr {
                th { "Cooperative Scheduling" }
                th { "🛠" }
                th { "Prioritize important events over non-important events" }
            }
            tr {
                th { "Server Components" }
                th { "🛠" }
                th { "Hybrid components for SPA and Server" }
            }
            tr {
                th { "Bundle Splitting" }
                th { "👀" }
                th { "Efficiently and asynchronously load the app" }
            }
            tr {
                th { "Lazy Components" }
                th { "👀" }
                th { "Dynamically load the new components as the page is loaded" }
            }
            tr {
                th { "1st class global state" }
                th { "✅" }
                th { "redux/recoil/mobx on top of context" }
            }
            tr {
                th { "Runs natively" }
                th { "✅" }
                th { "runs as a portable binary w/o a runtime (Node)" }
            }
            tr {
                th { "Subtree Memoization" }
                th { "✅" }
                th { "skip diffing static element subtrees" }
            }
            tr {
                th { "High-efficiency templates" }
                th { "✅" }
                th { "rsx! calls are translated to templates on the DOM's side" }
            }
            tr {
                th { "Compile-time correct" }
                th { "✅" }
                th { "Throw errors on invalid template layouts" }
            }
            tr {
                th { "Heuristic Engine" }
                th { "✅" }
                th { "track component memory usage to minimize future allocations" }
            }
            tr {
                th { "Fine-grained reactivity" }
                th { "👀" }
                th { "Skip diffing for fine-grain updates" }
            }
        }
        ul {
            li { "✅ = implemented and working" }
            li { "🛠 = actively being worked on" }
            li { "👀 = not yet implemented or being worked on" }
        }
        h2 { id: "roadmap",
            a { href: "#roadmap", class: "header", "Roadmap" }
        }
        p { "These Features are planned for the future of Dioxus:" }
        h3 { id: "core",
            a { href: "#core", class: "header", "Core" }
        }
        ul {
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "true",
                }
                "Release of Dioxus Core"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "true",
                }
                "Upgrade documentation to include more theory and be more comprehensive"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "true",
                }
                "Support for HTML-side templates for lightning-fast dom manipulation"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Support for multiple renderers for same virtualdom (subtrees)"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Support for ThreadSafe (Send + Sync)"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Support for Portals"
            }
        }
        h3 { id: "ssr",
            a { href: "#ssr", class: "header", "SSR" }
        }
        ul {
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "true",
                }
                "SSR Support + Hydration"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Integrated suspense support for SSR"
            }
        }
        h3 { id: "desktop",
            a { href: "#desktop", class: "header", "Desktop" }
        }
        ul {
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Declarative window management"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Templates for building/bundling"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Access to Canvas/WebGL context natively"
            }
        }
        h3 { id: "mobile",
            a { href: "#mobile", class: "header", "Mobile" }
        }
        ul {
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Mobile standard library"
                ul {
                    li {
                        input {
                            r#type: "checkbox",
                            readonly: true,
                            class: "mdbook-checkbox",
                            value: "false",
                        }
                        "GPS"
                    }
                    li {
                        input {
                            r#type: "checkbox",
                            readonly: true,
                            class: "mdbook-checkbox",
                            value: "false",
                        }
                        "Camera"
                    }
                    li {
                        input {
                            r#type: "checkbox",
                            readonly: true,
                            class: "mdbook-checkbox",
                            value: "false",
                        }
                        "filesystem"
                    }
                    li {
                        input {
                            r#type: "checkbox",
                            readonly: true,
                            class: "mdbook-checkbox",
                            value: "false",
                        }
                        "Biometrics"
                    }
                    li {
                        input {
                            r#type: "checkbox",
                            readonly: true,
                            class: "mdbook-checkbox",
                            value: "false",
                        }
                        "WiFi"
                    }
                    li {
                        input {
                            r#type: "checkbox",
                            readonly: true,
                            class: "mdbook-checkbox",
                            value: "false",
                        }
                        "Bluetooth"
                    }
                    li {
                        input {
                            r#type: "checkbox",
                            readonly: true,
                            class: "mdbook-checkbox",
                            value: "false",
                        }
                        "Notifications"
                    }
                    li {
                        input {
                            r#type: "checkbox",
                            readonly: true,
                            class: "mdbook-checkbox",
                            value: "false",
                        }
                        "Clipboard"
                    }
                }
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Animations"
            }
        }
        h3 { id: "bundling-cli",
            a { href: "#bundling-cli", class: "header", "Bundling (CLI)" }
        }
        ul {
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "true",
                }
                "Translation from HTML into RSX"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "true",
                }
                "Dev server"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "true",
                }
                "Live reload"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "true",
                }
                "Translation from JSX into RSX"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Hot module replacement"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Code splitting"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Asset macros"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Css pipeline"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Image pipeline"
            }
        }
        h3 { id: "essential-hooks",
            a { href: "#essential-hooks", class: "header", "Essential hooks" }
        }
        ul {
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "true",
                }
                "Router"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "true",
                }
                "Global state management"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Resize observer"
            }
        }
        h2 { id: "work-in-progress",
            a { href: "#work-in-progress", class: "header", "Work in Progress" }
        }
        h3 { id: "build-tool",
            a { href: "#build-tool", class: "header", "Build Tool" }
        }
        p {
            "We are currently working on our own build tool called "
            a { href: "https://github.com/DioxusLabs/dioxus/tree/master/packages/cli",
                "Dioxus CLI"
            }
            " which will support:"
        }
        ul {
            li { "an interactive TUI" }
            li { "on-the-fly reconfiguration" }
            li { "hot CSS reloading" }
            li { "two-way data binding between browser and source code" }
            li {
                "an interpreter for "
                code { "rsx!" }
            }
            li { "ability to publish to github/netlify/vercel" }
            li { "bundling for iOS/Desktop/etc" }
        }
        h3 { id: "server-component-support",
            a { href: "#server-component-support", class: "header", "Server Component Support" }
        }
        p {
            "While not currently fully implemented, the expectation is that LiveView apps can be a hybrid between Wasm and server-rendered where only portions of a page are \"live\" and the rest of the page is either server-rendered, statically generated, or handled by the host SPA."
        }
        h3 { id: "native-rendering",
            a { href: "#native-rendering", class: "header", "Native rendering" }
        }
        p {
            "We are currently working on a native renderer for Dioxus using WGPU called "
            a { href: "https://github.com/DioxusLabs/blitz/", "Blitz" }
            ". This will allow you to build apps that are rendered natively for iOS, Android, and Desktop."
        }
    }
}
#[component(no_case_check)]
pub fn Chapter3() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "assets",
            a { href: "#assets", class: "header", "Assets" }
        }
        p {
            "Some assets:"
            img {
                src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                alt: "some_external",
                title: "",
            }
            img {
                src: asset!("/example-book/assetsasd/logo"),
                alt: "some_local",
                title: "",
            }
            img {
                src: asset!("/example-book/assets1/logo.png", ImageAssetOptions::new().with_avif()),
                alt: "some_local1",
                title: "",
            }
            img {
                src: asset!("/example-book/assets2/logo.png", ImageAssetOptions::new().with_avif()),
                alt: "some_local2",
                title: "",
            }
        }
    }
}
