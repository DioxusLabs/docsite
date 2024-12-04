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
    #[route("/introducing-dioxus")]
    IntroducingDioxus {},
    #[route("/release-020")]
    Release020 {},
    #[route("/templates-diffing")]
    TemplatesDiffing {},
    #[route("/release-030")]
    Release030 {},
    #[route("/fulltime")]
    Fulltime {},
    #[route("/release-040")]
    Release040 {},
    #[route("/release-050")]
    Release050 {},
    #[route("/release-060")]
    Release060 {},
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
            BookRoute::IntroducingDioxus {} => use_mdbook::mdbook_shared::PageId(0usize),
            BookRoute::Release020 {} => use_mdbook::mdbook_shared::PageId(1usize),
            BookRoute::TemplatesDiffing {} => use_mdbook::mdbook_shared::PageId(2usize),
            BookRoute::Release030 {} => use_mdbook::mdbook_shared::PageId(3usize),
            BookRoute::Fulltime {} => use_mdbook::mdbook_shared::PageId(4usize),
            BookRoute::Release040 {} => use_mdbook::mdbook_shared::PageId(5usize),
            BookRoute::Release050 {} => use_mdbook::mdbook_shared::PageId(6usize),
            BookRoute::Release060 {} => use_mdbook::mdbook_shared::PageId(7usize),
        }
    }
}
impl Default for BookRoute {
    fn default() -> Self {
        BookRoute::Fulltime {}
    }
}
pub static LAZY_BOOK: use_mdbook::Lazy<use_mdbook::mdbook_shared::MdBook<BookRoute>> =
    use_mdbook::Lazy::new(|| {
        let mut page_id_mapping = ::std::collections::HashMap::new();
        let mut pages = Vec::new();
        pages
            .push((
                0usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Dioxus 0.1 $ Jan 3 2022 $ Release Notes $ After months of work, we're very excited to release the first version of Dioxus! Dioxus is a new library for building interactive user interfaces with Rust."
                            .to_string(),
                        url: BookRoute::IntroducingDioxus {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Introducing Dioxus v0.1 ✨".to_string(),
                                id: "introducing-dioxus-v0.1-✨".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Show me some examples of what can be built!"
                                    .to_string(),
                                id: "show-me-some-examples-of-what-can-be-built!"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why should I use Rust and Dioxus for frontend?"
                                    .to_string(),
                                id: "why-should-i-use-rust-and-dioxus-for-frontend?"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Difference from TypeScript/React:".to_string(),
                                id: "difference-from-typescript/react:".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Show me more".to_string(),
                                id: "show-me-more".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Building a new project is simple".to_string(),
                                id: "building-a-new-project-is-simple".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Support for JSX-style templating".to_string(),
                                id: "support-for-jsx-style-templating".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dioxus prioritizes developer experience"
                                    .to_string(),
                                id: "dioxus-prioritizes-developer-experience".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dioxus is perfected for the IDE".to_string(),
                                id: "dioxus-is-perfected-for-the-ide".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dioxus is extremely fast".to_string(),
                                id: "dioxus-is-extremely-fast".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Works on Desktop and Mobile".to_string(),
                                id: "works-on-desktop-and-mobile".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Did someone say TUI support?".to_string(),
                                id: "did-someone-say-tui-support?".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Things we didn't cover:".to_string(),
                                id: "things-we-didn't-cover:".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What's on the roadmap?".to_string(),
                                id: "what's-on-the-roadmap?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Community".to_string(),
                                id: "community".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(0usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::IntroducingDioxus {},
            ::use_mdbook::mdbook_shared::PageId(0usize),
        );
        pages
            .push((
                1usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Dioxus 0.2 $ Release Notes $ March 9, 2022 $ Just over two months in, and we already have a ton of awesome changes to Dioxus!"
                            .to_string(),
                        url: BookRoute::Release020 {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What's new?".to_string(),
                                id: "what's-new?".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "A New Renderer: Your terminal!".to_string(),
                                id: "a-new-renderer:-your-terminal!".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "New Router".to_string(),
                                id: "new-router".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Fermi for Global State Management".to_string(),
                                id: "fermi-for-global-state-management".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Inline Props Macro".to_string(),
                                id: "inline-props-macro".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Props optional fields".to_string(),
                                id: "props-optional-fields".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dioxus Web Speed Boost".to_string(),
                                id: "dioxus-web-speed-boost".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dioxus Desktop Window Context".to_string(),
                                id: "dioxus-desktop-window-context".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "CLI Tool".to_string(),
                                id: "cli-tool".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Async Improvements".to_string(),
                                id: "async-improvements".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "All New Features".to_string(),
                                id: "all-new-features".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Fixes".to_string(),
                                id: "fixes".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Community Additions".to_string(),
                                id: "community-additions".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Looking Forward".to_string(),
                                id: "looking-forward".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(1usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::Release020 {},
            ::use_mdbook::mdbook_shared::PageId(1usize),
        );
        pages
            .push((
                2usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Making Dioxus (almost) as fast as SolidJS $ Tech $ December 11, 2022 $ Using a new technique called subtree memoization, Dioxus is now almost as fast as SolidJS."
                            .to_string(),
                        url: BookRoute::TemplatesDiffing {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Making Dioxus (almost) as fast as SolidJS"
                                    .to_string(),
                                id: "making-dioxus-(almost)-as-fast-as-solidjs".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dioxus shares React’s DNA".to_string(),
                                id: "dioxus-shares-react’s-dna".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Overcoming the warts of React".to_string(),
                                id: "overcoming-the-warts-of-react".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Making Dioxus faster by doing less work"
                                    .to_string(),
                                id: "making-dioxus-faster-by-doing-less-work".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Taking it a step further".to_string(),
                                id: "taking-it-a-step-further".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What does this enable?".to_string(),
                                id: "what-does-this-enable?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Hot Reloading".to_string(),
                                id: "hot-reloading".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Performant LiveView".to_string(),
                                id: "performant-liveview".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Faster Server-Side-Rendering (SSR)".to_string(),
                                id: "faster-server-side-rendering-(ssr)".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Disclaimer".to_string(),
                                id: "disclaimer".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(2usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::TemplatesDiffing {},
            ::use_mdbook::mdbook_shared::PageId(2usize),
        );
        pages
            .push((
                3usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Dioxus 0.3 $ Release Notes $ February 8, 2023 $ The next big release of Dioxus is here! Templates, autoformatting, multiwindow support, and more!"
                            .to_string(),
                        url: BookRoute::Release030 {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Templates and performance improvements".to_string(),
                                id: "templates-and-performance-improvements".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Hot Reloading".to_string(),
                                id: "hot-reloading".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Autoformatting".to_string(),
                                id: "autoformatting".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "LiveView and LiveComponents".to_string(),
                                id: "liveview-and-livecomponents".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "TUI Input Widgets".to_string(),
                                id: "tui-input-widgets".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Multi-window Desktop Apps".to_string(),
                                id: "multi-window-desktop-apps".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Lowercase components".to_string(),
                                id: "lowercase-components".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "For Loops, If Chains, and more flexible RSX"
                                    .to_string(),
                                id: "for-loops,-if-chains,-and-more-flexible-rsx"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Preliminary WGPU renderer".to_string(),
                                id: "preliminary-wgpu-renderer".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Skia Renderer".to_string(),
                                id: "skia-renderer".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Completing support for cross-platform events"
                                    .to_string(),
                                id: "completing-support-for-cross-platform-events"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Lua Plugin System for CLI".to_string(),
                                id: "lua-plugin-system-for-cli".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Translations".to_string(),
                                id: "translations".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "A new landing page and better docs".to_string(),
                                id: "a-new-landing-page-and-better-docs".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Community Projects".to_string(),
                                id: "community-projects".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(3usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::Release030 {},
            ::use_mdbook::mdbook_shared::PageId(3usize),
        );
        pages
            .push((
                4usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Going fulltime on Dioxus $ Misc $ May 5, 2023 $ Dioxus is now my full time job! I'm so excited to be able to work on this full time."
                            .to_string(),
                        url: BookRoute::Fulltime {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Going full time".to_string(),
                                id: "going-full-time".to_string(),
                                level: 1usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(4usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::Fulltime {},
            ::use_mdbook::mdbook_shared::PageId(4usize),
        );
        pages
            .push((
                5usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Dioxus 0.4 $ Release Notes $ August 1, 2023 $ Server Functions, Suspense, Enum Router, Overhauled Docs, Bundler, Android Support, and more!"
                            .to_string(),
                        url: BookRoute::Release040 {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Weekly Office Hours".to_string(),
                                id: "weekly-office-hours".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Server Functions".to_string(),
                                id: "server-functions".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Suspense".to_string(),
                                id: "suspense".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Enum Router".to_string(),
                                id: "enum-router".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "New and beautiful interactive docs".to_string(),
                                id: "new-and-beautiful-interactive-docs".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Android Support, iOS fixes, Getting Started Guide for Mobile"
                                    .to_string(),
                                id: "android-support,-ios-fixes,-getting-started-guide-for-mobile"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Window-Close Behavior".to_string(),
                                id: "window-close-behavior".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Bidirectional Eval".to_string(),
                                id: "bidirectional-eval".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "New onmount event".to_string(),
                                id: "new-onmount-event".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Renaming dioxus-cli to dx".to_string(),
                                id: "renaming-dioxus-cli-to-dx".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Hot Reloading for Desktop".to_string(),
                                id: "hot-reloading-for-desktop".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dioxus-Bundle".to_string(),
                                id: "dioxus-bundle".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dioxus-Check".to_string(),
                                id: "dioxus-check".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "VSCode Extension Updates".to_string(),
                                id: "vscode-extension-updates".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "General Fixes".to_string(),
                                id: "general-fixes".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(5usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::Release040 {},
            ::use_mdbook::mdbook_shared::PageId(5usize),
        );
        pages
            .push((
                6usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Dioxus 0.5 $ Release Notes $ March 21, 2024 $ A signal rewrite, zero unsafe, no lifetimes, unified launch, and more!"
                            .to_string(),
                        url: BookRoute::Release050 {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What’s new?".to_string(),
                                id: "what’s-new?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Lifetime Problems".to_string(),
                                id: "lifetime-problems".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Goodbye scopes and lifetimes!".to_string(),
                                id: "goodbye-scopes-and-lifetimes!".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Removal of all Unsafe in Core".to_string(),
                                id: "removal-of-all-unsafe-in-core".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Signals!".to_string(),
                                id: "signals!".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Copy state".to_string(),
                                id: "copy-state".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Smarter subscriptions".to_string(),
                                id: "smarter-subscriptions".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "CSS Hot Reloading".to_string(),
                                id: "css-hot-reloading".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Event System Rewrite".to_string(),
                                id: "event-system-rewrite".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Cross platform launch".to_string(),
                                id: "cross-platform-launch".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Asset System Beta".to_string(),
                                id: "asset-system-beta".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "5x Faster Desktop Rendering".to_string(),
                                id: "5x-faster-desktop-rendering".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Spreading props".to_string(),
                                id: "spreading-props".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Shorthand attributes".to_string(),
                                id: "shorthand-attributes".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Multi-line attribute merging".to_string(),
                                id: "multi-line-attribute-merging".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Server function streaming".to_string(),
                                id: "server-function-streaming".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Fullstack CLI platform".to_string(),
                                id: "fullstack-cli-platform".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "LiveView router support".to_string(),
                                id: "liveview-router-support".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Custom Asset Handlers".to_string(),
                                id: "custom-asset-handlers".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Native File Handling".to_string(),
                                id: "native-file-handling".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Error handling".to_string(),
                                id: "error-handling".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Hot reloading by default and “develop” mode for Desktop"
                                    .to_string(),
                                id: "hot-reloading-by-default-and-“develop”-mode-for-desktop"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Updates to the Dioxus template".to_string(),
                                id: "updates-to-the-dioxus-template".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dioxus-Community and Dioxus-std".to_string(),
                                id: "dioxus-community-and-dioxus-std".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Coming soon".to_string(),
                                id: "coming-soon".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Sneak Peek: Dioxus-Blitz revival using Servo"
                                    .to_string(),
                                id: "sneak-peek:-dioxus-blitz-revival-using-servo"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "How can you contribute?".to_string(),
                                id: "how-can-you-contribute?".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(6usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::Release050 {},
            ::use_mdbook::mdbook_shared::PageId(6usize),
        );
        pages
            .push((
                7usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Dioxus 0.6 $ Release Notes $ December 9, 2024 $ Android/iOS simulator, Interactive CLI, RSX Autocomplete, Props Hotreloading, WGPU Integration, Asset Stabilization, and more."
                            .to_string(),
                        url: BookRoute::Release060 {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "About this Release".to_string(),
                                id: "about-this-release".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Android and iOS support for ".to_string(),
                                id: "android-and-ios-support-for".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Interactive Command Line Tools".to_string(),
                                id: "interactive-command-line-tools".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Inline WASM stacktraces and ".to_string(),
                                id: "inline-wasm-stacktraces-and".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Toasts and Loading Screens".to_string(),
                                id: "toasts-and-loading-screens".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Fullstack Desktop and Mobile".to_string(),
                                id: "fullstack-desktop-and-mobile".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Completely Revamped Autocomplete".to_string(),
                                id: "completely-revamped-autocomplete".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Completely Revamped Hot-Reloading".to_string(),
                                id: "completely-revamped-hot-reloading".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Hot-Reloading Formatted Strings".to_string(),
                                id: "hot-reloading-formatted-strings".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Hot-Reloading Rust Literals".to_string(),
                                id: "hot-reloading-rust-literals".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Hot-Reloading nested rsx (".to_string(),
                                id: "hot-reloading-nested-rsx-(".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Mobile Hot-Reloading".to_string(),
                                id: "mobile-hot-reloading".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Proper Workspace Hot-Reloading".to_string(),
                                id: "proper-workspace-hot-reloading".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Stabilizing Manganis ".to_string(),
                                id: "stabilizing-manganis".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Suspense and HTML Streaming for the Web"
                                    .to_string(),
                                id: "suspense-and-html-streaming-for-the-web".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Static Site Generation and ISG".to_string(),
                                id: "static-site-generation-and-isg".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Question Mark Error Handling".to_string(),
                                id: "question-mark-error-handling".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Document Elements: ".to_string(),
                                id: "document-elements:".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Synchronous ".to_string(),
                                id: "synchronous".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Tracking size with ".to_string(),
                                id: "tracking-size-with".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Tracking visibility with ".to_string(),
                                id: "tracking-visibility-with".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Hybrid WGPU Overlays".to_string(),
                                id: "hybrid-wgpu-overlays".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Web, iOS, and Android bundle support".to_string(),
                                id: "web,-ios,-and-android-bundle-support".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "JSON Output for CI / CLI".to_string(),
                                id: "json-output-for-ci-/-cli".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "New Starter Templates".to_string(),
                                id: "new-starter-templates".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Nightly Docs, Tutorials, and New Guides"
                                    .to_string(),
                                id: "nightly-docs,-tutorials,-and-new-guides".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Preview of In-Place Binary Patching".to_string(),
                                id: "preview-of-in-place-binary-patching".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Smaller changes:".to_string(),
                                id: "smaller-changes:".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Upgrading from 0.5 to 0.6".to_string(),
                                id: "upgrading-from-0.5-to-0.6".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Conclusion".to_string(),
                                id: "conclusion".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Thanks to the community!".to_string(),
                                id: "thanks-to-the-community!".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(7usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::Release060 {},
            ::use_mdbook::mdbook_shared::PageId(7usize),
        );
        ::use_mdbook::mdbook_shared::MdBook {
            summary: ::use_mdbook::mdbook_shared::Summary {
                title: Some("Summary".to_string()),
                prefix_chapters: vec![],
                numbered_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Dioxus 0.1 $ Jan 3 2022 $ Release Notes $ After months of work, we're very excited to release the first version of Dioxus! Dioxus is a new library for building interactive user interfaces with Rust."
                            .to_string(),
                        location: Some(BookRoute::IntroducingDioxus {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Dioxus 0.2 $ Release Notes $ March 9, 2022 $ Just over two months in, and we already have a ton of awesome changes to Dioxus!"
                            .to_string(),
                        location: Some(BookRoute::Release020 {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Making Dioxus (almost) as fast as SolidJS $ Tech $ December 11, 2022 $ Using a new technique called subtree memoization, Dioxus is now almost as fast as SolidJS."
                            .to_string(),
                        location: Some(BookRoute::TemplatesDiffing {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Dioxus 0.3 $ Release Notes $ February 8, 2023 $ The next big release of Dioxus is here! Templates, autoformatting, multiwindow support, and more!"
                            .to_string(),
                        location: Some(BookRoute::Release030 {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Going fulltime on Dioxus $ Misc $ May 5, 2023 $ Dioxus is now my full time job! I'm so excited to be able to work on this full time."
                            .to_string(),
                        location: Some(BookRoute::Fulltime {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Dioxus 0.4 $ Release Notes $ August 1, 2023 $ Server Functions, Suspense, Enum Router, Overhauled Docs, Bundler, Android Support, and more!"
                            .to_string(),
                        location: Some(BookRoute::Release040 {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![6u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Dioxus 0.5 $ Release Notes $ March 21, 2024 $ A signal rewrite, zero unsafe, no lifetimes, unified launch, and more!"
                            .to_string(),
                        location: Some(BookRoute::Release050 {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![7u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Dioxus 0.6 $ Release Notes $ December 9, 2024 $ Android/iOS simulator, Interactive CLI, RSX Autocomplete, Props Hotreloading, WGPU Integration, Asset Stabilization, and more."
                            .to_string(),
                        location: Some(BookRoute::Release060 {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![8u32]),
                        ),
                        nested_items: vec![],
                    }),
                ],
                suffix_chapters: vec![],
            },
            pages: pages.into_iter().collect(),
            page_id_mapping,
        }
    });
#[component(no_case_check)]
pub fn IntroducingDioxus() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "introducing-dioxus-v01-",
            a { href: "#introducing-dioxus-v01-", class: "header", "Introducing Dioxus v0.1 ✨" }
        }
        blockquote {
            p { "Jan 3, 2022" }
        }
        blockquote {
            p {
                a { href: "https://github.com/jkelleyrtp", "@jkelleyrtp" }
                ", thanks "
                a { href: "https://github.com/alexkirsz", "@alexkirsz" }
            }
        }
        p { "After many months of work, we're very excited to release the first version of Dioxus!" }
        p {
            "Dioxus is a new library for building interactive user interfaces (GUI) with Rust. It is built around a Virtual DOM, making it portable for the web, desktop, server, mobile, and more."
        }
        p { "Dioxus has the following design goals:" }
        ul {
            li {
                strong { "Familiar" }
                ": Offer a React-like mental model and API surface"
            }
            li {
                strong { "Robust" }
                ": Avoid runtime bugs by moving rules and error handling into the type system"
            }
            li {
                strong { "Performant" }
                ": Scale to the largest apps and the largest teams"
            }
            li {
                strong { "Productive" }
                ": Comprehensive inline documentation, fast recompiles, and deeply integrated tooling"
            }
            li {
                strong { "Extensible" }
                ": Reusable hooks and components that work on every platform"
            }
        }
        p {
            "Dioxus is designed to be familiar for developers already comfortable with React paradigms. Our goal is to ensure a smooth transition from TypeScript/React without having to learn any major new concepts."
        }
        p { "To give you an idea of what Dioxus looks like, here's a simple counter app:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">\tdioxus::desktop::launch(app)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;Count: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;+&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;-&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This simple counter is a complete desktop application, running at native speeds on a native thread. Dioxus automatically shuttles all events from the WebView runtime into the application code. In our app, we can interact natively with system APIs, run multi-threaded code, and do anything a regular native Rust application might do. Running  "
            code { "cargo build --release" }
            " will compile a portable binary that looks and feels the same on Windows, macOS, and Linux. We can then use  "
            code { "cargo-bundle" }
            " to bundle our binary into a native  "
            code { ".app" }
            "/ "
            code { ".exe" }
            "/ "
            code { ".deb" }
            "."
        }
        p { "Dioxus supports many of the same features React does including:" }
        ul {
            li { "Server-side-rendering, pre-rendering, and hydration" }
            li { "Mobile, desktop, and web support" }
            li { "Suspense, fibers, coroutines, and error handling" }
            li { "Hooks, first-class state management, components" }
            li { "Fragments, conditional rendering, and custom elements" }
        }
        p { "However, some things are different in Dioxus:" }
        ul {
            li { "Automatic memoization (opt-out rather than opt-in)" }
            li { "No effects - effectual code can only originate from actions or coroutines" }
            li {
                "Suspense is implemented as hooks - "
                em { "not" }
                " deeply ingrained within Dioxus Core"
            }
            li {
                "Async code is "
                em { "explicit" }
                " with a preference for "
                em { "coroutines" }
                " instead"
            }
        }
        p { "As a demo, here's our teaser example running on all our current supported platforms:" }
        p {
            img {
                src: "/assets/static/Untitled.png",
                alt: "Teaser Example",
                title: "",
            }
        }
        p {
            "This very site is built with Dioxus, and the source code is available "
            a { href: "https://github.com/dioxuslabs/docsite", "here" }
            "."
        }
        p {
            "To get started with Dioxus, check out any of the \"Getting Started\" guides for your platform of choice, or check out the GitHub Repository for more details."
        }
        ul {
            li {
                a { href: "https://dioxuslabs.com/docs/0.3/guide/en", "Getting Started with Dioxus" }
            }
            li {
                a { href: "https://dioxuslabs.com/reference/web", "Getting Started with Web" }
            }
            li {
                a { href: "https://dioxuslabs.com/reference/desktop", "Getting Started with Desktop" }
            }
            li {
                a { href: "https://dioxuslabs.com/reference/mobile", "Getting Started with Mobile" }
            }
            li {
                a { href: "https://dioxuslabs.com/reference/ssr", "Getting Started with SSR" }
            }
        }
        h2 { id: "show-me-some-examples-of-what-can-be-built",
            a {
                href: "#show-me-some-examples-of-what-can-be-built",
                class: "header",
                "Show me some examples of what can be built!"
            }
        }
        ul {
            li {
                a { href: "https://github.com/dioxuslabs/example-projects",
                    "File explorer desktop app"
                }
            }
            li {
                a { href: "https://github.com/dioxuslabs/example-projects",
                    "WiFi scanner desktop app"
                }
            }
            li {
                a { href: "https://github.com/dioxuslabs/example-projects", "Dog CEO API Search" }
            }
            li {
                a { href: "https://github.com/dioxuslabs/example-projects", "TodoMVC Mobile App" }
            }
            li {
                a { href: "https://github.com/dioxuslabs/example-projects", "E-Commerce Liveview App" }
            }
        }
        h2 { id: "why-should-i-use-rust-and-dioxus-for-frontend",
            a {
                href: "#why-should-i-use-rust-and-dioxus-for-frontend",
                class: "header",
                "Why should I use Rust and Dioxus for frontend?"
            }
        }
        p {
            "We believe that Rust's ability to write high-level and statically typed code should make it easier for frontend teams to take on even the most ambitious of projects. Rust projects can be refactored fearlessly: the powerful type system prevents an entire class of bugs at compile-time. No more  "
            code { "cannot read property of undefined" }
            " ever again! With Rust, all errors must be accounted for at compile time. You cannot ship an app that does not — in some way — handle its errors."
        }
        h3 { id: "difference-from-typescriptreact",
            a { href: "#difference-from-typescriptreact", class: "header",
                "Difference from TypeScript/React:"
            }
        }
        p {
            "TypeScript is still fundamentally JavaScript. If you've written enough TypeScript, you might be bogged down with lots of configuration options, lack of proper support for \"go-to-source,\" or incorrect ad-hoc typing. With Rust, strong types are built-in, saving tons of headache like  "
            code { "cannot read property of undefined" }
            "."
        }
        p { "By using Rust, we gain:" }
        ul {
            li { "Strong types for every library" }
            li { "Immutability by default" }
            li { "A simple and intuitive module system" }
            li {
                "Integrated documentation (go to source actually goes to source instead of the "
                code { ".d.ts" }
                " file)"
            }
            li { "Advanced pattern matching" }
            li { "Clean, efficient, composable iterators" }
            li { "Inline built-in unit/integration testing" }
            li { "High quality error handling" }
            li { "Flexible standard library and traits" }
            li { "Powerful macro system" }
            li {
                "Access to the "
                a { href: "https://crates.io", "crates.io" }
                " ecosystem"
            }
        }
        p { "Dioxus itself leverages this platform to provide the following guarantees:" }
        ul {
            li { "Correct use of immutable data structures" }
            li { "Guaranteed handling of errors and null-values in components" }
            li { "Native performance on mobile" }
            li { "Direct access to system IO" }
        }
        p {
            "And much more. Dioxus makes Rust apps just as fast to write as React apps, but affords more robustness, giving your frontend team greater confidence in making big changes in shorter timespans."
        }
        p {
            "Semantically, TypeScript-React and Rust-Dioxus are very similar. In TypeScript, we would declare a simple component as:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">type </span><span style=\"color:#f8f8f2;\">CardProps </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">  title: string;\n</span><span style=\"color:#f8f8f2;\">  paragraph: string;\n</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">const</span><span style=\"color:#f8f8f2;\"> Card: FunctionComponent&lt;CardProps&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">(props) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f8f8f2;\">[count, set_count] </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">return </span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">aside</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">h2</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">{{props.title}}</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">h2</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">p</span><span style=\"color:#f92672;\">&gt; </span><span style=\"color:#f8f8f2;\">{{props.paragraph}} </span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">p</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">button onclick</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{() </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#66d9ef;\">set_count</span><span style=\"color:#f8f8f2;\">(count </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">)}}</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\"> Count {{count}} </span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">button</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">aside</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">  );\n</span><span style=\"color:#f8f8f2;\">}};</span></pre>\n",
        }
        p { "In Dioxus, we would define the same component in a similar fashion:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Props, PartialEq)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">CardProps {{\n</span><span style=\"color:#f8f8f2;\">\ttitle: String,\n</span><span style=\"color:#f8f8f2;\">\tparagraph: String\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">static</span><span style=\"color:#f8f8f2;\"> Card: Component&lt;CardProps&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">|</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">| {{\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\tcx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">\t\taside {{\n</span><span style=\"color:#f8f8f2;\">\t\t\th2 {{ </span><span style=\"color:#ffee99;\">&quot;{{cx.props.title}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\t\t\tp {{ </span><span style=\"color:#ffee99;\">&quot;{{cx.props.paragraph}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\t\t\tbutton {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count</span><span style=\"color:#f92672;\">+=</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Count: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\t\t}}\n</span><span style=\"color:#f8f8f2;\">\t))\n</span><span style=\"color:#f8f8f2;\">}};</span></pre>\n",
        }
        p {
            "However, we recognize that not every project needs Rust - many are fine with JavaScript! We also acknowledge that Rust/Wasm/Dioxus does not fix \"everything that is wrong with frontend development.\" There are always going to be new patterns, frameworks, and languages that solve these problems better than Rust and Dioxus."
        }
        p { "As a general rule of thumb, Dioxus is for you if:" }
        ul {
            li { "your app will become very large" }
            li { "you need to share code across many platforms" }
            li { "you want a fast way to build for desktop" }
            li { "you want to avoid electron or need direct access to hardware" }
            li { "you're tired of JavaScript tooling" }
        }
        p {
            "Today, to publish a Dioxus app, you don't need NPM/WebPack/Parcel/etc. Dioxus simply builds with cargo, and for web builds, Dioxus happily works with the popular "
            a { href: "http://trunkrs.dev", "trunk" }
            " project."
        }
        h2 { id: "show-me-more",
            a { href: "#show-me-more", class: "header", "Show me more" }
        }
        p {
            "Here, we'll dive into some features of Dioxus and why it's so fun to use. The "
            a { href: "https://dioxuslabs.com/docs/0.3/guide/en/", "guide" }
            " serves as a deeper and more comprehensive look at what Dioxus can do."
        }
        h2 { id: "building-a-new-project-is-simple",
            a { href: "#building-a-new-project-is-simple", class: "header",
                "Building a new project is simple"
            }
        }
        p {
            "To start a new project, all you need is Cargo, which comes with Rust. For a simple desktop app, all we'll need is the  "
            code { "dioxus" }
            " crate with the appropriate  "
            code { "desktop" }
            " feature. We start by initializing a new binary crate:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">$</span><span style=\"color:#f8f8f2;\"> cargo init dioxus_example\n</span><span style=\"color:#f92672;\">$</span><span style=\"color:#f8f8f2;\"> cd dioxus_example</span></pre>\n" }
        p {
            "We then add a dependancy on Dioxus to the  "
            code { "Cargo.toml" }
            " file, with the \"desktop\" feature enabled:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[dependencies]\n</span><span style=\"color:#f8f8f2;\">dioxus </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;*&quot;</span><span style=\"color:#f8f8f2;\">, features </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;desktop&quot;</span><span style=\"color:#f8f8f2;\">] }}</span></pre>\n" }
        p { "We can add our counter from above." }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">\tdioxus::desktop::launch(app)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;Count: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;+&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;-&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "And voilà! We can  "
            code { "cargo run" }
            " our app"
        }
        p {
            img {
                src: "/assets/static/counter.png",
                alt: "Simple Counter Desktop App",
                title: "",
            }
        }
        h2 { id: "support-for-jsx-style-templating",
            a { href: "#support-for-jsx-style-templating", class: "header",
                "Support for JSX-style templating"
            }
        }
        p {
            "Dioxus ships with a templating macro called RSX, a spin on React's JSX. RSX is very similar to regular struct syntax for Rust so it integrates well with your IDE. If used with "
            a { href: "https://github.com/rust-analyzer/rust-analyzer", "Rust-Analyzer" }
            " (not tested anywhere else) RSX supports code-folding, block selection, bracket pair colorizing, autocompletion, symbol renaming — pretty much anything you would expect from writing regular struct-style code."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">\tdiv {{ </span><span style=\"color:#ffee99;\">&quot;Hello world&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\tbutton {{\n</span><span style=\"color:#f8f8f2;\">\t\tonclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">log::info</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;button pressed&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#ffee99;\">&quot;Press me&quot;\n</span><span style=\"color:#f8f8f2;\">\t}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p { "If macros aren't your style, you can always drop down to the factory API:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">LazyNodes::new(|</span><span style=\"font-style:italic;color:#fd971f;\">f</span><span style=\"color:#f8f8f2;\">| {{\n</span><span style=\"color:#f8f8f2;\">\tf.</span><span style=\"color:#66d9ef;\">fragment</span><span style=\"color:#f8f8f2;\">([\n</span><span style=\"color:#f8f8f2;\">\t\tf.</span><span style=\"color:#66d9ef;\">element</span><span style=\"color:#f8f8f2;\">(div, [f.</span><span style=\"color:#66d9ef;\">text</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;hello world&quot;</span><span style=\"color:#f8f8f2;\">)], [], </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">\t\tf.</span><span style=\"color:#66d9ef;\">element</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">\t\t\tbutton,\n</span><span style=\"color:#f8f8f2;\">\t\t\t[f.</span><span style=\"color:#66d9ef;\">text</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Press Me&quot;</span><span style=\"color:#f8f8f2;\">)],\n</span><span style=\"color:#f8f8f2;\">\t\t\t[on::click(</span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">log::info</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;button pressed&quot;</span><span style=\"color:#f8f8f2;\">))],\n</span><span style=\"color:#f8f8f2;\">\t\t\t</span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">\t\t\t</span><span style=\"font-style:italic;color:#66d9ef;\">None\n</span><span style=\"color:#f8f8f2;\">\t\t)\n</span><span style=\"color:#f8f8f2;\">\t])\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
        }
        p {
            "The  "
            code { "rsx!" }
            " macro generates idiomatic Rust code that uses the factory API — no different than what you'd write by hand yourself."
        }
        p {
            "To make it easier to work with RSX, we've built a small "
            a { href: "https://github.com/DioxusLabs/studio", "VSCode extension" }
            " with useful utilities. This extension provides a command that converts a selected block of HTML into RSX so you can easily reuse existing web templates."
        }
        h2 { id: "dioxus-prioritizes-developer-experience",
            a {
                href: "#dioxus-prioritizes-developer-experience",
                class: "header",
                "Dioxus prioritizes developer experience"
            }
        }
        p {
            "Many of the Rust UI frameworks are particularly difficult to work with. Even the ones branded as \"ergonomic\" are quite challenging to in comparison to TSX/JSX. With Dioxus, we've innovated on a number of Rust patterns to deliver a framework that is actually enjoyable to develop in."
        }
        p {
            "For example, many Rust frameworks require you to clone your data in for "
            em { "every" }
            " closure and handler you use. This can get really clumsy for large apps."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#66d9ef;\">div</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">\t.</span><span style=\"color:#66d9ef;\">children</span><span style=\"color:#f8f8f2;\">([\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#66d9ef;\">button</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">onclick</span><span style=\"color:#f8f8f2;\">(cloned!(name, date, age, description </span><span style=\"color:#f92672;\">=&gt; move |</span><span style=\"color:#f8f8f2;\">evt</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#8c8c8c;\">/* */ </span><span style=\"color:#f8f8f2;\">}})\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#66d9ef;\">button</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">onclick</span><span style=\"color:#f8f8f2;\">(cloned!(name, date, age, description </span><span style=\"color:#f92672;\">=&gt; move |</span><span style=\"color:#f8f8f2;\">evt</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#8c8c8c;\">/* */ </span><span style=\"color:#f8f8f2;\">}})\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#66d9ef;\">button</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">onclick</span><span style=\"color:#f8f8f2;\">(cloned!(name, date, age, description </span><span style=\"color:#f92672;\">=&gt; move |</span><span style=\"color:#f8f8f2;\">evt</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#8c8c8c;\">/* */ </span><span style=\"color:#f8f8f2;\">}})\n</span><span style=\"color:#f8f8f2;\">\t])</span></pre>\n",
        }
        p {
            "Dioxus understands the lifetimes of data borrowed from  "
            code { "Scope" }
            ", so you can safely return any borrowed data without declaring explicit captures. Hook handles all implement  "
            code { "Copy" }
            " so they can be shared between listeners without any ceremony."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx, || </span><span style=\"color:#ffee99;\">&quot;asd&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">\tdiv {{\n</span><span style=\"color:#f8f8f2;\">\t\tbutton {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> name.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;abc&quot;</span><span style=\"color:#f8f8f2;\">) }}\n</span><span style=\"color:#f8f8f2;\">\t\tbutton {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> name.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;def&quot;</span><span style=\"color:#f8f8f2;\">) }}\n</span><span style=\"color:#f8f8f2;\">\t\tbutton {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> name.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;ghi&quot;</span><span style=\"color:#f8f8f2;\">) }}\n</span><span style=\"color:#f8f8f2;\">\t}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Because we know the lifetime of your handlers, we can also expose this to children. No other Rust frameworks let us share borrowed state through the tree, forcing use of Rc/Arc everywhere. With Dioxus, all the Rc/Arc magic is tucked away in hooks, and just beautiful borrowed interfaces are exposed to your code. You don't need to know how Rc/RefCell work to build a competent Dioxus app."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx, || </span><span style=\"color:#ffee99;\">&quot;asd&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\tcx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">\t\tButton {{ name: name }}\n</span><span style=\"color:#f8f8f2;\">\t}})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Props)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">ButtonProps&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">\tname: UseState&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">&amp;&#39;static </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">&gt;\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Button</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">, Childprops&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\tcx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">\t\tbutton {{\n</span><span style=\"color:#f8f8f2;\">\t\t\tonclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> cx.props.name.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;bob&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">\t\t}}\n</span><span style=\"color:#f8f8f2;\">\t}})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "There's "
            em { "way" }
            " more to this story, but hopefully we've convinced you that Dioxus' DX somewhat approximates JSX/React."
        }
        h2 { id: "dioxus-is-perfected-for-the-ide",
            a { href: "#dioxus-is-perfected-for-the-ide", class: "header",
                "Dioxus is perfected for the IDE"
            }
        }
        p {
            "Note: all IDE-related features have only been tested with "
            a { href: "https://github.com/rust-analyzer/rust-analyzer", "Rust-Analyzer" }
            "."
        }
        p {
            "Dioxus code operates pleasantly with your IDE. For starters, most elements are documented through the Rustdoc system. A quick summary of the MDN docs is always under your finger tips:"
        }
        p {
            img {
                src: "/assets/static/ide_hover.png",
                alt: "Elements have hover context",
                title: "",
            }
        }
        p {
            "Dioxus also wraps platform-specific events with a custom synthetic event system. This means events enjoy proper autocomplete and documentation, unlike "
            a { href: "https://yew.rs/", "Yew" }
            " which currently relies on "
            a { href: "https://crates.io/crates/web-sys", "web-sys" }
            " with incomplete IDE support:"
        }
        p {
            img {
                src: "/assets/static/ide_autocomplete.png",
                alt: "Events are strongly typed",
                title: "",
            }
        }
        p { "Even element attributes and event handlers have top-notch documentation!" }
        p {
            img {
                src: "/assets/static/ide_listener.png",
                alt: "Element attributes and listeners have hover context",
                title: "",
            }
        }
        p {
            "The  "
            code { "rsx!" }
            " macro also benefits from code folding, batch renaming, and block selection, making most basic code navigation and completion tasks a breeze."
        }
        p {
            img {
                src: "/assets/static/ide_selection.png",
                alt: "Element blocks can be folded and renamed",
                title: "",
            }
        }
        p {
            "Furthermore, the  "
            code { "rsx!" }
            " macro itself is documented, so if you ever forget how to use a certain feature, the documentation remains close at hand:"
        }
        p {
            img {
                src: "/assets/static/ide_rsx.png",
                alt: "The RSX documentation is provided on hover",
                title: "",
            }
        }
        p { "We spent a ton of time on this and we hope you enjoy it!" }
        h2 { id: "dioxus-is-extremely-fast",
            a { href: "#dioxus-is-extremely-fast", class: "header", "Dioxus is extremely fast" }
        }
        p {
            "We take the performance of Dioxus seriously. Instead of resolving to \"good enough,\" Dioxus is designed to push the limits of what a declarative React-like framework can achieve. Dioxus is designed with multi-tenancy in mind: a single machine should be able to run thousands of simultaneous low-latency LiveView apps without skipping a beat. To accomplish this goal we've implemented a large number of optimizations:"
        }
        ul {
            li { "Usage of bump memory allocators and double-buffering" }
            li { "Compile-time hashing of templates" }
            li { "Automatic component memoization" }
            li { "Fiber-like scheduler" }
            li { "DOM Patch Batching" }
        }
        p {
            "Dioxus is humbly built off the work done by "
            a { href: "https://github.com/fitzgen/dodrio", "Dodrio" }
            ", a now-archived research project by fitzgen exploring the use of bump allocators in UI frameworks."
        }
        p {
            "Dioxus is "
            em { "substantially" }
            " more performant than many of the other Rust DOM-based UI libraries (Yew/Percy) and is "
            em { "significantly" }
            " more performant than React - roughly competitive with InfernoJS. While not as performant as libraries like SolidJS/Sycamore, Dioxus imposes roughly a "
            em { "plenty" }
            " fast."
        }
        h2 { id: "works-on-desktop-and-mobile",
            a { href: "#works-on-desktop-and-mobile", class: "header",
                "Works on Desktop and Mobile"
            }
        }
        p {
            "We’ve mentioned before that Dioxus works practically anywhere that Rust does. When running natively as a desktop or mobile app, your Dioxus code will run on its own thread, not inside of a web runtime. This means you can access hardware, file system, and platform APIs directly without needing to go through a shim layer. In our examples, we feature a "
            a { href: "https://github.com/DioxusLabs/example-projects/tree/master/file-explorer",
                "file explorer app"
            }
            " and "
            a { href: "https://github.com/DioxusLabs/example-projects/tree/master/wifi-scanner",
                "WiFi scanner app"
            }
            " where platform access occurs inside an asynchronous multithreaded coroutine. This solves the problem faced by React Native and other cross-platform toolkits where JavaScript apps incur a massive performance penalty with substantial maintenance overhead associated with platform API shims."
        }
        p { "A desktop app:" }
        p {
            a { href: "https://github.com/DioxusLabs/example-projects/blob/master/file-explorer",
                img {
                    src: "https://github.com/DioxusLabs/example-projects/raw/master/file-explorer/image.png",
                    alt: "Example Dioxus desktop app",
                    title: "",
                }
            }
        }
        p { "A mobile app:" }
        p {
            a { href: "https://github.com/DioxusLabs/example-projects/blob/master/ios_demo",
                img {
                    src: "https://github.com/DioxusLabs/example-projects/raw/master/ios_demo/assets/screenshot_smaller.jpeg",
                    alt: "Example Dioxus mobile app",
                    title: "",
                }
            }
        }
        p {
            "However, be warned that mobile is currently considered very experimental and there will likely be quirks. Dioxus is leveraging the work done by the "
            a { href: "https://github.com/tauri-apps/tauri", "Tauri" }
            " team to enable mobile support, and mobile support isn't technically complete in Tauri yet."
        }
        p {
            "iOS should be supported out of the box, but Android support will take custom some boilerplate that hasn't been completely figured out. If you're interested in contributing to Dioxus, improving mobile support would be extremely helpful."
        }
        h3 { id: "did-someone-say-tui-support",
            a { href: "#did-someone-say-tui-support", class: "header",
                "Did someone say TUI support?"
            }
        }
        p {
            "Yes, you can even build terminal user interfaces with Dioxus. Full support is still a work in progress, but the foundation is there."
        }
        p {
            a { href: "https://github.com/dioxusLabs/rink",
                img {
                    src: "https://github.com/DioxusLabs/rink/raw/master/examples/example.png",
                    alt: "TUI Support",
                    title: "",
                }
            }
        }
        h3 { id: "things-we-didnt-cover",
            a { href: "#things-we-didnt-cover", class: "header", "Things we didn't cover:" }
        }
        p {
            "There are a bunch of things we didn't talk about here. Check out the guide for more information, or peruse the examples and reference for more context."
        }
        ul {
            li { "Jank-free rendering with fiber scheduler" }
            li {
                a { href: "", "Support for borrowed props" }
            }
            li {
                a { href: "", "Conditional rendering" }
            }
            li {
                a { href: "", "CSS/Styling/Inline style support" }
            }
            li {
                a { href: "", "Support for inline Context Providing/Consuming" }
            }
            li {
                a { href: "", "First-class global state management" }
            }
        }
        p {
            "For a quick glance at party with React, check out the "
            a { href: "https://github.com/DioxusLabs/dioxus#parity-with-react", "Readme on Github" }
            "."
        }
        h2 { id: "whats-on-the-roadmap",
            a { href: "#whats-on-the-roadmap", class: "header", "What's on the roadmap?" }
        }
        p {
            "The world of Rust on the frontend is barely explored. Given the performance, ergonomics, and portability of Rust/Dioxus, we expect there to be a ton of different applications where having a React-like toolkit running natively can enable things previously considered impossible."
        }
        p {
            "In the coming weeks, our plan is to finish the remaining outstanding features where Dioxus is lacking in comparison to React:"
        }
        ul {
            li { "Transition effects for Suspense" }
            li { "Micro-optimizations and better cross-platform/browser bug mitigations" }
            li { "Heuristics to guide the diffing algorithm" }
            li { "Better support for subtree memoization (signals, etc.)" }
            li { "More thorough documentation, fleshing out sore spots" }
        }
        p { "We also need some help in important crates currently missing:" }
        ul {
            li { "First class cross-platform router (currently in progress)" }
            li { "An extension to DioxusStudio that enables lazy bundling of static assets" }
            li {
                "Animation library (see "
                a { href: "https://react-spring.io/", "React Spring" }
                ", "
                a { href: "https://www.framer.com/motion/", "Framer Motion" }
                ")"
            }
            li {
                "A "
                a { href: "https://github.com/dioxuslabs/rink", "TUI renderer for Dioxus" }
                " (see "
                a { href: "https://github.com/vadimdemedes/ink", "Ink" }
                ")"
            }
        }
        p {
            "And finally, some bigger, forward-thinking projects that are too big for a one-person team:"
        }
        ul {
            li {
                "Completely native renderer for the Dioxus Virtual DOM (see "
                a { href: "https://flutter.dev/", "Flutter" }
                ")"
            }
            li { "Better support for LiveView" }
            li { "Code-splitting" }
            li {
                "3D renderer (see "
                a { href: "https://github.com/pmndrs/react-three-fiber", "react-three-fiber" }
                ")"
            }
        }
        p {
            "Stay tuned for our next article, which will go over some of the optimization techniques that went into making Dioxus blazing fast."
        }
        h2 { id: "community",
            a { href: "#community", class: "header", "Community" }
        }
        p {
            "The future is bright for Rust frontends! If you'd like to get involved, we have a "
            a { href: "https://discord.gg/XgGxMSkvUM", "Discord server" }
            ", "
            a { href: "http://reddit.com/r/dioxus", "a subreddit" }
            ", and "
            a { href: "https://github.com/DioxusLabs/dioxus/discussions", "GitHub discussion pages" }
            "."
        }
        p { "Let us know what you build!" }
        p {
            "Check out the original  "
            code { "/r/rust" }
            " thread here."
        }
    }
}
#[component(no_case_check)]
pub fn Release020() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Thanks to these amazing folks for their financial support on OpenCollective:" }
        ul {
            li {
                a { href: "https://github.com/t1m0t", "@t1m0t" }
            }
            li {
                a { href: "https://github.com/t1m0t", "@alexkirsz" }
            }
            li {
                a { href: "https://github.com/freopen", "@freopen" }
            }
            li {
                a { href: "https://github.com/DannyMichaels", "@DannyMichaels" }
            }
            li {
                a { href: "https://github.com/Fatcat560", "@SweetLittleMUV" }
            }
        }
        p { "Thanks to these amazing folks for their code contributions:" }
        ul {
            li {
                a { href: "https://github.com/mrxiaozhuox", "@mrxiaozhuox" }
            }
            li {
                a { href: "https://github.com/autarch", "@autarch" }
            }
            li {
                a { href: "https://github.com/FruitieX", "@FruitieX" }
            }
            li {
                a { href: "https://github.com/t1m0t", "@t1m0t" }
            }
            li {
                a { href: "https://github.com/ealmloff", "@ealmloff" }
            }
            li {
                a { href: "https://github.com/oovm", "@oovm" }
            }
            li {
                a { href: "https://github.com/asaaki", "@asaaki" }
            }
        }
        p { "Just over two months in, and we already have a ton of awesome changes to Dioxus!" }
        p {
            "Dioxus is a recently-released library for building interactive user interfaces (GUI) with Rust. It is built around a Virtual DOM, making it portable for the web, desktop, server, mobile, and more. Dioxus looks and feels just like React, so if you know React, then you'll feel right at home."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;Count: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;+&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;-&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h1 { id: "whats-new",
            a { href: "#whats-new", class: "header", "What's new?" }
        }
        p {
            "A "
            em { "ton" }
            " of stuff happened in this release; 550+ commits, 23 contributors, 2 minor releases, and 6 backers on Open Collective."
        }
        p { "Some of the major new features include:" }
        ul {
            li {
                "We now can render into the terminal, similar to Ink.JS - a huge thanks to "
                a { href: "https://github.com/ealmloff", "@ealmloff" }
            }
            li {
                "We have a new router in the spirit of React-Router "
                a { href: "https://github.com/autarch", "@autarch" }
            }
            li {
                "We now have Fermi for global state management in the spirit of "
                a { href: "https://recoiljs.org", "Recoil.JS" }
            }
            li {
                "Our desktop platform got major upgrades, getting closer to parity with Electron "
                a { href: "https://github.com/mrxiaozhuox", "@mrxiaozhuox" }
            }
            li {
                "Our CLI tools now support HTML-to-RSX translation for converting 3rd party HTML into Dioxus "
                a { href: "https://github.com/mrxiaozhuox", "@mrxiaozhuox" }
            }
            li { "Dioxus-Web is sped up by 2.5x with JS-based DOM manipulation (3x faster than React)" }
        }
        p { "We also fixed and improved a bunch of stuff - check out the full list down below." }
        h2 { id: "a-new-renderer-your-terminal",
            a { href: "#a-new-renderer-your-terminal", class: "header",
                "A New Renderer: Your terminal!"
            }
        }
        p {
            "When Dioxus was initially released, we had very simple support for logging Dioxus elements out as TUI elements. In the past month or so, "
            a { href: "https://github.com/ealmloff", "@ealmloff" }
            " really stepped up and made the new crate a reality."
        }
        p {
            img {
                src: "https://i.imgur.com/GL7uu3r.png",
                alt: "Imgur",
                title: "",
            }
        }
        p {
            "The new TUI renderer even supports mouse movements, keyboard input, async tasks, borders, and a ton more."
        }
        h2 { id: "new-router",
            a { href: "#new-router", class: "header", "New Router" }
        }
        p {
            "We totally revamped the router, switching away from the old yew-router approach to the more familiar "
            a { href: "http://reactrouter.com", "React-Router" }
            ". It's less type-safe but provides more flexibility and support for beautiful URLs."
        }
        p {
            "Apps with routers are "
            em { "really" }
            " simple now. It's easy to compose the \"Router\", a \"Route\", and \"Links\" to define how your app is laid out:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Router {{\n</span><span style=\"color:#f8f8f2;\">            onchange: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">log::info</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Route changed!&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">            ul {{\n</span><span style=\"color:#f8f8f2;\">                Link {{ to: </span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">,  li {{ </span><span style=\"color:#ffee99;\">&quot;Go home!&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">                Link {{ to: </span><span style=\"color:#ffee99;\">&quot;users&quot;</span><span style=\"color:#f8f8f2;\">,  li {{ </span><span style=\"color:#ffee99;\">&quot;List all users&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">                Link {{ to: </span><span style=\"color:#ffee99;\">&quot;blog&quot;</span><span style=\"color:#f8f8f2;\">, li {{ </span><span style=\"color:#ffee99;\">&quot;Blog posts&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            Route {{ to: </span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Home&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            Route {{ to: </span><span style=\"color:#ffee99;\">&quot;/users&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;User list&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            Route {{ to: </span><span style=\"color:#ffee99;\">&quot;/users/:name&quot;</span><span style=\"color:#f8f8f2;\">, User {{}} }}\n</span><span style=\"color:#f8f8f2;\">            Route {{ to: </span><span style=\"color:#ffee99;\">&quot;/blog&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Blog list&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            Route {{ to: </span><span style=\"color:#ffee99;\">&quot;/blog/:post&quot;</span><span style=\"color:#f8f8f2;\">, BlogPost {{}} }}\n</span><span style=\"color:#f8f8f2;\">            Route {{ to: </span><span style=\"color:#ffee99;\">&quot;&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Err 404 Route Not Found&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "We're also using hooks to parse the URL parameters and segments so you can interact with the router from anywhere deeply nested in your app."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Deserialize)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">Query {{ name: String }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BlogPost</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> post </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_route</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx).</span><span style=\"color:#66d9ef;\">segment</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;post&quot;</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> query </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_route</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx).query::&lt;Query&gt;()</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Viewing post {{post}}&quot;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Name selected: {{query}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Give a big thanks to "
            a { href: "https://github.com/autarch", "@autarch" }
            " for putting in all the hard work to make this new router a reality."
        }
        p {
            "The Router guide is "
            a { href: "https://dioxuslabs.com/nightly/router/", "available here" }
            " - thanks to "
            a { href: "https://github.com/dogedark", "@dogedark" }
            "."
        }
        h2 { id: "fermi-for-global-state-management",
            a { href: "#fermi-for-global-state-management", class: "header",
                "Fermi for Global State Management"
            }
        }
        p {
            "Managing state in your app can be challenging. Building global state management solutions can be even more challenging. For the first big attempt at building a global state management solution for Dioxus, we chose to keep it simple and follow in the footsteps of the "
            a { href: "http://recoiljs.org", "Recoil.JS" }
            " project."
        }
        p {
            "Fermi uses the concept of \"Atoms\" for global state. These individual values can be get/set from anywhere in your app. Using state with Fermi is basically as simple as  "
            code { "use_state" }
            "."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Create a single value in an &quot;Atom&quot;\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">TITLE</span><span style=\"color:#f8f8f2;\">: Atom&lt;</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">|_| </span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Read the value from anywhere in the app, subscribing to any changes\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> title </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_read</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx, </span><span style=\"color:#ff80f4;\">TITLE</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;{{title}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        Child {{}}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Set the value from anywhere in the app\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> set_title </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx, </span><span style=\"color:#ff80f4;\">TITLE</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#66d9ef;\">set_title</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;goodbye&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Say goodbye&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "inline-props-macro",
            a { href: "#inline-props-macro", class: "header", "Inline Props Macro" }
        }
        p {
            "For internal components, explicitly declaring props structs can become tedious. That's why we've built the new  "
            code { "component" }
            " macro. This macro lets you inline your props definition right into your component function arguments."
        }
        p {
            "Simply add the  "
            code { "component" }
            " macro to your component:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">age</span><span style=\"color:#f8f8f2;\">: String,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">onclick</span><span style=\"color:#f8f8f2;\">: EventHandler&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">, ClickEvent&gt;\n</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello, {{name}}&quot;\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;You are {{age}} years old&quot;\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">evt</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> onclick.</span><span style=\"color:#66d9ef;\">call</span><span style=\"color:#f8f8f2;\">(evt)\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "You won't be able to document each field or attach attributes so you should refrain from using it in libraries."
        }
        h2 { id: "props-optional-fields",
            a { href: "#props-optional-fields", class: "header", "Props optional fields" }
        }
        p {
            "Sometimes you don't want to specify "
            em { "every" }
            " value in a component's props, since there might a lot. That's why the "
            code { "Props" }
            " macro now supports optional fields. You can use a combination of "
            code { "default" }
            ", "
            code { "strip_option" }
            ", and "
            code { "optional" }
            " to tune the exact behavior of properties fields."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Props, PartialEq)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">ChildProps {{\n</span><span style=\"color:#f8f8f2;\">    #[props(default = &quot;client&quot;)]\n</span><span style=\"color:#f8f8f2;\">    name: String,\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    #[props(default)]\n</span><span style=\"color:#f8f8f2;\">    age: Option&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u32</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    #[props(optional)]\n</span><span style=\"color:#f8f8f2;\">    age: Option&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u32</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// then to use the accompanying component\n</span><span style=\"color:#f8f8f2;\">rsx!{{\n</span><span style=\"color:#f8f8f2;\">    Child {{\n</span><span style=\"color:#f8f8f2;\">        name: </span><span style=\"color:#ffee99;\">&quot;asd&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "dioxus-web-speed-boost",
            a { href: "#dioxus-web-speed-boost", class: "header", "Dioxus Web Speed Boost" }
        }
        p {
            "We've changed how DOM patching works in Dioxus-Web; now, all of the DOM manipulation code is written in TypeScript and shared between our web, desktop, and mobile runtimes."
        }
        p {
            "On an M1-max, the \"create-rows\" operation used to take 45ms. Now, it takes a mere 17ms - 3x faster than React. We expect an upcoming optimization to bring this number as low as 3ms."
        }
        p {
            "Under the hood, we have a new string interning engine to cache commonly used tags and values on the Rust "
        }
        p { "Overall, Dioxus apps are even more snappy than before." }
        p {
            "Before and after: "
            img {
                src: "https://imgur.com/byTBGlO.png",
                alt: "Before and After",
                title: "",
            }
        }
        h2 { id: "dioxus-desktop-window-context",
            a { href: "#dioxus-desktop-window-context", class: "header",
                "Dioxus Desktop Window Context"
            }
        }
        p {
            "A very welcome change, thanks AGAIN to "
            a { href: "https://github.com/mrxiaozhuox", "@mrxiaozhuox" }
            " is support for imperatively controlling the desktop window from your Dioxus code."
        }
        p { "A bunch of new methods were added:" }
        ul {
            li { "Minimize and maximize window" }
            li { "Close window" }
            li { "Focus window" }
            li { "Enable devtools on the fly" }
        }
        p { "And more!" }
        p {
            "In addition, Dioxus Desktop now autoresolves asset locations, so you can easily add local images, JS, CSS, and then bundle it into an .app without hassle."
        }
        p { "You can now build entirely borderless desktop apps:" }
        p {
            img {
                src: "https://i.imgur.com/97zsVS1.png",
                alt: "img",
                title: "",
            }
        }
        h2 { id: "cli-tool",
            a { href: "#cli-tool", class: "header", "CLI Tool" }
        }
        p {
            "Thanks to the amazing work by "
            a { href: "https://github.com/mrxiaozhuox", "@mrxiaozhuox" }
            ", our CLI tool is fixed and working better than ever. The Dioxus-CLI sports a new development server, an HTML to RSX translation engine, a "
            code { "cargo fmt" }
            "-style command, a configuration scheme, and much more."
        }
        p {
            "Unlike its counterpart,  "
            code { "Trunk.rs" }
            ", the dioxus-cli supports running examples and tests, making it easier to test web-based projects and showcase web-focused libraries."
        }
        h2 { id: "async-improvements",
            a { href: "#async-improvements", class: "header", "Async Improvements" }
        }
        p {
            "Working with async isn't the easiest part of Rust. To help improve things, we've upgraded async support across the board in Dioxus."
        }
        p {
            "First, we upgraded the  "
            code { "use_future" }
            " hook. It now supports dependencies, which let you regenerate a future on the fly as its computed values change. It's never been easier to add datafetching to your Rust Web Apps:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">RenderDog</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope, </span><span style=\"font-style:italic;color:#fd971f;\">breed</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> dog_request </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_future</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx, (breed,), |(</span><span style=\"font-style:italic;color:#fd971f;\">breed</span><span style=\"color:#f8f8f2;\">,)| async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        reqwest::get(format!(</span><span style=\"color:#ffee99;\">&quot;https://dog.ceo/api/breed/</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">/images/random&quot;</span><span style=\"color:#f8f8f2;\">, breed))\n</span><span style=\"color:#f8f8f2;\">            .await\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">            .json::&lt;DogApi&gt;()\n</span><span style=\"color:#f8f8f2;\">            .await\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> dog_request.</span><span style=\"color:#66d9ef;\">value</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(url)) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx!{{ img {{ url: </span><span style=\"color:#ffee99;\">&quot;{{url}}&quot; </span><span style=\"color:#f8f8f2;\">}} }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(url)) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx!{{ span {{ </span><span style=\"color:#ffee99;\">&quot;Loading dog failed&quot; </span><span style=\"color:#f8f8f2;\">}}  }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx!{{ </span><span style=\"color:#ffee99;\">&quot;Loading dog...&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Additionally, we added better support for coroutines. You can now start, stop, resume, and message with asynchronous tasks. The coroutine is automatically exposed to the rest of your app via the Context API. For the vast majority of apps, Coroutines can satisfy all of your state management needs:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> sync_task </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_coroutine</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx, |</span><span style=\"font-style:italic;color:#fd971f;\">rx</span><span style=\"color:#f8f8f2;\">| async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#66d9ef;\">connect_to_server</span><span style=\"color:#f8f8f2;\">().await;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">MyState::new();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">while </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(action) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> rx.</span><span style=\"color:#66d9ef;\">next</span><span style=\"color:#f8f8f2;\">().await {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#66d9ef;\">reduce_state_with_action</span><span style=\"color:#f8f8f2;\">(action).await;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> sync_task.</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">(SyncAction::Username(</span><span style=\"color:#ffee99;\">&quot;Bob&quot;</span><span style=\"color:#f8f8f2;\">)),\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Click to sync your username to the server&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "all-new-features",
            a { href: "#all-new-features", class: "header", "All New Features" }
        }
        p { "We've covered the major headlining features, but there were so many more!" }
        ul {
            li { "A new router @autarch" }
            li { "Fermi for global state management" }
            li { "Translation of docs and Readme into Chinese @mrxiaozhuox" }
            li { "2.5x speedup by using JS-based DOM manipulation (3x faster than React)" }
            li { "Beautiful documentation overhaul" }
            li { "InlineProps macro allows definition of props within a component's function arguments" }
            li {
                "Improved dev server, hot reloading for desktop and web apps "
                a { href: "https://github.com/mrxiaozhuox", "@mrxiaozhuox" }
            }
            li {
                "Templates: desktop, web, web/hydration, Axum + SSR, and more "
                a { href: "https://github.com/mrxiaozhuox", "@mrxiaozhuox" }
            }
            li { "Web apps ship with console" }
            li { "Enhanced Hydration and server-side-rendering (recovery, validation)" }
            li { "Optional fields for component properties" }
            li {
                "Introduction of the "
                code { "EventHandler" }
                " type"
            }
            li { "Improved use" }
            li { "Improved use" }
            li { "New use" }
            li { "Prevent Default attribute" }
            li { "Provide Default Context allows injection of global contexts to the top of the app" }
            li { "push" }
            li {
                "Add gap and gap"
                a { href: "https://github.com/FruitieX", "@FruitieX" }
            }
            li { "File Drag n Drop support for Desktop" }
            li { "Custom handler support for desktop" }
            li { "Forms now collect all their values in oninput/onsubmit" }
            li { "Async tasks now are dropped when components unmount" }
            li { "Right-click menus are now disabled by default" }
        }
        h2 { id: "fixes",
            a { href: "#fixes", class: "header", "Fixes" }
        }
        ul {
            li { "Windows support improved across the board" }
            li { "Linux support improved across the board" }
            li { "Bug in Calculator example" }
            li { "Improved example running support" }
        }
        p { "A ton more! Dioxus is now much more stable than it was at release!" }
        h2 { id: "community-additions",
            a { href: "#community-additions", class: "header", "Community Additions" }
        }
        ul {
            li {
                a { href: "https://github.com/Zomatree/Revolt-Client/blob/master/src/utils.rs#14-27",
                    "Styled Components macro"
                }
                " "
                a { href: "https://github.com/Zomatree", "@Zomatree" }
            }
            li {
                a { href: "https://github.com/FruitieX/dioxus-websocket-hooks",
                    "Dioxus-Websocket hook"
                }
                " "
                a { href: "https://github.com/FruitieX", "@FruitieX" }
            }
            li {
                a { href: "https://github.com/FruitieX/homectl", "Home automation server app" }
                " "
                a { href: "https://github.com/FruitieX", "@FruitieX" }
            }
            li {
                a { href: "https://github.com/rustkid/recorder", "Video Recording app" }
            }
            li {
                a { href: "https://github.com/autarch/Crumb/tree/master/web-frontend",
                    "Music streaming app"
                }
                " "
                a { href: "https://github.com/autarch", "@autarch" }
            }
            li {
                a { href: "https://gist.github.com/FruitieX/73afe3eb15da45e0e05d5c9cf5d318fc",
                    "NixOS dependancy installation"
                }
                " "
                a { href: "https://github.com/FruitieX", "@FruitieX" }
            }
            li {
                a { href: "https://github.com/lucifer1004/dioxus-vercel-demo",
                    "Vercel Deploy Template"
                }
                " "
                a { href: "https://github.com/lucifer1004", "@lucifer1004" }
            }
            li {
                a { href: "https://github.com/oovm/katex-wasm", "Render Katex in Dioxus" }
            }
            li {
                a { href: "https://github.com/oovm/prism-wasm", "Render PrismJS in Dioxus" }
            }
            li {
                a { href: "https://github.com/houseabsolute/tailwindcss-to-rust",
                    "Compile-time correct TailwindCSS"
                }
            }
            li {
                a { href: "https://github.com/oovm/tailwind-rs", "Autogenerate tailwind CSS" }
            }
            li {
                a { href: "https://github.com/houseabsolute/dioxus-heroicons", "Heroicons library" }
            }
            li {
                a { href: "https://dioxus-convert.netlify.app", "RSX -> HTML translator app" }
            }
            li {
                a { href: "https://github.com/mrxiaozhuox/dioxus-toast", "Toast Support" }
            }
            li { "New Examples: forms, routers, linking, tui, and more!" }
        }
        h2 { id: "looking-forward",
            a { href: "#looking-forward", class: "header", "Looking Forward" }
        }
        p {
            "Dioxus is still under rapid, active development. We'd love for you to get involved! For the next release, we're looking to add:"
        }
        ul {
            li { "Native WGPU renderer support" }
            li { "A query library like react-query" }
            li { "Multiwindow desktop app support" }
            li { "Full LiveView integrations for Axum, Warp, and Actix" }
            li { "A builder pattern for elements (no need for rsx!)" }
            li { "Autoformatting of rsx! code (like cargo fmt)" }
            li { "Improvements to the VSCode Extension" }
        }
        p { "If you're interested in building an app with Dioxus, make sure to check us out on:" }
        ul {
            li {
                a { href: "http://github.com/dioxusLabs/dioxus", "Github" }
            }
            li {
                a { href: "http://reddit.com/r/dioxus/", "Reddit" }
            }
            li {
                a { href: "https://discord.gg/XgGxMSkvUM", "Discord" }
            }
            li {
                a { href: "http://twitter.com/dioxuslabs", "Twitter" }
            }
        }
    }
}
#[component(no_case_check)]
pub fn TemplatesDiffing() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "making-dioxus-almost-as-fast-as-solidjs",
            a {
                href: "#making-dioxus-almost-as-fast-as-solidjs",
                class: "header",
                "Making Dioxus (almost) as fast as SolidJS"
            }
        }
        p {
            a { href: "https://github.com/dioxuslabs/dioxus", "Dioxus" }
            " is a UI library for Rust that makes it easy to target almost any platform with the same React-like codebase. You can build apps for WASM, desktop, mobile, TUI, static-sites, SSR, LiveView, and more."
        }
        hr {}
        p {
            "In preparation for the next big release of Dioxus, one of the lead contributors, ealmloff, added a long-awaited feature: "
            strong { "subtree memoization" }
            "."
        }
        p {
            "Subtree memoization reduces the overall work that Dioxus needs to do to get your desired UI state to the screen by several orders of magnitude. In fact, it’s so fast, that it pushes Dioxus to the leading edge of performance for web frameworks, on par with the likes of SolidJS, even beating out signal-based Rust libraries like Sycamore 0.8 and Leptos 0.0.3."
        }
        p {
            img {
                src: "Making%20Dioxus%20(almost)%20as%20fast%20as%20SolidJS%20baea0d5b4e614351ac8e3d4fc4240d04/Untitled.png",
                alt: "Untitled",
                title: "",
            }
        }
        p {
            "There’s obviously still some room to grow as WASM-based UI libraries face unique challenges compared to their JavaScript counterparts. Ultimately, we hope that this update demonstrates what’s really possible with the current React paradigm."
        }
        p {
            "And, of course, we need to mention that benchmarks never give a truly clear insight into how performant a library is, especially as an app scales. It’s definitely reasonable to believe that as an app grows in size, any other library might come out ahead. You shouldn’t make a decision on the framework for your next project just because it’s slightly more or less performant than any other library based on entirely arbitrary benchmarks."
        }
        p {
            img {
                src: "Making%20Dioxus%20(almost)%20as%20fast%20as%20SolidJS%20baea0d5b4e614351ac8e3d4fc4240d04/Untitled%201.png",
                alt: "Untitled",
                title: "",
            }
        }
        p { "Anyways…" }
        h2 { id: "dioxus-shares-reacts-dna",
            a { href: "#dioxus-shares-reacts-dna", class: "header", "Dioxus shares React’s DNA" }
        }
        p {
            "As eloquently put by the creator of Svelte, the "
            a { href: "https://svelte.dev/blog/virtual-dom-is-pure-overhead",
                "“Virtual DOM is pure overhead”"
            }
            ". So, why does Dioxus continue to share the React DNA if it’s ultimately just frivolous work?"
        }
        p { "Well, we still love React, despite its warts, footguns, and idiosyncrasies." }
        ul {
            li { "React is just JavaScript, no magic compilation needed." }
            li { "Components are just tiny event loops with mostly predictable re-renders." }
            li { "React’s paradigm maps extremely well into Rust." }
        }
        p {
            "The final point is arguably the most important: React’s functional model maps well into Rust’s lifetime system. Any value provided to the component through  "
            code { "use_hook" }
            " is bounded by the lifetime of the  "
            code { "Scope" }
            " itself.  "
            code { "Scope" }
            " can be shared into any handler - like  "
            code { "onclick" }
            " in the following example. Since  "
            code { "value" }
            " shares a lifetime with  "
            code { "Scope" }
            ", it can be modified directly within element callbacks."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value: </span><span style=\"color:#f92672;\">&amp;mut </span><span style=\"font-style:italic;color:#66d9ef;\">u32 </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> cx.</span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Increment&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    ))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This clean mapping of React’s paradigms into Rust makes it possible for Dioxus to achieve excellent developer experience."
        }
        ul {
            li { "Components are just regular functions." }
            li {
                "The foundational "
                code { "use_hook" }
                " provides a direct mutable reference to a value."
            }
            li {
                "Values created with the Scope’s lifetime can be passed directly into children, unlike nearly all non-signal-based libraries."
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> doc </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_document_builder</span><span style=\"color:#f8f8f2;\">(cx);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">\tDoc {{ document: doc }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Doc</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;, </span><span style=\"font-style:italic;color:#fd971f;\">document</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;&#39;a</span><span style=\"color:#f8f8f2;\"> SomeBigDocument) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"color:#8c8c8c;\">// document is passed from a parent by reference!\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"color:#8c8c8c;\">// no smart pointers required!\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "All in all, we’ve learned to love lifetimes rather than fear them. But for all the good of React, we’re still stuck with the bad."
        }
        h2 { id: "overcoming-the-warts-of-react",
            a { href: "#overcoming-the-warts-of-react", class: "header",
                "Overcoming the warts of React"
            }
        }
        p {
            "One of the biggest issues React has is the need to recreate entire chunks of the virtual DOM between renders. If you’re not aware, in React, your JSX markup is converted directly to  "
            code { "React.createElement" }
            " calls."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// This markup\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div class</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;abc&quot;</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\"> Hello world </span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// becomes these calls\n</span><span style=\"color:#f8f8f2;\">React.createElement(div, {{ class: </span><span style=\"color:#ffee99;\">&quot;abc&quot; </span><span style=\"color:#f8f8f2;\">}}, [React.createText(</span><span style=\"color:#ffee99;\">&quot;hello world&quot;</span><span style=\"color:#f8f8f2;\">)]);</span></pre>\n" }
        p {
            "This means for every new element in your tree, the transpiled JS is allocating several objects, arrays, and complex structures between "
            em { "every" }
            " render. There’s no wonder why React isn’t on the top of the performance charts! In Rust, it’s generally not best practice to generate so many heap-allocated objects."
        }
        p { "Heck, there was even a very popular reddit post talking about this problem." }
        p {
            a { href: "https://www.reddit.com/r/rust/comments/yd9ngs/worried_about_modern_rust_gui_libraries/",
                "“Worried about “modern” Rust GUI libraries”"
            }
        }
        p {
            "In Dioxus, we noticed this early on and decided to see if we could reuse all the heap allocations instead of just tossing them out. Inspired by the work on Dodrio, Dioxus is implemented using a bump allocator and double-buffering, just like many high-performance GPU APIs."
        }
        p {
            "When you create a div, or a piece of text, or anything in Dioxus, it simply gets allocated inside a bump arena that gets reset when diffed. No cleanup, no extra heap allocations, just steady-state reuse of pre-allocated memory."
        }
        p {
            img {
                src: "Making%20Dioxus%20(almost)%20as%20fast%20as%20SolidJS%20baea0d5b4e614351ac8e3d4fc4240d04/Screen_Shot_2021-08-17_at_2.24.39_AM.png",
                alt: "Making%20Dioxus%20(almost)%20as%20fast%20as%20SolidJS%20baea0d5b4e614351ac8e3d4fc4240d04/Screen_Shot_2021-08-17_at_2.24.39_AM.png",
                title: "",
            }
        }
        p {
            "This is fast. Really fast. And when coupled with automatic component memoization, it’s really easy to write Dioxus apps that are memory efficient and faster than React."
        }
        p { "Great, case-closed, that’s it, right?" }
        p {
            "Well, no. Dioxus still wasn’t nearly as fast as Svelte, Sycamore, SolidJS, or even InfernoJS. We’ve optimized a bunch of tiny things, like string caching, batched DOM manipulations, faster PartialEq, diffing, and pretty much everything you could think of."
        }
        p {
            "Except, we’re creating new objects, still in the heap, and doing a lot of diffing work. In the words of the creator of Svelte,"
        }
        blockquote {
            p {
                "But you know what would be even faster?\u{a0}"
                em { "Not doing that." }
            }
        }
        h2 { id: "making-dioxus-faster-by-doing-less-work",
            a {
                href: "#making-dioxus-faster-by-doing-less-work",
                class: "header",
                "Making Dioxus faster by doing less work"
            }
        }
        p {
            "To really make Dioxus faster, we need to make it do less work - or, at the very least, less work at runtime. SolidJS does this by thrusting you into this world of signals. We love signals! They might even come to Dioxus at some point (aka Preact signals). But, in the world where we still care about providing  "
            code { "&mut T" }
            " from  "
            code { "use_hook" }
            " , we need to find a new solution that doesn’t require rewriting your React apps to use signals."
        }
        p {
            "Well, we’re working in Rust, we’ve got const, macros, custom PartialEq… let’s see if we can move some of this work to compile time."
        }
        p {
            "To build a Dioxus app, you pretty much have to use the  "
            code { "rsx!" }
            " proc macro. We unfortunately don’t support a builder API or alternatives. There’s a lot of good reasons to do this: performance, forward compatibility, tooling, ergonomics, etc."
        }
        p {
            "A block of  "
            code { "rsx!" }
            " might look like this:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">\tdiv {{\n</span><span style=\"color:#f8f8f2;\">\t\th1 {{</span><span style=\"color:#ffee99;\">&quot;Glorious Counter&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\t\tp {{ </span><span style=\"color:#ffee99;\">&quot;Count: {{val}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\t\tbutton {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> val </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Increment&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\t\tbutton {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> val </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Decrement&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\t}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "If you look closely, you’ll notice that the entire tree is declared within the macro. There aren’t  elements being created at compile time, except for the dynamic text within the paragraph element. In React, you’d have to create every element from scratch, one-by-one, every time. But in Dioxus, we can do better."
        }
        p {
            "The new technique Dioxus uses is to split each  "
            code { "rsx!" }
            " call into a static  "
            code { "Template" }
            " and a list of dynamic nodes. For the above  "
            code { "rsx!" }
            " call, this might look like"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">THIS_TEMPLATE</span><span style=\"color:#f8f8f2;\">: Template </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> Template {{ </span><span style=\"color:#8c8c8c;\">/* */ </span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">VNode {{\n</span><span style=\"color:#f8f8f2;\">\ttemplate: </span><span style=\"color:#ff80f4;\">THIS_TEMPLATE</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">\tdynamic_nodes: [\n</span><span style=\"color:#f8f8f2;\">\t\tText(format_args!(</span><span style=\"color:#ffee99;\">&quot;Count: {{val}}&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">\t]\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "Now, on every render, we only create the single dynamic node. When we go to diff the VNode, we only need to diff that one text node too. So now, instead of 11 comparisons (9 elements and 2 attributes) we have one comparison. Diffing this template takes 90% less time than before! This is a huge win! Our app can be 10x bigger for the same diffing cost. And the results speak for themselves. Combined with the integration of "
            a { href: "https://crates.io/crates/sledgehammer", "Sledgehammer" }
            ", Dioxus is pushing the limits of what the React model can reasonably achieve."
        }
        p {
            img {
                src: "Making%20Dioxus%20(almost)%20as%20fast%20as%20SolidJS%20baea0d5b4e614351ac8e3d4fc4240d04/Untitled.png",
                alt: "Untitled",
                title: "",
            }
        }
        p {
            "The React team also agrees that React can be better. That’s why they’ve started working on an experimental compiler for React."
        }
        p {
            a { href: "https://reactjs.org/blog/2022/06/15/react-labs-what-we-have-been-working-on-june-2022.html",
                "https://reactjs.org/blog/2022/06/15/react-labs-what-we-have-been-working-on-june-2022.html"
            }
        }
        p {
            "The plan here is to cache these elements and only update them when variables inside the "
            em { "component" }
            " change. However, React-Forget still doesn’t fix the underlying issue of node creation, memory usage, or anything of the other things compile-time memoization achieves."
        }
        h2 { id: "taking-it-a-step-further",
            a { href: "#taking-it-a-step-further", class: "header", "Taking it a step further" }
        }
        p {
            "Templates make diffing the tree faster, but they can also make building the UI faster too. Both SolidJS and LitHTML take advantage of this hack to achieve fantastic performance."
        }
        p {
            "With support from the renderer, Dioxus can actually serialize these parsed RSX templates and let the renderer do all the caching."
        }
        p {
            "Before, if we wanted to assemble a tree of nodes from an iterator, we’d have to do a ton of tedious work, creating each list item part by part."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// This tree\n</span><span style=\"color:#f8f8f2;\">ul {{\n</span><span style=\"color:#f8f8f2;\">\t(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">..</span><span style=\"color:#f8f8f2;\">len).</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">| rsx!{{\n</span><span style=\"color:#f8f8f2;\">\t\tli {{\n</span><span style=\"color:#f8f8f2;\">\t\t\th3 {{ </span><span style=\"color:#ffee99;\">&quot;user&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\t\t\tdiv {{ </span><span style=\"color:#ffee99;\">&quot;hello {{id}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\t\t}}\n</span><span style=\"color:#f8f8f2;\">\t}})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// item one...\n</span><span style=\"color:#f8f8f2;\">Edit::CreateElement(</span><span style=\"color:#ffee99;\">&quot;li&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::CreateElement(</span><span style=\"color:#ffee99;\">&quot;h3&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::CreateText(</span><span style=\"color:#ffee99;\">&quot;user&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::AppendChildren(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::CreateElement(</span><span style=\"color:#ffee99;\">&quot;div&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::CreateText(</span><span style=\"color:#ffee99;\">&quot;hello 0&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::AppendChildren(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::AppendChildren(</span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// item two...\n</span><span style=\"color:#f8f8f2;\">Edit::CreateElement(</span><span style=\"color:#ffee99;\">&quot;li&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::CreateElement(</span><span style=\"color:#ffee99;\">&quot;h3&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::CreateText(</span><span style=\"color:#ffee99;\">&quot;user&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::AppendChildren(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::CreateElement(</span><span style=\"color:#ffee99;\">&quot;div&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::CreateText(</span><span style=\"color:#ffee99;\">&quot;hello 0&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::AppendChildren(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">Edit::AppendChildren(</span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// and so on until we attach all the li to the ul\n</span><span style=\"color:#f8f8f2;\">Edit::AppendChildren(len)</span></pre>\n",
        }
        p { "With templates, we can serialize the tree and pass it to the renderer:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">TEMPLATE_HTML </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;&lt;li&gt;&lt;h3&gt;user&lt;/h3&gt;&lt;div&gt;hello _id_&lt;/div&gt;&lt;/li&gt;&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">Edit::SaveTemplate(</span><span style=\"color:#ffee99;\">&quot;demo.rs:5:1&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">TEMPLATE_HTML</span><span style=\"color:#f8f8f2;\">);</span></pre>\n" }
        p {
            "Now, whenever we create the list elements, it’s as simple as cloning some nodes that already exist and precisely modifying just a small part"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">Edit::LoadTemplate(</span><span style=\"color:#ffee99;\">&quot;demo.rs:5:1&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">Edit::HydateText(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;hello 0&quot;</span><span style=\"color:#f8f8f2;\">);</span></pre>\n" }
        p {
            "For the tiny case we’re showing here, the benefit might seem limited. However, for real-world apps with lots of elements, custom styles, and all sorts of extra metadata, this caching system is immensely powerful and extremely performant."
        }
        h2 { id: "what-does-this-enable",
            a { href: "#what-does-this-enable", class: "header", "What does this enable?" }
        }
        p {
            "Now that we’re working with the mindset of templates, we can start to build new functionality previously locked behind the old model."
        }
        h3 { id: "hot-reloading",
            a { href: "#hot-reloading", class: "header", "Hot Reloading" }
        }
        p {
            "One amazing feature added to Dioxus using the new template model is hot reloading. You can now modify your running Dioxus app without recompiling, provided you add, remove, or modify elements inside of  "
            code { "rsx!" }
            " . This mechanism works for "
            em { "any" }
            " renderer too, since each renderer has to implement the same protocol to manage edits."
        }
        p {
            a { href: "Making%20Dioxus%20(almost)%20as%20fast%20as%20SolidJS%20baea0d5b4e614351ac8e3d4fc4240d04/174206798-1b73e42a-0b36-4bce-83c4-aa7d875ec800.mp4",
                "174206798-1b73e42a-0b36-4bce-83c4-aa7d875ec800.mp4"
            }
        }
        p {
            "Not only can templates be cached inside of a renderer, they can be modified after-the-fact. The renderer is smart enough to track down the instance of every template node on the page and apply the same patches."
        }
        h3 { id: "performant-liveview",
            a { href: "#performant-liveview", class: "header", "Performant LiveView" }
        }
        p {
            "Another addition to Dioxus 0.3 is the new LiveView renderer. Much like its counterpart Phoenix LiveView, Dioxus LiveView enables entirely server-rendered apps and components while shipping minimal JS to the client. In the Liveview model, minimizing latency and bandwidth is crucial to keeping apps snappy, especially for lower-end clients."
        }
        p {
            img {
                src: "Making%20Dioxus%20(almost)%20as%20fast%20as%20SolidJS%20baea0d5b4e614351ac8e3d4fc4240d04/ElixirLivewview.jpg",
                alt: "ElixirLivewview.jpg",
                title: "",
            }
        }
        p {
            "Now, instead of sending hundreds or thousands of edits to the client to render things like lists and complex markup, Dioxus packages all the templates the client will use inside of the HTML shipped to the client. A sample HTML document that might be sent from the server to the client may look like this:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">head</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">template id</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;demo.rs:123:456&quot;</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">li</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">\t\t\t</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">h3</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">user&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">h3</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">\t\t\t</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">hello _id_&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">li</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">template</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">head</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">body</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div id</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;main&quot;</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#f92672;\">&lt;!--</span><span style=\"color:#f8f8f2;\"> pre</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">rendered page </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">-&gt;\n</span><span style=\"color:#f8f8f2;\">\t&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">body</span><span style=\"color:#f92672;\">&gt;</span></pre>\n",
        }
        p {
            "Notice how the templates are collected during SSR and inserted into the header. The only edits sent over the network from the server to the client are commands to create/remove template nodes and to precisely modify just the nodes that changed. Fast, simple, and scalable!"
        }
        h2 { id: "faster-server-side-rendering-ssr",
            a { href: "#faster-server-side-rendering-ssr", class: "header",
                "Faster Server-Side-Rendering (SSR)"
            }
        }
        p {
            "The other technique that SolidJS uses to achieve faster SSR performance is combining pre-rendered portions of templates together through string concatenation. Since the template is known at compile time, we can break it up into smaller chunks and just stitch them together during rendering. No need to build and traverse huge element trees!"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Cached template segments:\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">PreRendered(</span><span style=\"color:#ffee99;\">&quot;&lt;div class=</span><span style=\"color:#ff80f4;\">\\&quot;</span><span style=\"color:#ffee99;\">asdasdasd</span><span style=\"color:#ff80f4;\">\\&quot;</span><span style=\"color:#ffee99;\"> class=</span><span style=\"color:#ff80f4;\">\\&quot;</span><span style=\"color:#ffee99;\">asdasdasd</span><span style=\"color:#ff80f4;\">\\&quot;</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">(),),\n</span><span style=\"color:#f8f8f2;\">Attr(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,),\n</span><span style=\"color:#f8f8f2;\">PreRendered(</span><span style=\"color:#ffee99;\">&quot;&gt;Hello world 1 --&gt;&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">(),),\n</span><span style=\"color:#f8f8f2;\">Node(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,),\n</span><span style=\"color:#f8f8f2;\">PreRendered(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;&lt;-- Hello world 2&lt;div&gt;nest 1&lt;/div&gt;&lt;div&gt;&lt;/div&gt;&lt;div&gt;nest 2&lt;/div&gt;&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">Node(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">,),\n</span><span style=\"color:#f8f8f2;\">Node(</span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">,),\n</span><span style=\"color:#f8f8f2;\">PreRendered(</span><span style=\"color:#ffee99;\">&quot;&lt;/div&gt;&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">(),)</span></pre>\n",
        }
        h2 { id: "disclaimer",
            a { href: "#disclaimer", class: "header", "Disclaimer" }
        }
        p {
            "Even with all the innovations here, it’s still very important to remember that Dioxus still takes after React. No matter how many tweaks, optimizations, and improvements we make to Dioxus, you can still shoot yourself in the foot with the classic React footguns."
        }
        p { "These include" }
        ul {
            li { "Unkeyed lists" }
            li { "Poor use of memoization and comparisons" }
            li { "Misuse of use_effect" }
            li { "“God components” that do everything" }
        }
        p {
            "and a whole host of other issues that you might not find in frameworks like Solid and Sycamore."
        }
        p {
            "That being said, since Dioxus relies on a VirtualDom, it can be used as the primary state system for any renderer. And we have a ton of options for renderers:"
        }
        ul {
            li { "Desktop (webview)" }
            li { "Mobile (webview)" }
            li { "Web" }
            li { "TUI" }
            li { "Skia" }
            li { "LiveView" }
            li { "Blitz (WGPU)" }
            li { "SSR + Hydration" }
            li { "Static site generation" }
            li { "VR/AR (coming soon!)" }
        }
        p {
            "Note that all this work is being done for Dioxus 0.3 and hasn’t yet been released as a major version. We’re still dogfooding these changes to make sure no new bugs have slipped through. If you want these changes released sooner rather than later, feel free to build something with  "
            code { "main" }
            " and let us know!"
        }
        ul {
            li { "Discord" }
            li { "Github" }
            li { "Reddit Post" }
        }
    }
}
#[component(no_case_check)]
pub fn Release030() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p {
            "If you’re new here: Dioxus (dye•ox•us) is a library for building React-like user interface in Rust. Dioxus supports a ton of targets: web, desktop, mobile, TUI, and more. On the web it renders via the DOM and on desktop and mobile you can choose between the WebView DOM, WGPU, or Skia."
        }
        p {
            "Dioxus 0.3 is bringing a "
            em { "lot" }
            " of fantastic new features:"
        }
        ul {
            li { "Massive performance improvements" }
            li { "Hot reloading for web and desktop" }
            li {
                "Autoformatting for RSX via "
                code { "dioxus fmt" }
            }
            li { "New LiveView renderer" }
            li { "Input widgets for TUI" }
            li { "Lua plugin system for CLI and overhaul of CLI" }
            li { "Multi window desktop apps and direct access to Tao/Wry" }
            li { "General improvements to RSX (if chains, for loops, boolean attributes, any values)" }
            li { "Rusty event types with support for complex techniques like file uploading" }
            li { "Skia renderer and WGPU renderer" }
            li { "Chinese and Portuguese translations" }
            li { "A new landing page" }
        }
        p {
            "This release represents an absolutely massive jump forward for the Dioxus ecosystem. We hope to ship future features more quickly into stable now that many of the desired breaking changes have been incorporated into the core library."
        }
        h2 { id: "templates-and-performance-improvements",
            a {
                href: "#templates-and-performance-improvements",
                class: "header",
                "Templates and performance improvements"
            }
        }
        p {
            "We’ve made huge changes underpinning the architecture of Dioxus. The significance of these changes is hard to describe in this simple release document, but we did write a blog post about it "
            a { href: "https://dioxuslabs.com/blog/templates-diffing/", "here" }
            ". Now, Dioxus performance is on par with of SolidJS."
        }
        p {
            img {
                src: "https://i.imgur.com/9rbAXP9.png",
                alt: "Js-framework-benchmark of Dioxus showing good performance",
                title: "",
            }
        }
        p {
            "Additionally, we’ve reworked how desktop apps stream edits from the native thread into the webview, greatly improving performance."
        }
        h2 { id: "hot-reloading",
            a { href: "#hot-reloading", class: "header", "Hot Reloading" }
        }
        p {
            "Dioxus can now update how your app looks without recompiling the underlying Rust code. For developers who choose to write their user interfaces with the RSX macro, the Dioxus development server will automatically update the appearance of a running app whenever the macro body is modified."
        }
        p {
            "We’ve found hot reloading to significantly speed up development cycles, making it faster than ever to iterate your app."
        }
        p {
            img {
                src: "https://i.imgur.com/OzIURca.mp4",
                alt: "hotreload full",
                title: "",
            }
        }
        p {
            "Note that hot reloading works by interpreting the body of RSX macro calls. If the hot reloading engine detects a modification unrelated to RSX, then it will force a full refresh of the app."
        }
        h2 { id: "autoformatting",
            a { href: "#autoformatting", class: "header", "Autoformatting" }
        }
        p {
            "Another widely requested feature - autoformatting - is now built into the Dioxus CLI and VSCode Extension. Using the same interpreter engine in hot reloading, your code can now be formatted automatically. This saves a ton of time and ensures consistency between code commits."
        }
        p { "Autoformatting can be used via the VSCode Extension which will autoformat as you code." }
        p {
            img {
                src: "https://i.imgur.com/aPQEFNO.mp4",
                alt: "autofmt.mov",
                title: "",
            }
        }
        p {
            "Or directly for use in CI or non-vscode editors with the  "
            code { "dioxus fmt" }
            " command."
        }
        p {
            img {
                src: "https://i.imgur.com/WrNZZdW.mp4",
                alt: "dioxusfmt.mov",
                title: "",
            }
        }
        p {
            "Autoformatting respects some simple rustfmt features but is still in its early stages. If you find any quirks or disagree with the formatting style, feel free to file an issue."
        }
        h2 { id: "liveview-and-livecomponents",
            a { href: "#liveview-and-livecomponents", class: "header",
                "LiveView and LiveComponents"
            }
        }
        p {
            "Dioxus 0.3 marks the first official release of dedicated tooling for LiveView. LiveView is a new web-app development paradigm that combines the simplicity of server-side rendering with the rich interactivity of the single-page-application."
        }
        p {
            img {
                src: "https://i.imgur.com/Eiejo1h.mp4",
                alt: "liveviewdemo.mov",
                title: "",
            }
        }
        p {
            "Because there’s no frontend build step or need for a dedicated backend, writing LiveView apps is easy. LiveView lets you freely mix database access into your frontend code, saving the hassle of a dedicated backend. LiveView is the fastest way to build a complete app in Rust."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> router </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Router::new()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">route</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">dioxus_liveview::body(addr))\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">route</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;/app&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">ws</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">dioxus_liveview::render(ws));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    axum::Server::bind(</span><span style=\"color:#ffee99;\">&quot;127.0.0.1&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">serve</span><span style=\"color:#f8f8f2;\">(router.</span><span style=\"color:#66d9ef;\">into_make_service</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">        .await;\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> posts </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_db_query</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"color:#ff80f4;\">RECENT_POSTS</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\t\trender! {{\n</span><span style=\"color:#f8f8f2;\">\t\t\t\t</span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> post </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> posts {{\n</span><span style=\"color:#f8f8f2;\">\t\t\t\t\t\tPost {{ key: </span><span style=\"color:#ffee99;\">&quot;{{post.id}}&quot;</span><span style=\"color:#f8f8f2;\">, data: post }}\n</span><span style=\"color:#f8f8f2;\">\t\t\t\t}}\n</span><span style=\"color:#f8f8f2;\">\t\t}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "tui-input-widgets",
            a { href: "#tui-input-widgets", class: "header", "TUI Input Widgets" }
        }
        p {
            "Up to this point, Dioxus rendered into the terminal using just static elements. Now, with the release of Dioxus 0.3, we’re shipping a collection of input widgets for common utilities like buttons, sliders, text inputs, checkboxes, and more. These same widgets provide a basis of functionality for the native renderers we mention below."
        }
        p {
            img {
                src: "https://i.imgur.com/oXQC5o5.mp4",
                alt: "tuiinputs.mp4",
                title: "",
            }
        }
        h2 { id: "multi-window-desktop-apps",
            a { href: "#multi-window-desktop-apps", class: "header", "Multi-window Desktop Apps" }
        }
        p {
            "The Dioxus VirtualDom and tao/wry event loop now share the same scheduler, allowing full control of the window and event loop from within your desktop and mobile app. This unlocks everything a typical tauri app might have access to, allowing Dioxus to share more code with the rest of the Tauri ecosystem."
        }
        p {
            "One big advantage of this is the ability to open and close multiple windows from within your Dioxus app. With access to the event loop, you can even get a raw window handle, allowing alternative rendering engines like OpenGL or WGPU."
        }
        p {
            img {
                src: "https://i.imgur.com/4Yg9FWd.mp4",
                alt: "multiwindow.mov",
                title: "",
            }
        }
        h2 { id: "lowercase-components",
            a { href: "#lowercase-components", class: "header", "Lowercase components" }
        }
        p {
            "We’ve expanded what can be considered a component. Lowercase components are now accepted in the rsx macro provided that they either"
        }
        ul {
            li {
                "Use the path syntax (ie "
                code { "module::component" }
                ")"
            }
            li { "Container an underscore character" }
        }
        p {
            "This is a similar restriction as found in other frameworks. Note that you still cannot define a one-word component without referencing it via path syntax. We’re hoping to resolve this soon, but it’s not a very easy problem to solve."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">header {{}}              ❌\n</span><span style=\"color:#f8f8f2;\">module::header {{}}      ❌\n</span><span style=\"color:#f8f8f2;\">my_header {{}}           ✅</span></pre>\n" }
        h2 { id: "for-loops-if-chains-and-more-flexible-rsx",
            a {
                href: "#for-loops-if-chains-and-more-flexible-rsx",
                class: "header",
                "For Loops, If Chains, and more flexible RSX"
            }
        }
        p {
            "We’ve made the rsx macro a lot more flexible with some new features to simplify lists and if statements."
        }
        p {
            "Before, if you wanted to render a list, you’d need to create an iterator and map it to rsx. Now, we apply an automatic transformation of any  "
            code { "for" }
            " loop into an iterator. This should make lists more readable!"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> dog </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> doggos {{\n</span><span style=\"color:#f8f8f2;\">\tdiv {{ key: </span><span style=\"color:#ffee99;\">&quot;{{dog.id}}&quot;</span><span style=\"color:#f8f8f2;\">,  </span><span style=\"color:#ffee99;\">&quot;Dog: {{dog.name}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        h2 { id: "preliminary-wgpu-renderer",
            a { href: "#preliminary-wgpu-renderer", class: "header", "Preliminary WGPU renderer" }
        }
        p {
            "Dioxus 0.3 delivers on another commonly requested feature: native (non-web browser) rendering. This new update brings a very young, very unstable, but surprisingly capable WGPU renderer. This renderer is the culmination of many months of work: collaboration with the Bevy team to revive Taffy (flexbox), integration of the new Vello renderer, and research into highly efficient incremental screen patching."
        }
        p {
            "The renderer is very raw but already capable of rendering HTML, CSS, and responding to user input. We’re actively working on adding accessibility support using the work done by EGUI as inspiration."
        }
        p {
            img {
                src: "https://i.imgur.com/NVp4COt.mp4",
                alt: "wgpu",
                title: "",
            }
        }
        h2 { id: "skia-renderer",
            a { href: "#skia-renderer", class: "header", "Skia Renderer" }
        }
        p {
            "While not exactly a Dioxus Labs project, we wanted to make sure to call out the new Freya editor for Dioxus which uses Skia instead of Vello. Freya takes a different approach from Dioxus-Native in that instead of adhering to HTML and CSS, it sets its own styling and layout strategy. This has a different learning curve - you  can’t take your CSS knowledge with you, but you get a styling system better designed for the job."
        }
        p {
            "Freya is already an amazing piece of technology and has support for things like camera input and infinite canvas."
        }
        h2 { id: "completing-support-for-cross-platform-events",
            a {
                href: "#completing-support-for-cross-platform-events",
                class: "header",
                "Completing support for cross-platform events"
            }
        }
        p {
            "A common complaint with Dioxus’ event system is its reliance on imperfect web standards. For Dioxus 0.3, we overhauled the public API for events to be more “Rusty.” Instead of shipping our own types like keyboard keys, we now provide an API comfortable for the everyday Rustacean. You can now do mouse position math with  "
            code { "euclid" }
            ", match on keys native to  "
            code { "keyboard-types" }
            ", and get helpful docs with cargo-doc. Dioxus also now provides better support for file upload and drag-and-drop operations by downcasting the native event type if it exists."
        }
        p {
            img {
                src: "https://i.imgur.com/DHBvvVy.mp4",
                alt: "dragdropworks.mov",
                title: "",
            }
        }
        p {
            "Note that the old JS-like API is still available (but deprecated) and will be phased out in a future release of Dioxus."
        }
        h2 { id: "lua-plugin-system-for-cli",
            a { href: "#lua-plugin-system-for-cli", class: "header", "Lua Plugin System for CLI" }
        }
        p {
            "The CLI has been overhauled with a ton of new features and improved ergonomics. One major improvement to the CLI is the addition of a Lua-based plugin system. In the future we to expand the plugin system to any WASI-compatible modules but have simply opted for Lua support in the short term while we figure out the API."
        }
        h2 { id: "translations",
            a { href: "#translations", class: "header", "Translations" }
        }
        p {
            "The community seems to really enjoy Dioxus! And they want their friends to know about Dioxus, too! But, our guides have not been available in every language that developers want. In this release, we’re adding two new languages to our guide:"
        }
        ul {
            li {
                "Chinese provided by "
                a { href: "https://github.com/mrxiaozhuox",
                    code { "@mrxiaozhuox" }
                }
            }
            li {
                "Portuguese provided by "
                a { href: "https://github.com/whoeverdidthis",
                    code { "@whoeverdidthis" }
                }
            }
        }
        h2 { id: "a-new-landing-page-and-better-docs",
            a { href: "#a-new-landing-page-and-better-docs", class: "header",
                "A new landing page and better docs"
            }
        }
        p {
            "If you haven’t already noticed, our homepage is cleaner, more direct, and a bit more eye-catching. Check it out if you haven’t!"
        }
        p {
            "As part of our translation and Rust-ification work, "
            a { href: "https://github.com/renis",
                code { "@renis" }
            }
            " has overhauled our guide to be more familiar for Rust developers. This skips some of the boilerplate (IE install Rust) and gets straight into the action of building Dioxus apps."
        }
        h2 { id: "community-projects",
            a { href: "#community-projects", class: "header", "Community Projects" }
        }
        ul {
            li { "Styled components" }
            li { "Opinionated starter pack" }
            li { "Icon pack" }
            li { "Caesar cyhper" }
            li { "LED Strip controller" }
            li { "GTK Renderer" }
            li { "Formalize" }
            li { "Story diagrammer" }
            li { "Table crate" }
            li { "Dioxus Helmet" }
            li { "Skia renderer" }
            li { "Use fetch" }
            li { "Bevy Integration" }
        }
    }
}
#[component(no_case_check)]
pub fn Fulltime() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "going-full-time",
            a { href: "#going-full-time", class: "header", "Going full time" }
        }
        blockquote {
            p { "Jan 5, 2023" }
        }
        blockquote {
            p {
                a { href: "https://github.com/jkelleyrtp", "@jkelleyrtp" }
            }
        }
        p {
            "Hey folks, we’re going to deviate from the typical release post or technical discussion and talk about the future of Dioxus. If you’re new here, Dioxus is a UI library for Rust that supports web, desktop, mobile, liveview, TUI, and more. Our goal is to simplify app development, combining projects like React, Electron, Flutter, NextJS, InkJS, and Phoenix under one unified stack."
        }
        p {
            "I’m happy to announce that I’m now working on Dioxus Labs full time. Thanks to the generous support of Futurewei, Satellite.im, the GitHub Accelerator program, and several amazing individuals, Dioxus Labs is now able to employ both myself and top contributors like ealmloff full time."
        }
        p {
            "Over the past year, Dioxus has grown significantly. We’ve made huge strides in pushing forward the Rust frontend ecosystem. Some of the amazing innovations in this space include hot-reloading, syn-based autoformatting, and dioxus-liveview. Built on top of these innovations are breakthrough projects like Sledgehammer, Taffy, Freya, and Blitz. We want to continue innovating while also maturing Dioxus on all fronts."
        }
        p {
            "Going full time on open source is a huge jump. It takes a lot of courage to leave a company as great as Cloudflare. Being independent truly means independent - no work colleagues, no free snacks, no transit card, no beautiful office, and no company-sponsored health insurance. That being said, I’m eternally grateful to have the opportunity to pursue Dioxus Labs with my entire passion. We are committed to helping developers build better apps."
        }
        p {
            "We have big plans for the future. Here’s a rough sketch of what the future holds for Dioxus:"
        }
        ul {
            li { "Massively overhauled docs with tutorial videos and one-click-deploy example projects" }
            li {
                "Stable support for "
                code { "async" }
                " components, suspense boundaries, and error boundaries"
            }
            li { "Stable release of Blitz (our HTML/CSS renderer built on Vello) with an open API" }
            li { "A deployment and release management platform for Dioxus-based apps" }
            li { "An overhaul of the Dioxus Router with support for many NextJS features" }
            li { "A standard library of cross-platform functionality (GPS/notifications)" }
            li {
                "Better DevTool including VirtualDom visualization, live state inspection, and visual editing"
            }
            li {
                "Support for panic recovery and bundle splitting in rustc for "
                code { "wasm32-unknown-unknown" }
            }
        }
        p {
            "There’s a lot more on the roadmap. If you’re at all interested in contributing to Dioxus, let us know in the community discord, and we’ll be there to help. If you’re interested in supporting the project to help us grow, please reach out."
        }
        p {
            "Again, a huge thanks to our wonderful sponsors and an even bigger thanks to the Rust community who have used and contributed to Dioxus over the past year."
        }
        p { "Yours truly," }
        p { "Jonathan Kelley" }
    }
}
#[component(no_case_check)]
pub fn Release040() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Welcome back, get your snacks, Dioxus 0.4 just dropped." }
        p {
            "If you’re new here: Dioxus (dye•ox•us) is a library for building React-like user interface in Rust. Dioxus supports a ton of targets: web, desktop, mobile, TUI, and more."
        }
        p { "Dioxus 0.4 is adding support for the next frontier: the server backend." }
        p { "You can now write your entire fullstack web app in one file." }
        p {
            img { src: "/assets/static/04meme.png", alt: "meme", title: "" }
        }
        p { "The gist of this release:" }
        ul {
            li { "Server functions" }
            li { "Server-compatible suspense" }
            li { "Overhauled (and beautiful!) interactive documentation" }
            li { "Overhauled and supercharged new router" }
            li { "First-party support for Android with new getting started guides" }
            li { "Backend-agnostic hooks for common app uses cases" }
            li { "Desktop Hot Reloading" }
            li { "Tauri-bundle built into the Dioxus CLI" }
            li { "Polish, bug fixes, stability, testing, and more!" }
        }
        h2 { id: "weekly-office-hours",
            a { href: "#weekly-office-hours", class: "header", "Weekly Office Hours" }
        }
        p {
            "Before we dive right into the bulk of this release, we want to make sure everyone knows that Dioxus Labs now has weekly office hours, every Friday at 9am PST."
        }
        p { "These are held on the community Discord - with an invite here:" }
        p {
            a { href: "https://discord.gg/XgGxMSkvUM", "Join the Dioxus Labs Discord Server!" }
        }
        p {
            "In the office hours you can get help with your app, learn how to contribute, get feedback on code, and "
            a { href: "https://www.notion.so/Dioxus-Labs-Public-Roadmap-771939f47d13423abe2a2195b5617555?pvs=21",
                "help shape the roadmap."
            }
            " We hope to see you there!"
        }
        h2 { id: "server-functions",
            a { href: "#server-functions", class: "header", "Server Functions" }
        }
        p {
            "These days, every cool UI library has some sort of backend framework to do server stuff. This could be interacting with a database, uploading files, working with websockets, you name it. With Dioxus 0.4, we’re adding our first backend solution: Server Functions."
        }
        p {
            "Server Functions are functions annotated with the  "
            code { "server" }
            " procedural macro that generates an RPC client and server for your app. With a single function declaration, you get both the server endpoint "
            em { "and" }
            " the client required to interact with the endpoint."
        }
        p {
            "For example, take this simple Server Function. We’re using the awesome "
            a { href: "https://github.com/trevyn/turbosql", "turbosql" }
            " crate by "
            a { href: "https://github.com/trevyn", "trevyn" }
            " to interact with a sqlite database to load a person’s username."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[server]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_username</span><span style=\"color:#f8f8f2;\">() -&gt; Result&lt;String&gt; {{\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"color:#8c8c8c;\">// Using turbosql to extract some data from the DB\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(select!(</span><span style=\"font-style:italic;color:#66d9ef;\">String </span><span style=\"color:#ffee99;\">&quot;SELECT name FROM person&quot;</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "The  "
            code { "server" }
            " macro will generate a different helper function depending on the configuration flags of your project. If you build your project for the backend ( "
            code { "server" }
            "), then your endpoint will be automatically injected into any axum/salvo/warp router created with  "
            code { "dioxus_fullstack" }
            ". However, if you build your project with any other feature flag, like,  "
            code { "client" }
            ", then the macro generates the "
            em { "call" }
            " to the server."
        }
        p {
            img {
                src: "/assets/static/split_codegen.png",
                alt: "Server / Client code splitting",
                title: "",
            }
        }
        p {
            "This approach makes it easy to incrementally add new backend functionality to your app. Of course, this has some disadvantages - the backend is rather tightly coupled to the frontend - but for most apps, Server Functions will be a huge productivity win."
        }
        p {
            "Server Functions are agnostic to the backend framework you’re using, and support a number of generic operations common across axum, warp, and salvo. For instance, you can extract any header from the request, guarding on things like valid headers and cookies:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">axum::{{TypedHeader, headers::UserAgent}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_fullstack::extract;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[server]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">log_user_agent</span><span style=\"color:#f8f8f2;\">() -&gt; ServerFnResult {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> user_agent: TypedHeader&lt;UserAgent&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">extract</span><span style=\"color:#f8f8f2;\">().await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(())\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "You can attach middleware either at the server level or individually on server functions. The new fullstack utilities work seamlessly with "
            a { href: "https://docs.rs/tower/latest/tower/index.html", "Tower" }
            ", so any server function can be annotated with a middleware."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Add a timeout middleware to the server function that will return\n</span><span style=\"color:#8c8c8c;\">// an error if the function takes longer than 1 second to execute\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::time::Duration;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">tower_http::timeout::TimeoutLayer;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">tokio::time::sleep;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[server]\n</span><span style=\"color:#f8f8f2;\">#[middleware(TimeoutLayer::new(Duration::from_secs(1)))]\n</span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">timeout</span><span style=\"color:#f8f8f2;\">() -&gt; ServerFnResult {{\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#66d9ef;\">sleep</span><span style=\"color:#f8f8f2;\">(Duration::from_secs(</span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">)).await;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(())\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Combining all these concepts together, you can quickly add features like Authentication to your fullstack app. We’ve put together a "
            a { href: "https://github.com/dioxuslabs/dioxus/blob/main/packages/fullstack/examples/axum-auth/src/main.rs",
                "simple axum-auth example for you to get started"
            }
            "."
        }
        p {
            "Our goal with Server Functions is to lay the foundation for our final vision of Dioxus: a fullstack, crossplatform, fully typed, and lightning fast toolkit for building, deploying, monitoring, and scaling any app you can dream of. With one ecosystem, you can quickly build complete apps that run on desktop, mobile, web with a type-safe backend to boot."
        }
        h2 { id: "suspense",
            a { href: "#suspense", class: "header", "Suspense" }
        }
        p {
            "One feature that has been missing in Dioxus since its release is the ability to wait for futures to complete before generating the final server-side-rendered HTML. Before, the expectation was that you’d load any data "
            em { "ahead of time," }
            " and pass in the data via Root Props. We quickly learned this caused issues with scalability: you might not want to fetch every piece of props depending on the route."
        }
        p {
            img {
                src: "/assets/static/old_fetch.png",
                alt: "Diagram of how SSR data used to be fetched",
                title: "",
            }
        }
        p {
            "To solve this, we’re adding future-compatible  "
            code { "Suspense" }
            " - now integrated with Dioxus Fullstack and Server Functions.  Suspense is freely available to components via the  "
            code { "cx.suspend()" }
            " method. Calling  "
            code { "suspend" }
            " will tell Dioxus that this particular component is blocking the completion of the final render due to a pending future. The most basic usage of Suspense is pausing rendering until a data fetch has been completed:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Username</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> username </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_future</span><span style=\"color:#f8f8f2;\">(cx, (), |_| </span><span style=\"color:#66d9ef;\">get_username</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"color:#8c8c8c;\">// Try to extract the current value of the future\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(username) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> username.</span><span style=\"color:#66d9ef;\">value</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#8c8c8c;\">// Early return and mark this component as suspended\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#f92672;\">return</span><span style=\"color:#f8f8f2;\"> cx.</span><span style=\"color:#66d9ef;\">suspend</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\t}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\trender! {{ </span><span style=\"color:#ffee99;\">&quot;Username: {{username}}&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Now, we can do datafetching "
            em { "inside" }
            " components, greatly simplifying our project structure. With the new "
            code { "use_server_future" }
            " hook, we can persist the result of the fetch between server render and client render, allowing hydration to work seamlessly."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Dashboard</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// use_server_future will persist the result of this call during SSR\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> ip </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_server_future</span><span style=\"color:#f8f8f2;\">(cx, (), |_| </span><span style=\"color:#66d9ef;\">get_server_ip</span><span style=\"color:#f8f8f2;\">())</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    render!{{ </span><span style=\"color:#ffee99;\">&quot;The edge node is {{ip}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// When used on the server, this is just a simple function call\n</span><span style=\"color:#f8f8f2;\">#[server]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_server_ip</span><span style=\"color:#f8f8f2;\">() -&gt; Result&lt;String&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(reqwest::get(</span><span style=\"color:#ffee99;\">&quot;https://httpbin.org/ip&quot;</span><span style=\"color:#f8f8f2;\">).await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">text</span><span style=\"color:#f8f8f2;\">().await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "With inline datafetching, we can greatly simplify the amount of work required to build fullstack apps. In this diagram of a Dioxus app with suspense, we can see that suspended components are marked yellow. When their futures completed, Dioxus will continue rendering them, cascading into their children until all futures have been resolved."
        }
        p {
            img {
                src: "/assets/static/new_fetch.png",
                alt: "Diagram of how SSR data is fetched now",
                title: "",
            }
        }
        p {
            "Adopting suspense and fullstack should be easy. Now, when you go to render the app, you can simply call  "
            code { "wait_for_suspense()" }
            " before pre-rendering the markup:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> app </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">VirtualDom::new(app_fn);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Start the futures\n</span><span style=\"color:#f8f8f2;\">app.</span><span style=\"color:#66d9ef;\">rebuild</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Wait for suspended futures to complete\n</span><span style=\"color:#f8f8f2;\">app.</span><span style=\"color:#66d9ef;\">wait_for_suspense</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Now render the app out\n</span><span style=\"color:#f8f8f2;\">dioxus_ssr::prerender(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">app);</span></pre>\n",
        }
        p {
            "Note that this is not 100% equivalent to React’s suspense as it’s not currently possible to render loading states or error boundaries. These features are currently experimental and will be stabilized during the 0.4 release cycle."
        }
        h2 { id: "enum-router",
            a { href: "#enum-router", class: "header", "Enum Router" }
        }
        p {
            "Since the initial release of Dioxus, we’ve had a very simple App Router, inspired by the older versions of React Router. Most UI toolkits these days provide a Router object that interacts with the browser’s router, and to date, Dioxus’ router has been pretty simple."
        }
        p {
            "In the beginning we opted for simplicity and flexibility. The old router let you create route trees with just components. This router was easy to add new routes to and easy to compose."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n</pre>\n" }
        p {
            "However, after thinking about the new features we wanted to add to Dioxus, we realized that this router design wouldn’t cut it in the future. We were lacking many crucial features like nested routes and type-safe URLs."
        }
        p {
            "So, we designed a new router built around the Rust  "
            code { "enum" }
            ". Now, you assemble your app’s route by deriving the  "
            code { "Routable" }
            " trait for a given enum. Routes are simply enum variants! Even better, everything is 100% typesafe. No longer can you craft invalid URLs - saving your apps from a whole host of problems."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize, Debug)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">\t#[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">\t#[redirect(</span><span style=\"color:#ffee99;\">&quot;/platforms&quot;</span><span style=\"color:#f8f8f2;\">, || Route::Homepage {{}})]\n</span><span style=\"color:#f8f8f2;\">\tHomepage {{}},\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\t#[route(</span><span style=\"color:#ffee99;\">&quot;/awesome&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">\tAwesome {{}},\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\t#[route(</span><span style=\"color:#ffee99;\">&quot;/learn&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">\tLearn {{}},\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\t#[route(</span><span style=\"color:#ffee99;\">&quot;/tutorials/:id&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">\tTutorial {{ id: </span><span style=\"font-style:italic;color:#66d9ef;\">usize </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\t#[route(</span><span style=\"color:#ffee99;\">&quot;/blog&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">\tBlogList {{}},\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\t#[route(</span><span style=\"color:#ffee99;\">&quot;/blog/:post&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">\tBlogPost {{ post: </span><span style=\"font-style:italic;color:#66d9ef;\">String </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\t#[route(</span><span style=\"color:#ffee99;\">&quot;/:..segments&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">  Err404 {{ segments: Vec&lt;String&gt; }},\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "To render the new router, pass in your app’s Route enum as the generic type in the Router, and Dioxus will render the right component, given that the enum variant."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\trender! {{ Router::&lt;Route&gt; {{}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "The  "
            code { "#[derive(Routable)]" }
            " will automatically generate the  "
            code { "render" }
            " function for your Router. The Router will render the right route given that a similarly named component is in scope. You can override this yourself, or just stick with the default. The simplest app using the new router would look something like this:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// 1. Declare our app&#39;s routes\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">\t#[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">\tHomepage {{}},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// 2. Make sure we have a component in scope that matches the enum variant\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Homepage</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\trender! {{ </span><span style=\"color:#ffee99;\">&quot;Welcome home!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// 3. Now render our app, using the Router and Route\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\trender! {{ Router::&lt;Route&gt; {{}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Passing in attributes from the route itself is easy too. The  "
            code { "Routable" }
            " macro will accept any fields that implement  "
            code { "FromStr" }
            ", so you can easily add attributes, queries, and named parameters to your app."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Declare our app&#39;s routes\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">\t#[route(</span><span style=\"color:#ffee99;\">&quot;/blog/:post&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">\tBlogPost {{ post: </span><span style=\"font-style:italic;color:#66d9ef;\">String </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BlogPost</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope, </span><span style=\"font-style:italic;color:#fd971f;\">post</span><span style=\"color:#f8f8f2;\">: String) {{\n</span><span style=\"color:#f8f8f2;\">\trender!{{ </span><span style=\"color:#ffee99;\">&quot;Currently viewing: {{post}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Likewise, you can catch 404s using the trailing segments syntax." }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Declare our app&#39;s routes\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">\t#[route(</span><span style=\"color:#ffee99;\">&quot;/:..segments&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">  Err404 {{ segments: Vec&lt;String&gt; }},\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "Another exciting feature is layouts. We’re borrowing this concept from popular frameworks like Remix and Next.JS. Layouts make it easy to wrap Route components together in a shared set of components. A common use case is wrapping your app in a Header, Footer, or Navbar. Without layouts, you’d have a lot of code duplication"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Home</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\trender! {{\n</span><span style=\"color:#f8f8f2;\">\t\tHeader {{}}\n</span><span style=\"color:#f8f8f2;\">\t\tNavbar {{}}\n</span><span style=\"color:#f8f8f2;\">\t\tdiv {{ </span><span style=\"color:#ffee99;\">&quot;actual home content&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\t\tFooter {{}}\n</span><span style=\"color:#f8f8f2;\">\t}}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Blog</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\trender! {{\n</span><span style=\"color:#f8f8f2;\">\t\tHeader {{}}\n</span><span style=\"color:#f8f8f2;\">\t\tNavbar {{}}\n</span><span style=\"color:#f8f8f2;\">\t\tdiv {{ </span><span style=\"color:#ffee99;\">&quot;actual blog content&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\t\tFooter {{}}\n</span><span style=\"color:#f8f8f2;\">\t}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Now, with layouts, you can declare your layout in the Route enum itself." }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">  #[layout(HeaderFooterNav)]\n</span><span style=\"color:#f8f8f2;\">\t\t#[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">\t\tHome {{}},\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\t\t#[route(</span><span style=\"color:#ffee99;\">&quot;/blog/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">\t\tBlog {{}},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Wrap the rendered content of the Router with a header, navbar, and footer\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">HeaderFooterNav</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\trender! {{\n</span><span style=\"color:#f8f8f2;\">\t\tHeader {{}}\n</span><span style=\"color:#f8f8f2;\">\t\tNavbar {{}}\n</span><span style=\"color:#f8f8f2;\">\t\tOutlet::&lt;Route&gt; {{}}\n</span><span style=\"color:#f8f8f2;\">\t\tFooter {{}}\n</span><span style=\"color:#f8f8f2;\">\t}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "The new router was an absolutely massive amount of work, spearheaded by @TeFiLeDo, improved by @ealmloff, and made possible thanks to community members like @stephenandary and @attilio-oliva."
        }
        p {
            img {
                src: "/assets/static/enum_router.png",
                alt: "PR of enum router",
                title: "",
            }
        }
        h2 { id: "new-and-beautiful-interactive-docs",
            a { href: "#new-and-beautiful-interactive-docs", class: "header",
                "New and beautiful interactive docs"
            }
        }
        p {
            "It’s about time we revamped our documentation. When we launched, the original docs were pretty good, at least for a 0.1 crate. Now, we have over 16 crates in the main repo with another half dozen scattered around the github organization. New users deserve a better introduction experience."
        }
        p { "To start, we revamped our landing page. Short, sweet, to the point." }
        p {
            img {
                src: "/assets/static/landing_1.png",
                alt: "Screenshot of new doc site landing page",
                title: "",
            }
        }
        p {
            "At the top of the page, you’ll see a new search bar. You can search for anything in our docs with just a  "
            code { "ctrl+/" }
            " .  This new search functionality uses a "
            a { href: "https://github.com/dioxusLabs/dioxus-search",
                "new search crate we designed and built"
            }
            ". "
            code { "Dioxus-search" }
            " is fully crossplatform and ready to use in your next Dioxus app."
        }
        p {
            img {
                src: "/assets/static/landing_2.png",
                alt: "Screenshot of new doc site search",
                title: "",
            }
        }
        p {
            "Selecting any of the search results will take you straight to the docs page without bouncing out to an external mdbook."
        }
        p {
            img {
                src: "/assets/static/landing_3.png",
                alt: "Screenshot of new doc site mdbook",
                title: "",
            }
        }
        p { "We’ve added a bunch more content to the docs, including new pages like:" }
        ul {
            li { "Building a hackernews clone from scratch" }
            li { "Setup guides for mobile" }
            li { "Suggestions for useful crates" }
            li { "How to bundle your app" }
            li { "Working with server functions" }
        }
        p {
            "The best part? The docs are interactive! Examples like the  "
            code { "hello-world" }
            " and even the  "
            code { "hackernews" }
            " clone come to life from within the docs page."
        }
        p {
            img {
                src: "/assets/static/landing_4.png",
                alt: "Screenshot of new doc site interactivity",
                title: "",
            }
        }
        p {
            "We also moved the  "
            code { "awesome" }
            " dioxus page from GitHub to the docsite, so you can explore the various crates that other developers have submitted as “awesome.”"
        }
        p {
            img {
                src: "/assets/static/landing_5.png",
                alt: "Screenshot of new doc site awesome page",
                title: "",
            }
        }
        p { "The new docs leverage many of the amazing new features from the router, including:" }
        ul {
            li {}
        }
        h2 { id: "android-support-ios-fixes-getting-started-guide-for-mobile",
            a {
                href: "#android-support-ios-fixes-getting-started-guide-for-mobile",
                class: "header",
                "Android Support, iOS fixes, Getting Started Guide for Mobile"
            }
        }
        p {
            "To date, Dioxus has provided first-party support for mobile via iOS, but our Android support has been rather spotty and untested. In this release, we finally added iOS and Android testing to our suite of continuous integration. To round off mobile support, we’ve added a "
            a { href: "https://dioxuslabs.com/learn/0.4/getting_started/mobile",
                "mobile-specific getting started guide"
            }
            " with a walkthrough on setting up platform-specific dependencies, handling basic cross-compilation, and booting up a mobile simulator. We’ve also fixed some bugs in upstream libraries like Tauri’s Tao which gives Dioxus its window-creation capabilities."
        }
        p { "iOS Demo:" }
        p {
            img {
                src: "/assets/static/ios_demo.png",
                alt: "Screenshot of xcode with dioxus app",
                title: "",
            }
        }
        p { "Android Demo:" }
        p {
            img {
                src: "/assets/static/android_demo.png",
                alt: "Screenshot of android studio with dioxus app",
                title: "",
            }
        }
        h2 { id: "window-close-behavior",
            a { href: "#window-close-behavior", class: "header", "Window-Close Behavior" }
        }
        p {
            "Another great contribution from the community: Dioxus desktop now provides functionality for managing the “close” behavior of windows in your app. We support three modes now:"
        }
        ul {
            li { "Closing the last window kills the app" }
            li { "Closing the last window doesn’t kill the app" }
            li {
                "Closing the last window simply hides the window (so the window can be cmd-tabbed back)"
            }
        }
        p {
            img {
                src: "https://i.imgur.com/m4wJ6gN.mp4",
                alt: "window_close.mov",
                title: "",
            }
        }
        h2 { id: "bidirectional-eval",
            a { href: "#bidirectional-eval", class: "header", "Bidirectional Eval" }
        }
        p {
            "The use_eval hook allows you to run snippets of Javascript in your Dioxus application when needed. @doge has made some improvements that make this hook has significantly more powerful. The new version of the hook is compatible between the desktop, web, and Liveview renderers. It also allows you to send messages to and from Javascript asynchronously. This makes it possible to listen for Javascript events that Dioxus doesn’t officially support like the intersection observer API."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    dioxus_desktop::launch(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> eval </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_eval</span><span style=\"color:#f8f8f2;\">(cx);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> future </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_future</span><span style=\"color:#f8f8f2;\">(cx, (), |_| {{\n</span><span style=\"color:#f8f8f2;\">        to_owned![eval];\n</span><span style=\"color:#f8f8f2;\">        async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// Eval some javascript\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> eval </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">eval</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;dioxus.send(&quot;Hi from JS!&quot;);\n</span><span style=\"color:#ffee99;\">                let msg = await dioxus.recv();\n</span><span style=\"color:#ffee99;\">                console.log(msg);\n</span><span style=\"color:#ffee99;\">                return &quot;hello world&quot;;&quot;#</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            )\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// Send messages into the running javascript\n</span><span style=\"color:#f8f8f2;\">            eval.</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hi from Rust!&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">()).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// Receive messages from the javascript\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> res </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> eval.</span><span style=\"color:#66d9ef;\">recv</span><span style=\"color:#f8f8f2;\">().await.</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// Wait for it to complete\n</span><span style=\"color:#f8f8f2;\">            println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{:?}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, eval.await);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">            res\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    render!{{ </span><span style=\"color:#ffee99;\">&quot;{{future.value():?}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "new-onmount-event",
            a { href: "#new-onmount-event", class: "header", "New onmount event" }
        }
        p {
            "This release also introduces a new onmounted event that provides access to elements after they are created in a cross platform way. The onmounted event makes it possible to:"
        }
        ul {
            li { "Scroll to an element" }
            li { "Read the size of an element" }
            li { "Focus an element" }
            li { "Get the platform specific element" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    dioxus_desktop::launch(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> header_element </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            h1 {{\n</span><span style=\"color:#f8f8f2;\">                onmounted: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">cx</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    header_element.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(cx.</span><span style=\"color:#66d9ef;\">inner</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">()));\n</span><span style=\"color:#f8f8f2;\">                }},\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;Scroll to top example&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> i </span><span style=\"color:#f92672;\">in </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">..</span><span style=\"color:#ff80f4;\">40 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                div {{ </span><span style=\"color:#ffee99;\">&quot;Item {{i}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">            button {{\n</span><span style=\"color:#f8f8f2;\">                onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(header) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> header_element.</span><span style=\"color:#66d9ef;\">as_ref</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">                        header.</span><span style=\"color:#66d9ef;\">scroll_to</span><span style=\"color:#f8f8f2;\">(ScrollBehavior::Smooth);\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }},\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;Scroll to top&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    ))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            img {
                src: "https://i.imgur.com/yp7GyIf.mp4",
                alt: "Scroll demo",
                title: "",
            }
        }
        h2 { id: "renaming-dioxus-cli-to-dx",
            a { href: "#renaming-dioxus-cli-to-dx", class: "header", "Renaming dioxus-cli to dx" }
        }
        p {
            "We made a small tweak to the CLI this release to rename the CLI from  "
            code { "dioxus" }
            " to  "
            code { "dx" }
            ". This is a tiny change but has a few benefits:"
        }
        ul {
            li { "It’s faster to type" }
            li {
                "It emphasizes the "
                em { "developer experience" }
                " of Dioxus"
            }
            li {
                "It keeps our tooling agnostic to other projects that might want to take advantage of features like hotreloading, autoformatting, wasm-pack, tailwind integration, plugins, and more."
            }
        }
        p { "To install the new CLI, use the same old instructions:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo install dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">cli </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">force</span></pre>\n" }
        h2 { id: "hot-reloading-for-desktop",
            a { href: "#hot-reloading-for-desktop", class: "header", "Hot Reloading for Desktop" }
        }
        p {
            "Yet another small tweak the CLI: you can now use  "
            code { "dx serve" }
            " on desktop apps with hot reloading enabled! Using the same hot reloading engine that powers web,  "
            code { "dx serve" }
            " can now modify your currently-running desktop without causing a full rebuild. In the event that we can’t hot reload a particular change, then  "
            code { "dx serve" }
            " will shutdown the app and restart it with the new changes applied."
        }
        p {
            img {
                src: "https://i.imgur.com/ML93XtT.mp4",
                alt: "Hotreloading on desktop",
                title: "",
            }
        }
        h2 { id: "dioxus-bundle",
            a { href: "#dioxus-bundle", class: "header", "Dioxus-Bundle" }
        }
        p {
            "So far, the CLI has supported useful commands like  "
            code { "dx fmt" }
            " ,  "
            code { "dx build" }
            " ,  "
            code { "dx serve" }
            " . Until date, we haven’t provided a command to build a final distributable image of your app. In 0.4, we’re incorporating cargo-bundle support into the Dioxus CLI. Now, from the Dioxus CLI, you can bundle your app using the same configuration options as the Tauri bundler, making it easy to migrate existing projects over.  "
            code { "dx bundle" }
            " supports bundling apps for macOS, iOS, Windows, and Linux (.deb, .rpm)."
        }
        p {
            img {
                src: "/assets/static/bundle.png",
                alt: "A bundled app on macos",
                title: "",
            }
        }
        p { "This is a great place for new contributors to help flesh out the ecosystem!" }
        h2 { id: "dioxus-check",
            a { href: "#dioxus-check", class: "header", "Dioxus-Check" }
        }
        p {
            "The Dioxus CLI has a new helper command:  "
            code { "dx check" }
            ". Thanks to the work from community member @eventualbuddha,  "
            code { "dx check" }
            " will now identify and help mitigate issues like hooks being called inside conditionals and loops."
        }
        p {
            img {
                src: "/assets/static/dxcheck.png",
                alt: "The new check command for dx",
                title: "",
            }
        }
        p {
            "These lints make it easier for newcomers to Dioxus to get started, especially if they’re not super familiar with React."
        }
        h2 { id: "vscode-extension-updates",
            a { href: "#vscode-extension-updates", class: "header", "VSCode Extension Updates" }
        }
        p {
            "As part of improving stability and fixing bugs, we’ve made some improvements to the VSCode Extension."
        }
        ul {
            li {
                "We fixed a bug with activation events where the extension wouldn’t activate until one of the commands were manually fired"
            }
            li { "We fixed a handful of autoformatting bugs around event handlers" }
            li {
                "We’ve moved the code for the extension out of the CLI and into a small WebAssembly binary so you don’t need the CLI installed and version matched"
            }
        }
        p {
            img {
                src: "/assets/static/extension.png",
                alt: "The Dioxus VSCode extension page",
                title: "",
            }
        }
        p { "The extension is a great place for new contributors to dive into the Dioxus codebase!" }
        h2 { id: "general-fixes",
            a { href: "#general-fixes", class: "header", "General Fixes" }
        }
        ul {
            li { "Several SSR and Hydration bugs were fixed including escaping text, and" }
            li {
                "We have improved testing around Dioxus. We have added end to end tests for each platform and fuzzing tests for core libraries."
            }
            li { "Fix the provide context docs" }
            li { "Fix trait bounds on generics with component" }
            li { "Fix hot reloading in a workspace" }
            li { "Fix anchor link for block-level elements" }
            li { "Fix Guide link in README" }
            li { "Fix new Clippy lints" }
            li { "Fix event bubbling within a single template" }
            li { "Fix panic when waking future on shutdown" }
            li { "Fix style attributes in SSR" }
            li { "Fix more complex workspaces with hot reloading" }
            li { "Fix non-bubbling listener hydration" }
            li { "Hotfix wry segfaulting" }
            li { "Fix dangerous_inner_html with SSR" }
            li { "Fix Linux Wry dependencies" }
            li { "Fix native core dependencies in a different direction than the pass direction" }
            li { "Fix raw attribute values in SSR" }
            li { "fix: Update logos and custom assets example" }
            li { "Fix non-ascii string decoding" }
            li { "fix rng in svg dice example" }
            li { "Fix clippy in the generic component example" }
            li { "fix: change crossterm poll timeout to 10ms from 1s" }
            li { "Fix HTML to RSX conversions example" }
            li { "Fix LiveView Tokio features" }
            li { "Fix non-bubbling event propagation" }
            li { "Fix selected and checked with boolean attributes" }
            li { "Fix form events with select multiple" }
            li { "Fix click and input events on desktop" }
            li { "Fix the link to tui widgets in the guide" }
            li { "Fix duplicate example and test names" }
            li { "fix router book link" }
            li { "Fix web events starting on a text node" }
            li { "Fix links in Liveview" }
            li { "Fix URL encoded desktop assets" }
            li { "Fix ssr guide examples" }
            li { "Fix hot reloading with namespaces" }
            li { "Add initial_value attribute & fix static special attribute handling" }
            li { "Fix liveview interpreter JS" }
            li { "Fix autoformatting raw strings" }
            li { "Fix panic when events trigger on placeholders" }
            li { "fix: Remove dbg that was causing TUI rendering problems" }
            li { "Fix restarting MacOs hot reloading after a forced exit" }
            li {
                "Fix "
                code { "cargo doc" }
            }
            li { "Fix: Remove play button from guide example" }
            li { "Fix: bump liveview salvo version to 0.44.1. (#1124)" }
            li { "fix: Remove conflicting rustfmt config for cli" }
            li { "fix(docs): Fix dioxus-hooks crate description" }
            li { "Fix CLI testing issue" }
            li { "Fix boolean attributes with raw boolean values" }
            li { "fix: Properly drop UseFuture's values to avoid leaking memory" }
            li { "Fix the onload event" }
            li { "fix: Fix stop_propagation example" }
            li { "Fix playwrite tests" }
            li { "Fix playwright fullstack clippy" }
            li { "fix: readme awesome link" }
            li { "fix: Remove duplicated doc links and improved some descriptions" }
            li {
                "Fix the issue of duplicate unique ID for atoms using "
                code { "newtype" }
                "."
            }
            li { "fix(cli): improve error message for invalid config" }
            li { "fix: Update use_ref docs" }
            li { "Fix playwright (again) and move the playwright stuff into the playwright directory" }
            li { "Fix: dont use bumpslab anymore, just box scopestates" }
            li { "Fix race condition in use_future" }
            li { "Fix use_future always spawning a new task and never updating" }
            li { "Fix route prerendering" }
            li { "fix: Use correct cfg for file_watcher feature in dioxus-hot-reload" }
            li {
                "docs(clean): fix copy-paste docs from "
                code { "build" }
            }
            li { "fix: Update use_coroutine example" }
            li {
                "fix(playwright): remove unnecessary "
                code { "await" }
                "s"
            }
            li { "Fix all broken examples" }
            li { "Fix root component suspense" }
        }
    }
}
#[component(no_case_check)]
pub fn Release050() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Here at Dioxus Labs, we have an unofficial rule: only one rewrite per year." }
        p {
            "Our last rewrite brought some amazing features: templates, hot reloading, and insane performance. However, don’t be mistaken, rewrites are scary, time consuming, and a huge gamble. We started this new rewrite on January 1st of 2024, completed it by Feburary 1st, and then spent another month and a half writing tests, squashing bugs, and polishing documentation. Rewrites are absolutely not for the faint of heart."
        }
        p {
            "If you’re new here, Dioxus (dye•ox•us) is a library for building GUIs in Rust. Originally, I built Dioxus as a rewrite of Yew with the intention of supporting proper server-side-rendering. Eventually, Dioxus got popular, we got some amazing sponsors, and I went full time. We’ve grown from a team of 1 (me) to a team of 4(!) - pulled entirely from the wonderful Dioxus community."
        }
        p {
            "Now, Dioxus is something a little different. Real life, actual companies are shipping web apps, desktop apps, and mobile apps with Dioxus. What was once just a fun little side project powers a small fraction of apps out in the wild. We now have lofty goals of simplifying the entire app development ecosystem. Web, Desktop, Mobile, all end-to-end typesafe, blazing fast, living under one codebase. The dream!"
        }
        p {
            "With 0.5 we took a hard look at how Dioxus would need to change to achieve those goals. The request we got from the community was clear: make it simpler, make it robust, make it polished."
        }
        h2 { id: "whats-new",
            a { href: "#whats-new", class: "header", "What’s new?" }
        }
        p {
            "This is probably the biggest release of Dioxus ever, with so many new features, bug fixes, and improvements that I can’t list them all. We churned over 100,000 lines of code (yes, 100,000+) with over 1,400 commits between 0.4.3 and 0.5.0. Here’s a quick overview:"
        }
        ul {
            li {
                "Complete rewrite of "
                code { "dioxus-core" }
                ", removing all unsafe code"
            }
            li {
                "Abandoning "
                code { "use_state" }
                " and "
                code { "use_ref" }
                " for a clone-free "
                code { "Signal" }
                "-based API"
            }
            li {
                "Removal of all lifetimes and the "
                code { "cx: Scope" }
                " state"
            }
            li {
                "A single, unified "
                code { "launch" }
                " function that starts your app for any platform"
            }
            li { "Asset hot reloading that supports Tailwind and Vanilla CSS" }
            li {
                "Rewrite of events, allowing access to the native "
                code { "WebSys" }
                " event types"
            }
            li {
                "Extension of components with element properties (IE a Link now takes all of "
                code { "<a/>" }
                " properties)"
            }
            li { "Integrated Error Boundaries and Server Futures with Suspense integration" }
            li { "5x faster desktop reconciliation and custom asset handlers for streaming bytes" }
            li { "Streaming server functions and Fullstack hot reloading" }
            li { "Tons of QoL improvements, bug fixes, and more!" }
        }
        p {
            "💡 If you are updating from Dioxus 0.4, a "
            a { href: "https://dioxuslabs.com/learn/0.5/migration",
                code { "migration guide" }
            }
            " is available"
        }
        h2 { id: "lifetime-problems",
            a { href: "#lifetime-problems", class: "header", "Lifetime Problems" }
        }
        p {
            "To make Dioxus simpler, we wanted to remove lifetimes entirely. Newcomers to rust are easily scared off by lifetime issues, and even experienced Rustaceans find wading through obtuse error messages exhausting."
        }
        p {
            "In Dioxus 0.1-0.4, every value in a component lives for a  "
            code { "'bump" }
            " lifetime. This lifetime lets you easily use hooks, props and the scope within event listeners without cloning anything. It was the chief innovation that made Dioxus so much easier to use than Yew when it was released."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Scope and Element have the lifetime &#39;bump\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">OldDioxusComponent</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#8c8c8c;\">// hook has the lifetime &#39;bump\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">  cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">    button {{\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#8c8c8c;\">// The closure has the lifetime &#39;bump which means you don&#39;t\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#8c8c8c;\">// need to clone hook before you move it into the closure\n</span><span style=\"color:#f8f8f2;\">      onclick: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">_event</span><span style=\"color:#f92672;\">| *</span><span style=\"color:#f8f8f2;\">state </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">  }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This works great for hooks "
            em { "most" }
            " of the time. The lifetime lets you omit a bunch of manual clones every time you want to use a value inside an EventHandler (onclick, oninput, etc)."
        }
        p {
            "However, the lifetime doesn’t work for futures. Futures in Dioxus need to be  "
            code { "'static" }
            " which means you always need to clone values before you use them in the future. Since a future might need to run while the component is rendering, it can’t share the component’s lifetime."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Scope and Element have the lifetime &#39;bump\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">OldDioxusComponent</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// state has the lifetime &#39;bump\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">spawn</span><span style=\"color:#f8f8f2;\">({{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Because state has the lifetime &#39;bump, we need to clone it to make it\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// &#39;static before we move it into the &#39;static future\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> state.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{state}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// ...\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "If you don’t clone the value, you will run into an error like this:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#ff80f4;\">4  </span><span style=\"color:#f92672;\">|   </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">OldDioxusComponent</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">   </span><span style=\"color:#f92672;\">|                         --\n</span><span style=\"color:#f8f8f2;\">   </span><span style=\"color:#f92672;\">|                         |\n</span><span style=\"color:#f8f8f2;\">   </span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\">                         `cx` is a reference that is only valid </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> the function body\n</span><span style=\"color:#f8f8f2;\">   </span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\">                         has </span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\"> `</span><span style=\"color:#f92672;\">&amp;&#39;</span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f8f8f2;\">Scoped&lt;&#39;1&gt;`\n</span><span style=\"color:#f92672;\">...\n</span><span style=\"color:#ff80f4;\">8  </span><span style=\"color:#f92672;\">| /</span><span style=\"color:#f8f8f2;\">     cx.</span><span style=\"color:#66d9ef;\">spawn</span><span style=\"color:#f8f8f2;\">(async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#ff80f4;\">9  </span><span style=\"color:#f92672;\">| |         </span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{state}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#ff80f4;\">10 </span><span style=\"color:#f92672;\">| |     </span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">   | |      </span><span style=\"color:#f92672;\">^\n</span><span style=\"color:#f8f8f2;\">   </span><span style=\"color:#f92672;\">| |      |\n</span><span style=\"color:#f8f8f2;\">   </span><span style=\"color:#f92672;\">| |</span><span style=\"color:#ff80f4;\">______</span><span style=\"color:#f8f8f2;\">`cx` escapes the function body here\n</span><span style=\"color:#f8f8f2;\">   </span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\">        argument requires that `</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">` must outlive `</span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">`</span></pre>\n",
        }
        p {
            "The error complains that  "
            code { "cx" }
            " must outlive  "
            code { "'static" }
            " without mentioning the hook at all which can be very confusing."
        }
        p {
            "Dioxus 0.5 fixes this issue by first removing scopes and the  "
            code { "'bump" }
            " lifetime and then introducing a new  "
            code { "Copy" }
            " state management solution called signals. Here is what the component looks like in Dioxus 0.5:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Element has no lifetime, and you don&#39;t need a Scope\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">NewComponent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#8c8c8c;\">// state is &#39;static and Copy, even if the inner value you store is not Copy\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#8c8c8c;\">// State is already &#39;static and Copy, so it is copied into the future automatically\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#66d9ef;\">spawn</span><span style=\"color:#f8f8f2;\">(async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{state}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">  }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">  rsx! {{\n</span><span style=\"color:#f8f8f2;\">    button {{\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#8c8c8c;\">// The closure has the lifetime &#39;static, but state is copy so you don&#39;t need to clone into the closure\n</span><span style=\"color:#f8f8f2;\">      onclick: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">_event</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">  }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "While this might seem like a rather innocuous change, it has an impressively huge impact on how easy it is to write new components. I’d say building a new Dioxus app is about 2-5x easier with this change alone."
        }
        h2 { id: "goodbye-scopes-and-lifetimes",
            a { href: "#goodbye-scopes-and-lifetimes", class: "header",
                "Goodbye scopes and lifetimes!"
            }
        }
        p {
            "In the new version of Dioxus, scopes and the  "
            code { "'bump" }
            " lifetime have been removed! This makes declaring a component and using runtime functions within that component much easier:"
        }
        p {
            "You can now declare a component by just accepting your props directly instead of a scope parameter"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">MyComponent</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  rsx! {{ </span><span style=\"color:#ffee99;\">&quot;Hello {{name}}!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p { "And inside that component, you can use runtime functions directly" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#66d9ef;\">spawn</span><span style=\"color:#f8f8f2;\">(async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">  tokio::time::sleep(Duration::from_millis(</span><span style=\"color:#ff80f4;\">100</span><span style=\"color:#f8f8f2;\">)).await;\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#8c8c8c;\">// You can even use runtime functions inside futures and event handlers!\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> context: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">consume_context</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">}});</span></pre>\n" }
        p {
            "Now that lifetimes are gone,  "
            code { "Element" }
            "s are  "
            code { "'static" }
            " which means you can use them in hooks or even provide them through the context API. This makes some APIs like "
            a { href: "https://github.com/matthunz/dioxus-lazy", "virtual lists in Dioxus" }
            " significantly easier. We expect more interesting APIs to emerge from the community now that you don’t need to be a Rust wizard to implement things like virtualization and offscreen rendering."
        }
        h2 { id: "removal-of-all-unsafe-in-core",
            a { href: "#removal-of-all-unsafe-in-core", class: "header",
                "Removal of all Unsafe in Core"
            }
        }
        p {
            "Removing the  "
            code { "'bump" }
            " lifetime along with the scope gave us a chance to remove a lot of unsafe from Dioxus. "
            strong { "dioxus-core 0.5 contains no unsafe code 🎉" }
        }
        p {
            img {
                src: "https://i.imgur.com/B0kf5Df.png",
                alt: "No more unsafe in core",
                title: "",
            }
        }
        p {
            "There’s still a tiny bit of unsafe floating around various dependencies that we plan to remove throughout the 0.5 release cycle, but way less: all quite simple to cut or unfortunately necessary due to FFI."
        }
        h2 { id: "signals",
            a { href: "#signals", class: "header", "Signals!" }
        }
        p {
            "Dioxus 0.5 introduces Signals as the core state primitive for components. Signals have two key advantages over the existing  "
            code { "use_state" }
            " and  "
            code { "use_ref" }
            " hooks: They are always  "
            code { "Copy" }
            " and they don’t require manual subscriptions."
        }
        h3 { id: "copy-state",
            a { href: "#copy-state", class: "header", "Copy state" }
        }
        p {
            code { "Signal<T>" }
            " is  "
            code { "Copy" }
            ", even if the inner  "
            code { "T" }
            " values is not. This is enabled by our new "
            a { href: "https://crates.io/crates/generational-box", "generational-box" }
            " crate (implemented with zero unsafe). Signals can even optionally be "
            code { "Send+Sync" }
            " if you need to move them between threads, removing the need for a whole class of specialized state management solutions."
        }
        p {
            "The combination of  "
            code { "Copy + Send + Sync" }
            " Signals, and static components makes it incredibly easy to move state to anywhere you need it:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#8c8c8c;\">// We use a sync signal here so that we can use it in other threads,\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#8c8c8c;\">// but you could use a normal signal if you have !Send data\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal_sync</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#66d9ef;\">spawn</span><span style=\"color:#f8f8f2;\">(async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Signals have a ton of helper methods that make them easy to work with.\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// You can call a signal like a function to get the current value\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">state</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">  }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#8c8c8c;\">// Because signals can be sync, we can copy them into threads easily\n</span><span style=\"color:#f8f8f2;\">  std::thread::spawn(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">loop </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">      std::thread::sleep(Duration::from_millis(</span><span style=\"color:#ff80f4;\">100</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">      println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{state}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">  }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">  rsx! {{\n</span><span style=\"color:#f8f8f2;\">    button {{\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#8c8c8c;\">// You can easily move it into an event handler just like use_state\n</span><span style=\"color:#f8f8f2;\">      onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">  }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "With  "
            code { "Copy" }
            " state, we’ve essentially bolted on a light form of garbage collection into Rust that uses component lifecycles as the triggers for dropping state. From a memory perspective, this is basically the same as 0.4, but with the added benefit of not needing to explicitly  "
            code { "Clone" }
            " anything."
        }
        h3 { id: "smarter-subscriptions",
            a { href: "#smarter-subscriptions", class: "header", "Smarter subscriptions" }
        }
        p {
            "Signals are smarter about what components rerun when they are changed. A component will only rerun if you read the value of the signal in the component (not in an async task or event handler). In this example, only the child will re-render when the button is clicked because only the child component is reading the signal:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">  rsx! {{\n</span><span style=\"color:#f8f8f2;\">    button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;increment&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    Child {{ state }}\n</span><span style=\"color:#f8f8f2;\">  }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">state</span><span style=\"color:#f8f8f2;\">: Signal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  rsx! {{ </span><span style=\"color:#ffee99;\">&quot;{{state}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Smarter subscriptions let us merge several different hooks into signals. For example, we were able to remove an entire crate dedicated to state management: Fermi. Fermi provided what was essentially a  "
            code { "use_state" }
            " API where statics were used as keys. This meant you could declare some global state, and then read it in your components:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">: Atom&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Atom::new(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Demo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_read_atom</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">  rsx! {{ </span><span style=\"color:#ffee99;\">&quot;{{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Since fermi didn’t support smart subscriptions, you had to explicitly declare use the right  "
            code { "use_read" }
            "/  "
            code { "use_write" }
            " hooks to subscribe to the value. In Dioxus 0.5, we just use signals, eliminating the need for any sort of external state management solution altogether."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// You can use a lazily initialized signal called\n</span><span style=\"color:#8c8c8c;\">// GlobalSignal in static instead of special Fermi atoms\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">: GlobalSignal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Signal::global(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Using the GlobalSignal is just the same as any other signal!\n</span><span style=\"color:#8c8c8c;\">// No need for use_read or use_write\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Demo</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">   rsx! {{ </span><span style=\"color:#ffee99;\">&quot;{{COUNT}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Signals even work with the context API, so you can quickly share state between components in your app:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#8c8c8c;\">// Create a new signal and provide it to the context API\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#8c8c8c;\">// without a special use_shared_state hook\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_context_provider</span><span style=\"color:#f8f8f2;\">(|| Signal::new(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">  rsx! {{\n</span><span style=\"color:#f8f8f2;\">    button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Increment&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    Child {{}}\n</span><span style=\"color:#f8f8f2;\">  }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#8c8c8c;\">// Get the state from the context API\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">use_context::&lt;Signal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt;&gt;();\n</span><span style=\"color:#f8f8f2;\">  rsx! {{ </span><span style=\"color:#ffee99;\">&quot;{{state}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Smart subscriptions also apply to hooks. Hooks like  "
            code { "use_future" }
            " and  "
            code { "use_memo" }
            " will now automatically add signals you read inside the hook to the dependencies of the hook:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// You can use a lazily initialized signal called GlobalSignal in static instead of special Fermi atoms\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">: GlobalSignal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Signal::global(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#8c8c8c;\">// Because we read COUNT inside the memo, it is automatically added to the memo&#39;s dependencies\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#8c8c8c;\">// If we change COUNT, then the memo knows it needs to rerun\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> memo </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">/ </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">  rsx! {{ </span><span style=\"color:#ffee99;\">&quot;{{memo}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "css-hot-reloading",
            a { href: "#css-hot-reloading", class: "header", "CSS Hot Reloading" }
        }
        p {
            "As part of our asset system overhaul, we implemented hot reloading of CSS files in the asset directory. If a CSS file appears in your RSX, the  "
            code { "dx" }
            " CLI will watch that file and immediately stream its updates to the running app. This works for Web, Desktop, and Fullstack, with mobile support coming in a future mobile-centric update."
        }
        p {
            "When combined with the Tailwind watcher, we now support hot reloading of Tailwind CSS! On top of that, we also support IDE hinting of Tailwind classes in VSCode with a "
            a { href: "https://github.com/tailwindlabs/tailwindcss/discussions/7073",
                "custom regex extension"
            }
        }
        p {
            img {
                src: "https://imgur.com/CSjVVLL.mp4",
                alt: "CSS Hot reloading",
                title: "",
            }
        }
        p {
            "What’s even niftier is that you can stream these changes to several devices at once, unlocking simultaneous hot reloading across all devices that you target:"
        }
        p {
            img {
                src: "https://i.imgur.com/cZ8qZCz.mp4",
                alt: "CSS Hot reloading",
                title: "",
            }
        }
        h2 { id: "event-system-rewrite",
            a { href: "#event-system-rewrite", class: "header", "Event System Rewrite" }
        }
        p {
            "Since its release, Dioxus has used a synthetic event system to create a cross platform event API. Synthetic events can be incredibly useful to make events work across platforms and even serialize them across the network, but they do have some drawbacks."
        }
        p {
            "Dioxus 0.5 finally exposes the underlying event type for each platform along with a trait with a cross platform API. This has two advantages:"
        }
        ol {
            li {
                "You can get whatever information you need from the platform event type or pass that type to another library:"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Button</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  rsx! {{\n</span><span style=\"color:#f8f8f2;\">    button {{\n</span><span style=\"color:#f8f8f2;\">      onclick: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> web_sys_event: web_sys::MouseEvent </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> event.</span><span style=\"color:#66d9ef;\">web_event</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        web_sys::console::log_1(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">web_sys_event.related_target.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">      }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">  }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        ol {
            li {
                "Dioxus can bundle split code for events apps don’t use. For a hello world example, this shrinks the gzipped size "
            }
        }
        p {
            img {
                src: "https://i.imgur.com/6hZruyO.png",
                alt: "Smaller bundles",
                title: "",
            }
        }
        p {
            "Again, this seems like a small change on the surface, but opens up dozens of new use cases and possible libraries you can build with Dioxus."
        }
        p {
            "💡 The "
            a { href: "https://dioxuslabs.com/learn/0.5/cookbook/optimizing#build-configuration",
                "Dioxus optimization guide"
            }
            " has tips to help you make the smallest possible bundle"
        }
        h2 { id: "cross-platform-launch",
            a { href: "#cross-platform-launch", class: "header", "Cross platform launch" }
        }
        p {
            "Dioxus 0.5 introduces a new cross platform API to launch your app. This makes it easy to target multiple platforms with the same application. Instead of pulling in a separate renderer package, you can now enable a feature on the Dioxus crate and call the launch function from the prelude:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[dependencies]\n</span><span style=\"color:#f8f8f2;\">dioxus </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.5&quot;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">[features]\n</span><span style=\"color:#f8f8f2;\">default </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[]\n</span><span style=\"color:#f8f8f2;\">desktop </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;dioxus/desktop&quot;</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">fullstack </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;dioxus/fullstack&quot;</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">server </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;dioxus/axum&quot;</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">web </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;dioxus/web&quot;</span><span style=\"color:#f8f8f2;\">]</span></pre>\n",
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    dioxus::launch(|| rsx!{{ </span><span style=\"color:#ffee99;\">&quot;hello world&quot; </span><span style=\"color:#f8f8f2;\">}})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p { "With that single application, you can easily target:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> Desktop\n</span><span style=\"color:#f8f8f2;\">dx serve </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">platform desktop\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\"># </span><span style=\"color:#ff80f4;\">SPA</span><span style=\"color:#f8f8f2;\"> web\n</span><span style=\"color:#f8f8f2;\">dx serve </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">platform web\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> Or a fullstack application\n</span><span style=\"color:#f8f8f2;\">dx serve </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">platform fullstack</span></pre>\n" }
        p {
            "The CLI is now smart enough to automatically pass in the appropriate build features depending on the platform you’re targeting."
        }
        h2 { id: "asset-system-beta",
            a { href: "#asset-system-beta", class: "header", "Asset System Beta" }
        }
        p {
            "Currently assets in Dioxus (and web applications in general) can be difficult to get right. Links to your asset can easily get out of date, the link to your asset can be different between desktop and web applications, and you need to manually add assets you want to use into your bundled application. In addition to all of that, assets can be a huge performance bottleneck."
        }
        p { "Lets take a look at the Dioxus Mobile guide in the docsite as an example:" }
        p {
            img {
                src: "https://i.imgur.com/f7sGEdJ.png",
                alt: "docsite_mobile_old.png",
                title: "",
            }
        }
        p {
            "The 0.4 mobile guide takes 7 seconds to load and transfers 9 MB of resources. The page has 6 different large image files which slows down the page loading times significantly. We could switch to a more optimized image format like  "
            code { "avif" }
            " , but manually converting every screenshot is tedious and time consuming."
        }
        p { "Lets take a look at the 0.5 mobile guide with the new asset system:" }
        p {
            img {
                src: "https://i.imgur.com/GabzFJm.png",
                alt: "docsite_mobile_new.png",
                title: "",
            }
        }
        p {
            "The new mobile guide takes less than 1 second to load and requires only 1/3 of the resources with the exact same images!"
        }
        p {
            "Dioxus 0.5 introduces a new asset system called "
            a { href: "https://github.com/DioxusLabs/manganis", "manganis" }
            ". Manganis integrates with the CLI to check, bundle and optimize assets in your application. The API is currently unstable so the asset system is currently published as a separate crate. In the new asset system, you can just wrap your assets in the "
            code { "mg!" }
            " macro and they will automatically be picked up by the CLI. You can read more about the new asset system in the "
            a { href: "https://docs.rs/crate/manganis/latest", "manganis docs" }
            "."
        }
        p {
            "As we continue to iterate on the 0.5 release, we plan to add hot reloading to manganis assets, so you can interactively add new the features to your app like CSS, images, Tailwind classes, and more without forcing a complete reload."
        }
        h2 { id: "5x-faster-desktop-rendering",
            a { href: "#5x-faster-desktop-rendering", class: "header",
                "5x Faster Desktop Rendering"
            }
        }
        p {
            "Dioxus implements several optimizations to make diffing rendering fast. "
            a { href: "https://dioxuslabs.com/blog/templates-diffing", "Templates" }
            " let Dioxus skip diffing on any static parts of the rsx macro. However, diffing is only one side of the story. After you create a list of changes you need to make to the DOM, you need to apply them."
        }
        p {
            "We developed "
            a { href: "https://github.com/ealmloff/sledgehammer_bindgen", "sledgehammer" }
            " for Dioxus Web to make applying those mutations as fast as possible. It makes manipulating the DOM from Rust almost as "
            a { href: "https://krausest.github.io/js-framework-benchmark/2023/table_chrome_114.0.5735.90.html",
                "fast as native JavaScript"
            }
            "."
        }
        p {
            "In Dioxus 0.5, we apply that same technique to apply changes across the network as fast as possible. Instead of using JSON to communicate changes to the Desktop and LiveView renderers, Dioxus 0.5 uses a binary protocol."
        }
        p {
            "For render intensive workloads, the new renderer takes only 1/5 the time to apply the changes in the browser with 1/2 the latency. Here is one of the benchmarks we developed while working on the new binary protocol. In Dioxus 0.4, the renderer was constantly freezing. In Dioxus 0.5, it runs smoothly:"
        }
        p {
            strong { "Dioxus 0.4" }
            img {
                src: "https://i.imgur.com/CX7DREF.mp4",
                alt: "Desktop performance 0.4",
                title: "",
            }
        }
        p {
            strong { "Dioxus 0.5" }
            img {
                src: "https://i.imgur.com/3l65D0G.mp4",
                alt: "Desktop performance 0.5",
                title: "",
            }
        }
        h2 { id: "spreading-props",
            a { href: "#spreading-props", class: "header", "Spreading props" }
        }
        p {
            "One common pattern when creating components is providing some additional functionality to a specific element. When you wrap an element, it is often useful to provide some control over what attributes are set in the final element. Instead of manually copying over each attribute from the element, Dioxus 0.5 supports extending specific elements and spreading the attributes into an element:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Props, PartialEq, Clone)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">Props {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// You can extend a specific element or global attributes\n</span><span style=\"color:#f8f8f2;\">    #[props(extends = img)]\n</span><span style=\"color:#f8f8f2;\">    attributes: Vec&lt;Attribute&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ImgPlus</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">props</span><span style=\"color:#f8f8f2;\">: Props) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can spread those attributes into any element\n</span><span style=\"color:#f8f8f2;\">        img {{ </span><span style=\"color:#f92672;\">..</span><span style=\"color:#f8f8f2;\">props.attributes }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        ImgPlus {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// You can use any attributes you would normally use on the img element\n</span><span style=\"color:#f8f8f2;\">            width: </span><span style=\"color:#ffee99;\">&quot;10px&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            height: </span><span style=\"color:#ffee99;\">&quot;10px&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            src: </span><span style=\"color:#ffee99;\">&quot;https://example.com/image.png&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "shorthand-attributes",
            a { href: "#shorthand-attributes", class: "header", "Shorthand attributes" }
        }
        p {
            "Another huge quality-of-life feature we added was the ability to use shorthand struct initialization syntax to pass attributes into elements and components. We got tired of passing  "
            code { "class: class" }
            " everywhere and decided to finally implement this long awaited feature, at the expense of some code breakage. Now, it’s super simple to declare attributes from props:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ImgPlus</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">class</span><span style=\"color:#f8f8f2;\">: String, </span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">: String, </span><span style=\"font-style:italic;color:#fd971f;\">src</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        img {{ class, id, src }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "This feature works for anything implementing  "
            code { "IntoAttribute" }
            ", meaning signals also benefit from shorthand initialization. While signals as attributes don’t yet skip diffing, we plan to add this as a performance optimization throughout the 0.5 release cycle."
        }
        h2 { id: "multi-line-attribute-merging",
            a { href: "#multi-line-attribute-merging", class: "header",
                "Multi-line attribute merging"
            }
        }
        p {
            "Another amazing feature added this cycle was attribute merging. When working with libraries like Tailwind, you’ll occasionally want to make certain attributes conditional. Before, you had to format the attribute using an empty string. Now, you can simply add an extra attribute with a conditional, and the attribute will be merged using a space as a delimiter:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Blog</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">enabled</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">bool</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            class: </span><span style=\"color:#ffee99;\">&quot;bg-gray-200 border rounded shadow&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            class: </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> enabled {{ </span><span style=\"color:#ffee99;\">&quot;text-white&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This is particularly important when using libraries like Tailwind where attributes need to be parsed at compile time but also dynamic at runtime. This syntax integrates with the Tailwind compiler, removing the runtime overhead for libraries like tailwind-merge."
        }
        h2 { id: "server-function-streaming",
            a { href: "#server-function-streaming", class: "header", "Server function streaming" }
        }
        p {
            "Dioxus 0.5 supports the latest version of "
            a { href: "https://crates.io/crates/server_fn", "the server functions crate" }
            " which supports streaming data. Server functions can now choose to stream data to or from the client. This makes it easier to do a whole class of tasks on the server."
        }
        p {
            "Creating a streaming server function is as easy as defining the output type and returning a TextStream from the server function. Streaming server functions are great for updating the client during any long running task."
        }
        p {
            "We built an AI text generation example here: "
            a { href: "https://github.com/ealmloff/dioxus-streaming-llm",
                "https://github.com/ealmloff/dioxus-streaming-llm"
            }
            " that uses Kalosm and local LLMS to serve what is essentially a clone of OpenAI’s ChatGPT endpoint on commodity hardware."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[server(output </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> StreamingText)]\n</span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">mistral</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">text</span><span style=\"color:#f8f8f2;\">: String) -&gt; Result&lt;TextStream, ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">   </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> text_generation_stream </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">todo!();\n</span><span style=\"color:#f8f8f2;\">   </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(TextStream::new(text_generation_stream))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            img {
                src: "https://i.imgur.com/JJaMT0Z.mp4",
                alt: "Streaming server function AI app",
                title: "",
            }
        }
        p {
            "Side note, the AI metaframework used here - Kalosm - is maintained by the Dioxus core team member ealmloff, and his AI GUI app Floneum is built with Dioxus!"
        }
        h2 { id: "fullstack-cli-platform",
            a { href: "#fullstack-cli-platform", class: "header", "Fullstack CLI platform" }
        }
        p {
            "The CLI now supports a  "
            code { "fullstack" }
            " platform with hot reloading and parallel builds for the client and sever. You can now serve your fullstack app with the  "
            code { "dx" }
            " command:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dx serve\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> Or with an explicit platform\n</span><span style=\"color:#f8f8f2;\">dx serve </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">platform fullstack</span></pre>\n" }
        h2 { id: "liveview-router-support",
            a { href: "#liveview-router-support", class: "header", "LiveView router support" }
        }
        p {
            a { href: "https://github.com/DioxusLabs/dioxus/pull/1505",
                "https://github.com/DioxusLabs/dioxus/pull/1505"
            }
        }
        p {
            a { href: "https://github.com/DonAlonzo",
                code { "@DonAlonzo" }
            }
            " added LiveView support for the router in Dioxus 0.5. The router will now work out of the box with your LiveView apps!"
        }
        h2 { id: "custom-asset-handlers",
            a { href: "#custom-asset-handlers", class: "header", "Custom Asset Handlers" }
        }
        p {
            a { href: "https://github.com/DioxusLabs/dioxus/pull/1719",
                "https://github.com/DioxusLabs/dioxus/pull/1719"
            }
        }
        p {
            a { href: "https://github.com/willcrichton",
                code { "@willcrichton" }
            }
            " added support for custom asset handlers to Dioxus Desktop. Custom asset handlers let you efficiently stream data from your rust code into the browser without going through JavaScript. This is great for high bandwidth communication like "
            a { href: "https://github.com/DioxusLabs/dioxus/pull/1727", "video streaming" }
            ":"
        }
        p {
            img {
                src: "https://i.imgur.com/6bdUBdF.mp4",
                alt: "Custom asset handlers",
                title: "",
            }
        }
        p {
            "Now, you can do things like work with gstreamer or webrtc and pipe data directly into the webview without needing to encode/decode frames by hand."
        }
        h2 { id: "native-file-handling",
            a { href: "#native-file-handling", class: "header", "Native File Handling" }
        }
        p { "This is a bit smaller of a tweak, but now we properly support file drops for Desktop:" }
        p {
            img {
                src: "https://i.imgur.com/vkkDDid.mp4",
                alt: "Native file drop",
                title: "",
            }
            "Previously we just gave you the option to intercept filedrops but now it’s natively integrated into the event system"
        }
        h2 { id: "error-handling",
            a { href: "#error-handling", class: "header", "Error handling" }
        }
        p {
            "Error handling: You can use error boundaries and the throw trait to easily handle errors higher up in your app"
        }
        p {
            "Dioxus provides a much easier way to handle errors: throwing them. Throwing errors combines the best parts of an error state and early return: you can easily throw an error with  "
            code { "?" }
            ", but you keep information about the error so that you can handle it in a parent component."
        }
        p {
            "You can call  "
            code { "throw" }
            " on any  "
            code { "Result" }
            " type that implements  "
            code { "Debug" }
            " to turn it into an error state and then use  "
            code { "?" }
            " to return early if you do hit an error. You can capture the error state with an  "
            code { "ErrorBoundary" }
            " component that will render the a different component if an error is thrown in any of its children."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ErrorBoundary {{\n</span><span style=\"color:#f8f8f2;\">        handle_error: |</span><span style=\"font-style:italic;color:#fd971f;\">error</span><span style=\"color:#f8f8f2;\">| rsx! {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Oops, we encountered an error. Please report {{error}} to the developer of this application&quot;\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        ThrowsError {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">  }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ThrowsError</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;1.234&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">throw</span><span style=\"color:#f8f8f2;\">()</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "You can even nest  "
            code { "ErrorBoundary" }
            " components to capture errors at different levels of your app."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ErrorBoundary {{\n</span><span style=\"color:#f8f8f2;\">        handle_error: |</span><span style=\"font-style:italic;color:#fd971f;\">error</span><span style=\"color:#f8f8f2;\">| rsx! {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hmm, something went wrong. Please report {{error}} to the developer&quot;\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        Parent {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">  }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ErrorBoundary {{\n</span><span style=\"color:#f8f8f2;\">        handle_error: |</span><span style=\"font-style:italic;color:#fd971f;\">error</span><span style=\"color:#f8f8f2;\">| rsx! {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;The child component encountered an error: {{error}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">      ThrowsError {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">  }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ThrowsError</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;1.234&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">throw</span><span style=\"color:#f8f8f2;\">()</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">  todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This pattern is particularly helpful whenever your code generates a non-recoverable error. You can gracefully capture these \"global\" error states without panicking or handling state for each error yourself."
        }
        h2 { id: "hot-reloading-by-default-and-develop-mode-for-desktop",
            a {
                href: "#hot-reloading-by-default-and-develop-mode-for-desktop",
                class: "header",
                "Hot reloading by default and “develop” mode for Desktop"
            }
        }
        p {
            "We shipped hot reloading in 0.3, added it to Desktop in 0.4, and now we’re finally enabling it by default in 0.5. By default, when you  "
            code { "dx serve" }
            " your app, hot reloading is enabled in development mode."
        }
        p {
            "Additionally, we’ve drastically improved the developer experience of building desktop apps. When we can’t hot reload the app and have to do a full recompile, we now preserve the state of the open windows and resume that state. This means your app won’t block your entire screen on every edit and it will maintain its size and position, leading to a more magical experience. Once you’ve played with it, you can never go back - it’s that good."
        }
        p {
            img {
                src: "https://i.imgur.com/qjHB4ho.mp4",
                alt: "Hot reloading by default",
                title: "",
            }
        }
        h2 { id: "updates-to-the-dioxus-template",
            a { href: "#updates-to-the-dioxus-template", class: "header",
                "Updates to the Dioxus template"
            }
        }
        p {
            img {
                src: "https://i.imgur.com/jpXNW5P.mp4",
                alt: "Dioxus template update",
                title: "",
            }
        }
        p {
            "With this update, our newest core team member Miles put serious work into overhauling documentation and our templates. We now have templates to create new Dioxus apps for Web, Desktop, Mobile, TUI, and Fullstack under one command."
        }
        p {
            "We also updated the default app you get when using  "
            code { "dx new" }
            " to be closer to the traditional create-react-app. The template is now seeded with assets, CSS, and some basic deploy configuration. Plus, it includes links to useful resources like dioxus-std, the VSCode Extension, docs, tutorials, and more."
        }
        p {
            img {
                src: "https://i.imgur.com/DCrrDxD.png",
                alt: "New templates",
                title: "",
            }
        }
        h2 { id: "dioxus-community-and-dioxus-std",
            a { href: "#dioxus-community-and-dioxus-std", class: "header",
                "Dioxus-Community and Dioxus-std"
            }
        }
        p {
            "The Dioxus Community is something special: discord members marc and Doge have been hard at working updating important ecosystem crates for the 0.5 release. With this release, important crates like icons, charts, and the Dioxus-specific standard library are ready to use right out the gate. The  "
            code { "Dioxus Community" }
            " project is a new GitHub organization that keeps important crates up-to-date even when the original maintainers step down. If you build a library for Dioxus, we’ll be happy to help maintain it, keeping it at what is essentially “Tier 2” support."
        }
        p {
            img {
                src: "https://i.imgur.com/yoLSrwj.png",
                alt: "dioxus_community",
                title: "",
            }
        }
        h2 { id: "coming-soon",
            a { href: "#coming-soon", class: "header", "Coming soon" }
        }
        p {
            "At a certain point we had to stop adding new features to this release. There’s plenty of cool projects on the horizon:"
        }
        ul {
            li { "Stabilizing and more deeply integrating the asset system" }
            li {
                "Bundle splitting the outputted "
                code { ".wasm" }
                " directly - with lazy components"
            }
            li { "Islands and resumable interactivity (serializing signals!)" }
            li { "Server components and merging LiveView into Fullstack" }
            li { "Enhanced Devtools (potentially featuring some AI!) and testing framework" }
            li { "Complete Mobile overhaul" }
            li { "Fullstack overhaul with WebSocket, SSE, progressive forms, and more" }
        }
        h2 { id: "sneak-peek-dioxus-blitz-revival-using-servo",
            a {
                href: "#sneak-peek-dioxus-blitz-revival-using-servo",
                class: "header",
                "Sneak Peek: Dioxus-Blitz revival using Servo"
            }
        }
        p {
            "We’re not going to say much about this now, but here’s a sneak peek at “Blitz 2.0”… we’re finally integrating servo into Blitz so you can render natively with WGPU using the same CSS engine that powers Firefox. To push this effort forward, we’ve brought the extremely talented Nico Burns (the wizard behind our layout library Taffy) on full time. More about this later, but here’s a little demo of "
            a { href: "http://google.com", "google.com" }
            " being rendered at 900 FPS entirely on the GPU:"
        }
        p {
            img {
                src: "https://i.imgur.com/I1HRiBd.png",
                alt: "Google rendered with blitz",
                title: "",
            }
        }
        p {
            "Admittedly the current iteration is not quite there (google.com is in fact a little wonky) but we’re progressing rapidly here and are quickly approaching something quite usable. The repo is here if you want to take a look and get involved:"
        }
        p {
            a { href: "https://github.com/jkelleyrtp/stylo-dioxus",
                "https://github.com/jkelleyrtp/stylo-dioxus"
            }
        }
        h2 { id: "how-can-you-contribute",
            a { href: "#how-can-you-contribute", class: "header", "How can you contribute?" }
        }
        p {
            "Well, that’s it for the new features. We might’ve missed a few things (there’s so much new!). If you find Dioxus as exciting as we do, we’d love your help to completely transform app development. We’d love contributions including:"
        }
        ul {
            li { "Translating docs into your native language" }
            li { "Attempting “Good First Issues”" }
            li { "Improving our documentation" }
            li { "Contributing to the CLI" }
            li { "Help answer questions from the discord community" }
        }
        p {
            "That’s it! We’re super grateful for the community support and excited for the rest of 2024."
        }
        p { "Build cool things! ✌\u{fe0f}" }
    }
}
#[component(no_case_check)]
pub fn Release060() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p {
            "Happy Holidays! As an early holiday present, we're happy to announce the release of Dioxus 0.6! 🎉"
        }
        p {
            "Dioxus (Dye-ox-us) is a framework for building fullstack web, desktop, and mobiles apps in Rust."
        }
        p {
            "Dioxus 0.6 is the biggest release of Dioxus ever: over 350 pull requests, hundreds of issues closed, huge new features, and a complete overhaul the framework. With this release, we focused on fixing bugs, polishing existing features, and improving tooling."
        }
        p { "Here's what's new in Dioxus 0.6:" }
        ul {
            li {
                strong {
                    a { href: "#android-and-ios-support-for",
                        code { "dx serve" }
                        " for mobile"
                    }
                }
                ": Serve your app on Android and iOS simulators and devices."
            }
            li {
                strong {
                    a { href: "#interactive-command-line-tools", "Interactive CLI" }
                }
                ": Rewrite of the Dioxus CLI with a new, interactive UX inspired by Astro."
            }
            li {
                strong {
                    a { href: "#inline-wasm-stacktraces-and", "Inline Stack Traces" }
                }
                ": Capture WASM panics and logs directly into your terminal."
            }
            li {
                strong {
                    a { href: "#toasts-and-loading-screens", "Toasts and Loading Screens" }
                }
                ": New toasts and loading screens for web apps in development."
            }
            li {
                strong {
                    a { href: "#fullstack-desktop-and-mobile", "Server Functions for Native" }
                }
                ": Inline Server RPC for Desktop and Mobile apps."
            }
            li {
                strong {
                    a { href: "#completely-revamped-autocomplete", "Revamped Autocomplete" }
                }
                ": Massively improved autocomplete using Rust-analyzer itself."
            }
            li {
                strong {
                    a { href: "#completely-revamped-hot-reloading", "Magical Hot-Reloading" }
                }
                ": Hot-Reloading of formatted strings, properties, and nested "
                code { "rsx!{{}}" }
                "."
            }
            li {
                strong {
                    a { href: "#mobile-hot-reloading", "Mobile Hot-Reloading" }
                }
                ": Hot-Reloading of "
                code { "rsx!{{}}" }
                " and assets for mobile devices and emulators."
            }
            li {
                strong {
                    a { href: "#stabilizing-manganis",
                        code { "asset!" }
                        " Stabilization"
                    }
                }
                ": Stabilizing our linker-based asset system integrated for native apps."
            }
            li {
                strong {
                    a { href: "#suspense-and-html-streaming-for-the-web", "Streaming HTML" }
                }
                ": Stream "
                code { "Suspense" }
                " and "
                code { "Error" }
                " Boundaries from the server to the client."
            }
            li {
                strong {
                    a { href: "#static-site-generation-and-isg", "SSG and ISG" }
                }
                ": Support for Static Site Generation and Incremental Static Regeneration."
            }
            li {
                strong {
                    a { href: "#question-mark-error-handling",
                        "Error Handling with  "
                        code { "?" }
                    }
                }
                ": Use "
                code { "?" }
                " to handle errors in event handlers, tasks, and components."
            }
            li {
                strong {
                    a { href: "#document-elements", "Meta Elements" }
                }
                ": New "
                code { "Head" }
                ", "
                code { "Title" }
                ", "
                code { "Meta" }
                ", and "
                code { "Link" }
                " elements for setting document attributes."
            }
            li {
                strong {
                    a { href: "#synchronous",
                        "Synchronous  "
                        code { "prevent_default" }
                    }
                }
                ": Handle events synchronously across all platforms."
            }
            li {
                strong {
                    a { href: "#tracking-size-with",
                        code { "onresize" }
                        " Event Handler"
                    }
                }
                ": Track an element's size without an IntersectionObserver."
            }
            li {
                strong {
                    a { href: "#tracking-visibility-with",
                        code { "onvisible" }
                        " Event Handler"
                    }
                }
                ": Track an element's visibility without an IntersectionObserver."
            }
            li {
                strong {
                    a { href: "#hybrid-wgpu-overlays", "WGPU Integration" }
                }
                ": Render Dioxus as an overlay on top of WGPU surfaces and child windows."
            }
            li {
                strong {
                    a { href: "#web-ios-and-android-bundle-support",
                        code { "dx bundle" }
                        " for Web, iOS, and Android"
                    }
                }
                ": Complete "
                code { "dx bundle" }
                " support for every platform."
            }
            li {
                strong {
                    a { href: "#json-output-for-ci--cli",
                        code { "json" }
                        " mode"
                    }
                }
                ": Emit CLI messages as JSON for use by 3rd party tools and CI/CD pipelines."
            }
            li {
                strong {
                    a { href: "#new-starter-templates", "New Templates" }
                }
                ": Three new starter templates for cross-platform apps."
            }
            li {
                strong {
                    a { href: "#nightly-docs-tutorials-and-new-guides", "Nightly Tutorial and Guides" }
                }
                ": New tutorials and guides for Dioxus 0.6 and beyond."
            }
            li {
                strong {
                    a { href: "#preview-of-in-place-binary-patching", "Binary Patching Prototype" }
                }
                ": Prototype of our new pure Rust hot-reloading engine."
            }
        }
        h2 { id: "about-this-release",
            a { href: "#about-this-release", class: "header", "About this Release" }
        }
        p {
            "I’m happy to say that Dioxus is the most mature and complete it’s ever been, "
            em { "finally" }
            " living up to the original mission."
        }
        p {
            "Dioxus 0.6 is the culmination of nearly 6 months of hard work. We originally intended Dioxus 0.6 to be a much smaller release, but with the complexity of assets and tooling improvements, we eventually chose to merge the features we had planned for Dioxus 0.7 as well. With this release, we set a very high bar for quality and polish: everything from CLI tools to APIs and ecosystem libraries have seen huge improvements."
        }
        p {
            "With 0.6, we focused on improving existing features and cleaning up various holes in the framework. The end result: a rebirth of Dioxus with hundreds of bug fixes, massively improved tooling, and the “ungating” of essential APIs. Everything from CLI tooling to hot-reloading and autocomplete saw huge jumps in quality. We're not "
            em { "completely done" }
            " - early next year we'd still like to ship our Rust binary patching and WASM bundle splitting prototypes - but we're still quite confident in Dioxus' improved developer experience."
        }
        p {
            "Since this post is quite long, we made a quick video highlighting new features, bugs fixed, and a quick tour of everything you can do with Dioxus now:"
        }
        p {
            class: "inline-html-block",
            dangerous_inner_html: "<iframe style=\"width: 120%\" height=\"500px\" class=\"centered-overflow\" src=\"https://www.youtube.com/embed/-RN4LB3-9AE\" title=\"Dioxus 0.5 Overview preview\" frameborder=\"0\" allow=\"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share\" referrerpolicy=\"strict-origin-when-cross-origin\" allowfullscreen></iframe>\n",
        }
        h2 { id: "android-and-ios-support-for",
            a { href: "#android-and-ios-support-for", class: "header",
                "Android and iOS support for "
            }
            code { "dx serve" }
        }
        p {
            "Our goal with Dioxus has always been to support web, desktop, mobile, and more. And while Dioxus has supported mobile since its release, the Rust tooling for mobile has been extremely unstable. Users constantly ran into issues with tools like "
            a { href: "https://github.com/BrainiumLLC/cargo-mobile",
                code { "cargo-mobile" }
            }
            " and Tauri's fork "
            a { href: "https://github.com/tauri-apps/cargo-mobile2", "cargo-mobile2" }
            ". These tools, while useful, take a different architectural approach than what is a good fit for Dioxus."
        }
        p {
            "We want to provide features like asset bundling, hot-reloading, and proper support for \"regular\" apps built with a traditional  "
            code { "main.rs" }
            " - none of which we can properly do with the existing tools."
        }
        p {
            "With this release, we wrote our entire mobile tooling system from scratch. Now, you can go from  "
            code { "dx new" }
            " to  "
            code { "dx serve --platform ios" }
            " in a matter of seconds."
        }
        p {
            img {
                src: "/assets/06assets/image.png",
                alt: "Dioxus Mobile Support",
                title: "",
            }
        }
        p {
            "The Android and iOS simulator targets support all the same features as desktop: hot-reloading, fast rebuilds, asset bundling, logging, etc. One notable accomplishment: you can build Rust mobile apps with a simple  "
            code { "main.rs" }
            ". Existing solutions like "
            a { href: "http://github.com/rust-mobile/xbuild", "xbuild" }
            " and "
            a { href: "http://github.com/tauri-apps/tauri", "Tauri" }
            " require you to fundamentally restructure your app. Your app also needs to be converted to a "
            code { "cdylib" }
            ", meaning you can’t share a launch function between desktop and mobile."
        }
        p { "The simplest Dioxus 0.6 Mobile app is tiny:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    dioxus::launch(|| rsx! {{ </span><span style=\"color:#ffee99;\">&quot;hello dioxus! 🧬&quot; </span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "Especially, when compared to v0.5 which required you to migrate your app to a  "
            code { "cdylib" }
            " and manually set up the binding layer:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// no main allowed! - must expose a `start_app` function\n</span><span style=\"color:#f8f8f2;\">#[no_mangle]\n</span><span style=\"color:#f8f8f2;\">#[inline(never)]\n</span><span style=\"color:#f8f8f2;\">#[cfg(any(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;android&quot;</span><span style=\"color:#f8f8f2;\">, target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;ios&quot;</span><span style=\"color:#f8f8f2;\">))]\n</span><span style=\"color:#f92672;\">pub extern </span><span style=\"color:#ffee99;\">&quot;C&quot; </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">start_app</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    #[cfg(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;android&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    {{\n</span><span style=\"color:#f8f8f2;\">        tao::android_binding</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">            com_dioxuslabs,\n</span><span style=\"color:#f8f8f2;\">            app_name,\n</span><span style=\"color:#f8f8f2;\">            WryActivity,\n</span><span style=\"color:#f8f8f2;\">            wry::android_setup,\n</span><span style=\"color:#f8f8f2;\">            _start_app\n</span><span style=\"color:#f8f8f2;\">        );\n</span><span style=\"color:#f8f8f2;\">        wry::android_binding</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(com_dioxuslabs, app_name);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// need to manually catch panics!\n</span><span style=\"color:#f8f8f2;\">    #[cfg(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;ios&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">stop_unwind</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#66d9ef;\">main</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[cfg(any(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;android&quot;</span><span style=\"color:#f8f8f2;\">, target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;ios&quot;</span><span style=\"color:#f8f8f2;\">))]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">stop_unwind</span><span style=\"color:#f8f8f2;\">&lt;F: FnOnce() -&gt; T, T&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">f</span><span style=\"color:#f8f8f2;\">: F) -&gt; T {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match </span><span style=\"color:#f8f8f2;\">std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(t) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> t,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(err) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            eprintln!(</span><span style=\"color:#ffee99;\">&quot;attempt to unwind out of `rust` with err: {{:?}}&quot;</span><span style=\"color:#f8f8f2;\">, err);\n</span><span style=\"color:#f8f8f2;\">            std::process::abort()\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Our inline mobile support requires no extra configurations, no manual setup for Gradle, Java, Cocoapods, and no other 3rd party tooling. If you already have the Android NDK or iOS Simulator installed, you currently are less than 30 seconds away from a production-ready mobile app written entirely in Rust. In the time it takes for you to watch this gif, you could have your very own mobile app:"
        }
        p {
            img {
                src: "/assets/06assets/dxnew.mp4",
                alt: "dx-serve.mp4",
                title: "",
            }
        }
        p {
            "While 1st-class support for mobile platforms is quite exciting, there are certainly many limitations: the Rust mobile ecosystem is nascent, we don’t have great ways of configuring the many platform-specific build flags, and there isn’t a particularly great Rust/Java interop story. However, we’re  dedicated to improving Rust's mobile app development capabilities and will be rolling out improvements to the mobile ecosystem over the next year."
        }
        p {
            "If you're interested in helping us build out mobile support, please join us on "
            a { href: "https://discord.gg/XgGxMSkvUM", "Discord" }
            ". We have ambitious goals for Dioxus 0.7+ to integrate cool features like automatic deploys to app stores and a build a comprehensive SDK for mobile development in Rust."
        }
        h2 { id: "interactive-command-line-tools",
            a { href: "#interactive-command-line-tools", class: "header",
                "Interactive Command Line Tools"
            }
        }
        p {
            "You might have noticed in the GIFs above... Dioxus 0.6 is shipping with a completely overhauled CLI experience! We’ve completely rewritten the CLI to support a ton of new features and improve stability:"
        }
        p {
            img {
                src: "/assets/06assets/image%201.png",
                alt: "new-cli.png",
                title: "",
            }
        }
        p {
            "The new CLI sports live progress bars, animations, an interactive filter system, the ability to change log levels on the fly, and more."
        }
        p {
            img {
                src: "/assets/06assets/cli-new.mp4",
                alt: "cli_animation",
                title: "",
            }
        }
        p {
            "The CLI rewrite alone took more than half this release cycle. We went through several different design iterations and solved tons of bugs along the way. A few of the highlights:"
        }
        ul {
            li {
                "You can manually rebuild your app by pressing "
                code { "r" }
            }
            li {
                "You can toggle the log level of the CLI output on the fly and even inspect Cargo internal logs"
            }
            li { "We output all internal logs of the CLI so you can debug any issues" }
            li { "We capture logs for WASM tracing and panics" }
            li {
                "We dropped the "
                code { "outdir" }
                " concept and instead use "
                code { "target/dx" }
                " for all output."
            }
        }
        p {
            "You can install the new CLI using "
            a { href: "https://github.com/cargo-bins/cargo-binstall", "cargo binstall" }
            " with "
            code { "cargo binstall dioxus-cli@0.6.0 --force" }
            "."
        }
        h2 { id: "inline-wasm-stacktraces-and",
            a { href: "#inline-wasm-stacktraces-and", class: "header",
                "Inline WASM stacktraces and "
            }
            code { "tracing" }
            " integration"
        }
        p {
            "Along with the rewrite of the CLI, we shipped a  "
            code { "tracing" }
            " integration for WASM apps that captures panics and logs and sends them  "
            code { "dx" }
            " in your terminal. When you build your app with debug symbols, stack traces directly integrate with your editor, allowing you to jump directly to the troublesome files from within your terminal."
        }
        p {
            img {
                src: "/assets/06assets/Screenshot_2024-11-14_at_8.52.18_PM.png",
                alt: "Inline Stack Traces",
                title: "",
            }
        }
        p {
            "Thanks to this integration, we now have much nicer logging around fullstack apps, showing status codes, fetched assets, and other helpful information during development. With the toggle-able verbosity modes, you can now inspect the internal logs of the CLI itself, making it easier to debug issues with tooling to understand what exactly  "
            code { "dx" }
            " is doing when it builds your app. Simply type  "
            code { "v" }
            " to turn on “verbose” mode and  "
            code { "t" }
            " to turn on “trace” mode for more helpful logs:"
        }
        p {
            img {
                src: "/assets/06assets/Screenshot_2024-11-14_at_9.06.05_PM.png",
                alt: "Screenshot 2024-11-14 at 9.06.05\u{202f}PM.png",
                title: "",
            }
        }
        h2 { id: "toasts-and-loading-screens",
            a { href: "#toasts-and-loading-screens", class: "header", "Toasts and Loading Screens" }
        }
        p {
            "As part of our CLI overhaul, we wanted to provide better feedback for developers when building web apps. Dioxus 0.6 will now show Popup Toasts and Loading Screens for web apps in development mode."
        }
        p {
            "Now, when your app is building, Dioxus will rendering a loading screen with the current progress of the build:"
        }
        p {
            img {
                src: "/assets/06assets/Screenshot_2024-11-14_at_9.41.38_PM.png",
                alt: "Screenshot 2024-11-14 at 9.41.38\u{202f}PM.png",
                title: "",
            }
        }
        p {
            "Additionally, once the app is rebuilt, you’ll receive a toast indicating the status of the build:"
        }
        p {
            img {
                src: "/assets/06assets/Screenshot_2024-11-14_at_9.42.33_PM.png",
                alt: "Screenshot 2024-11-14 at 9.42.33\u{202f}PM.png",
                title: "",
            }
        }
        h2 { id: "fullstack-desktop-and-mobile",
            a { href: "#fullstack-desktop-and-mobile", class: "header",
                "Fullstack Desktop and Mobile"
            }
        }
        p {
            "Additionally, as improving tooling, we properly integrated server functions with native apps. Server functions finally work out-of-the-box when targeting desktop and mobile:"
        }
        p {
            img {
                src: "/assets/06assets/native-serverfn12.mp4",
                alt: "native-server12.mp4",
                title: "",
            }
        }
        p {
            "By default, in development, we set the server function endpoint to be localhost, so in production you need to make sure to point the functions to your deployed server:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    #[cfg(feature </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;production&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    server_fn::client::set_server_url(</span><span style=\"color:#ffee99;\">&quot;app.endpoint.com&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    dioxus::launch(app)\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        h2 { id: "completely-revamped-autocomplete",
            a { href: "#completely-revamped-autocomplete", class: "header",
                "Completely Revamped Autocomplete"
            }
        }
        p {
            "Another huge overhaul in Dioxus 0.6: greatly improved autocomplete of  "
            code { "rsx! {{}}" }
            ".  Our old implementation of  "
            code { "rsx! {{}}" }
            " suffered from poor integration with tools like Rust-analyzer which provide language-server integration for your code. If the input to the macro wasn’t perfectly parsable, we failed to generate any tokens at all, meaning rust-analyzer couldn’t jump in to provide completions."
        }
        p {
            "The work to fix this was immense. Macro parsing libraries like  "
            code { "syn" }
            " don’t provide great facilities for “partial parsing” Rust code which is necessary for implementing better errors and autocomplete. We had to rewrite the entire internals of  "
            code { "rsx! {{}}" }
            " to support partial parsing of  "
            code { "rsx! {{}}" }
            " , but finally, in 0.6, we’re able to provide stellar autocomplete. Not only can we autocomplete Rust code in attribute positions, but with a few tricks, we’re able to automatically insert the appropriate braces next to element names:"
        }
        p {
            img {
                src: "/assets/06assets/Screenshot_2024-11-14_at_9.55.12_PM.png",
                alt: "Screenshot 2024-11-14 at 9.55.12\u{202f}PM.png",
                title: "",
            }
        }
        p {
            "The autocomplete experience is much nicer now, with all attributes, elements, components, and inline Rust code benefiting from the overhauled experience. All Rust expressions participate in proper Rust-analyzer autocomplete and we're even able to provide warnings when  "
            code { "rsx!{{}}" }
            " input is malformed instead of panicking."
        }
        p {
            img {
                src: "/assets/06assets/autocomplete-overhaul.mp4",
                alt: "autocomplete-overhaul.mp4",
                title: "",
            }
        }
        h2 { id: "completely-revamped-hot-reloading",
            a { href: "#completely-revamped-hot-reloading", class: "header",
                "Completely Revamped Hot-Reloading"
            }
        }
        p {
            "As part of our effort to improve the  "
            code { "rsx! {{}}" }
            " experience, we shipped massive improvements to the hot-reloading engine powering Dioxus. Our internal goal was to iterate on the Dioxus Docsite content with zero full rebuilds - we only wanted full rebuilds when modifying real Rust code."
        }
        p { "This means we needed to add support for a number of new hot-reloading engine changes:" }
        ul {
            li {
                a { href: "#hot-reloading-formatted-strings", "Hot-reload formatted strings" }
            }
            li {
                a { href: "#hot-reloading-rust-literals",
                    "Hot-reload component properties and simple Rust expressions"
                }
            }
            li {
                a { href: "#hot-reloading-nested-rsx-",
                    "Hot-reload  "
                    code { "for" }
                    " and  "
                    code { "if" }
                    " blocks in RSX"
                }
            }
            li {
                a { href: "#hot-reloading-nested-rsx-", "Hot-reload nested rsx blocks" }
            }
            li {
                a { href: "#mobile-hot-reloading", "Hot-reload mobile platforms" }
            }
        }
        p {
            "The new hot-reloading engine almost feels like magic - you can quickly iterate on new designs without waiting for full Rust rebuilds:"
        }
        p {
            img {
                src: "/assets/06assets/dogapphr2.mp4",
                alt: "dog_app.mp4",
                title: "",
            }
        }
        h2 { id: "hot-reloading-formatted-strings",
            a { href: "#hot-reloading-formatted-strings", class: "header",
                "Hot-Reloading Formatted Strings"
            }
        }
        p {
            "We can now hot-reload any formatted string in your markup! For this component, we can hot-reload both the  "
            code { "class" }
            " attribute on button as well as the text in the button itself."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Counter</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">class_ext</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ class: </span><span style=\"color:#ffee99;\">&quot;btn-{{class_ext}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Count {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Notice that these are "
            em { "formatted strings." }
            " Very frequently, when working on the docsite, we’d want to modify formatted tailwind classes, but these changes would cause a full rebuild. This drastically slowed down iteration time, making working on the docsite a rather unpleasant experience. Hot-Reloading of formatted strings works "
            em { "everywhere" }
            " in rsx. This means you can get string hot-reloading in component props too."
        }
        h2 { id: "hot-reloading-rust-literals",
            a { href: "#hot-reloading-rust-literals", class: "header",
                "Hot-Reloading Rust Literals"
            }
        }
        p {
            "As part of the hot-reloading overhauls, we also now support hot-reloading of any literals we can find inside your rsx. We built a very simple interpreter for Rust code! Any changes to literals are automatically propagated through the signal-based reactivity system shipped in 0.5. This means you can change the bounds on component props without causing a full rebuild."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">LoopIt</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Change either prop without causing a full rebuild\n</span><span style=\"color:#f8f8f2;\">        Link {{\n</span><span style=\"color:#f8f8f2;\">            to: </span><span style=\"color:#ffee99;\">&quot;google.com&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            enabled: </span><span style=\"color:#ff80f4;\">false\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p { "While limited in some ways, this can feel downright magical." }
        h2 { id: "hot-reloading-nested-rsx-",
            a { href: "#hot-reloading-nested-rsx-", class: "header", "Hot-Reloading nested rsx (" }
            code { "for" }
            "/ "
            code { "if" }
            " / "
            code { "component" }
            " )"
        }
        p {
            "With Dioxus 0.4, we shipped improvements that enabled a simpler syntax for  "
            code { "for" }
            " loops and  "
            code { "if" }
            " chains in rsx. However, we never properly implemented hot-reloading for the contents of these items, leading to frequent unnecessary rebuilds. With Dioxus 0.6, we finally had a chance to iron out hot-reloading in every possible instance. Now, more places properly support hot-reloading, like  "
            code { "for" }
            " loops and  "
            code { "if" }
            " chains:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">LoopIt</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> x </span><span style=\"color:#f92672;\">in </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">..</span><span style=\"color:#ff80f4;\">10 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// modifying the body of this loop is hot-reloadable!\n</span><span style=\"color:#f8f8f2;\">            li {{ </span><span style=\"color:#ffee99;\">&quot;{{x}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "We also now support hot-reloading of bodies of components:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">LoopIt</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Counter {{\n</span><span style=\"color:#f8f8f2;\">            div {{ </span><span style=\"color:#ffee99;\">&quot;These div contents get hot-reloaded too!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "This new engine also allows you to move and clone Rust expressions between contexts, allowing you to re-use components and formatted strings between element properties without a full rebuild."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">LoopIt</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// If we start with this formatted text\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Thing1 {{a}}&quot;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// we can add this formatted text on the fly\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Thing2 {{a}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        h2 { id: "mobile-hot-reloading",
            a { href: "#mobile-hot-reloading", class: "header", "Mobile Hot-Reloading" }
        }
        p {
            "With Dioxus 0.6, we also wanted to fix the longstanding issue where mobile simulators didn’t properly get hot-reloading. Mobile can be tricky to work with - and will take a long time to get 100% right - but this is a solid step in making mobile targets better supported with Dioxus."
        }
        p {
            img {
                src: "/assets/06assets/bundled-ios-reload.mp4",
                alt: "bundled-ios-reload.mp4",
                title: "",
            }
        }
        p {
            "The changes here also unlocked hot-reloading of bundled assets used by the  "
            code { "asset!()" }
            " macro. If you're using Tailwind with Dioxus, you can now simply run your Tailwind watcher in the background and Dioxus will automatically hot-reload your CSS files across web, desktop, and mobile."
        }
        h2 { id: "proper-workspace-hot-reloading",
            a { href: "#proper-workspace-hot-reloading", class: "header",
                "Proper Workspace Hot-Reloading"
            }
        }
        p {
            "We now properly support hot-reloading across multiple projects in a workspace. This solves the longstanding issue where we didn't propagate changes from a component library in one crate and in your main app. Our new hot-reload engine intelligently walks your project’s dependencies across the filesystem and watches all the related Rust files (respecting their gitignore)."
        }
        h2 { id: "stabilizing-manganis",
            a { href: "#stabilizing-manganis", class: "header", "Stabilizing Manganis " }
            code { "asset!()" }
            " system"
        }
        p {
            "We introduced our new asset system,\u{a0}"
            a { href: "https://github.com/DioxusLabs/manganis", "Manganis" }
            ", in an alpha state with the 0.5 release. Dioxus 0.6 stabilizes the asset system and fixes several bugs and performance issues. You can try out the new\u{a0}"
            a { href: "https://github.com/DioxusLabs/manganis/pull/30", "linker based asset system" }
            "\u{a0}by including an\u{a0}"
            code { "asset!" }
            "\u{a0}anywhere in your code. It will automatically be optimized and bundled across all platforms:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    img {{ src: asset!(</span><span style=\"color:#ffee99;\">&quot;/assets/image.png&quot;</span><span style=\"color:#f8f8f2;\">) }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "Manganis is a crucial step in supporting assets cross-platform, and specifically, through dependencies. Previously, if an upstream library wanted to export an asset like an image or a stylesheet, your app would need to manually add those assets in your  "
            code { "assets" }
            " folder. This gets complex and messy when libraries generate CSS: many classes are duplicated and might even conflict with each other. Now, all CSS collected by the  "
            code { "asset!()" }
            " macro is processed via our build pipeline, benefiting from minification and deduplication. Libraries can include their stylesheets and images and components and you can be guaranteed that those assets make it bundled into your app:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        cool_icons::SomeCoolImage {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// in a distant library....\n</span><span style=\"font-style:italic;color:#66d9ef;\">mod </span><span style=\"color:#f8f8f2;\">cool_icons {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">SomeCoolImage</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">        rsx! {{\n</span><span style=\"color:#f8f8f2;\">            img {{ src: asset!(</span><span style=\"color:#ffee99;\">&quot;/assets/some_cool_image.png&quot;</span><span style=\"color:#f8f8f2;\">) }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Even better, assets like images are automatically optimized to generate thumbnails and more optimized formats. This can cut huge amounts of data from your site - AVIF and Webp can reduce file sizes by up to 90%. A funny note - platforms like Vercel actually "
            a { href: "https://vercel.com/docs/image-optimization",
                "provide paid products for image optimization"
            }
            " while Manganis can do this for you, for free, at build time!"
        }
        p {
            img {
                src: "/assets/06assets/manganis-opt.avif",
                alt: "manganis-opt",
                title: "",
            }
        }
        p {
            "Additionally, manganis automatically hashes the images and modifies the generated asset name, allowing for better integration with CDNs and browser caching."
        }
        p {
            img {
                src: "/assets/06assets/Screenshot_2024-11-14_at_10.22.48_PM.png",
                alt: "Screenshot 2024-11-14 at 10.22.48\u{202f}PM.png",
                title: "",
            }
        }
        p {
            "Manganis can handle a wide variety of formats - applying optimizations to assets like CSS, JavaScript, images, videos, and more."
        }
        p {
            "In Dioxus 0.5, we released Manganis in “alpha” status - and in 0.6 we’re stabilizing it. We’ve adjusted the API, so you’ll need to update any existing code that already uses it. Our new implementation is much more reliable, solving many of the bugs users were running into after the 0.5 release."
        }
        p {
            "Our new system leverages "
            em { "the linker" }
            " to extract asset locations from the compiled binary. This is a rather advanced technique and took a while to get right, but we believe it’s a more robust solution in the long term. If you’re interested in integrating Manganis into your libraries and apps (like say, Bevy!), we have a guide just for that."
        }
        h2 { id: "suspense-and-html-streaming-for-the-web",
            a {
                href: "#suspense-and-html-streaming-for-the-web",
                class: "header",
                "Suspense and HTML Streaming for the Web"
            }
        }
        p {
            "Async is a core component of any UI framework. Dioxus provides hooks to handle async state. You can start a future and handle the loading and resolved states within the component:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Article</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Use resource starts a future and returns the current state\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> article </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(fetch_article);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can `match` the state of the future.\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">match </span><span style=\"color:#66d9ef;\">article</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(article) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{ </span><span style=\"color:#ffee99;\">&quot;{{article}}&quot; </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f92672;\">=&gt;  </span><span style=\"color:#f8f8f2;\">rsx! {{ p {{ </span><span style=\"color:#ffee99;\">&quot;Loading...&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This works ok if you have a single future, but it quickly gets messy when combining many futures into one UI:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Article</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Use resource starts a future in the background and returns the current state\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(title) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(fetch_title).</span><span style=\"color:#66d9ef;\">cloned</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">return </span><span style=\"color:#f8f8f2;\">rsx! {{ </span><span style=\"color:#ffee99;\">&quot;loading...&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(article) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(fetch_article).</span><span style=\"color:#66d9ef;\">cloned</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">return </span><span style=\"color:#f8f8f2;\">rsx! {{ </span><span style=\"color:#ffee99;\">&quot;loading...&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(category) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> article.</span><span style=\"color:#66d9ef;\">title</span><span style=\"color:#f8f8f2;\">()).</span><span style=\"color:#66d9ef;\">cloned</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">return </span><span style=\"color:#f8f8f2;\">rsx! {{ </span><span style=\"color:#ffee99;\">&quot;loading...&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Title {{ </span><span style=\"color:#ffee99;\">&quot;{{title}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        Body {{ category, article }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "In addition to hooks, we need a way to display a different state when async is loading. Dioxus 0.6 introduces a new core primitive for async UI called suspense boundaries. A suspense boundary is a component that renders a placeholder when any child component is loading:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    SuspenseBoundary {{\n</span><span style=\"color:#f8f8f2;\">        fallback: |</span><span style=\"font-style:italic;color:#fd971f;\">context</span><span style=\"color:#f8f8f2;\">: SuspenseContext| rsx! {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// Render a loading placeholder if\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// any child component is &quot;suspended&quot;\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Loading...&quot;\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        Article {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "In any child component, you can simply bubble up the pending state with\u{a0} "
            code { "?" }
            "\u{a0}to pause rendering until the future is finished:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Article</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> title </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(fetch_title).</span><span style=\"color:#66d9ef;\">suspend</span><span style=\"color:#f8f8f2;\">()</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> article </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(fetch_article).</span><span style=\"color:#66d9ef;\">suspend</span><span style=\"color:#f8f8f2;\">()</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> category </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> article.</span><span style=\"color:#66d9ef;\">title</span><span style=\"color:#f8f8f2;\">()).</span><span style=\"color:#66d9ef;\">suspend</span><span style=\"color:#f8f8f2;\">()</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Since we bubbled up all the pending futures with `?` we can just\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// handle the happy path in the component\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Title {{ </span><span style=\"color:#ffee99;\">&quot;{{title}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        Body {{ category, article }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Along with suspense boundaries, dioxus fullstack also supports streaming each suspense boundary in from the server. Instead of waiting for the whole page to load, dioxus fullstack streams in each chunk with the resolved futures as they finish:"
        }
        p {
            img {
                src: "/assets/06assets/streamingsuspense.mp4",
                alt: "streaming-suspense.mp4",
                title: "",
            }
        }
        p {
            "Many of these features are quite cutting-edge and are just now being rolled out in frameworks in the JavaScript ecosystem. Getting the details right for Dioxus was quite difficult. We wanted to support both the fullstack web as well as native desktop and mobile apps. These two platforms often have competing design considerations. Fortunately, suspense also works for desktop and mobile, allowing you to emulate web-like data fetching patterns for native apps."
        }
        h2 { id: "static-site-generation-and-isg",
            a { href: "#static-site-generation-and-isg", class: "header",
                "Static Site Generation and ISG"
            }
        }
        p {
            "As part of our work on streaming, we also wanted to support another cutting-edge web feature: incremental static generation (ISG) and its cousin static site generation (SSG)."
        }
        p {
            "Static site generation is a technique used by many web frameworks like Jekyll, Hugo, or Zola, to emit static  "
            code { ".html" }
            " not reliant on JavaScript. Sites like blogs and portfolios typically use static site generation since platforms like GitHub Pages allow hosting static sites for free. In fact, this very docsite uses Dioxus SSG deployed to GitHub Pages! SSG helps improve SEO and speed up load times for your users."
        }
        p {
            "In Dioxus 0.6, we now support static-site-generation out of the box for all fullstack projects. Simply add a server function to your app called  "
            code { "static_routes" }
            " that returns the list of routes that  "
            code { "dx" }
            " should generate:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[server(endpoint </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;static_routes&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">static_routes</span><span style=\"color:#f8f8f2;\">() -&gt; Result&lt;Vec&lt;String&gt;, ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(Route::static_routes()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">into_iter</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">route</span><span style=\"color:#f8f8f2;\">| route.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">        .collect::&lt;Vec&lt;</span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">&gt;&gt;())\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Now, when you want to emit your static  "
            code { ".html" }
            ", add the  "
            code { "--ssg" }
            "  flag to  "
            code { "dx build" }
            ":"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dx build </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">platform web </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">ssg</span></pre>\n" }
        p {
            "Static-site-generation is built on a new feature in Dioxus called incremental-site-generation (ISG). ISG is a technique similar to static-site-generation where the server generates pages on demand and caches them on the system filesystem. This allows the server to cache huge amounts of pages (for something like a school’s facebook directory or an e-commerce site with thousands of products) that get periodically invalidated. ISG is a somewhat advanced technique but is required to enable when using static-site-generation:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        dioxus::LaunchBuilder::new()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">with_cfg</span><span style=\"color:#f8f8f2;\">(server_only! {{\n</span><span style=\"color:#f8f8f2;\">            ServeConfig::builder()\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// turn on incremental site generation with the .incremental() method\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">incremental</span><span style=\"color:#f8f8f2;\">(IncrementalRendererConfig::new())\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">build</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        }})\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">            rsx! {{\n</span><span style=\"color:#f8f8f2;\">                Router::&lt;Route&gt; {{}}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "We will likely be changing these APIs in future releases, but we are eager to let users experiment with these new features to simplify the existing static site setup."
        }
        h2 { id: "question-mark-error-handling",
            a { href: "#question-mark-error-handling", class: "header",
                "Question Mark Error Handling"
            }
        }
        p {
            "With this release, we’ve finally made the transition where  "
            code { "Element" }
            " is no longer an  "
            code { "Option<Node>" }
            " but rather a  "
            code { "Result<Node>" }
            ". This means we’re "
            em { "finally" }
            " able to open up the use of typical rust error handling in components:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Slider</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value: </span><span style=\"font-style:italic;color:#66d9ef;\">f64 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;1234&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">()</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Handle {{ offset: value }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "The new  "
            code { "RenderError" }
            " acts like anyhow’s  "
            code { "Error" }
            " type that can take in any  "
            code { "dyn std::Error" }
            " type and propagate it upwards to the nearest error boundary."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Input</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        ErrorBoundary {{\n</span><span style=\"color:#f8f8f2;\">            handle_error: |</span><span style=\"font-style:italic;color:#fd971f;\">errors</span><span style=\"color:#f8f8f2;\">| rsx! {{\n</span><span style=\"color:#f8f8f2;\">                h3 {{ </span><span style=\"color:#ffee99;\">&quot;Oops, we encountered an error.!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                pre {{ </span><span style=\"color:#ffee99;\">&quot;Please report {{errors:?}} to the developers.&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            Slider {{}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "What’s even better: the  "
            code { "?" }
            " syntax also works in event handlers, so you can quickly add things like server functions to your app without worrying about manual error handling:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Counter</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> data </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| Data::default());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// the `?` automatically throws this error upwards\n</span><span style=\"color:#f8f8f2;\">                data.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#66d9ef;\">request_server_data</span><span style=\"color:#f8f8f2;\">().await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(())\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;{{data}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This new syntax works with suspense and HTML-streaming return errors while rendering that don’t bring down the entire page."
        }
        h2 { id: "document-elements",
            a { href: "#document-elements", class: "header", "Document Elements: " }
            code { "Title {{}}" }
            " , "
            code { "Link {{}}" }
            " , "
            code { "Stylesheet" }
            " , and "
            code { "Meta {{}}" }
        }
        p {
            "To date, it’s been rather cumbersome seemingly simple JavaScript operations in Dioxus. Due to our cross-platform nature, we need to find solutions to simple problems in ways that work for web, desktop, and mobile with a single abstraction."
        }
        p {
            "With Dioxus 0.6, we’re providing special elements under the  "
            code { "document" }
            " namespace that make it possible to interact with the HTML  "
            code { "document" }
            " object without needing to write extra JavaScript."
        }
        p {
            "Now, to set the  "
            code { "title" }
            " of your HTML document, simply use the  "
            code { "document::Title {{}}" }
            " component:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::document::Title;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    dioxus::launch(|| rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Title {{ </span><span style=\"color:#ffee99;\">&quot;WebAssembly rocks!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;A site dedicated to webassembly&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "And accordingly, the title of the page will update:" }
        p {
            img {
                src: "/assets/06assets/Screenshot_2024-11-14_at_11.28.42_PM.png",
                alt: "Screenshot 2024-11-14 at 11.28.42\u{202f}PM.png",
                title: "",
            }
        }
        p {
            "Similarly, with  "
            code { "Link" }
            " ,  "
            code { "Stylesheet" }
            " , and  "
            code { "Style" }
            ", you can include elements that automatically get merged into the document’s  "
            code { "<head>" }
            " element. During server side rendering, these links get collected, deduplicated, and minified. With these built-in  "
            code { "document" }
            " components, you’re now guaranteed that your  "
            code { "<head>" }
            " element is properly set for pre-loading heavy assets like stylesheets and external JavaScript."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Title {{ </span><span style=\"color:#ffee99;\">&quot;WebAssembly rocks!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        Stylesheet {{ href: asset!(</span><span style=\"color:#ffee99;\">&quot;/assets/main.css&quot;</span><span style=\"color:#f8f8f2;\">) }}\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;A site dedicated to webassembly&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            img {
                src: "/assets/06assets/Screenshot_2024-11-14_at_11.31.18_PM.png",
                alt: "Screenshot 2024-11-14 at 11.31.18\u{202f}PM.png",
                title: "",
            }
        }
        h2 { id: "synchronous",
            a { href: "#synchronous", class: "header", "Synchronous " }
            code { "prevent_default" }
        }
        p {
            "In addition to being able to access the native event type, Dioxus 0.6 also makes all event handling synchronous. Previously, all event handling in Dioxus had to occur outside the normal browser event handling flow to support platforms like  "
            code { "dioxus-desktop" }
            " which need to communicate over an interprocess communication (IPC) layer with the host webview. With this release, we’ve finally figured out how to enable blocking communication for  "
            code { "dioxus-desktop" }
            " and can finally make event handling synchronous!"
        }
        p {
            "As such, we no longer need the special  "
            code { "dioxus_prevent_default" }
            " attribute and you can directly call  "
            code { "event.prevent_default()" }
            "."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Form</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// we no longer need this!\n</span><span style=\"color:#f8f8f2;\">            dioxus_prevent_default: </span><span style=\"color:#ffee99;\">&quot;onclick&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// instead we can just call .prevent_default()\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">evt</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                evt.</span><span style=\"color:#66d9ef;\">prevent_default</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">                todos.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">remove</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">id);\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This now makes it possible to implement  "
            code { "prevent_default" }
            " conditionally which has previously been a limitation with Dioxus. Components like  "
            code { "Link {{}}" }
            " now exhibit behavior exactly aligned with their native counterparts, solving long-standing issues with Dioxus apps."
        }
        h2 { id: "tracking-size-with",
            a { href: "#tracking-size-with", class: "header", "Tracking size with " }
            code { "onresize" }
        }
        p {
            "Thanks to the community, we now have two special handlers "
            em { "not" }
            " found in the HTML spec: "
            code { "onvisible" }
            " and "
            code { "onresize" }
            ". These handlers are “special” dioxus handlers that automatically set up an "
            code { "IntersectionObserver" }
            " which previously required JavaScript."
        }
        p { "You can now implement particularly rich interactions with little hassle:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> items </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">100</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;A site dedicated to webassembly&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Adding a value will cause the `div` to be re-rendered with an extra div\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> items </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Add one&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// This will be called when the `div` is resized\n</span><span style=\"color:#f8f8f2;\">            onresize: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">data</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                tracing::info</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;resized to {{:#?}}&quot;</span><span style=\"color:#f8f8f2;\">, data.</span><span style=\"color:#66d9ef;\">get_border_box_size</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> x </span><span style=\"color:#f92672;\">in </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">..</span><span style=\"color:#66d9ef;\">items</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">                div {{ </span><span style=\"color:#ffee99;\">&quot;{{x}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "tracking-visibility-with",
            a { href: "#tracking-visibility-with", class: "header", "Tracking visibility with " }
            code { "onvisible" }
        }
        p {
            "In addition to  "
            code { "onresize" }
            ", we now have a special handler "
            em { "not" }
            " found in the HTML spec: "
            code { "onvisible" }
            "."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            onvisible: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">data</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                println!(</span><span style=\"color:#ffee99;\">&quot;visible!&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello world!&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This makes it possible to add rich animations to your app without needing to write custom JavaScript."
        }
        p {
            img {
                src: "/assets/06assets/onvisible.mp4",
                alt: "gif_of_visible_working.mp4",
                title: "",
            }
        }
        h2 { id: "hybrid-wgpu-overlays",
            a { href: "#hybrid-wgpu-overlays", class: "header", "Hybrid WGPU Overlays" }
        }
        p {
            img {
                src: "/assets/06assets/wgpu-windows.mp4",
                alt: "wgpu-windows.mp4",
                title: "",
            }
        }
        h2 { id: "web-ios-and-android-bundle-support",
            a { href: "#web-ios-and-android-bundle-support", class: "header",
                "Web, iOS, and Android bundle support"
            }
        }
        p {
            "With this release, we added support for web and mobile with  "
            code { "dx bundle" }
            ". Previously,  "
            code { "dx bundle" }
            " only worked for desktop apps. Now you can bundle for a wide variety of targets:"
        }
        ul {
            li { "macOS (.app, .dmg)" }
            li { "Windows (.exe, .msi)" }
            li { "Linux (.deb, .rpm, .appimage)" }
            li { "Android (.apk)" }
            li { "iOS (.ipa, .app)" }
            li { "Web (.appimage, /public folder)" }
        }
        h2 { id: "json-output-for-ci--cli",
            a { href: "#json-output-for-ci--cli", class: "header", "JSON Output for CI / CLI" }
        }
        p {
            "As part of our overhaul with the CLI, we’re also shipping a  "
            code { "json-output" }
            " mode. Now, when you pass  "
            code { "--json-output" }
            " to Dioxus commands, you will receive the logging in json format:"
        }
        p {
            img {
                src: "/assets/06assets/Screenshot_2024-11-14_at_10.38.33_PM.png",
                alt: "Screenshot 2024-11-14 at 10.38.33\u{202f}PM.png",
                title: "",
            }
        }
        p {
            "This is particularly important for users of  "
            code { "dx bundle" }
            " who want to automatically upload the their bundles to their hosting provider of choice. You can easily combine the output of  "
            code { "dx" }
            " with a tool like  "
            code { "jq" }
            " to extract important information like bundle outputs with a simple one-liner:"
        }
        p {
            img {
                src: "/assets/06assets/Screenshot_2024-11-14_at_10.40.56_PM.png",
                alt: "Screenshot 2024-11-14 at 10.40.56\u{202f}PM.png",
                title: "",
            }
        }
        h2 { id: "new-starter-templates",
            a { href: "#new-starter-templates", class: "header", "New Starter Templates" }
        }
        p {
            "Dioxus 0.6 ships with three new starter templates for cross-platform apps. Each template is a fully-featured, production-ready app that you can use as a starting point for your own Dioxus apps."
        }
        ul {
            li { "Bare-Bones: A bare-bones starter template with no styling, assets, or structure." }
            li { "Jumpstart: A starter template with a basic structure, components, and a few pages." }
            li { "Workspace: A starter template with separate crates for web, desktop, and mobile." }
        }
        p {
            "These are baked directly into the  "
            code { "dx new" }
            " command - simply run  "
            code { "dx new" }
            " and follow the prompts to select the template you want."
        }
        h2 { id: "nightly-docs-tutorials-and-new-guides",
            a {
                href: "#nightly-docs-tutorials-and-new-guides",
                class: "header",
                "Nightly Docs, Tutorials, and New Guides"
            }
        }
        p {
            "As usual with these large release, Dioxus 0.6 features a rather sizable overhaul to the documentation. We’ve completely overhauled the tutorial to be less heavy on code. The new tutorial focuses on basics like including assets and deploying to production."
        }
        p {
            img {
                src: "/assets/06assets/Screenshot_2024-11-14_at_11.35.23_PM.png",
                alt: "Screenshot 2024-11-14 at 11.35.23\u{202f}PM.png",
                title: "",
            }
        }
        p {
            "The docsite now includes all “modern” versions of Dioxus inline: 0.3, 0.4, 0.5, and 0.6 are all accessible under the same top-level website. Previously, we linked out to different MDbooks which eventually became a hassle. Now, you can simply switch between each version inline:"
        }
        p {
            img {
                src: "/assets/06assets/version_switch_shadow.png",
                alt: "Screenshot 2024-11-15 at 1.02.23\u{202f}AM.png",
                title: "",
            }
        }
        p {
            "The inline version switcher means we’ll now be able to publish documentation for alpha releases of Dioxus, hopefully making your life easier as we ship new features for the future. The new docs also feature small quality-of-life upgrades like breadcrumbs:"
        }
        p {
            img {
                src: "/assets/06assets/breadcrumbs_shadow.png",
                alt: "Screenshot 2024-11-15 at 1.04.13\u{202f}AM.png",
                title: "",
            }
        }
        p { "as well as new codeblocks with interactive examples:" }
        p {
            img {
                src: "/assets/06assets/interacitve_widget_shadow.png",
                alt: "Screenshot 2024-11-15 at 1.05.03\u{202f}AM.png",
                title: "",
            }
        }
        h2 { id: "preview-of-in-place-binary-patching",
            a { href: "#preview-of-in-place-binary-patching", class: "header",
                "Preview of In-Place Binary Patching"
            }
        }
        p {
            "While working on the new hot-reloading engine, we experimented with adding hot-reloading to Dioxus apps. The work here was inspired by Andrew Kelley’s “in-place-binary-patching” goal for Zig. Unfortunately, we didn’t have a chance to productionize the prototype for this release (way too many features already!) but we did put together a "
            a { href: "http://github.com/jkelleyrtp/ipbp", "small prototype" }
            ":"
        }
        p {
            img {
                src: "/assets/06assets/full_hr_dioxus_fast.mp4",
                alt: "full_hr_dioxus_fast.mp4",
                title: "",
            }
        }
        p {
            "We likely won’t have the time to ship true Rust hot-reloading in 0.7, but stay tuned for early next year!"
        }
        h2 { id: "smaller-changes",
            a { href: "#smaller-changes", class: "header", "Smaller changes:" }
        }
        p {
            "Not every change gets a particularly large section in the release notes, but we did land several new features and refactors."
        }
        ul {
            li {
                "System tray support: we now have proper support for System Trays again, thanks to a wonderful community contribution."
            }
            li {
                "Custom event loops: you can provide your own event loop, making it possible to use Dioxus in contexts where you already have other windows."
            }
            li {
                code { "dioxus-document" }
                ": we split out our "
                code { "document" }
                " abstraction so any renderer can implement the "
                code { "Document" }
                " trait to integrate with "
                code { "Title {{}}" }
                ", "
                code { "Script {{}}" }
                " , and "
                code { "eval" }
            }
            li {
                code { "dioxus-history" }
                ": we also split out our "
                code { "history" }
                " abstraction so other renderers can benefit from "
                code { "Link" }
                " and "
                code { "Router" }
                " without needing a dedicated feature flag on "
                code { "dioxus-router" }
            }
            li {
                code { "eval" }
                " API was simplified to allow "
                code { ".recv::<T>().await" }
                " on evals, making interoperating with JavaScript easier."
            }
            li {
                code { "dx fmt" }
                " now supports "
                code { "#[rustfmt::skip]" }
                " attributes, respects "
                code { "rustfmt.toml" }
                " settings, and is generally more reliable"
            }
        }
        h2 { id: "upgrading-from-05-to-06",
            a { href: "#upgrading-from-05-to-06", class: "header", "Upgrading from 0.5 to 0.6" }
        }
        p {
            "Generally there are few huge breaking changes in this release. However, we did change a few APIs that might break your existing apps but are easy to fix."
        }
        ul {
            li {
                code { "asset!()" }
                " syntax changes"
            }
            li {
                code { "eval()" }
                " API small changes"
            }
            li {
                "migrating to "
                code { "prevent_default()" }
            }
            li {
                "migrating from VNode::None to "
                code { "rsx! {{}}" }
                " for empty nodes"
            }
        }
        p { "We’ve assembled a migration guide here to help." }
        h2 { id: "conclusion",
            a { href: "#conclusion", class: "header", "Conclusion" }
        }
        p {
            "That’s it for this release! Due to the sheer size of this release, we might have missed a features and bug fixes. Countless issues including bundling bugs, spurious hot-reloads, and compatibility with unusual platforms and editors has been addressed."
        }
        p {
            "Dioxus 0.6 has been in alpha for quite a while, and we’re very thankful for all the testing the community has done to make this the most polished release yet. It’s quite difficult to run a large open source project such a wide scope. This release took "
            em { "much" }
            " longer to get out than we wanted - consuming two release cycles instead of just one."
        }
        p {
            "We focused hard this release to polish up as many rough edges as possible. Our continuous integration and deployment is in a much nicer place. We’re finally able to release nightly versions of documentation and the alpha release system has worked well for users eager to test out new features and bug fixes."
        }
        p {
            "Unfortunately, this release contained many connected pieces which made it hard to release incrementally. Systems like assets integrate tightly with CLI tooling and cross-platform support: to get one configuration right you need to test them all. With 0.6 behind us, the future seems much more “incremental” which should let us release major versions with faster cadence."
        }
        p {
            "We plan to keep 0.6 around for a while. Instead of shipping new features for a while, we're excited to make tutorial videos, write documentation, fix bugs, improve performance, and work with the community. The Dioxus team wants to spend time building our own apps!"
        }
        p { "That being said, we do have a few major items planned for Dioxus 0.7 and beyond:" }
        ul {
            li { "Rust hot-reloading with binary patching" }
            li { "Integrating wasm bundle splitting with the router" }
            li {
                code { "dx deploy" }
                " to a hosted deploy platform (Fly.io, AWS, Cloudflare, etc.)"
            }
        }
        p {
            "We’re also hiring - if you want to come build Dioxus with me in San Francisco (or remote) please reach out!"
        }
        h2 { id: "thanks-to-the-community",
            a { href: "#thanks-to-the-community", class: "header", "Thanks to the community!" }
        }
        p {
            "I want to extend a huge thank-you to everyone who helped test and improve this release. We saw an incredible number of contributors fix bugs and add features. Special thanks to:"
        }
        p {
            a { href: "https://github.com/ASR-ASU", "@ASR-ASU" }
            a { href: "https://github.com/Aandreba", "@Aandreba" }
            a { href: "https://github.com/Andrew15-5", "@Andrew15-5" }
            a { href: "https://github.com/DogeDark", "@DogeDark" }
            a { href: "https://github.com/Klemen2", "@Klemen2" }
            a { href: "https://github.com/LeWimbes", "@LeWimbes" }
            a { href: "https://github.com/LeoDog896", "@LeoDog896" }
            a { href: "https://github.com/MrGVSV", "@MrGVSV" }
            a { href: "https://github.com/Rahul721999", "@Rahul721999" }
            a { href: "https://github.com/Septimus", "@Septimus" }
            a { href: "https://github.com/Tahinli", "@Tahinli" }
            a { href: "https://github.com/WilliamRagstad", "@WilliamRagstad" }
            a { href: "https://github.com/ahqsoftwares", "@ahqsoftwares" }
            a { href: "https://github.com/airblast-dev", "@airblast-dev" }
            a { href: "https://github.com/alilosoft", "@alilosoft" }
            a { href: "https://github.com/azamara", "@azamara" }
            a { href: "https://github.com/chungwong", "@chungwong" }
            a { href: "https://github.com/d3rpp", "@d3rpp" }
            a { href: "https://github.com/daixiwen", "@daixiwen" }
            a { href: "https://github.com/dependabot", "@dependabot" }
            a { href: "https://github.com/ealmloff", "@ealmloff" }
            a { href: "https://github.com/hackartists", "@hackartists" }
            a { href: "https://github.com/hardBSDk", "@hardBSDk" }
            a { href: "https://github.com/houseme", "@houseme" }
            a { href: "https://github.com/i123iu", "@i123iu" }
            a { href: "https://github.com/ilaborie", "@ilaborie" }
            a { href: "https://github.com/imgurbot12", "@imgurbot12" }
            a { href: "https://github.com/jacklund", "@jacklund" }
            a { href: "https://github.com/jingchanglu", "@jingchanglu" }
            a { href: "https://github.com/luveti", "@luveti" }
            a { href: "https://github.com/marc2332", "@marc2332" }
            a { href: "https://github.com/matthunz", "@matthunz" }
            a { href: "https://github.com/nayo0513", "@nayo0513" }
            a { href: "https://github.com/opensource-inemar-net", "@opensource-inemar-net" }
            a { href: "https://github.com/oskardotglobal", "@oskardotglobal" }
            a { href: "https://github.com/panglars", "@panglars" }
            a { href: "https://github.com/pyrrho", "@pyrrho" }
            a { href: "https://github.com/ribelo", "@ribelo" }
            a { href: "https://github.com/rogusdev", "@rogusdev" }
            a { href: "https://github.com/ryo33", "@ryo33" }
            a { href: "https://github.com/samtay", "@samtay" }
            a { href: "https://github.com/sknauff", "@sknauff" }
            a { href: "https://github.com/srid", "@srid" }
            a { href: "https://github.com/tigerros", "@tigerros" }
            a { href: "https://github.com/tpoliaw", "@tpoliaw" }
            a { href: "https://github.com/uzytkownik", "@uzytkownik" }
        }
    }
}

use super::*;
