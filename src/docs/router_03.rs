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
    #[route("/#:section")]
    Index { section: IndexSection },
    #[route("/getting_started/#:section")]
    GettingStartedIndex { section: GettingStartedIndexSection },
    #[route("/getting_started/desktop#:section")]
    GettingStartedDesktop {
        section: GettingStartedDesktopSection,
    },
    #[route("/getting_started/web#:section")]
    GettingStartedWeb { section: GettingStartedWebSection },
    #[route("/getting_started/hot_reload#:section")]
    GettingStartedHotReload {
        section: GettingStartedHotReloadSection,
    },
    #[route("/getting_started/ssr#:section")]
    GettingStartedSsr { section: GettingStartedSsrSection },
    #[route("/getting_started/liveview#:section")]
    GettingStartedLiveview {
        section: GettingStartedLiveviewSection,
    },
    #[route("/getting_started/tui#:section")]
    GettingStartedTui { section: GettingStartedTuiSection },
    #[route("/getting_started/mobile#:section")]
    GettingStartedMobile {
        section: GettingStartedMobileSection,
    },
    #[route("/describing_ui/#:section")]
    DescribingUiIndex { section: DescribingUiIndexSection },
    #[route("/describing_ui/special_attributes#:section")]
    DescribingUiSpecialAttributes {
        section: DescribingUiSpecialAttributesSection,
    },
    #[route("/describing_ui/components#:section")]
    DescribingUiComponents {
        section: DescribingUiComponentsSection,
    },
    #[route("/describing_ui/component_props#:section")]
    DescribingUiComponentProps {
        section: DescribingUiComponentPropsSection,
    },
    #[route("/describing_ui/component_children#:section")]
    DescribingUiComponentChildren {
        section: DescribingUiComponentChildrenSection,
    },
    #[route("/interactivity/#:section")]
    InteractivityIndex { section: InteractivityIndexSection },
    #[route("/interactivity/event_handlers#:section")]
    InteractivityEventHandlers {
        section: InteractivityEventHandlersSection,
    },
    #[route("/interactivity/hooks#:section")]
    InteractivityHooks { section: InteractivityHooksSection },
    #[route("/interactivity/user_input#:section")]
    InteractivityUserInput {
        section: InteractivityUserInputSection,
    },
    #[route("/interactivity/sharing_state#:section")]
    InteractivitySharingState {
        section: InteractivitySharingStateSection,
    },
    #[route("/interactivity/custom_hooks#:section")]
    InteractivityCustomHooks {
        section: InteractivityCustomHooksSection,
    },
    #[route("/interactivity/dynamic_rendering#:section")]
    InteractivityDynamicRendering {
        section: InteractivityDynamicRenderingSection,
    },
    #[route("/interactivity/router#:section")]
    InteractivityRouter { section: InteractivityRouterSection },
    #[route("/async/#:section")]
    AsyncIndex { section: AsyncIndexSection },
    #[route("/async/use_future#:section")]
    AsyncUseFuture { section: AsyncUseFutureSection },
    #[route("/async/use_coroutine#:section")]
    AsyncUseCoroutine { section: AsyncUseCoroutineSection },
    #[route("/async/spawn#:section")]
    AsyncSpawn { section: AsyncSpawnSection },
    #[route("/best_practices/#:section")]
    BestPracticesIndex { section: BestPracticesIndexSection },
    #[route("/best_practices/error_handling#:section")]
    BestPracticesErrorHandling {
        section: BestPracticesErrorHandlingSection,
    },
    #[route("/best_practices/antipatterns#:section")]
    BestPracticesAntipatterns {
        section: BestPracticesAntipatternsSection,
    },
    #[route("/publishing/#:section")]
    PublishingIndex { section: PublishingIndexSection },
    #[route("/publishing/desktop#:section")]
    PublishingDesktop { section: PublishingDesktopSection },
    #[route("/publishing/web#:section")]
    PublishingWeb { section: PublishingWebSection },
    #[route("/custom_renderer/#:section")]
    CustomRendererIndex { section: CustomRendererIndexSection },
    #[route("/roadmap#:section")]
    Roadmap { section: RoadmapSection },
    #[route("/contributing#:section")]
    Contributing { section: ContributingSection },
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
            BookRoute::Index { .. } => use_mdbook::mdbook_shared::PageId(0usize),
            BookRoute::GettingStartedIndex { .. } => use_mdbook::mdbook_shared::PageId(1usize),
            BookRoute::GettingStartedDesktop { .. } => use_mdbook::mdbook_shared::PageId(2usize),
            BookRoute::GettingStartedWeb { .. } => use_mdbook::mdbook_shared::PageId(3usize),
            BookRoute::GettingStartedHotReload { .. } => use_mdbook::mdbook_shared::PageId(4usize),
            BookRoute::GettingStartedSsr { .. } => use_mdbook::mdbook_shared::PageId(5usize),
            BookRoute::GettingStartedLiveview { .. } => use_mdbook::mdbook_shared::PageId(6usize),
            BookRoute::GettingStartedTui { .. } => use_mdbook::mdbook_shared::PageId(7usize),
            BookRoute::GettingStartedMobile { .. } => use_mdbook::mdbook_shared::PageId(8usize),
            BookRoute::DescribingUiIndex { .. } => use_mdbook::mdbook_shared::PageId(9usize),
            BookRoute::DescribingUiSpecialAttributes { .. } => {
                use_mdbook::mdbook_shared::PageId(10usize)
            }
            BookRoute::DescribingUiComponents { .. } => use_mdbook::mdbook_shared::PageId(11usize),
            BookRoute::DescribingUiComponentProps { .. } => {
                use_mdbook::mdbook_shared::PageId(12usize)
            }
            BookRoute::DescribingUiComponentChildren { .. } => {
                use_mdbook::mdbook_shared::PageId(13usize)
            }
            BookRoute::InteractivityIndex { .. } => use_mdbook::mdbook_shared::PageId(14usize),
            BookRoute::InteractivityEventHandlers { .. } => {
                use_mdbook::mdbook_shared::PageId(15usize)
            }
            BookRoute::InteractivityHooks { .. } => use_mdbook::mdbook_shared::PageId(16usize),
            BookRoute::InteractivityUserInput { .. } => use_mdbook::mdbook_shared::PageId(17usize),
            BookRoute::InteractivitySharingState { .. } => {
                use_mdbook::mdbook_shared::PageId(18usize)
            }
            BookRoute::InteractivityCustomHooks { .. } => {
                use_mdbook::mdbook_shared::PageId(19usize)
            }
            BookRoute::InteractivityDynamicRendering { .. } => {
                use_mdbook::mdbook_shared::PageId(20usize)
            }
            BookRoute::InteractivityRouter { .. } => use_mdbook::mdbook_shared::PageId(21usize),
            BookRoute::AsyncIndex { .. } => use_mdbook::mdbook_shared::PageId(22usize),
            BookRoute::AsyncUseFuture { .. } => use_mdbook::mdbook_shared::PageId(23usize),
            BookRoute::AsyncUseCoroutine { .. } => use_mdbook::mdbook_shared::PageId(24usize),
            BookRoute::AsyncSpawn { .. } => use_mdbook::mdbook_shared::PageId(25usize),
            BookRoute::BestPracticesIndex { .. } => use_mdbook::mdbook_shared::PageId(26usize),
            BookRoute::BestPracticesErrorHandling { .. } => {
                use_mdbook::mdbook_shared::PageId(27usize)
            }
            BookRoute::BestPracticesAntipatterns { .. } => {
                use_mdbook::mdbook_shared::PageId(28usize)
            }
            BookRoute::PublishingIndex { .. } => use_mdbook::mdbook_shared::PageId(29usize),
            BookRoute::PublishingDesktop { .. } => use_mdbook::mdbook_shared::PageId(30usize),
            BookRoute::PublishingWeb { .. } => use_mdbook::mdbook_shared::PageId(31usize),
            BookRoute::CustomRendererIndex { .. } => use_mdbook::mdbook_shared::PageId(32usize),
            BookRoute::Roadmap { .. } => use_mdbook::mdbook_shared::PageId(33usize),
            BookRoute::Contributing { .. } => use_mdbook::mdbook_shared::PageId(34usize),
        }
    }
}
impl Default for BookRoute {
    fn default() -> Self {
        BookRoute::Index {
            section: IndexSection::Empty,
        }
    }
}
pub static LAZY_BOOK: use_mdbook::Lazy<use_mdbook::mdbook_shared::MdBook<BookRoute>> =
    use_mdbook::Lazy::new(|| {
        let mut page_id_mapping = ::std::collections::HashMap::new();
        let mut pages = Vec::new();
        pages.push((0usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Introduction".to_string(),
                url: BookRoute::Index {
                    section: IndexSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Introduction".to_string(),
                        id: "introduction".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Features".to_string(),
                        id: "features".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Multiplatform".to_string(),
                        id: "multiplatform".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Stability".to_string(),
                        id: "stability".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(0usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::Index {
                section: IndexSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(0usize),
        );
        pages.push((1usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Getting Started".to_string(),
                url: BookRoute::GettingStartedIndex {
                    section: GettingStartedIndexSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Getting Started".to_string(),
                        id: "getting-started".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Prerequisites".to_string(),
                        id: "prerequisites".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "An Editor".to_string(),
                        id: "an-editor".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Rust".to_string(),
                        id: "rust".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Setup Guides".to_string(),
                        id: "setup-guides".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(1usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::GettingStartedIndex {
                section: GettingStartedIndexSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(1usize),
        );
        pages.push((2usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Desktop".to_string(),
                url: BookRoute::GettingStartedDesktop {
                    section: GettingStartedDesktopSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Desktop Overview".to_string(),
                        id: "desktop-overview".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Support".to_string(),
                        id: "support".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Getting started".to_string(),
                        id: "getting-started".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Platform-Specific Dependencies".to_string(),
                        id: "platform-specific-dependencies".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Windows".to_string(),
                        id: "windows".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Linux".to_string(),
                        id: "linux".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "MacOS".to_string(),
                        id: "macos".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Creating a Project".to_string(),
                        id: "creating-a-project".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(2usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::GettingStartedDesktop {
                section: GettingStartedDesktopSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(2usize),
        );
        pages.push((3usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Web".to_string(),
                url: BookRoute::GettingStartedWeb {
                    section: GettingStartedWebSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Web".to_string(),
                        id: "web".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Support".to_string(),
                        id: "support".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Tooling".to_string(),
                        id: "tooling".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Creating a Project".to_string(),
                        id: "creating-a-project".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(3usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::GettingStartedWeb {
                section: GettingStartedWebSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(3usize),
        );
        pages.push((4usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Hot Reload".to_string(),
                url: BookRoute::GettingStartedHotReload {
                    section: GettingStartedHotReloadSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Setting Up Hot Reload".to_string(),
                        id: "setting-up-hot-reload".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Setup".to_string(),
                        id: "setup".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Usage".to_string(),
                        id: "usage".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Limitations".to_string(),
                        id: "limitations".to_string(),
                        level: 1usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(4usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::GettingStartedHotReload {
                section: GettingStartedHotReloadSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(4usize),
        );
        pages.push((5usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Server-Side Rendering".to_string(),
                url: BookRoute::GettingStartedSsr {
                    section: GettingStartedSsrSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Server-Side Rendering".to_string(),
                        id: "server-side-rendering".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Multithreaded Support".to_string(),
                        id: "multithreaded-support".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Setup".to_string(),
                        id: "setup".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(5usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::GettingStartedSsr {
                section: GettingStartedSsrSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(5usize),
        );
        pages.push((6usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Liveview".to_string(),
                url: BookRoute::GettingStartedLiveview {
                    section: GettingStartedLiveviewSection::Empty,
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
                id: ::use_mdbook::mdbook_shared::PageId(6usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::GettingStartedLiveview {
                section: GettingStartedLiveviewSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(6usize),
        );
        pages.push((7usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Terminal UI".to_string(),
                url: BookRoute::GettingStartedTui {
                    section: GettingStartedTuiSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Terminal UI".to_string(),
                        id: "terminal-ui".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Support".to_string(),
                        id: "support".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Getting Set up".to_string(),
                        id: "getting-set-up".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(7usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::GettingStartedTui {
                section: GettingStartedTuiSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(7usize),
        );
        pages.push((8usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Mobile".to_string(),
                url: BookRoute::GettingStartedMobile {
                    section: GettingStartedMobileSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Mobile App".to_string(),
                        id: "mobile-app".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Support".to_string(),
                        id: "support".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Getting Set up".to_string(),
                        id: "getting-set-up".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(8usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::GettingStartedMobile {
                section: GettingStartedMobileSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(8usize),
        );
        pages.push((9usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Describing the UI".to_string(),
                url: BookRoute::DescribingUiIndex {
                    section: DescribingUiIndexSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Describing the UI".to_string(),
                        id: "describing-the-ui".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "RSX Features".to_string(),
                        id: "rsx-features".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Attributes".to_string(),
                        id: "attributes".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Custom Attributes".to_string(),
                        id: "custom-attributes".to_string(),
                        level: 4usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Interpolation".to_string(),
                        id: "interpolation".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Children".to_string(),
                        id: "children".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Fragments".to_string(),
                        id: "fragments".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Expressions".to_string(),
                        id: "expressions".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Loops".to_string(),
                        id: "loops".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "If statements".to_string(),
                        id: "if-statements".to_string(),
                        level: 3usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(9usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::DescribingUiIndex {
                section: DescribingUiIndexSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(9usize),
        );
        pages.push((10usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Special Attributes".to_string(),
                url: BookRoute::DescribingUiSpecialAttributes {
                    section: DescribingUiSpecialAttributesSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Special Attributes".to_string(),
                        id: "special-attributes".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "The HTML Escape Hatch".to_string(),
                        id: "the-html-escape-hatch".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Boolean Attributes".to_string(),
                        id: "boolean-attributes".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(10usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::DescribingUiSpecialAttributes {
                section: DescribingUiSpecialAttributesSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(10usize),
        );
        pages.push((11usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Components".to_string(),
                url: BookRoute::DescribingUiComponents {
                    section: DescribingUiComponentsSection::Empty,
                },
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "Components".to_string(),
                    id: "components".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(11usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::DescribingUiComponents {
                section: DescribingUiComponentsSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(11usize),
        );
        pages.push((12usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Props".to_string(),
                url: BookRoute::DescribingUiComponentProps {
                    section: DescribingUiComponentPropsSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Component Props".to_string(),
                        id: "component-props".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "derive(Props)".to_string(),
                        id: "derive(props)".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Owned Props".to_string(),
                        id: "owned-props".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Borrowed Props".to_string(),
                        id: "borrowed-props".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Prop Options".to_string(),
                        id: "prop-options".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Optional Props".to_string(),
                        id: "optional-props".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Explicitly Required ".to_string(),
                        id: "explicitly-required".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Default Props".to_string(),
                        id: "default-props".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Automatic Conversion with ".to_string(),
                        id: "automatic-conversion-with".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "The inline_props macro".to_string(),
                        id: "the-inline_props-macro".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(12usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::DescribingUiComponentProps {
                section: DescribingUiComponentPropsSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(12usize),
        );
        pages.push((13usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Component Children".to_string(),
                url: BookRoute::DescribingUiComponentChildren {
                    section: DescribingUiComponentChildrenSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Component Children".to_string(),
                        id: "component-children".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "The children field".to_string(),
                        id: "the-children-field".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(13usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::DescribingUiComponentChildren {
                section: DescribingUiComponentChildrenSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(13usize),
        );
        pages.push((14usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Interactivity".to_string(),
                url: BookRoute::InteractivityIndex {
                    section: InteractivityIndexSection::Empty,
                },
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "Interactivity".to_string(),
                    id: "interactivity".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(14usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::InteractivityIndex {
                section: InteractivityIndexSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(14usize),
        );
        pages.push((15usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Event Listeners".to_string(),
                url: BookRoute::InteractivityEventHandlers {
                    section: InteractivityEventHandlersSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Event Handlers".to_string(),
                        id: "event-handlers".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "The Event object".to_string(),
                        id: "the-event-object".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Event propagation".to_string(),
                        id: "event-propagation".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Prevent Default".to_string(),
                        id: "prevent-default".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Handler Props".to_string(),
                        id: "handler-props".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(15usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::InteractivityEventHandlers {
                section: InteractivityEventHandlersSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(15usize),
        );
        pages.push((16usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Hooks & Component State".to_string(),
                url: BookRoute::InteractivityHooks {
                    section: InteractivityHooksSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Hooks and Component State".to_string(),
                        id: "hooks-and-component-state".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "use_state Hook".to_string(),
                        id: "use_state-hook".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Rules of Hooks".to_string(),
                        id: "rules-of-hooks".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "No Hooks in Conditionals".to_string(),
                        id: "no-hooks-in-conditionals".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "No Hooks in Closures".to_string(),
                        id: "no-hooks-in-closures".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "No Hooks in Loops".to_string(),
                        id: "no-hooks-in-loops".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "use_ref Hook".to_string(),
                        id: "use_ref-hook".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(16usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::InteractivityHooks {
                section: InteractivityHooksSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(16usize),
        );
        pages.push((17usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "User Input".to_string(),
                url: BookRoute::InteractivityUserInput {
                    section: InteractivityUserInputSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "User Input".to_string(),
                        id: "user-input".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Controlled Inputs".to_string(),
                        id: "controlled-inputs".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Uncontrolled Inputs".to_string(),
                        id: "uncontrolled-inputs".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(17usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::InteractivityUserInput {
                section: InteractivityUserInputSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(17usize),
        );
        pages.push((18usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Sharing State".to_string(),
                url: BookRoute::InteractivitySharingState {
                    section: InteractivitySharingStateSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Sharing State".to_string(),
                        id: "sharing-state".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Lifting State".to_string(),
                        id: "lifting-state".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Using Context".to_string(),
                        id: "using-context".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(18usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::InteractivitySharingState {
                section: InteractivitySharingStateSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(18usize),
        );
        pages.push((19usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Custom Hooks".to_string(),
                url: BookRoute::InteractivityCustomHooks {
                    section: InteractivityCustomHooksSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Custom Hooks".to_string(),
                        id: "custom-hooks".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Composing Hooks".to_string(),
                        id: "composing-hooks".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Custom Hook Logic".to_string(),
                        id: "custom-hook-logic".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(19usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::InteractivityCustomHooks {
                section: InteractivityCustomHooksSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(19usize),
        );
        pages.push((20usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Dynamic Rendering".to_string(),
                url: BookRoute::InteractivityDynamicRendering {
                    section: InteractivityDynamicRenderingSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Dynamic Rendering".to_string(),
                        id: "dynamic-rendering".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Conditional Rendering".to_string(),
                        id: "conditional-rendering".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Improving the ".to_string(),
                        id: "improving-the".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Inspecting ".to_string(),
                        id: "inspecting".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Rendering Nothing".to_string(),
                        id: "rendering-nothing".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Rendering Lists".to_string(),
                        id: "rendering-lists".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Inline for loops".to_string(),
                        id: "inline-for-loops".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "The key Attribute".to_string(),
                        id: "the-key-attribute".to_string(),
                        level: 3usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(20usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::InteractivityDynamicRendering {
                section: InteractivityDynamicRenderingSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(20usize),
        );
        pages.push((21usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Routing".to_string(),
                url: BookRoute::InteractivityRouter {
                    section: InteractivityRouterSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Router".to_string(),
                        id: "router".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "What is it?".to_string(),
                        id: "what-is-it?".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Using the router".to_string(),
                        id: "using-the-router".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Links".to_string(),
                        id: "links".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "More reading".to_string(),
                        id: "more-reading".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(21usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::InteractivityRouter {
                section: InteractivityRouterSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(21usize),
        );
        pages.push((22usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Async".to_string(),
                url: BookRoute::AsyncIndex {
                    section: AsyncIndexSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Working with Async".to_string(),
                        id: "working-with-async".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "The Runtime".to_string(),
                        id: "the-runtime".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(22usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::AsyncIndex {
                section: AsyncIndexSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(22usize),
        );
        pages.push((23usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "UseFuture".to_string(),
                url: BookRoute::AsyncUseFuture {
                    section: AsyncUseFutureSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "UseFuture".to_string(),
                        id: "usefuture".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Restarting the Future".to_string(),
                        id: "restarting-the-future".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Dependencies".to_string(),
                        id: "dependencies".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(23usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::AsyncUseFuture {
                section: AsyncUseFutureSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(23usize),
        );
        pages.push((24usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "UseCoroutine".to_string(),
                url: BookRoute::AsyncUseCoroutine {
                    section: AsyncUseCoroutineSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Coroutines".to_string(),
                        id: "coroutines".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "use_coroutine".to_string(),
                        id: "use_coroutine".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Yielding Values".to_string(),
                        id: "yielding-values".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Sending Values".to_string(),
                        id: "sending-values".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Automatic injection into the Context API".to_string(),
                        id: "automatic-injection-into-the-context-api".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(24usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::AsyncUseCoroutine {
                section: AsyncUseCoroutineSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(24usize),
        );
        pages.push((25usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Spawning Futures".to_string(),
                url: BookRoute::AsyncSpawn {
                    section: AsyncSpawnSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Spawning Futures".to_string(),
                        id: "spawning-futures".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Spawning Tokio Tasks".to_string(),
                        id: "spawning-tokio-tasks".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(25usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::AsyncSpawn {
                section: AsyncSpawnSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(25usize),
        );
        pages.push((26usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Best Practices".to_string(),
                url: BookRoute::BestPracticesIndex {
                    section: BestPracticesIndexSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Best Practices".to_string(),
                        id: "best-practices".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Reusable Components".to_string(),
                        id: "reusable-components".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Minimize State Dependencies".to_string(),
                        id: "minimize-state-dependencies".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(26usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::BestPracticesIndex {
                section: BestPracticesIndexSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(26usize),
        );
        pages.push((27usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Error Handling".to_string(),
                url: BookRoute::BestPracticesErrorHandling {
                    section: BestPracticesErrorHandlingSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Error handling".to_string(),
                        id: "error-handling".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "The simplest – returning None".to_string(),
                        id: "the-simplest-–-returning-none".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Early return on result".to_string(),
                        id: "early-return-on-result".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Match results".to_string(),
                        id: "match-results".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Passing error states through components".to_string(),
                        id: "passing-error-states-through-components".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Going global".to_string(),
                        id: "going-global".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(27usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::BestPracticesErrorHandling {
                section: BestPracticesErrorHandlingSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(27usize),
        );
        pages.push((28usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Antipatterns".to_string(),
                url: BookRoute::BestPracticesAntipatterns {
                    section: BestPracticesAntipatternsSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Antipatterns".to_string(),
                        id: "antipatterns".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Unnecessarily Nested Fragments".to_string(),
                        id: "unnecessarily-nested-fragments".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Incorrect Iterator Keys".to_string(),
                        id: "incorrect-iterator-keys".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Avoid Interior Mutability in Props".to_string(),
                        id: "avoid-interior-mutability-in-props".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Avoid Updating State During Render".to_string(),
                        id: "avoid-updating-state-during-render".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(28usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::BestPracticesAntipatterns {
                section: BestPracticesAntipatternsSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(28usize),
        );
        pages.push((29usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Publishing".to_string(),
                url: BookRoute::PublishingIndex {
                    section: PublishingIndexSection::Empty,
                },
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "Publishing".to_string(),
                    id: "publishing".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(29usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::PublishingIndex {
                section: PublishingIndexSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(29usize),
        );
        pages.push((30usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Desktop".to_string(),
                url: BookRoute::PublishingDesktop {
                    section: PublishingDesktopSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Publishing".to_string(),
                        id: "publishing".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Install ".to_string(),
                        id: "install".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Setting up your project".to_string(),
                        id: "setting-up-your-project".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Building".to_string(),
                        id: "building".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(30usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::PublishingDesktop {
                section: PublishingDesktopSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(30usize),
        );
        pages.push((31usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Web".to_string(),
                url: BookRoute::PublishingWeb {
                    section: PublishingWebSection::Empty,
                },
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "Publishing with Github Pages".to_string(),
                    id: "publishing-with-github-pages".to_string(),
                    level: 2usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(31usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::PublishingWeb {
                section: PublishingWebSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(31usize),
        );
        pages.push((32usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Custom Renderer".to_string(),
                url: BookRoute::CustomRendererIndex {
                    section: CustomRendererIndexSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Custom Renderer".to_string(),
                        id: "custom-renderer".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "The specifics:".to_string(),
                        id: "the-specifics:".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Templates".to_string(),
                        id: "templates".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Mutations".to_string(),
                        id: "mutations".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "An Example".to_string(),
                        id: "an-example".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Event loop".to_string(),
                        id: "event-loop".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Custom raw elements".to_string(),
                        id: "custom-raw-elements".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Native Core".to_string(),
                        id: "native-core".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "RealDom".to_string(),
                        id: "realdom".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Example".to_string(),
                        id: "example".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Layout".to_string(),
                        id: "layout".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Conclusion".to_string(),
                        id: "conclusion".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(32usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::CustomRendererIndex {
                section: CustomRendererIndexSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(32usize),
        );
        pages.push((33usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Roadmap".to_string(),
                url: BookRoute::Roadmap {
                    section: RoadmapSection::Empty,
                },
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
                id: ::use_mdbook::mdbook_shared::PageId(33usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::Roadmap {
                section: RoadmapSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(33usize),
        );
        pages.push((34usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Contributing".to_string(),
                url: BookRoute::Contributing {
                    section: ContributingSection::Empty,
                },
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Contributing".to_string(),
                        id: "contributing".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Improving Docs".to_string(),
                        id: "improving-docs".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Working on the Ecosystem".to_string(),
                        id: "working-on-the-ecosystem".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Bugs & Features".to_string(),
                        id: "bugs-&-features".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(34usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::Contributing {
                section: ContributingSection::Empty,
            },
            ::use_mdbook::mdbook_shared::PageId(34usize),
        );
        ::use_mdbook::mdbook_shared::MdBook {
            summary: ::use_mdbook::mdbook_shared::Summary {
                title: Some("Summary".to_string()),
                prefix_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Introduction".to_string(),
                        location: Some(BookRoute::Index {
                            section: IndexSection::Empty,
                        }),
                        number: None,
                        nested_items: vec![],
                    }),
                ],
                numbered_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Getting Started".to_string(),
                        location: Some(BookRoute::GettingStartedIndex {
                            section: GettingStartedIndexSection::Empty,
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Desktop".to_string(),
                                location: Some(BookRoute::GettingStartedDesktop {
                                    section: GettingStartedDesktopSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Web".to_string(),
                                location: Some(BookRoute::GettingStartedWeb {
                                    section: GettingStartedWebSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32, 2u32]),
                                ),
                                nested_items: vec![
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Hot Reload".to_string(),
                                        location: Some(BookRoute::GettingStartedHotReload {
                                            section: GettingStartedHotReloadSection::Empty,
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![1u32, 2u32, 1u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                ],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Server-Side Rendering".to_string(),
                                location: Some(BookRoute::GettingStartedSsr {
                                    section: GettingStartedSsrSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32, 3u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Liveview".to_string(),
                                location: Some(BookRoute::GettingStartedLiveview {
                                    section: GettingStartedLiveviewSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32, 4u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Terminal UI".to_string(),
                                location: Some(BookRoute::GettingStartedTui {
                                    section: GettingStartedTuiSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32, 5u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Mobile".to_string(),
                                location: Some(BookRoute::GettingStartedMobile {
                                    section: GettingStartedMobileSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32, 6u32]),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Describing the UI".to_string(),
                        location: Some(BookRoute::DescribingUiIndex {
                            section: DescribingUiIndexSection::Empty,
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Special Attributes".to_string(),
                                location: Some(BookRoute::DescribingUiSpecialAttributes {
                                    section: DescribingUiSpecialAttributesSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Components".to_string(),
                                location: Some(BookRoute::DescribingUiComponents {
                                    section: DescribingUiComponentsSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 2u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Props".to_string(),
                                location: Some(BookRoute::DescribingUiComponentProps {
                                    section: DescribingUiComponentPropsSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 3u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Component Children".to_string(),
                                location: Some(BookRoute::DescribingUiComponentChildren {
                                    section: DescribingUiComponentChildrenSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 4u32]),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Interactivity".to_string(),
                        location: Some(BookRoute::InteractivityIndex {
                            section: InteractivityIndexSection::Empty,
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Event Listeners".to_string(),
                                location: Some(BookRoute::InteractivityEventHandlers {
                                    section: InteractivityEventHandlersSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Hooks & Component State".to_string(),
                                location: Some(BookRoute::InteractivityHooks {
                                    section: InteractivityHooksSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 2u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "User Input".to_string(),
                                location: Some(BookRoute::InteractivityUserInput {
                                    section: InteractivityUserInputSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 3u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Sharing State".to_string(),
                                location: Some(BookRoute::InteractivitySharingState {
                                    section: InteractivitySharingStateSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 4u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Custom Hooks".to_string(),
                                location: Some(BookRoute::InteractivityCustomHooks {
                                    section: InteractivityCustomHooksSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 5u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Dynamic Rendering".to_string(),
                                location: Some(BookRoute::InteractivityDynamicRendering {
                                    section: InteractivityDynamicRenderingSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 6u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Routing".to_string(),
                                location: Some(BookRoute::InteractivityRouter {
                                    section: InteractivityRouterSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 7u32]),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Async".to_string(),
                        location: Some(BookRoute::AsyncIndex {
                            section: AsyncIndexSection::Empty,
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "UseFuture".to_string(),
                                location: Some(BookRoute::AsyncUseFuture {
                                    section: AsyncUseFutureSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "UseCoroutine".to_string(),
                                location: Some(BookRoute::AsyncUseCoroutine {
                                    section: AsyncUseCoroutineSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 2u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Spawning Futures".to_string(),
                                location: Some(BookRoute::AsyncSpawn {
                                    section: AsyncSpawnSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 3u32]),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Best Practices".to_string(),
                        location: Some(BookRoute::BestPracticesIndex {
                            section: BestPracticesIndexSection::Empty,
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Error Handling".to_string(),
                                location: Some(BookRoute::BestPracticesErrorHandling {
                                    section: BestPracticesErrorHandlingSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Antipatterns".to_string(),
                                location: Some(BookRoute::BestPracticesAntipatterns {
                                    section: BestPracticesAntipatternsSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32, 2u32]),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Publishing".to_string(),
                        location: Some(BookRoute::PublishingIndex {
                            section: PublishingIndexSection::Empty,
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![6u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Desktop".to_string(),
                                location: Some(BookRoute::PublishingDesktop {
                                    section: PublishingDesktopSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![6u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Web".to_string(),
                                location: Some(BookRoute::PublishingWeb {
                                    section: PublishingWebSection::Empty,
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![6u32, 2u32]),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Separator,
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Custom Renderer".to_string(),
                        location: Some(BookRoute::CustomRendererIndex {
                            section: CustomRendererIndexSection::Empty,
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![7u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Separator,
                ],
                suffix_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Roadmap".to_string(),
                        location: Some(BookRoute::Roadmap {
                            section: RoadmapSection::Empty,
                        }),
                        number: None,
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Contributing".to_string(),
                        location: Some(BookRoute::Contributing {
                            section: ContributingSection::Empty,
                        }),
                        number: None,
                        nested_items: vec![],
                    }),
                ],
            },
            pages: pages.into_iter().collect(),
            page_id_mapping,
        }
    });
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum IndexSection {
    #[default]
    Empty,
    Introduction,
    Features,
    Multiplatform,
    Stability,
}
impl std::str::FromStr for IndexSection {
    type Err = IndexSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "introduction" => Ok(Self::Introduction),
            "features" => Ok(Self::Features),
            "multiplatform" => Ok(Self::Multiplatform),
            "stability" => Ok(Self::Stability),
            _ => Err(IndexSectionParseError),
        }
    }
}
impl std::fmt::Display for IndexSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Introduction => f.write_str("introduction"),
            Self::Features => f.write_str("features"),
            Self::Multiplatform => f.write_str("multiplatform"),
            Self::Stability => f.write_str("stability"),
        }
    }
}
#[derive(Debug)]
pub struct IndexSectionParseError;
impl std::fmt::Display for IndexSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of IndexSectionintroduction, features, multiplatform, stability",
        )?;
        Ok(())
    }
}
impl std::error::Error for IndexSectionParseError {}
#[component(no_case_check)]
pub fn Index(section: IndexSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "introduction",
            Link {
                to: BookRoute::Index {
                    section: IndexSection::Introduction,
                },
                class: "header",
                "Introduction"
            }
        }
        p {
            img {
                src: asset!(
                    "/assets/static/images03/dioxuslogo_full.png", ImageAssetOptions::new()
                    .with_avif()
                ),
                alt: "dioxuslogo",
                title: "",
            }
        }
        p {
            "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust. This guide will help you get started with writing Dioxus apps for the Web, Desktop, Mobile, and more."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;High-Five counter: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Up high!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Down low!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    ))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Dioxus is heavily inspired by React. If you know React, getting started with Dioxus will be a breeze."
        }
        blockquote {
            p {
                "This guide assumes you already know some "
                Link { to: "https://www.rust-lang.org/", "Rust" }
                "! If not, we recommend reading "
                Link { to: "https://doc.rust-lang.org/book/ch01-00-getting-started.html",
                    em { "the book" }
                }
                " to learn Rust first."
            }
        }
        h2 { id: "features",
            Link {
                to: BookRoute::Index {
                    section: IndexSection::Features,
                },
                class: "header",
                "Features"
            }
        }
        ul {
            li { "Desktop apps running natively (no Electron!) in less than 10 lines of code." }
            li { "Incredibly ergonomic and powerful state management." }
            li {
                "Comprehensive inline documentation – hover and guides for all HTML elements, listeners, and events."
            }
            li { "Extremely memory efficient – 0 global allocations for steady-state components." }
            li { "Multi-channel asynchronous scheduler for first-class async support." }
            li {
                "And more! Read the "
                Link { to: "https://dioxuslabs.com/blog/introducing-dioxus/", "full release post" }
                "."
            }
        }
        h3 { id: "multiplatform",
            Link {
                to: BookRoute::Index {
                    section: IndexSection::Multiplatform,
                },
                class: "header",
                "Multiplatform"
            }
        }
        p {
            "Dioxus is a "
            em { "portable" }
            " toolkit, meaning the Core implementation can run anywhere with no platform-dependent linking. Unlike many other Rust frontend toolkits, Dioxus is not intrinsically linked to WebSys. In fact, every element and event listener can be swapped out at compile time. By default, Dioxus ships with the "
            code { "html" }
            " feature enabled, but this can be disabled depending on your target renderer."
        }
        p { "Right now, we have several 1st-party renderers:" }
        ul {
            li { "WebSys (for WASM): Great support" }
            li { "Tao/Tokio (for Desktop apps): Good support" }
            li { "Tao/Tokio (for Mobile apps): Poor support" }
            li { "SSR (for generating static markup)" }
            li { "TUI/Rink (for terminal-based apps): Experimental" }
        }
        h2 { id: "stability",
            Link {
                to: BookRoute::Index {
                    section: IndexSection::Stability,
                },
                class: "header",
                "Stability"
            }
        }
        p { "Dioxus has not reached a stable release yet." }
        p {
            "Web: Since the web is a fairly mature platform, we expect there to be very little API churn for web-based features."
        }
        p {
            "Desktop: APIs will likely be in flux as we figure out better patterns than our ElectronJS counterpart."
        }
        p { "SSR: We don't expect the SSR API to change drastically in the future." }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum GettingStartedIndexSection {
    #[default]
    Empty,
    GettingStarted,
    Prerequisites,
    AnEditor,
    Rust,
    SetupGuides,
}
impl std::str::FromStr for GettingStartedIndexSection {
    type Err = GettingStartedIndexSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "getting-started" => Ok(Self::GettingStarted),
            "prerequisites" => Ok(Self::Prerequisites),
            "an-editor" => Ok(Self::AnEditor),
            "rust" => Ok(Self::Rust),
            "setup-guides" => Ok(Self::SetupGuides),
            _ => Err(GettingStartedIndexSectionParseError),
        }
    }
}
impl std::fmt::Display for GettingStartedIndexSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::GettingStarted => f.write_str("getting-started"),
            Self::Prerequisites => f.write_str("prerequisites"),
            Self::AnEditor => f.write_str("an-editor"),
            Self::Rust => f.write_str("rust"),
            Self::SetupGuides => f.write_str("setup-guides"),
        }
    }
}
#[derive(Debug)]
pub struct GettingStartedIndexSectionParseError;
impl std::fmt::Display for GettingStartedIndexSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of GettingStartedIndexSectiongetting-started, prerequisites, an-editor, rust, setup-guides",
        )?;
        Ok(())
    }
}
impl std::error::Error for GettingStartedIndexSectionParseError {}
#[component(no_case_check)]
pub fn GettingStartedIndex(section: GettingStartedIndexSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "getting-started",
            Link {
                to: BookRoute::GettingStartedIndex {
                    section: GettingStartedIndexSection::GettingStarted,
                },
                class: "header",
                "Getting Started"
            }
        }
        p { "This section will help you set up your Dioxus project!" }
        h2 { id: "prerequisites",
            Link {
                to: BookRoute::GettingStartedIndex {
                    section: GettingStartedIndexSection::Prerequisites,
                },
                class: "header",
                "Prerequisites"
            }
        }
        h3 { id: "an-editor",
            Link {
                to: BookRoute::GettingStartedIndex {
                    section: GettingStartedIndexSection::AnEditor,
                },
                class: "header",
                "An Editor"
            }
        }
        p {
            "Dioxus integrates very well with the "
            Link { to: "https://rust-analyzer.github.io", "Rust-Analyzer LSP plugin" }
            " which will provide appropriate syntax highlighting, code navigation, folding, and more."
        }
        h3 { id: "rust",
            Link {
                to: BookRoute::GettingStartedIndex {
                    section: GettingStartedIndexSection::Rust,
                },
                class: "header",
                "Rust"
            }
        }
        p {
            "Head over to "
            Link { to: "http://rust-lang.org", "https://rust-lang.org" }
            " and install the Rust compiler."
        }
        p {
            "We strongly recommend going through the "
            Link { to: "https://doc.rust-lang.org/book/ch01-00-getting-started.html",
                "official Rust book"
            }
            " "
            em { "completely" }
            ". However, we hope that a Dioxus app can serve as a great first Rust project. With Dioxus, you'll learn about:"
        }
        ul {
            li { "Error handling" }
            li { "Structs, Functions, Enums" }
            li { "Closures" }
            li { "Macros" }
        }
        p {
            "We've put a lot of care into making Dioxus syntax familiar and easy to understand, so you won't need deep knowledge of async, lifetimes, or smart pointers until you start building complex Dioxus apps."
        }
        h2 { id: "setup-guides",
            Link {
                to: BookRoute::GettingStartedIndex {
                    section: GettingStartedIndexSection::SetupGuides,
                },
                class: "header",
                "Setup Guides"
            }
        }
        p {
            "Dioxus supports multiple platforms. Choose the platform you want to target below to get platform-specific setup instructions:"
        }
        ul {
            li {
                Link {
                    to: BookRoute::GettingStartedWeb {
                        section: GettingStartedWebSection::Empty,
                    },
                    "Web"
                }
                ": runs in the browser through WebAssembly"
            }
            li {
                Link {
                    to: BookRoute::GettingStartedSsr {
                        section: GettingStartedSsrSection::Empty,
                    },
                    "Server Side Rendering"
                }
                ": renders to HTML text on the server"
            }
            li {
                Link {
                    to: BookRoute::GettingStartedLiveview {
                        section: GettingStartedLiveviewSection::Empty,
                    },
                    "Liveview"
                }
                ": runs on the server, renders in the browser using WebSockets"
            }
            li {
                Link {
                    to: BookRoute::GettingStartedDesktop {
                        section: GettingStartedDesktopSection::Empty,
                    },
                    "Desktop"
                }
                ": runs in a web view on desktop"
            }
            li {
                Link {
                    to: BookRoute::GettingStartedMobile {
                        section: GettingStartedMobileSection::Empty,
                    },
                    "Mobile"
                }
                ": runs in a web view on mobile"
            }
            li {
                Link {
                    to: BookRoute::GettingStartedTui {
                        section: GettingStartedTuiSection::Empty,
                    },
                    "Terminal UI"
                }
                ": renders text-based graphics in the terminal"
            }
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum GettingStartedDesktopSection {
    #[default]
    Empty,
    DesktopOverview,
    Support,
    GettingStarted,
    PlatformSpecificDependencies,
    Windows,
    Linux,
    Macos,
    CreatingAProject,
}
impl std::str::FromStr for GettingStartedDesktopSection {
    type Err = GettingStartedDesktopSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "desktop-overview" => Ok(Self::DesktopOverview),
            "support" => Ok(Self::Support),
            "getting-started" => Ok(Self::GettingStarted),
            "platform-specific-dependencies" => Ok(Self::PlatformSpecificDependencies),
            "windows" => Ok(Self::Windows),
            "linux" => Ok(Self::Linux),
            "macos" => Ok(Self::Macos),
            "creating-a-project" => Ok(Self::CreatingAProject),
            _ => Err(GettingStartedDesktopSectionParseError),
        }
    }
}
impl std::fmt::Display for GettingStartedDesktopSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::DesktopOverview => f.write_str("desktop-overview"),
            Self::Support => f.write_str("support"),
            Self::GettingStarted => f.write_str("getting-started"),
            Self::PlatformSpecificDependencies => f.write_str("platform-specific-dependencies"),
            Self::Windows => f.write_str("windows"),
            Self::Linux => f.write_str("linux"),
            Self::Macos => f.write_str("macos"),
            Self::CreatingAProject => f.write_str("creating-a-project"),
        }
    }
}
#[derive(Debug)]
pub struct GettingStartedDesktopSectionParseError;
impl std::fmt::Display for GettingStartedDesktopSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of GettingStartedDesktopSectiondesktop-overview, support, getting-started, platform-specific-dependencies, windows, linux, macos, creating-a-project",
        )?;
        Ok(())
    }
}
impl std::error::Error for GettingStartedDesktopSectionParseError {}
#[component(no_case_check)]
pub fn GettingStartedDesktop(section: GettingStartedDesktopSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "desktop-overview",
            Link {
                to: BookRoute::GettingStartedDesktop {
                    section: GettingStartedDesktopSection::DesktopOverview,
                },
                class: "header",
                "Desktop Overview"
            }
        }
        p {
            "Build a standalone native desktop app that looks and feels the same across operating systems."
        }
        p {
            "Apps built with Dioxus are typically "
            "<"
            " "
            "5mb in size and use existing system resources, so they won't hog extreme amounts of RAM or memory."
        }
        p { "Examples:" }
        ul {
            li {
                Link { to: "https://github.com/DioxusLabs/example-projects/blob/master/file-explorer",
                    "File Explorer"
                }
            }
            li {
                Link { to: "https://github.com/DioxusLabs/example-projects/blob/master/wifi-scanner",
                    "WiFi Scanner"
                }
            }
        }
        p {
            Link { to: "https://github.com/DioxusLabs/example-projects/tree/master/file-explorer",
                img {
                    src: "https://github.com/DioxusLabs/example-projects/raw/master/file-explorer/assets/image.png",
                    alt: "File ExplorerExample",
                    title: "",
                }
            }
        }
        h2 { id: "support",
            Link {
                to: BookRoute::GettingStartedDesktop {
                    section: GettingStartedDesktopSection::Support,
                },
                class: "header",
                "Support"
            }
        }
        p {
            "The desktop is a powerful target for Dioxus but is currently limited in capability when compared to the Web platform. Currently, desktop apps are rendered with the platform's WebView library, but your Rust code is running natively on a native thread. This means that browser APIs are "
            em { "not" }
            " available, so rendering WebGL, Canvas, etc is not as easy as the Web. However, native system APIs "
            em { "are" }
            " accessible, so streaming, WebSockets, filesystem, etc are all viable APIs. In the future, we plan to move to a custom web renderer-based DOM renderer with WGPU integrations."
        }
        p {
            "Dioxus Desktop is built off "
            Link { to: "https://tauri.app/", "Tauri" }
            ". Right now there aren't any Dioxus abstractions over keyboard shortcuts, menubar, handling, etc, so you'll want to leverage Tauri – mostly "
            Link { to: "http://github.com/tauri-apps/wry/", "Wry" }
            " and "
            Link { to: "http://github.com/tauri-apps/tao", "Tao" }
            ") directly."
        }
        h1 { id: "getting-started",
            Link {
                to: BookRoute::GettingStartedDesktop {
                    section: GettingStartedDesktopSection::GettingStarted,
                },
                class: "header",
                "Getting started"
            }
        }
        h2 { id: "platform-specific-dependencies",
            Link {
                to: BookRoute::GettingStartedDesktop {
                    section: GettingStartedDesktopSection::PlatformSpecificDependencies,
                },
                class: "header",
                "Platform-Specific Dependencies"
            }
        }
        p {
            "Dioxus desktop renders through a web view. Depending on your platform, you might need to install some dependancies."
        }
        h3 { id: "windows",
            Link {
                to: BookRoute::GettingStartedDesktop {
                    section: GettingStartedDesktopSection::Windows,
                },
                class: "header",
                "Windows"
            }
        }
        p {
            "Windows Desktop apps depend on WebView2 – a library that should be installed in all modern Windows distributions. If you have Edge installed, then Dioxus will work fine. If you "
            em { "don't" }
            " have Webview2, "
            Link { to: "https://developer.microsoft.com/en-us/microsoft-edge/webview2/",
                "then you can install it through Microsoft"
            }
            ". MS provides 3 options:"
        }
        ol {
            li {
                "A tiny \"evergreen\" "
                em { "bootstrapper" }
                " that fetches an installer from Microsoft's CDN"
            }
            li {
                "A tiny "
                em { "installer" }
                " that fetches Webview2 from Microsoft's CDN"
            }
            li { "A statically linked version of Webview2 in your final binary for offline users" }
        }
        p { "For development purposes, use Option 1." }
        h3 { id: "linux",
            Link {
                to: BookRoute::GettingStartedDesktop {
                    section: GettingStartedDesktopSection::Linux,
                },
                class: "header",
                "Linux"
            }
        }
        p {
            "Webview Linux apps require WebkitGtk. When distributing, this can be part of your dependency tree in your  "
            code { ".rpm" }
            " or  "
            code { ".deb" }
            ". However, likely, your users will already have WebkitGtk."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">sudo apt install libwebkit2gtk</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">4.0</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libgtk</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">3</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libappindicator3</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev</span></pre>\n" }
        p {
            "When using Debian/bullseye  "
            code { "libappindicator3-dev" }
            " is no longer available but replaced by  "
            code { "libayatana-appindicator3-dev" }
            "."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> on Debian</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">bullseye </span><span style=\"color:#f92672;\">use</span><span style=\"color:#f8f8f2;\">:\n</span><span style=\"color:#f8f8f2;\">sudo apt install libwebkit2gtk</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">4.0</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libgtk</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">3</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libayatana</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">appindicator3</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev</span></pre>\n" }
        p {
            "If you run into issues, make sure you have all the basics installed, as outlined in the "
            Link { to: "https://tauri.studio/v1/guides/getting-started/prerequisites#setting-up-linux",
                "Tauri docs"
            }
            "."
        }
        h3 { id: "macos",
            Link {
                to: BookRoute::GettingStartedDesktop {
                    section: GettingStartedDesktopSection::Macos,
                },
                class: "header",
                "MacOS"
            }
        }
        p {
            "Currently – everything for macOS is built right in! However, you might run into an issue if you're using nightly Rust due to some permissions issues in our Tao dependency (which have been resolved but not published)."
        }
        h2 { id: "creating-a-project",
            Link {
                to: BookRoute::GettingStartedDesktop {
                    section: GettingStartedDesktopSection::CreatingAProject,
                },
                class: "header",
                "Creating a Project"
            }
        }
        p { "Create a new crate:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo new </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">bin demo\n</span><span style=\"color:#f8f8f2;\">cd demo</span></pre>\n" }
        p {
            "Add Dioxus and the desktop renderer as dependencies (this will edit your  "
            code { "Cargo.toml" }
            "):"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add dioxus\n</span><span style=\"color:#f8f8f2;\">cargo add dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">desktop</span></pre>\n" }
        p {
            "Edit your  "
            code { "main.rs" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case)]\n</span><span style=\"color:#8c8c8c;\">// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// launch the dioxus app in a webview\n</span><span style=\"color:#f8f8f2;\">    dioxus_desktop::launch(App);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// define a component that renders a div with the text &quot;Hello, world!&quot;\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello, world!&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hello_world_desktop.rs".to_string(),
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum GettingStartedWebSection {
    #[default]
    Empty,
    Web,
    Support,
    Tooling,
    CreatingAProject,
}
impl std::str::FromStr for GettingStartedWebSection {
    type Err = GettingStartedWebSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "web" => Ok(Self::Web),
            "support" => Ok(Self::Support),
            "tooling" => Ok(Self::Tooling),
            "creating-a-project" => Ok(Self::CreatingAProject),
            _ => Err(GettingStartedWebSectionParseError),
        }
    }
}
impl std::fmt::Display for GettingStartedWebSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Web => f.write_str("web"),
            Self::Support => f.write_str("support"),
            Self::Tooling => f.write_str("tooling"),
            Self::CreatingAProject => f.write_str("creating-a-project"),
        }
    }
}
#[derive(Debug)]
pub struct GettingStartedWebSectionParseError;
impl std::fmt::Display for GettingStartedWebSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of GettingStartedWebSectionweb, support, tooling, creating-a-project",
        )?;
        Ok(())
    }
}
impl std::error::Error for GettingStartedWebSectionParseError {}
#[component(no_case_check)]
pub fn GettingStartedWeb(section: GettingStartedWebSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "web",
            Link {
                to: BookRoute::GettingStartedWeb {
                    section: GettingStartedWebSection::Web,
                },
                class: "header",
                "Web"
            }
        }
        p {
            "Build single-page applications that run in the browser with Dioxus. To run on the Web, your app must be compiled to WebAssembly and depend on the  "
            code { "dioxus" }
            " and  "
            code { "dioxus-web" }
            " crates."
        }
        p {
            "A build of Dioxus for the web will be roughly equivalent to the size of a React build (70kb vs 65kb) but it will load significantly faster because "
            Link { to: "https://hacks.mozilla.org/2018/01/making-webassembly-even-faster-firefoxs-new-streaming-and-tiering-compiler/",
                "WebAssembly can be compiled as it is streamed"
            }
            "."
        }
        p { "Examples:" }
        ul {
            li {
                Link { to: "https://github.com/DioxusLabs/example-projects/tree/master/todomvc",
                    "TodoMVC"
                }
            }
            li {
                Link { to: "https://github.com/DioxusLabs/example-projects/tree/master/ecommerce-site",
                    "ECommerce"
                }
            }
        }
        p {
            Link { to: "https://github.com/DioxusLabs/example-projects/blob/master/todomvc",
                img {
                    src: "https://github.com/DioxusLabs/example-projects/raw/master/todomvc/example.png",
                    alt: "TodoMVC example",
                    title: "",
                }
            }
        }
        blockquote {
            p {
                "Note: Because of the limitations of Wasm, "
                Link { to: "https://rustwasm.github.io/docs/book/reference/which-crates-work-with-wasm.html",
                    "not every crate will work"
                }
                " with your web apps, so you'll need to make sure that your crates work without native system calls (timers, IO, etc)."
            }
        }
        h2 { id: "support",
            Link {
                to: BookRoute::GettingStartedWeb {
                    section: GettingStartedWebSection::Support,
                },
                class: "header",
                "Support"
            }
        }
        p { "The Web is the best-supported target platform for Dioxus." }
        ul {
            li {
                "Because your app will be compiled to WASM you have access to browser APIs through "
                Link { to: "https://rustwasm.github.io/docs/wasm-bindgen/introduction.html",
                    "wasm-bingen"
                }
                "."
            }
            li {
                "Dioxus provides hydration to resume apps that are rendered on the server. See the "
                Link { to: "https://github.com/DioxusLabs/dioxus/blob/master/packages/web/examples/hydrate.rs",
                    "hydration example"
                }
                " for more details."
            }
        }
        h2 { id: "tooling",
            Link {
                to: BookRoute::GettingStartedWeb {
                    section: GettingStartedWebSection::Tooling,
                },
                class: "header",
                "Tooling"
            }
        }
        p {
            "To develop your Dioxus app for the web, you'll need a tool to build and serve your assets. We recommend using "
            Link { to: "https://github.com/DioxusLabs/cli", "dioxus-cli" }
            " which includes a build system, Wasm optimization, a dev server, and support hot reloading:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo install dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">cli</span></pre>\n" }
        p {
            "Make sure the  "
            code { "wasm32-unknown-unknown" }
            " target for rust is installed:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rustup target add wasm32</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">unknown</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">unknown</span></pre>\n" }
        h2 { id: "creating-a-project",
            Link {
                to: BookRoute::GettingStartedWeb {
                    section: GettingStartedWebSection::CreatingAProject,
                },
                class: "header",
                "Creating a Project"
            }
        }
        p { "Create a new crate:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo new </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">bin demo\n</span><span style=\"color:#f8f8f2;\">cd demo</span></pre>\n" }
        p {
            "Add Dioxus and the web renderer as dependencies (this will edit your  "
            code { "Cargo.toml" }
            "):"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add dioxus\n</span><span style=\"color:#f8f8f2;\">cargo add dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">web</span></pre>\n" }
        p {
            "Edit your  "
            code { "main.rs" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case)]\n</span><span style=\"color:#8c8c8c;\">// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// launch the web app\n</span><span style=\"color:#f8f8f2;\">    dioxus_web::launch(App);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// create a component that renders a div with the text &quot;Hello, world!&quot;\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello, world!&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hello_world_web.rs".to_string(),
        }
        p { "And to serve our app:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dioxus serve</span></pre>\n" }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum GettingStartedHotReloadSection {
    #[default]
    Empty,
    SettingUpHotReload,
    Setup,
    Usage,
    Limitations,
}
impl std::str::FromStr for GettingStartedHotReloadSection {
    type Err = GettingStartedHotReloadSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "setting-up-hot-reload" => Ok(Self::SettingUpHotReload),
            "setup" => Ok(Self::Setup),
            "usage" => Ok(Self::Usage),
            "limitations" => Ok(Self::Limitations),
            _ => Err(GettingStartedHotReloadSectionParseError),
        }
    }
}
impl std::fmt::Display for GettingStartedHotReloadSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::SettingUpHotReload => f.write_str("setting-up-hot-reload"),
            Self::Setup => f.write_str("setup"),
            Self::Usage => f.write_str("usage"),
            Self::Limitations => f.write_str("limitations"),
        }
    }
}
#[derive(Debug)]
pub struct GettingStartedHotReloadSectionParseError;
impl std::fmt::Display for GettingStartedHotReloadSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of GettingStartedHotReloadSectionsetting-up-hot-reload, setup, usage, limitations",
        )?;
        Ok(())
    }
}
impl std::error::Error for GettingStartedHotReloadSectionParseError {}
#[component(no_case_check)]
pub fn GettingStartedHotReload(
    section: GettingStartedHotReloadSection,
) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "setting-up-hot-reload",
            Link {
                to: BookRoute::GettingStartedHotReload {
                    section: GettingStartedHotReloadSection::SettingUpHotReload,
                },
                class: "header",
                "Setting Up Hot Reload"
            }
        }
        ol {
            li {
                "Hot reloading allows much faster iteration times inside of rsx calls by interpreting them and streaming the edits."
            }
            li {
                "It is useful when changing the styling/layout of a program, but will not help with changing the logic of a program."
            }
            li { "Currently the cli only implements hot reloading for the web renderer." }
        }
        h1 { id: "setup",
            Link {
                to: BookRoute::GettingStartedHotReload {
                    section: GettingStartedHotReloadSection::Setup,
                },
                class: "header",
                "Setup"
            }
        }
        p {
            "Install "
            Link { to: "https://github.com/DioxusLabs/cli", "dioxus-cli" }
            "."
            " "
            "Hot reloading is automatically enabled when using the web renderer on debug builds."
        }
        h1 { id: "usage",
            Link {
                to: BookRoute::GettingStartedHotReload {
                    section: GettingStartedHotReloadSection::Usage,
                },
                class: "header",
                "Usage"
            }
        }
        ol {
            li { "run:" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dioxus serve </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">hot</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">reload</span></pre>\n" }
        ol {
            li { "change some code within a rsx macro" }
            li { "open your localhost in a browser" }
            li { "save and watch the style change without recompiling" }
        }
        h1 { id: "limitations",
            Link {
                to: BookRoute::GettingStartedHotReload {
                    section: GettingStartedHotReloadSection::Limitations,
                },
                class: "header",
                "Limitations"
            }
        }
        ol {
            li {
                "The interpreter can only use expressions that existed on the last full recompile. If you introduce a new variable or expression to the rsx call, it will trigger a full recompile to capture the expression."
            }
            li {
                "Components and Iterators can contain arbitrary rust code and will trigger a full recompile when changed."
            }
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum GettingStartedSsrSection {
    #[default]
    Empty,
    ServerSideRendering,
    MultithreadedSupport,
    Setup,
}
impl std::str::FromStr for GettingStartedSsrSection {
    type Err = GettingStartedSsrSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "server-side-rendering" => Ok(Self::ServerSideRendering),
            "multithreaded-support" => Ok(Self::MultithreadedSupport),
            "setup" => Ok(Self::Setup),
            _ => Err(GettingStartedSsrSectionParseError),
        }
    }
}
impl std::fmt::Display for GettingStartedSsrSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::ServerSideRendering => f.write_str("server-side-rendering"),
            Self::MultithreadedSupport => f.write_str("multithreaded-support"),
            Self::Setup => f.write_str("setup"),
        }
    }
}
#[derive(Debug)]
pub struct GettingStartedSsrSectionParseError;
impl std::fmt::Display for GettingStartedSsrSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of GettingStartedSsrSectionserver-side-rendering, multithreaded-support, setup",
        )?;
        Ok(())
    }
}
impl std::error::Error for GettingStartedSsrSectionParseError {}
#[component(no_case_check)]
pub fn GettingStartedSsr(section: GettingStartedSsrSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "server-side-rendering",
            Link {
                to: BookRoute::GettingStartedSsr {
                    section: GettingStartedSsrSection::ServerSideRendering,
                },
                class: "header",
                "Server-Side Rendering"
            }
        }
        p { "The Dioxus VirtualDom can be rendered server-side." }
        p {
            Link { to: "https://github.com/dioxusLabs/docsite", "Example: Dioxus DocSite" }
        }
        h2 { id: "multithreaded-support",
            Link {
                to: BookRoute::GettingStartedSsr {
                    section: GettingStartedSsrSection::MultithreadedSupport,
                },
                class: "header",
                "Multithreaded Support"
            }
        }
        p {
            "The Dioxus VirtualDom, sadly, is not currently  "
            code { "Send" }
            ". Internally, we use quite a bit of interior mutability which is not thread-safe. This means you can't easily use Dioxus with most web frameworks like Tide, Rocket, Axum, etc."
        }
        p {
            "To solve this, you'll want to spawn a VirtualDom on its own thread and communicate with it via channels."
        }
        p {
            "When working with web frameworks that require  "
            code { "Send" }
            ", it is possible to render a VirtualDom immediately to a String – but you cannot hold the VirtualDom across an await point. For retained-state SSR (essentially LiveView), you'll need to create a pool of VirtualDoms."
        }
        h2 { id: "setup",
            Link {
                to: BookRoute::GettingStartedSsr {
                    section: GettingStartedSsrSection::Setup,
                },
                class: "header",
                "Setup"
            }
        }
        p {
            "For this guide, we're going to show how to use Dioxus SSR with "
            Link { to: "https://docs.rs/axum/latest/axum/", "Axum" }
            "."
        }
        p { "Make sure you have Rust and Cargo installed, and then create a new project:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo new </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">bin demo\n</span><span style=\"color:#f8f8f2;\">cd app</span></pre>\n" }
        p { "Add Dioxus and the ssr renderer as dependencies:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add dioxus\n</span><span style=\"color:#f8f8f2;\">cargo add dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">ssr</span></pre>\n" }
        p {
            "Next, add all the Axum dependencies. This will be different if you're using a different Web Framework"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add tokio </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features full\n</span><span style=\"color:#f8f8f2;\">cargo add axum</span></pre>\n" }
        p { "Your dependencies should look roughly like this:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[dependencies]\n</span><span style=\"color:#f8f8f2;\">axum </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.4.5&quot;\n</span><span style=\"color:#f8f8f2;\">dioxus </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;*&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">ssr </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;*&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">tokio </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;1.15.0&quot;</span><span style=\"color:#f8f8f2;\">, features </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;full&quot;</span><span style=\"color:#f8f8f2;\">] }}</span></pre>\n",
        }
        p { "Now, set up your Axum app to respond on an endpoint." }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">axum::{{response::Html, routing::get, Router}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[tokio::main]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> addr </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">std::net::SocketAddr::from(([</span><span style=\"color:#ff80f4;\">127</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">], </span><span style=\"color:#ff80f4;\">3000</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;listening on http://</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, addr);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    axum::Server::bind(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">addr)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">serve</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">            Router::new()\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">route</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">(app_endpoint))\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">into_make_service</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        )\n</span><span style=\"color:#f8f8f2;\">        .await\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "And then add our endpoint. We can either render  "
            code { "rsx!" }
            " directly:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app_endpoint</span><span style=\"color:#f8f8f2;\">() -&gt; Html&lt;String&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// render the rsx! macro to HTML\n</span><span style=\"color:#f8f8f2;\">    Html(dioxus_ssr::render_lazy(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;hello world!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p { "Or we can render VirtualDoms." }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app_endpoint</span><span style=\"color:#f8f8f2;\">() -&gt; Html&lt;String&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// create a component that renders a div with the text &quot;hello world&quot;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">        cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(div {{ </span><span style=\"color:#ffee99;\">&quot;hello world&quot; </span><span style=\"color:#f8f8f2;\">}}))\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// create a VirtualDom with the app component\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> app </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">VirtualDom::new(app);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// rebuild the VirtualDom before rendering\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">_ =</span><span style=\"color:#f8f8f2;\"> app.</span><span style=\"color:#66d9ef;\">rebuild</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// render the VirtualDom to HTML\n</span><span style=\"color:#f8f8f2;\">    Html(dioxus_ssr::render_vdom(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">app))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "And that's it!" }
        blockquote {
            p {
                "You might notice that you cannot hold the VirtualDom across an await point. Dioxus is currently not ThreadSafe, so it "
                em { "must" }
                " remain on the thread it started. We are working on loosening this requirement."
            }
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum GettingStartedLiveviewSection {
    #[default]
    Empty,
    Liveview,
    Support,
    Setup,
}
impl std::str::FromStr for GettingStartedLiveviewSection {
    type Err = GettingStartedLiveviewSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "liveview" => Ok(Self::Liveview),
            "support" => Ok(Self::Support),
            "setup" => Ok(Self::Setup),
            _ => Err(GettingStartedLiveviewSectionParseError),
        }
    }
}
impl std::fmt::Display for GettingStartedLiveviewSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Liveview => f.write_str("liveview"),
            Self::Support => f.write_str("support"),
            Self::Setup => f.write_str("setup"),
        }
    }
}
#[derive(Debug)]
pub struct GettingStartedLiveviewSectionParseError;
impl std::fmt::Display for GettingStartedLiveviewSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of GettingStartedLiveviewSectionliveview, support, setup",
        )?;
        Ok(())
    }
}
impl std::error::Error for GettingStartedLiveviewSectionParseError {}
#[component(no_case_check)]
pub fn GettingStartedLiveview(section: GettingStartedLiveviewSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "liveview",
            Link {
                to: BookRoute::GettingStartedLiveview {
                    section: GettingStartedLiveviewSection::Liveview,
                },
                class: "header",
                "Liveview"
            }
        }
        p {
            "Liveview allows apps to "
            em { "run" }
            " on the server and "
            em { "render" }
            " in the browser. It uses WebSockets to communicate between the server and the browser."
        }
        p { "Examples:" }
        ul {
            li {
                Link { to: "https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/axum.rs",
                    code { "Axum Example" }
                }
            }
            li {
                Link { to: "https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/salvo.rs",
                    code { "Salvo Example" }
                }
            }
            li {
                Link { to: "https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/warp.rs",
                    code { "Warp Example" }
                }
            }
        }
        h2 { id: "support",
            Link {
                to: BookRoute::GettingStartedLiveview {
                    section: GettingStartedLiveviewSection::Support,
                },
                class: "header",
                "Support"
            }
        }
        p {
            "Liveview is currently limited in capability when compared to the Web platform. Liveview apps run on the server in a native thread. This means that browser APIs are not available, so rendering WebGL, Canvas, etc is not as easy as the Web. However, native system APIs are accessible, so streaming, WebSockets, filesystem, etc are all viable APIs."
        }
        h2 { id: "setup",
            Link {
                to: BookRoute::GettingStartedLiveview {
                    section: GettingStartedLiveviewSection::Setup,
                },
                class: "header",
                "Setup"
            }
        }
        p {
            "For this guide, we're going to show how to use Dioxus Liveview with "
            Link { to: "https://docs.rs/axum/latest/axum/", "Axum" }
            "."
        }
        p { "Make sure you have Rust and Cargo installed, and then create a new project:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo new </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">bin demo\n</span><span style=\"color:#f8f8f2;\">cd app</span></pre>\n" }
        p { "Add Dioxus and the liveview renderer with the Axum feature as dependencies:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add dioxus\n</span><span style=\"color:#f8f8f2;\">cargo add dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">liveview </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features axum</span></pre>\n" }
        p {
            "Next, add all the Axum dependencies. This will be different if you're using a different Web Framework"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add tokio </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features full\n</span><span style=\"color:#f8f8f2;\">cargo add axum</span></pre>\n" }
        p { "Your dependencies should look roughly like this:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[dependencies]\n</span><span style=\"color:#f8f8f2;\">axum </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.4.5&quot;\n</span><span style=\"color:#f8f8f2;\">dioxus </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;*&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">liveview </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;*&quot;</span><span style=\"color:#f8f8f2;\">, features </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;axum&quot;</span><span style=\"color:#f8f8f2;\">] }}\n</span><span style=\"color:#f8f8f2;\">tokio </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;1.15.0&quot;</span><span style=\"color:#f8f8f2;\">, features </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;full&quot;</span><span style=\"color:#f8f8f2;\">] }}</span></pre>\n",
        }
        p { "Now, set up your Axum app to respond on an endpoint." }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[tokio::main]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> addr: std::net::SocketAddr </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">([</span><span style=\"color:#ff80f4;\">127</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">], </span><span style=\"color:#ff80f4;\">3030</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> view </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">dioxus_liveview::LiveViewPool::new();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> app </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Router::new()\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// The root route contains the glue code to connect to the WebSocket\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">route</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                Html(format!(\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;\n</span><span style=\"color:#ffee99;\">                &lt;!DOCTYPE html&gt;\n</span><span style=\"color:#ffee99;\">                &lt;html&gt;\n</span><span style=\"color:#ffee99;\">                &lt;head&gt; &lt;title&gt;Dioxus LiveView with Axum&lt;/title&gt;  &lt;/head&gt;\n</span><span style=\"color:#ffee99;\">                &lt;body&gt; &lt;div id=&quot;main&quot;&gt;&lt;/div&gt; &lt;/body&gt;\n</span><span style=\"color:#ffee99;\">                </span><span style=\"color:#ff80f4;\">{{glue}}\n</span><span style=\"color:#ffee99;\">                &lt;/html&gt;\n</span><span style=\"color:#ffee99;\">                &quot;#</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#8c8c8c;\">// Create the glue code to connect to the WebSocket on the &quot;/ws&quot; route\n</span><span style=\"color:#f8f8f2;\">                    glue </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">dioxus_liveview::interpreter_glue(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;ws://</span><span style=\"color:#ff80f4;\">{{addr}}</span><span style=\"color:#ffee99;\">/ws&quot;</span><span style=\"color:#f8f8f2;\">))\n</span><span style=\"color:#f8f8f2;\">                ))\n</span><span style=\"color:#f8f8f2;\">            }}),\n</span><span style=\"color:#f8f8f2;\">        )\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// The WebSocket route is what Dioxus uses to communicate with the browser\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">route</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;/ws&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">ws: WebSocketUpgrade</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                ws.</span><span style=\"color:#66d9ef;\">on_upgrade</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">socket</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#8c8c8c;\">// When the WebSocket is upgraded, launch the LiveView with the app component\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#f92672;\">_ =</span><span style=\"color:#f8f8f2;\"> view.</span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(dioxus_liveview::axum_socket(socket), app).await;\n</span><span style=\"color:#f8f8f2;\">                }})\n</span><span style=\"color:#f8f8f2;\">            }}),\n</span><span style=\"color:#f8f8f2;\">        );\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;Listening on http://</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, addr);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    axum::Server::bind(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">addr.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">serve</span><span style=\"color:#f8f8f2;\">(app.</span><span style=\"color:#66d9ef;\">into_make_service</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">        .await\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hello_world_liveview.rs".to_string(),
        }
        p { "And then add our app component:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello, world!&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hello_world_liveview.rs".to_string(),
        }
        p { "And that's it!" }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum GettingStartedTuiSection {
    #[default]
    Empty,
    TerminalUi,
    Support,
    GettingSetUp,
}
impl std::str::FromStr for GettingStartedTuiSection {
    type Err = GettingStartedTuiSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "terminal-ui" => Ok(Self::TerminalUi),
            "support" => Ok(Self::Support),
            "getting-set-up" => Ok(Self::GettingSetUp),
            _ => Err(GettingStartedTuiSectionParseError),
        }
    }
}
impl std::fmt::Display for GettingStartedTuiSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::TerminalUi => f.write_str("terminal-ui"),
            Self::Support => f.write_str("support"),
            Self::GettingSetUp => f.write_str("getting-set-up"),
        }
    }
}
#[derive(Debug)]
pub struct GettingStartedTuiSectionParseError;
impl std::fmt::Display for GettingStartedTuiSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of GettingStartedTuiSectionterminal-ui, support, getting-set-up",
        )?;
        Ok(())
    }
}
impl std::error::Error for GettingStartedTuiSectionParseError {}
#[component(no_case_check)]
pub fn GettingStartedTui(section: GettingStartedTuiSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "terminal-ui",
            Link {
                to: BookRoute::GettingStartedTui {
                    section: GettingStartedTuiSection::TerminalUi,
                },
                class: "header",
                "Terminal UI"
            }
        }
        p { "You can build a text-based interface that will run in the terminal using Dioxus." }
        p {
            img {
                src: "https://github.com/DioxusLabs/rink/raw/master/examples/example.png",
                alt: "Hello World screenshot",
                title: "",
            }
        }
        blockquote {
            p {
                "Note: this book was written with HTML-based platforms in mind. You might be able to follow along with TUI, but you'll have to adapt a bit."
            }
        }
        h2 { id: "support",
            Link {
                to: BookRoute::GettingStartedTui {
                    section: GettingStartedTuiSection::Support,
                },
                class: "header",
                "Support"
            }
        }
        p {
            "TUI support is currently quite experimental. But, if you're willing to venture into the realm of the unknown, this guide will get you started."
        }
        ul {
            li { "It uses flexbox for the layout" }
            li { "It only supports a subset of the attributes and elements" }
            li {
                "Regular widgets will not work in the tui render, but the tui renderer has its own widget components that start with a capital letter. See the "
                Link { to: "https://github.com/DioxusLabs/dioxus/blob/master/packages/tui/examples/tui_widgets.rs",
                    "widgets example"
                }
            }
            li { "1px is one character line height. Your regular CSS px does not translate" }
            li { "If your app panics, your terminal is wrecked. This will be fixed eventually" }
        }
        h2 { id: "getting-set-up",
            Link {
                to: BookRoute::GettingStartedTui {
                    section: GettingStartedTuiSection::GettingSetUp,
                },
                class: "header",
                "Getting Set up"
            }
        }
        p { "Start by making a new package and adding Dioxus and the TUI renderer as dependancies." }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo new </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">bin demo\n</span><span style=\"color:#f8f8f2;\">cd demo\n</span><span style=\"color:#f8f8f2;\">cargo add dioxus\n</span><span style=\"color:#f8f8f2;\">cargo add dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">tui</span></pre>\n" }
        p {
            "Then, edit your  "
            code { "main.rs" }
            " with the basic template."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case)]\n</span><span style=\"color:#8c8c8c;\">// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// launch the app in the terminal\n</span><span style=\"color:#f8f8f2;\">    dioxus_tui::launch(App);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// create a component that renders a div with the text &quot;Hello, world!&quot;\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello, world!&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hello_world_tui.rs".to_string(),
        }
        p { "To run our app:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo run</span></pre>\n" }
        p {
            "Press \"ctrl-c\" to close the app. To switch from \"ctrl-c\" to just \"q\" to quit you can launch the app with a configuration to disable the default quit and use the root TuiContext to quit on your own."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// todo remove deprecated\n</span><span style=\"color:#f8f8f2;\">#![allow(non_snake_case, deprecated)]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::events::{{KeyCode, KeyboardEvent}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_tui::TuiContext;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    dioxus_tui::launch_cfg(\n</span><span style=\"color:#f8f8f2;\">        App,\n</span><span style=\"color:#f8f8f2;\">        dioxus_tui::Config::new()\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">without_ctrl_c_quit</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// Some older terminals only support 16 colors or ANSI colors\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// If your terminal is one of these, change this to BaseColors or ANSI\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">with_rendering_mode</span><span style=\"color:#f8f8f2;\">(dioxus_tui::RenderingMode::Rgb),\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> tui_ctx: TuiContext </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> cx.</span><span style=\"color:#66d9ef;\">consume_context</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            width: </span><span style=\"color:#ffee99;\">&quot;100%&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            height: </span><span style=\"color:#ffee99;\">&quot;10px&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            background_color: </span><span style=\"color:#ffee99;\">&quot;red&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            justify_content: </span><span style=\"color:#ffee99;\">&quot;center&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            align_items: </span><span style=\"color:#ffee99;\">&quot;center&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            onkeydown: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">k: KeyboardEvent</span><span style=\"color:#f92672;\">| if </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f8f8f2;\">KeyCode::Q </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> k.key_code {{\n</span><span style=\"color:#f8f8f2;\">                tui_ctx.</span><span style=\"color:#66d9ef;\">quit</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello world!&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hello_world_tui_no_ctrl_c.rs".to_string(),
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum GettingStartedMobileSection {
    #[default]
    Empty,
    MobileApp,
    Support,
    GettingSetUp,
}
impl std::str::FromStr for GettingStartedMobileSection {
    type Err = GettingStartedMobileSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "mobile-app" => Ok(Self::MobileApp),
            "support" => Ok(Self::Support),
            "getting-set-up" => Ok(Self::GettingSetUp),
            _ => Err(GettingStartedMobileSectionParseError),
        }
    }
}
impl std::fmt::Display for GettingStartedMobileSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::MobileApp => f.write_str("mobile-app"),
            Self::Support => f.write_str("support"),
            Self::GettingSetUp => f.write_str("getting-set-up"),
        }
    }
}
#[derive(Debug)]
pub struct GettingStartedMobileSectionParseError;
impl std::fmt::Display for GettingStartedMobileSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of GettingStartedMobileSectionmobile-app, support, getting-set-up",
        )?;
        Ok(())
    }
}
impl std::error::Error for GettingStartedMobileSectionParseError {}
#[component(no_case_check)]
pub fn GettingStartedMobile(section: GettingStartedMobileSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "mobile-app",
            Link {
                to: BookRoute::GettingStartedMobile {
                    section: GettingStartedMobileSection::MobileApp,
                },
                class: "header",
                "Mobile App"
            }
        }
        p { "Build a mobile app with Dioxus!" }
        p {
            "Example: "
            Link { to: "https://github.com/DioxusLabs/example-projects/blob/master/ios_demo",
                "Todo App"
            }
        }
        h2 { id: "support",
            Link {
                to: BookRoute::GettingStartedMobile {
                    section: GettingStartedMobileSection::Support,
                },
                class: "header",
                "Support"
            }
        }
        p {
            "Mobile is currently the least-supported renderer target for Dioxus. Mobile apps are rendered with either the platform's WebView or experimentally through "
            Link { to: "https://github.com/DioxusLabs/blitz", "WGPU" }
            ". WebView doesn't support animations, transparency, and native widgets."
        }
        p {
            "Mobile support is currently best suited for CRUD-style apps, ideally for internal teams who need to develop quickly but don't care much about animations or native widgets."
        }
        p {
            "This guide is primarily targeted at iOS apps, however, you can follow it while using the  "
            code { "android" }
            " guide in  "
            code { "cargo-mobile" }
            "."
        }
        h2 { id: "getting-set-up",
            Link {
                to: BookRoute::GettingStartedMobile {
                    section: GettingStartedMobileSection::GettingSetUp,
                },
                class: "header",
                "Getting Set up"
            }
        }
        p {
            "Getting set up with mobile can be quite challenging. The tooling here isn't great (yet) and might take some hacking around to get things working. macOS M1 is broadly unexplored and might not work for you."
        }
        p {
            "We're going to be using  "
            code { "cargo-mobile" }
            " to build for mobile. First, install it:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo install </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">git https:</span><span style=\"color:#8c8c8c;\">//github.com/BrainiumLLC/cargo-mobile</span></pre>\n" }
        p {
            "And then initialize your app for the right platform. Use the  "
            code { "winit" }
            " template for now. Right now, there's no \"Dioxus\" template in cargo-mobile."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo mobile init</span></pre>\n" }
        p {
            "We're going to completely clear out the  "
            code { "dependencies" }
            " it generates for us, swapping out  "
            code { "winit" }
            " with  "
            code { "dioxus-mobile" }
            "."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">[package]\n</span><span style=\"color:#f8f8f2;\">name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;dioxus-ios-demo&quot;\n</span><span style=\"color:#f8f8f2;\">version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.1.0&quot;\n</span><span style=\"color:#f8f8f2;\">authors </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[]\n</span><span style=\"color:#f8f8f2;\">edition </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;2018&quot;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> leave the `lib` declaration\n</span><span style=\"color:#f8f8f2;\">[lib]\n</span><span style=\"color:#f92672;\">crate-</span><span style=\"font-style:italic;color:#66d9ef;\">type </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;staticlib&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;cdylib&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;rlib&quot;</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> leave the binary it generates </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> us\n</span><span style=\"color:#f8f8f2;\">[[bin]]\n</span><span style=\"color:#f8f8f2;\">name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;dioxus-ios-demo-desktop&quot;\n</span><span style=\"color:#f8f8f2;\">path </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;gen/bin/desktop.rs&quot;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> clear all the dependencies\n</span><span style=\"color:#f8f8f2;\">[dependencies]\n</span><span style=\"color:#f8f8f2;\">mobile</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">entry</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">point </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.1.0&quot;\n</span><span style=\"color:#f8f8f2;\">dioxus </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;*&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">desktop </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;*&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">simple_logger </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;*&quot;</span></pre>\n",
        }
        p {
            "Edit your  "
            code { "lib.rs" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    dioxus_desktop::launch(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;hello world!&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum DescribingUiIndexSection {
    #[default]
    Empty,
    DescribingTheUi,
    RsxFeatures,
    Attributes,
    CustomAttributes,
    Interpolation,
    Children,
    Fragments,
    Expressions,
    Loops,
    IfStatements,
}
impl std::str::FromStr for DescribingUiIndexSection {
    type Err = DescribingUiIndexSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "describing-the-ui" => Ok(Self::DescribingTheUi),
            "rsx-features" => Ok(Self::RsxFeatures),
            "attributes" => Ok(Self::Attributes),
            "custom-attributes" => Ok(Self::CustomAttributes),
            "interpolation" => Ok(Self::Interpolation),
            "children" => Ok(Self::Children),
            "fragments" => Ok(Self::Fragments),
            "expressions" => Ok(Self::Expressions),
            "loops" => Ok(Self::Loops),
            "if-statements" => Ok(Self::IfStatements),
            _ => Err(DescribingUiIndexSectionParseError),
        }
    }
}
impl std::fmt::Display for DescribingUiIndexSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::DescribingTheUi => f.write_str("describing-the-ui"),
            Self::RsxFeatures => f.write_str("rsx-features"),
            Self::Attributes => f.write_str("attributes"),
            Self::CustomAttributes => f.write_str("custom-attributes"),
            Self::Interpolation => f.write_str("interpolation"),
            Self::Children => f.write_str("children"),
            Self::Fragments => f.write_str("fragments"),
            Self::Expressions => f.write_str("expressions"),
            Self::Loops => f.write_str("loops"),
            Self::IfStatements => f.write_str("if-statements"),
        }
    }
}
#[derive(Debug)]
pub struct DescribingUiIndexSectionParseError;
impl std::fmt::Display for DescribingUiIndexSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of DescribingUiIndexSectiondescribing-the-ui, rsx-features, attributes, custom-attributes, interpolation, children, fragments, expressions, loops, if-statements",
        )?;
        Ok(())
    }
}
impl std::error::Error for DescribingUiIndexSectionParseError {}
#[component(no_case_check)]
pub fn DescribingUiIndex(section: DescribingUiIndexSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "describing-the-ui",
            Link {
                to: BookRoute::DescribingUiIndex {
                    section: DescribingUiIndexSection::DescribingTheUi,
                },
                class: "header",
                "Describing the UI"
            }
        }
        p {
            "Dioxus is a "
            em { "declarative" }
            " framework. This means that instead of telling Dioxus what to do (e.g. to \"create an element\" or \"set the color to red\") we simply "
            em { "declare" }
            " what we want the UI to look like using RSX."
        }
        p { "You have already seen a simple example of RSX syntax in the \"hello world\" application:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// define a component that renders a div with the text &quot;Hello, world!&quot;\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello, world!&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hello_world_desktop.rs".to_string(),
        }
        p {
            "Here, we use the  "
            code { "rsx!" }
            " macro to "
            em { "declare" }
            " that we want a "
            code { "div" }
            " element, containing the text "
            code { "\"Hello, world!\"" }
            ". Dioxus takes the RSX and constructs a UI from it."
        }
        h2 { id: "rsx-features",
            Link {
                to: BookRoute::DescribingUiIndex {
                    section: DescribingUiIndexSection::RsxFeatures,
                },
                class: "header",
                "RSX Features"
            }
        }
        p {
            "RSX is very similar to HTML in that it describes elements with attributes and children. Here's an empty  "
            code { "div" }
            " element in RSX, as well as the resulting HTML:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(div {{\n</span><span style=\"color:#8c8c8c;\">// attributes / listeners\n</span><span style=\"color:#8c8c8c;\">// children\n</span><span style=\"color:#f8f8f2;\">}}))</span></pre>\n",
            name: "rsx_overview.rs".to_string(),
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span></pre>\n" }
        h3 { id: "attributes",
            Link {
                to: BookRoute::DescribingUiIndex {
                    section: DescribingUiIndexSection::Attributes,
                },
                class: "header",
                "Attributes"
            }
        }
        p {
            "Attributes (and "
            Link {
                to: BookRoute::InteractivityIndex {
                    section: InteractivityIndexSection::Empty,
                },
                "listeners"
            }
            ") modify the behavior or appearance of the element they are attached to. They are specified inside the "
            code { "{{}}" }
            " brackets, using the "
            code { "name: value" }
            " syntax. You can provide the value as a literal in the RSX:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(a {{\n</span><span style=\"color:#f8f8f2;\">href: </span><span style=\"color:#ffee99;\">&quot;https://www.youtube.com/watch?v=dQw4w9WgXcQ&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">class: </span><span style=\"color:#ffee99;\">&quot;primary_button&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">color: </span><span style=\"color:#ffee99;\">&quot;red&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}}))</span></pre>\n",
            name: "rsx_overview.rs".to_string(),
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">a href</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;https://www.youtube.com/watch?v=dQw4w9WgXcQ&quot;</span><span style=\"color:#f8f8f2;\"> class</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;primary_button&quot;</span><span style=\"color:#f8f8f2;\"> autofocus</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;true&quot;</span><span style=\"color:#f8f8f2;\"> style</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;color: red&quot;</span><span style=\"color:#f92672;\">&gt;&lt;/</span><span style=\"color:#f8f8f2;\">a</span><span style=\"color:#f92672;\">&gt;</span></pre>\n" }
        blockquote {
            p {
                "Note: All attributes defined in  "
                code { "dioxus-html" }
                " follow the snake_case naming convention. They transform their  "
                code { "snake_case" }
                " names to HTML's  "
                code { "camelCase" }
                " attributes."
            }
        }
        blockquote {
            p {
                "Note: Styles can be used directly outside of the  "
                code { "style:" }
                " attribute. In the above example,  "
                code { "color: \"red\"" }
                " is turned into  "
                code { "style=\"color: red\"" }
                "."
            }
        }
        h4 { id: "custom-attributes",
            Link {
                to: BookRoute::DescribingUiIndex {
                    section: DescribingUiIndexSection::CustomAttributes,
                },
                class: "header",
                "Custom Attributes"
            }
        }
        p {
            "Dioxus has a pre-configured set of attributes that you can use. RSX is validated at compile time to make sure you didn't specify an invalid attribute. If you want to override this behavior with a custom attribute name, specify the attribute in quotes:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(b {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;customAttribute&quot;</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;value&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}}))</span></pre>\n",
            name: "rsx_overview.rs".to_string(),
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">b customAttribute</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;value&quot;</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">b</span><span style=\"color:#f92672;\">&gt;</span></pre>\n" }
        h3 { id: "interpolation",
            Link {
                to: BookRoute::DescribingUiIndex {
                    section: DescribingUiIndexSection::Interpolation,
                },
                class: "header",
                "Interpolation"
            }
        }
        p {
            "Similarly to how you can "
            Link { to: "https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html",
                "format"
            }
            " Rust strings, you can also interpolate in RSX text. Use "
            code { "{{variable}}" }
            " to Display the value of a variable in a string, or "
            code { "{{variable:?}}" }
            " to use the Debug representation:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> coordinates </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">42</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> country </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;es&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(div {{\n</span><span style=\"color:#f8f8f2;\">class: </span><span style=\"color:#ffee99;\">&quot;country-{{country}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#ffee99;\">&quot;position&quot;</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;{{coordinates:?}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#8c8c8c;\">// arbitrary expressions are allowed,\n</span><span style=\"color:#8c8c8c;\">// as long as they don&#39;t contain `{{}}`\n</span><span style=\"color:#f8f8f2;\">div {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;{{country.to_uppercase()}}&quot;\n</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">div {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;{{7*6}}&quot;\n</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#8c8c8c;\">// {{}} can be escaped with {{{{}}}}\n</span><span style=\"color:#f8f8f2;\">div {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;{{{{}}}}&quot;\n</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">}}))</span></pre>\n",
            name: "rsx_overview.rs".to_string(),
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div class</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;country-es&quot;</span><span style=\"color:#f8f8f2;\"> position</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;(42, 0)&quot;</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">ES&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#ff80f4;\">42</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">{{}}</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span></pre>\n",
        }
        h3 { id: "children",
            Link {
                to: BookRoute::DescribingUiIndex {
                    section: DescribingUiIndexSection::Children,
                },
                class: "header",
                "Children"
            }
        }
        p {
            "To add children to an element, put them inside the  "
            code { "{{}}" }
            " brackets after all attributes and listeners in the element. They can be other elements, text, or "
            Link {
                to: BookRoute::DescribingUiComponents {
                    section: DescribingUiComponentsSection::Empty,
                },
                "components"
            }
            ". For example, you could have an "
            code { "ol" }
            " (ordered list) element, containing 3 "
            code { "li" }
            " (list item) elements, each of which contains some text:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(ol {{\n</span><span style=\"color:#f8f8f2;\">li {{</span><span style=\"color:#ffee99;\">&quot;First Item&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">li {{</span><span style=\"color:#ffee99;\">&quot;Second Item&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">li {{</span><span style=\"color:#ffee99;\">&quot;Third Item&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}))</span></pre>\n",
            name: "rsx_overview.rs".to_string(),
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">ol</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">li</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">First Item&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">li</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">li</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">Second Item&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">li</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">li</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">Third Item&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">li</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">ol</span><span style=\"color:#f92672;\">&gt;</span></pre>\n",
        }
        h3 { id: "fragments",
            Link {
                to: BookRoute::DescribingUiIndex {
                    section: DescribingUiIndexSection::Fragments,
                },
                class: "header",
                "Fragments"
            }
        }
        p {
            "You can render multiple elements at the top level of  "
            code { "rsx!" }
            " and they will be automatically grouped."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">p {{</span><span style=\"color:#ffee99;\">&quot;First Item&quot;</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">p {{</span><span style=\"color:#ffee99;\">&quot;Second Item&quot;</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">))</span></pre>\n",
            name: "rsx_overview.rs".to_string(),
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">p</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">First Item&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">p</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">p</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">Second Item&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">p</span><span style=\"color:#f92672;\">&gt;</span></pre>\n" }
        h3 { id: "expressions",
            Link {
                to: BookRoute::DescribingUiIndex {
                    section: DescribingUiIndexSection::Expressions,
                },
                class: "header",
                "Expressions"
            }
        }
        p {
            "You can include arbitrary Rust expressions as children within RSX that implements "
            Link { to: "https://docs.rs/dioxus-core/0.3/dioxus_core/trait.IntoDynNode.html",
                "IntoDynNode"
            }
            ". This is useful for displaying data from an "
            Link { to: "https://doc.rust-lang.org/stable/book/ch13-02-iterators.html#processing-a-series-of-items-with-iterators",
                "iterator"
            }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> text </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;Dioxus&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(span {{\n</span><span style=\"color:#f8f8f2;\">text.</span><span style=\"color:#66d9ef;\">to_uppercase</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#8c8c8c;\">// create a list of text from 0 to 9\n</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">..</span><span style=\"color:#ff80f4;\">10</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">i</span><span style=\"color:#f8f8f2;\">| rsx!{{ i.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">() }})\n</span><span style=\"color:#f8f8f2;\">}}))</span></pre>\n",
            name: "rsx_overview.rs".to_string(),
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">span</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">DIOXUS0123456789&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">span</span><span style=\"color:#f92672;\">&gt;</span></pre>\n" }
        h3 { id: "loops",
            Link {
                to: BookRoute::DescribingUiIndex {
                    section: DescribingUiIndexSection::Loops,
                },
                class: "header",
                "Loops"
            }
        }
        p { "In addition to iterators you can also use for loops directly within RSX:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#8c8c8c;\">// use a for loop where the body itself is RSX\n</span><span style=\"color:#f8f8f2;\">div {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// create a list of text from 0 to 9\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> i </span><span style=\"color:#f92672;\">in </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">..</span><span style=\"color:#ff80f4;\">3 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// NOTE: the body of the loop is RSX not a rust statement\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;{{i}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#8c8c8c;\">// iterator equivalent\n</span><span style=\"color:#f8f8f2;\">div {{\n</span><span style=\"color:#f8f8f2;\">    (</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">..</span><span style=\"color:#ff80f4;\">3</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">i</span><span style=\"color:#f8f8f2;\">| rsx!{{ div {{ </span><span style=\"color:#ffee99;\">&quot;{{i}}&quot; </span><span style=\"color:#f8f8f2;\">}} }})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "rsx_overview.rs".to_string(),
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span></pre>\n",
        }
        h3 { id: "if-statements",
            Link {
                to: BookRoute::DescribingUiIndex {
                    section: DescribingUiIndexSection::IfStatements,
                },
                class: "header",
                "If statements"
            }
        }
        p { "You can also use if statements without an else branch within RSX:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#8c8c8c;\">// use if statements without an else\n</span><span style=\"color:#f92672;\">if </span><span style=\"color:#ff80f4;\">true </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    rsx!(div {{ </span><span style=\"color:#ffee99;\">&quot;true&quot; </span><span style=\"color:#f8f8f2;\">}})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "rsx_overview.rs".to_string(),
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span></pre>\n" }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum DescribingUiSpecialAttributesSection {
    #[default]
    Empty,
    SpecialAttributes,
    TheHtmlEscapeHatch,
    BooleanAttributes,
}
impl std::str::FromStr for DescribingUiSpecialAttributesSection {
    type Err = DescribingUiSpecialAttributesSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "special-attributes" => Ok(Self::SpecialAttributes),
            "the-html-escape-hatch" => Ok(Self::TheHtmlEscapeHatch),
            "boolean-attributes" => Ok(Self::BooleanAttributes),
            _ => Err(DescribingUiSpecialAttributesSectionParseError),
        }
    }
}
impl std::fmt::Display for DescribingUiSpecialAttributesSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::SpecialAttributes => f.write_str("special-attributes"),
            Self::TheHtmlEscapeHatch => f.write_str("the-html-escape-hatch"),
            Self::BooleanAttributes => f.write_str("boolean-attributes"),
        }
    }
}
#[derive(Debug)]
pub struct DescribingUiSpecialAttributesSectionParseError;
impl std::fmt::Display for DescribingUiSpecialAttributesSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of DescribingUiSpecialAttributesSectionspecial-attributes, the-html-escape-hatch, boolean-attributes",
        )?;
        Ok(())
    }
}
impl std::error::Error for DescribingUiSpecialAttributesSectionParseError {}
#[component(no_case_check)]
pub fn DescribingUiSpecialAttributes(
    section: DescribingUiSpecialAttributesSection,
) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "special-attributes",
            Link {
                to: BookRoute::DescribingUiSpecialAttributes {
                    section: DescribingUiSpecialAttributesSection::SpecialAttributes,
                },
                class: "header",
                "Special Attributes"
            }
        }
        p { "While most attributes are simply passed on to the HTML, some have special behaviors." }
        h2 { id: "the-html-escape-hatch",
            Link {
                to: BookRoute::DescribingUiSpecialAttributes {
                    section: DescribingUiSpecialAttributesSection::TheHtmlEscapeHatch,
                },
                class: "header",
                "The HTML Escape Hatch"
            }
        }
        p {
            "If you're working with pre-rendered assets, output from templates, or output from a JS library, then you might want to pass HTML directly instead of going through Dioxus. In these instances, reach for  "
            code { "dangerous_inner_html" }
            "."
        }
        p {
            "For example, shipping a markdown-to-Dioxus converter might significantly bloat your final application size. Instead, you'll want to pre-render your markdown to HTML and then include the HTML directly in your output. We use this approach for the "
            Link { to: "https://dioxuslabs.com", "Dioxus homepage" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// this should come from a trusted source\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> contents </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;live &lt;b&gt;dangerously&lt;/b&gt;&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">div {{\n</span><span style=\"color:#f8f8f2;\">    dangerous_inner_html: </span><span style=\"color:#ffee99;\">&quot;{{contents}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "dangerous_inner_html.rs".to_string(),
        }
        blockquote {
            p {
                "Note! This attribute is called \"dangerous_inner_html\" because it is "
                strong { "dangerous" }
                " to pass it data you don't trust. If you're not careful, you can easily expose "
                Link { to: "https://en.wikipedia.org/wiki/Cross-site_scripting",
                    "cross-site scripting (XSS)"
                }
                " attacks to your users."
            }
            p {
                "If you're handling untrusted input, make sure to sanitize your HTML before passing it into  "
                code { "dangerous_inner_html" }
                " – or just pass it to a Text Element to escape any HTML tags."
            }
        }
        h2 { id: "boolean-attributes",
            Link {
                to: BookRoute::DescribingUiSpecialAttributes {
                    section: DescribingUiSpecialAttributesSection::BooleanAttributes,
                },
                class: "header",
                "Boolean Attributes"
            }
        }
        p {
            "Most attributes, when rendered, will be rendered exactly as the input you provided. However, some attributes are considered \"boolean\" attributes and just their presence determines whether they affect the output. For these attributes, a provided value of  "
            code { "\"false\"" }
            " will cause them to be removed from the target element."
        }
        p {
            "So this RSX wouldn't actually render the  "
            code { "hidden" }
            " attribute:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">div {{\n</span><span style=\"color:#f8f8f2;\">    hidden: </span><span style=\"color:#ffee99;\">&quot;false&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;hello&quot;\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "boolean_attribute.rs".to_string(),
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">hello&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;</span></pre>\n" }
        p {
            "Not all attributes work like this however. "
            em { "Only the following attributes" }
            " have this behavior:"
        }
        ul {
            li {
                code { "allowfullscreen" }
            }
            li {
                code { "allowpaymentrequest" }
            }
            li {
                code { "async" }
            }
            li {
                code { "autofocus" }
            }
            li {
                code { "autoplay" }
            }
            li {
                code { "checked" }
            }
            li {
                code { "controls" }
            }
            li {
                code { "default" }
            }
            li {
                code { "defer" }
            }
            li {
                code { "disabled" }
            }
            li {
                code { "formnovalidate" }
            }
            li {
                code { "hidden" }
            }
            li {
                code { "ismap" }
            }
            li {
                code { "itemscope" }
            }
            li {
                code { "loop" }
            }
            li {
                code { "multiple" }
            }
            li {
                code { "muted" }
            }
            li {
                code { "nomodule" }
            }
            li {
                code { "novalidate" }
            }
            li {
                code { "open" }
            }
            li {
                code { "playsinline" }
            }
            li {
                code { "readonly" }
            }
            li {
                code { "required" }
            }
            li {
                code { "reversed" }
            }
            li {
                code { "selected" }
            }
            li {
                code { "truespeed" }
            }
        }
        p {
            "For any other attributes, a value of  "
            code { "\"false\"" }
            " will be sent directly to the DOM."
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum DescribingUiComponentsSection {
    #[default]
    Empty,
    Components,
}
impl std::str::FromStr for DescribingUiComponentsSection {
    type Err = DescribingUiComponentsSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "components" => Ok(Self::Components),
            _ => Err(DescribingUiComponentsSectionParseError),
        }
    }
}
impl std::fmt::Display for DescribingUiComponentsSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Components => f.write_str("components"),
        }
    }
}
#[derive(Debug)]
pub struct DescribingUiComponentsSectionParseError;
impl std::fmt::Display for DescribingUiComponentsSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of DescribingUiComponentsSectioncomponents",
        )?;
        Ok(())
    }
}
impl std::error::Error for DescribingUiComponentsSectionParseError {}
#[component(no_case_check)]
pub fn DescribingUiComponents(section: DescribingUiComponentsSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "components",
            Link {
                to: BookRoute::DescribingUiComponents {
                    section: DescribingUiComponentsSection::Components,
                },
                class: "header",
                "Components"
            }
        }
        p {
            "Just like you wouldn't want to write a complex program in a single, long,  "
            code { "main" }
            " function, you shouldn't build a complex UI in a single  "
            code { "App" }
            " function. Instead, you should break down the functionality of an app in logical parts called components."
        }
        p {
            "A component is a Rust function, named in UpperCammelCase, that takes a  "
            code { "Scope" }
            " parameter and returns an  "
            code { "Element" }
            " describing the UI it wants to render. In fact, our  "
            code { "App" }
            " function is a component!"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// define a component that renders a div with the text &quot;Hello, world!&quot;\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello, world!&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hello_world_desktop.rs".to_string(),
        }
        blockquote {
            p {
                "You'll probably want to add  "
                code { "#![allow(non_snake_case)]" }
                " to the top of your crate to avoid warnings about UpperCammelCase component names"
            }
        }
        p {
            "A Component is responsible for some rendering task – typically, rendering an isolated part of the user interface. For example, you could have an  "
            code { "About" }
            " component that renders a short description of Dioxus Labs:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">About</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(p {{\n</span><span style=\"color:#f8f8f2;\">        b {{</span><span style=\"color:#ffee99;\">&quot;Dioxus Labs&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot; An Open Source project dedicated to making Rust UI wonderful.&quot;\n</span><span style=\"color:#f8f8f2;\">    }}))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "components.rs".to_string(),
        }
        p {
            "Then, you can render your component in another component, similarly to how elements are rendered:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        About {{}},\n</span><span style=\"color:#f8f8f2;\">        About {{}},\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "components.rs".to_string(),
        }
        p {
            img {
                src: asset!(
                    "/assets/static/images03/screenshot_about_component.png",
                    ImageAssetOptions::new().with_avif()
                ),
                alt: "Screenshot containing the About component twice",
                title: "",
            }
        }
        blockquote {
            p {
                "At this point, it might seem like components are nothing more than functions. However, as you learn more about the features of Dioxus, you'll see that they are actually more powerful!"
            }
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum DescribingUiComponentPropsSection {
    #[default]
    Empty,
    ComponentProps,
    Deriveprops,
    OwnedProps,
    BorrowedProps,
    PropOptions,
    OptionalProps,
    ExplicitlyRequiredOptionS,
    DefaultProps,
    AutomaticConversionWithInto,
    TheInlinePropsMacro,
}
impl std::str::FromStr for DescribingUiComponentPropsSection {
    type Err = DescribingUiComponentPropsSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "component-props" => Ok(Self::ComponentProps),
            "deriveprops" => Ok(Self::Deriveprops),
            "owned-props" => Ok(Self::OwnedProps),
            "borrowed-props" => Ok(Self::BorrowedProps),
            "prop-options" => Ok(Self::PropOptions),
            "optional-props" => Ok(Self::OptionalProps),
            "explicitly-required-option-s" => Ok(Self::ExplicitlyRequiredOptionS),
            "default-props" => Ok(Self::DefaultProps),
            "automatic-conversion-with-into" => Ok(Self::AutomaticConversionWithInto),
            "the-inline-props-macro" => Ok(Self::TheInlinePropsMacro),
            _ => Err(DescribingUiComponentPropsSectionParseError),
        }
    }
}
impl std::fmt::Display for DescribingUiComponentPropsSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::ComponentProps => f.write_str("component-props"),
            Self::Deriveprops => f.write_str("deriveprops"),
            Self::OwnedProps => f.write_str("owned-props"),
            Self::BorrowedProps => f.write_str("borrowed-props"),
            Self::PropOptions => f.write_str("prop-options"),
            Self::OptionalProps => f.write_str("optional-props"),
            Self::ExplicitlyRequiredOptionS => f.write_str("explicitly-required-option-s"),
            Self::DefaultProps => f.write_str("default-props"),
            Self::AutomaticConversionWithInto => f.write_str("automatic-conversion-with-into"),
            Self::TheInlinePropsMacro => f.write_str("the-inline-props-macro"),
        }
    }
}
#[derive(Debug)]
pub struct DescribingUiComponentPropsSectionParseError;
impl std::fmt::Display for DescribingUiComponentPropsSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of DescribingUiComponentPropsSectioncomponent-props, deriveprops, owned-props, borrowed-props, prop-options, optional-props, explicitly-required-option-s, default-props, automatic-conversion-with-into, the-inline-props-macro",
        )?;
        Ok(())
    }
}
impl std::error::Error for DescribingUiComponentPropsSectionParseError {}
#[component(no_case_check)]
pub fn DescribingUiComponentProps(
    section: DescribingUiComponentPropsSection,
) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "component-props",
            Link {
                to: BookRoute::DescribingUiComponentProps {
                    section: DescribingUiComponentPropsSection::ComponentProps,
                },
                class: "header",
                "Component Props"
            }
        }
        p {
            "Just like you can pass arguments to a function, you can pass props to a component that customize its behavior! The components we've seen so far didn't accept any props – so let's write some components that do."
        }
        h2 { id: "deriveprops",
            Link {
                to: BookRoute::DescribingUiComponentProps {
                    section: DescribingUiComponentPropsSection::Deriveprops,
                },
                class: "header",
                "derive(Props)"
            }
        }
        p {
            "Component props are a single struct annotated with  "
            code { "#[derive(Props)]" }
            ". For a component to accept props, the type of its argument must be  "
            code { "Scope<YourPropsStruct>" }
            ". Then, you can access the value of the props using  "
            code { "cx.props" }
            "."
        }
        p { "There are 2 flavors of Props structs:" }
        ul {
            li {
                "Owned props:"
                ul {
                    li { "Don't have an associated lifetime" }
                    li {
                        "Implement "
                        code { "PartialEq" }
                        ", allow for memoization (if the props don't change, Dioxus won't re-render the component)"
                    }
                }
            }
            li {
                "Borrowed props:"
                ul {
                    li {
                        Link { to: "https://doc.rust-lang.org/beta/rust-by-example/scope/borrow.html",
                            "Borrow"
                        }
                        " from a parent component"
                    }
                    li { "Cannot be memoized due to lifetime constraints" }
                }
            }
        }
        h3 { id: "owned-props",
            Link {
                to: BookRoute::DescribingUiComponentProps {
                    section: DescribingUiComponentPropsSection::OwnedProps,
                },
                class: "header",
                "Owned Props"
            }
        }
        p { "Owned Props are very simple – they don't borrow anything. Example:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Remember: Owned props must implement `PartialEq`!\n</span><span style=\"color:#f8f8f2;\">#[derive(PartialEq, Props)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">LikesProps {{\n</span><span style=\"color:#f8f8f2;\">    score: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Likes</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;LikesProps&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;This post has &quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            b {{ </span><span style=\"color:#ffee99;\">&quot;{{cx.props.score}}&quot; </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot; likes&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_owned_props.rs".to_string(),
        }
        p {
            "You can then pass prop values to the component the same way you would pass attributes to an element:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Likes {{\n</span><span style=\"color:#f8f8f2;\">            score: </span><span style=\"color:#ff80f4;\">42</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_owned_props.rs".to_string(),
        }
        p {
            img {
                src: asset!(
                    "/assets/static/images03/component_owned_props_screenshot.png",
                    ImageAssetOptions::new().with_avif()
                ),
                alt: "Screenshot: Likes component",
                title: "",
            }
        }
        h3 { id: "borrowed-props",
            Link {
                to: BookRoute::DescribingUiComponentProps {
                    section: DescribingUiComponentPropsSection::BorrowedProps,
                },
                class: "header",
                "Borrowed Props"
            }
        }
        p {
            "Owned props work well if your props are easy to copy around – like a single number. But what if we need to pass a larger data type, like a String from an  "
            code { "App" }
            " Component to a  "
            code { "TitleCard" }
            " subcomponent? A naive solution might be to "
            Link { to: "https://doc.rust-lang.org/std/clone/trait.Clone.html",
                code { ".clone()" }
            }
            " the String, creating a copy of it for the subcomponent – but this would be inefficient, especially for larger Strings."
        }
        p {
            "Rust allows for something more efficient – borrowing the String as a  "
            code { "&str" }
            " – this is what Borrowed Props are for!"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Props)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">TitleCardProps&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    title: </span><span style=\"color:#f92672;\">&amp;&#39;a </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">TitleCard</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">, TitleCardProps&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;{{cx.props.title}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_borrowed_props.rs".to_string(),
        }
        p { "We can then use the component like this:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> hello </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;Hello Dioxus!&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(TitleCard {{ title: hello }}))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_borrowed_props.rs".to_string(),
        }
        p {
            img {
                src: asset!(
                    "/assets/static/images03/component_borrowed_props_screenshot.png",
                    ImageAssetOptions::new().with_avif()
                ),
                alt: "Screenshot: TitleCard component",
                title: "",
            }
        }
        p {
            "Borrowed props can be very useful, but they do not allow for memorization so they will "
            em { "always" }
            " rerun when the parent scope is rerendered. Because of this Borrowed Props should be reserved for components that are cheap to rerun or places where cloning data is an issue. Using Borrowed Props everywhere will result in large parts of your app rerunning every interaction."
        }
        h2 { id: "prop-options",
            Link {
                to: BookRoute::DescribingUiComponentProps {
                    section: DescribingUiComponentPropsSection::PropOptions,
                },
                class: "header",
                "Prop Options"
            }
        }
        p {
            "The  "
            code { "#[derive(Props)]" }
            " macro has some features that let you customize the behavior of props."
        }
        h3 { id: "optional-props",
            Link {
                to: BookRoute::DescribingUiComponentProps {
                    section: DescribingUiComponentPropsSection::OptionalProps,
                },
                class: "header",
                "Optional Props"
            }
        }
        p {
            "You can create optional fields by using the  "
            code { "Option<…>" }
            " type for a field:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Props)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">OptionalProps&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    title: </span><span style=\"color:#f92672;\">&amp;&#39;a </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    subtitle: Option&lt;</span><span style=\"color:#f92672;\">&amp;&#39;a </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Title</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">, OptionalProps&gt;) -&gt; Element&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(h1{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;{{cx.props.title}}: &quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        cx.props.subtitle.</span><span style=\"color:#66d9ef;\">unwrap_or</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;No subtitle provided&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    }}))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_props_options.rs".to_string(),
        }
        p { "Then, you can choose to either provide them or not:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">Title {{\n</span><span style=\"color:#f8f8f2;\">title: </span><span style=\"color:#ffee99;\">&quot;Some Title&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">Title {{\n</span><span style=\"color:#f8f8f2;\">title: </span><span style=\"color:#ffee99;\">&quot;Some Title&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">subtitle: </span><span style=\"color:#ffee99;\">&quot;Some Subtitle&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#8c8c8c;\">// Providing an Option explicitly won&#39;t compile though:\n</span><span style=\"color:#8c8c8c;\">// Title {{\n</span><span style=\"color:#8c8c8c;\">//     title: &quot;Some Title&quot;,\n</span><span style=\"color:#8c8c8c;\">//     subtitle: None,\n</span><span style=\"color:#8c8c8c;\">// }},</span></pre>\n",
            name: "component_props_options.rs".to_string(),
        }
        h3 { id: "explicitly-required-option-s",
            Link {
                to: BookRoute::DescribingUiComponentProps {
                    section: DescribingUiComponentPropsSection::ExplicitlyRequiredOptionS,
                },
                class: "header",
                "Explicitly Required Option s"
            }
        }
        p {
            "If you want to explicitly require an  "
            code { "Option" }
            ", and not an optional prop, you can annotate it with  "
            code { "#[props(!optional)]" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Props)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">ExplicitOptionProps&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    title: </span><span style=\"color:#f92672;\">&amp;&#39;a </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    #[props(!optional)]\n</span><span style=\"color:#f8f8f2;\">    subtitle: Option&lt;</span><span style=\"color:#f92672;\">&amp;&#39;a </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ExplicitOption</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">, ExplicitOptionProps&gt;) -&gt; Element&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(h1 {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;{{cx.props.title}}: &quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        cx.props.subtitle.</span><span style=\"color:#66d9ef;\">unwrap_or</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;No subtitle provided&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    }}))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_props_options.rs".to_string(),
        }
        p {
            "Then, you have to explicitly pass either  "
            code { "Some(\"str\")" }
            " or  "
            code { "None" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">ExplicitOption {{\n</span><span style=\"color:#f8f8f2;\">title: </span><span style=\"color:#ffee99;\">&quot;Some Title&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">subtitle: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">ExplicitOption {{\n</span><span style=\"color:#f8f8f2;\">title: </span><span style=\"color:#ffee99;\">&quot;Some Title&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">subtitle: </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Some Title&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#8c8c8c;\">// This won&#39;t compile:\n</span><span style=\"color:#8c8c8c;\">// ExplicitOption {{\n</span><span style=\"color:#8c8c8c;\">//     title: &quot;Some Title&quot;,\n</span><span style=\"color:#8c8c8c;\">// }},</span></pre>\n",
            name: "component_props_options.rs".to_string(),
        }
        h3 { id: "default-props",
            Link {
                to: BookRoute::DescribingUiComponentProps {
                    section: DescribingUiComponentPropsSection::DefaultProps,
                },
                class: "header",
                "Default Props"
            }
        }
        p {
            "You can use  "
            code { "#[props(default = 42)]" }
            " to make a field optional and specify its default value:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(PartialEq, Props)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">DefaultProps {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// default to 42 when not provided\n</span><span style=\"color:#f8f8f2;\">    #[props(default = 42)]\n</span><span style=\"color:#f8f8f2;\">    number: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">DefaultComponent</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;DefaultProps&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(h1 {{ </span><span style=\"color:#ffee99;\">&quot;{{cx.props.number}}&quot; </span><span style=\"color:#f8f8f2;\">}}))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_props_options.rs".to_string(),
        }
        p { "Then, similarly to optional props, you don't have to provide it:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">DefaultComponent {{\n</span><span style=\"color:#f8f8f2;\">number: </span><span style=\"color:#ff80f4;\">5</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">DefaultComponent {{}},</span></pre>\n",
            name: "component_props_options.rs".to_string(),
        }
        h3 { id: "automatic-conversion-with-into",
            Link {
                to: BookRoute::DescribingUiComponentProps {
                    section: DescribingUiComponentPropsSection::AutomaticConversionWithInto,
                },
                class: "header",
                "Automatic Conversion with .into"
            }
        }
        p {
            "It is common for Rust functions to accept  "
            code { "impl Into<SomeType>" }
            " rather than just  "
            code { "SomeType" }
            " to support a wider range of parameters. If you want similar functionality with props, you can use  "
            code { "#[props(into)]" }
            ". For example, you could add it on a  "
            code { "String" }
            " prop – and  "
            code { "&str" }
            " will also be automatically accepted, as it can be converted into  "
            code { "String" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(PartialEq, Props)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">IntoProps {{\n</span><span style=\"color:#f8f8f2;\">    #[props(into)]\n</span><span style=\"color:#f8f8f2;\">    string: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">IntoComponent</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;IntoProps&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(h1 {{ </span><span style=\"color:#ffee99;\">&quot;{{cx.props.string}}&quot; </span><span style=\"color:#f8f8f2;\">}}))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_props_options.rs".to_string(),
        }
        p { "Then, you can use it so:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">IntoComponent {{\n</span><span style=\"color:#f8f8f2;\">string: </span><span style=\"color:#ffee99;\">&quot;some &amp;str&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}},</span></pre>\n",
            name: "component_props_options.rs".to_string(),
        }
        h2 { id: "the-inline-props-macro",
            Link {
                to: BookRoute::DescribingUiComponentProps {
                    section: DescribingUiComponentPropsSection::TheInlinePropsMacro,
                },
                class: "header",
                "The inline_props macro"
            }
        }
        p {
            "So far, every Component function we've seen had a corresponding ComponentProps struct to pass in props. This was quite verbose... Wouldn't it be nice to have props as simple function arguments? Then we wouldn't need to define a Props struct, and instead of typing  "
            code { "cx.props.whatever" }
            ", we could just use  "
            code { "whatever" }
            " directly!"
        }
        p {
            code { "inline_props" }
            " allows you to do just that. Instead of typing the \"full\" version:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Props, PartialEq)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">TitleCardProps {{\n</span><span style=\"color:#f8f8f2;\">    title: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">TitleCard</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;TitleCardProps&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;{{cx.props.title}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "...you can define a function that accepts props as arguments. Then, just annotate it with  "
            code { "#[inline_props]" }
            ", and the macro will turn it into a regular Component for you:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[inline_props]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">TitleCard</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope, </span><span style=\"font-style:italic;color:#fd971f;\">title</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;{{title}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        blockquote {
            p {
                "While the new Component is shorter and easier to read, this macro should not be used by library authors since you have less control over Prop documentation."
            }
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum DescribingUiComponentChildrenSection {
    #[default]
    Empty,
    ComponentChildren,
    TheChildrenField,
}
impl std::str::FromStr for DescribingUiComponentChildrenSection {
    type Err = DescribingUiComponentChildrenSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "component-children" => Ok(Self::ComponentChildren),
            "the-children-field" => Ok(Self::TheChildrenField),
            _ => Err(DescribingUiComponentChildrenSectionParseError),
        }
    }
}
impl std::fmt::Display for DescribingUiComponentChildrenSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::ComponentChildren => f.write_str("component-children"),
            Self::TheChildrenField => f.write_str("the-children-field"),
        }
    }
}
#[derive(Debug)]
pub struct DescribingUiComponentChildrenSectionParseError;
impl std::fmt::Display for DescribingUiComponentChildrenSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of DescribingUiComponentChildrenSectioncomponent-children, the-children-field",
        )?;
        Ok(())
    }
}
impl std::error::Error for DescribingUiComponentChildrenSectionParseError {}
#[component(no_case_check)]
pub fn DescribingUiComponentChildren(
    section: DescribingUiComponentChildrenSection,
) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "component-children",
            Link {
                to: BookRoute::DescribingUiComponentChildren {
                    section: DescribingUiComponentChildrenSection::ComponentChildren,
                },
                class: "header",
                "Component Children"
            }
        }
        p {
            "In some cases, you may wish to create a component that acts as a container for some other content, without the component needing to know what that content is. To achieve this, create a prop of type  "
            code { "Element" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Props)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">ClickableProps&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    href: </span><span style=\"color:#f92672;\">&amp;&#39;a </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    body: Element&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Clickable</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">, ClickableProps&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">        a {{\n</span><span style=\"color:#f8f8f2;\">            href: </span><span style=\"color:#ffee99;\">&quot;{{cx.props.href}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            class: </span><span style=\"color:#ffee99;\">&quot;fancy-button&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx.props.body\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    ))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_element_props.rs".to_string(),
        }
        p {
            "Then, when rendering the component, you can pass in the output of  "
            code { "cx.render(rsx!(...))" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">    Clickable {{\n</span><span style=\"color:#f8f8f2;\">        href: </span><span style=\"color:#ffee99;\">&quot;https://www.youtube.com/watch?v=C-M2hs3sXGo&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        body: cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(</span><span style=\"color:#ffee99;\">&quot;How to &quot;</span><span style=\"color:#f8f8f2;\"> i {{</span><span style=\"color:#ffee99;\">&quot;not&quot;</span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#ffee99;\">&quot; be seen&quot;</span><span style=\"color:#f8f8f2;\">)),\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "component_element_props.rs".to_string(),
        }
        blockquote {
            p {
                "Note: Since  "
                code { "Element<'a>" }
                " is a borrowed prop, there will be no memoization."
            }
        }
        blockquote {
            p {
                "Warning: While it may compile, do not include the same  "
                code { "Element" }
                " more than once in the RSX. The resulting behavior is unspecified."
            }
        }
        h2 { id: "the-children-field",
            Link {
                to: BookRoute::DescribingUiComponentChildren {
                    section: DescribingUiComponentChildrenSection::TheChildrenField,
                },
                class: "header",
                "The children field"
            }
        }
        p {
            "Rather than passing the RSX through a regular prop, you may wish to accept children similarly to how elements can have children. The \"magic\"  "
            code { "children" }
            " prop lets you achieve this:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Props)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">ClickableProps&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    href: </span><span style=\"color:#f92672;\">&amp;&#39;a </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    children: Element&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Clickable</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">, ClickableProps&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">        a {{\n</span><span style=\"color:#f8f8f2;\">            href: </span><span style=\"color:#ffee99;\">&quot;{{cx.props.href}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            class: </span><span style=\"color:#ffee99;\">&quot;fancy-button&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">cx.props.children\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    ))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_children.rs".to_string(),
        }
        p {
            "This makes using the component much simpler: simply put the RSX inside the  "
            code { "{{}}" }
            " brackets – and there is no need for a  "
            code { "render" }
            " call or another macro!"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">    Clickable {{\n</span><span style=\"color:#f8f8f2;\">        href: </span><span style=\"color:#ffee99;\">&quot;https://www.youtube.com/watch?v=C-M2hs3sXGo&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;How to &quot;</span><span style=\"color:#f8f8f2;\"> i {{</span><span style=\"color:#ffee99;\">&quot;not&quot;</span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#ffee99;\">&quot; be seen&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "component_children.rs".to_string(),
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum InteractivityIndexSection {
    #[default]
    Empty,
    Interactivity,
}
impl std::str::FromStr for InteractivityIndexSection {
    type Err = InteractivityIndexSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "interactivity" => Ok(Self::Interactivity),
            _ => Err(InteractivityIndexSectionParseError),
        }
    }
}
impl std::fmt::Display for InteractivityIndexSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Interactivity => f.write_str("interactivity"),
        }
    }
}
#[derive(Debug)]
pub struct InteractivityIndexSectionParseError;
impl std::fmt::Display for InteractivityIndexSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of InteractivityIndexSectioninteractivity",
        )?;
        Ok(())
    }
}
impl std::error::Error for InteractivityIndexSectionParseError {}
#[component(no_case_check)]
pub fn InteractivityIndex(section: InteractivityIndexSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "interactivity",
            Link {
                to: BookRoute::InteractivityIndex {
                    section: InteractivityIndexSection::Interactivity,
                },
                class: "header",
                "Interactivity"
            }
        }
        p {
            "So far, we've learned how to describe the structure and properties of our user interfaces. However, most interfaces need to be interactive in order to be useful. In this chapter, we describe how to make a Dioxus app that responds to the user."
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum InteractivityEventHandlersSection {
    #[default]
    Empty,
    EventHandlers,
    TheEventObject,
    EventPropagation,
    PreventDefault,
    HandlerProps,
}
impl std::str::FromStr for InteractivityEventHandlersSection {
    type Err = InteractivityEventHandlersSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "event-handlers" => Ok(Self::EventHandlers),
            "the-event-object" => Ok(Self::TheEventObject),
            "event-propagation" => Ok(Self::EventPropagation),
            "prevent-default" => Ok(Self::PreventDefault),
            "handler-props" => Ok(Self::HandlerProps),
            _ => Err(InteractivityEventHandlersSectionParseError),
        }
    }
}
impl std::fmt::Display for InteractivityEventHandlersSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::EventHandlers => f.write_str("event-handlers"),
            Self::TheEventObject => f.write_str("the-event-object"),
            Self::EventPropagation => f.write_str("event-propagation"),
            Self::PreventDefault => f.write_str("prevent-default"),
            Self::HandlerProps => f.write_str("handler-props"),
        }
    }
}
#[derive(Debug)]
pub struct InteractivityEventHandlersSectionParseError;
impl std::fmt::Display for InteractivityEventHandlersSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of InteractivityEventHandlersSectionevent-handlers, the-event-object, event-propagation, prevent-default, handler-props",
        )?;
        Ok(())
    }
}
impl std::error::Error for InteractivityEventHandlersSectionParseError {}
#[component(no_case_check)]
pub fn InteractivityEventHandlers(
    section: InteractivityEventHandlersSection,
) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "event-handlers",
            Link {
                to: BookRoute::InteractivityEventHandlers {
                    section: InteractivityEventHandlersSection::EventHandlers,
                },
                class: "header",
                "Event Handlers"
            }
        }
        p {
            "Event handlers are used to respond to user actions. For example, an event handler could be triggered when the user clicks, scrolls, moves the mouse, or types a character."
        }
        p {
            "Event handlers are attached to elements. For example, we usually don't care about all the clicks that happen within an app, only those on a particular button."
        }
        p {
            "Event handlers are similar to regular attributes, but their name usually starts with  "
            code { "on" }
            "- and they accept closures as values. The closure will be called whenever the event it listens for is triggered and will be passed that event."
        }
        p {
            "For example, to handle clicks on an element, we can specify an  "
            code { "onclick" }
            " handler:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">button {{\n</span><span style=\"color:#f8f8f2;\">    onclick: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;Clicked! Event: </span><span style=\"color:#ff80f4;\">{{event:?}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;click me!&quot;\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "event_click.rs".to_string(),
        }
        h2 { id: "the-event-object",
            Link {
                to: BookRoute::InteractivityEventHandlers {
                    section: InteractivityEventHandlersSection::TheEventObject,
                },
                class: "header",
                "The Event object"
            }
        }
        p {
            "Event handlers receive an "
            Link { to: "https://docs.rs/dioxus-core/latest/dioxus_core/struct.Event.html",
                code { "Event" }
            }
            " object containing information about the event. Different types of events contain different types of data. For example, mouse-related events contain "
            Link { to: "https://docs.rs/dioxus/latest/dioxus/events/struct.MouseData.html",
                code { "MouseData" }
            }
            ", which tells you things like where the mouse was clicked and what mouse buttons were used."
        }
        p { "In the example above, this event data was logged to the terminal:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">Clicked</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\"> Event: UiEvent {{ bubble_state: Cell {{ value: </span><span style=\"color:#ff80f4;\">true </span><span style=\"color:#f8f8f2;\">}}, data: MouseData {{ coordinates: Coordinates {{ screen: (</span><span style=\"color:#ff80f4;\">242.0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">256.0</span><span style=\"color:#f8f8f2;\">), client: (</span><span style=\"color:#ff80f4;\">26.0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">17.0</span><span style=\"color:#f8f8f2;\">), element: (</span><span style=\"color:#ff80f4;\">16.0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">7.0</span><span style=\"color:#f8f8f2;\">), page: (</span><span style=\"color:#ff80f4;\">26.0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">17.0</span><span style=\"color:#f8f8f2;\">) }}, modifiers: (empty), held_buttons: EnumSet(), trigger_button: </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(Primary) }} }}\n</span><span style=\"color:#f8f8f2;\">Clicked</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\"> Event: UiEvent {{ bubble_state: Cell {{ value: </span><span style=\"color:#ff80f4;\">true </span><span style=\"color:#f8f8f2;\">}}, data: MouseData {{ coordinates: Coordinates {{ screen: (</span><span style=\"color:#ff80f4;\">242.0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">256.0</span><span style=\"color:#f8f8f2;\">), client: (</span><span style=\"color:#ff80f4;\">26.0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">17.0</span><span style=\"color:#f8f8f2;\">), element: (</span><span style=\"color:#ff80f4;\">16.0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">7.0</span><span style=\"color:#f8f8f2;\">), page: (</span><span style=\"color:#ff80f4;\">26.0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">17.0</span><span style=\"color:#f8f8f2;\">) }}, modifiers: (empty), held_buttons: EnumSet(), trigger_button: </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(Primary) }} }}</span></pre>\n",
        }
        p {
            "To learn what the different event types for HTML provide, read the "
            Link { to: "https://docs.rs/dioxus-html/latest/dioxus_html/events/index.html",
                "events module docs"
            }
            "."
        }
        h3 { id: "event-propagation",
            Link {
                to: BookRoute::InteractivityEventHandlers {
                    section: InteractivityEventHandlersSection::EventPropagation,
                },
                class: "header",
                "Event propagation"
            }
        }
        p {
            "Some events will trigger first on the element the event originated at upward. For example, a click event on a  "
            code { "button" }
            " inside a  "
            code { "div" }
            " would first trigger the button's event listener and then the div's event listener."
        }
        blockquote {
            p {
                "For more information about event propigation see "
                Link { to: "https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Building_blocks/Events#event_bubbling",
                    "the mdn docs on event bubling"
                }
            }
        }
        p {
            "If you want to prevent this behavior, you can call  "
            code { "stop_propogation()" }
            " on the event:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">div {{\n</span><span style=\"color:#f8f8f2;\">    onclick: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">_event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{}},\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;outer&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    button {{\n</span><span style=\"color:#f8f8f2;\">        onclick: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// now, outer won&#39;t be triggered\n</span><span style=\"color:#f8f8f2;\">            event.</span><span style=\"color:#66d9ef;\">stop_propagation</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;inner&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "event_nested.rs".to_string(),
        }
        h2 { id: "prevent-default",
            Link {
                to: BookRoute::InteractivityEventHandlers {
                    section: InteractivityEventHandlersSection::PreventDefault,
                },
                class: "header",
                "Prevent Default"
            }
        }
        p {
            "Some events have a default behavior. For keyboard events, this might be entering the typed character. For mouse events, this might be selecting some text."
        }
        p {
            "In some instances, might want to avoid this default behavior. For this, you can add the  "
            code { "prevent_default" }
            " attribute with the name of the handler whose default behavior you want to stop. This attribute is special: you can attach it multiple times for multiple attributes:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">input {{\n</span><span style=\"color:#f8f8f2;\">    prevent_default: </span><span style=\"color:#ffee99;\">&quot;oninput&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    prevent_default: </span><span style=\"color:#ffee99;\">&quot;onclick&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "event_prevent_default.rs".to_string(),
        }
        p { "Any event handlers will still be called." }
        blockquote {
            p {
                "Normally, in React or JavaScript, you'd call \"preventDefault\" on the event in the callback. Dioxus does "
                em { "not" }
                " currently support this behavior. Note: this means you cannot conditionally prevent default behavior based on the data in the event."
            }
        }
        h2 { id: "handler-props",
            Link {
                to: BookRoute::InteractivityEventHandlers {
                    section: InteractivityEventHandlersSection::HandlerProps,
                },
                class: "header",
                "Handler Props"
            }
        }
        p {
            "Sometimes, you might want to make a component that accepts an event handler. A simple example would be a  "
            code { "FancyButton" }
            " component, which accepts an  "
            code { "on_click" }
            " handler:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Props)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">FancyButtonProps&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    on_click: EventHandler&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">, MouseEvent&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">FancyButton</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">, FancyButtonProps&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;&gt;) -&gt; Element&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(button {{\n</span><span style=\"color:#f8f8f2;\">        class: </span><span style=\"color:#ffee99;\">&quot;fancy-button&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        onclick: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">evt</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> cx.props.on_click.</span><span style=\"color:#66d9ef;\">call</span><span style=\"color:#f8f8f2;\">(evt),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;click me pls.&quot;\n</span><span style=\"color:#f8f8f2;\">    }}))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "event_handler_prop.rs".to_string(),
        }
        p { "Then, you can use it like any other handler:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">    FancyButton {{\n</span><span style=\"color:#f8f8f2;\">        on_click: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;Clicked! </span><span style=\"color:#ff80f4;\">{{event:?}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "event_handler_prop.rs".to_string(),
        }
        blockquote {
            p {
                "Note: just like any other attribute, you can name the handlers anything you want! Though they must start with  "
                code { "on" }
                ", for the prop to be automatically turned into an  "
                code { "EventHandler" }
                " at the call site."
            }
            p {
                "You can also put custom data in the event, rather than e.g.  "
                code { "MouseData" }
            }
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum InteractivityHooksSection {
    #[default]
    Empty,
    HooksAndComponentState,
    UseStateHook,
    RulesOfHooks,
    NoHooksInConditionals,
    NoHooksInClosures,
    NoHooksInLoops,
    UseRefHook,
}
impl std::str::FromStr for InteractivityHooksSection {
    type Err = InteractivityHooksSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "hooks-and-component-state" => Ok(Self::HooksAndComponentState),
            "use-state-hook" => Ok(Self::UseStateHook),
            "rules-of-hooks" => Ok(Self::RulesOfHooks),
            "no-hooks-in-conditionals" => Ok(Self::NoHooksInConditionals),
            "no-hooks-in-closures" => Ok(Self::NoHooksInClosures),
            "no-hooks-in-loops" => Ok(Self::NoHooksInLoops),
            "use-ref-hook" => Ok(Self::UseRefHook),
            _ => Err(InteractivityHooksSectionParseError),
        }
    }
}
impl std::fmt::Display for InteractivityHooksSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::HooksAndComponentState => f.write_str("hooks-and-component-state"),
            Self::UseStateHook => f.write_str("use-state-hook"),
            Self::RulesOfHooks => f.write_str("rules-of-hooks"),
            Self::NoHooksInConditionals => f.write_str("no-hooks-in-conditionals"),
            Self::NoHooksInClosures => f.write_str("no-hooks-in-closures"),
            Self::NoHooksInLoops => f.write_str("no-hooks-in-loops"),
            Self::UseRefHook => f.write_str("use-ref-hook"),
        }
    }
}
#[derive(Debug)]
pub struct InteractivityHooksSectionParseError;
impl std::fmt::Display for InteractivityHooksSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of InteractivityHooksSectionhooks-and-component-state, use-state-hook, rules-of-hooks, no-hooks-in-conditionals, no-hooks-in-closures, no-hooks-in-loops, use-ref-hook",
        )?;
        Ok(())
    }
}
impl std::error::Error for InteractivityHooksSectionParseError {}
#[component(no_case_check)]
pub fn InteractivityHooks(section: InteractivityHooksSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "hooks-and-component-state",
            Link {
                to: BookRoute::InteractivityHooks {
                    section: InteractivityHooksSection::HooksAndComponentState,
                },
                class: "header",
                "Hooks and Component State"
            }
        }
        p {
            "So far our components have had no state like a normal rust functions. However, in a UI component, it is often useful to have stateful functionality to build user interactions. For example, you might want to track whether the user has opened a drop-down, and render different things accordingly."
        }
        p {
            "Hooks allow us to create state in our components. Hooks are Rust functions that take a reference to  "
            code { "ScopeState" }
            " (in a component, you can pass  "
            code { "cx" }
            "), and provide you with functionality and state."
        }
        h2 { id: "use-state-hook",
            Link {
                to: BookRoute::InteractivityHooks {
                    section: InteractivityHooksSection::UseStateHook,
                },
                class: "header",
                "use_state Hook"
            }
        }
        p {
            Link { to: "https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_state.html",
                code { "use_state" }
            }
            " is one of the simplest hooks."
        }
        ul {
            li { "You provide a closure that determines the initial value" }
            li {
                code { "use_state" }
                " gives you the current value, and a way to update it by setting it to something else"
            }
            li {
                "When the value updates, "
                code { "use_state" }
                " makes the component re-render, and provides you with the new value"
            }
        }
        p {
            "For example, you might have seen the counter example, in which state (a number) is tracked using the  "
            code { "use_state" }
            " hook:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// count will be initialized to 0 the first time the component is rendered\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;High-Five counter: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// changing the count will cause the component to re-render\n</span><span style=\"color:#f8f8f2;\">                count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Up high!&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// changing the count will cause the component to re-render\n</span><span style=\"color:#f8f8f2;\">                count </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Down low!&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    ))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_counter.rs".to_string(),
        }
        p {
            img {
                src: asset!("/assets/static/images03/counter.png", ImageAssetOptions::new().with_avif()),
                alt: "Screenshot: counter app",
                title: "",
            }
        }
        p {
            "Every time the component's state changes, it re-renders, and the component function is called, so you can describe what you want the new UI to look like. You don't have to worry about \"changing\" anything – just describe what you want in terms of the state, and Dioxus will take care of the rest!"
        }
        blockquote {
            p {
                code { "use_state" }
                " returns your value wrapped in a smart pointer of type "
                Link { to: "https://docs.rs/dioxus/latest/dioxus/prelude/struct.UseState.html",
                    code { "UseState" }
                }
                ". This is why you can both read the value and update it, even within an event handler."
            }
        }
        p { "You can use multiple hooks in the same component if you want:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count_a </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count_b </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;Counter_a: {{count_a}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count_a </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;a++&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count_a </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;a--&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;Counter_b: {{count_b}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count_b </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;b++&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count_b </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;b--&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    ))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_counter_two_state.rs".to_string(),
        }
        p {
            img {
                src: asset!(
                    "/assets/static/images03/counter_two_state.png", ImageAssetOptions::new()
                    .with_avif()
                ),
                alt: "Screenshot: app with two counters",
                title: "",
            }
        }
        h2 { id: "rules-of-hooks",
            Link {
                to: BookRoute::InteractivityHooks {
                    section: InteractivityHooksSection::RulesOfHooks,
                },
                class: "header",
                "Rules of Hooks"
            }
        }
        p {
            "The above example might seem a bit magic, since Rust functions are typically not associated with state. Dioxus allows hooks to maintain state across renders through a reference to  "
            code { "ScopeState" }
            ", which is why you must pass  "
            code { "&cx" }
            " to them."
        }
        p {
            "But how can Dioxus differentiate between multiple hooks in the same component? As you saw in the second example, both  "
            code { "use_state" }
            " functions were called with the same parameters, so how come they can return different things when the counters are different?"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count_a </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count_b </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "hooks_counter_two_state.rs".to_string(),
        }
        p {
            "This is only possible because the two hooks are always called in the same order, so Dioxus knows which is which. Because the order you call hooks matters, you must follow certain rules when using hooks:"
        }
        ol {
            li { "Hooks may be only used in components or other hooks (we'll get to that later)" }
            li {
                "On every call to the component function"
                ol {
                    li { "The same hooks must be called" }
                    li { "In the same order" }
                }
            }
            li {
                "Hooks name's should start with "
                code { "use_" }
                " so you don't accidentally confuse them with regular functions"
            }
        }
        p { "These rules mean that there are certain things you can't do with hooks:" }
        h3 { id: "no-hooks-in-conditionals",
            Link {
                to: BookRoute::InteractivityHooks {
                    section: InteractivityHooksSection::NoHooksInConditionals,
                },
                class: "header",
                "No Hooks in Conditionals"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ don&#39;t call hooks in conditionals!\n</span><span style=\"color:#8c8c8c;\">// We must ensure that the same hooks will be called every time\n</span><span style=\"color:#8c8c8c;\">// But `if` statements only run if the conditional is true!\n</span><span style=\"color:#8c8c8c;\">// So we might violate rule 2.\n</span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> you_are_happy </span><span style=\"color:#f92672;\">&amp;&amp;</span><span style=\"color:#f8f8f2;\"> you_know_it {{\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> something </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ffee99;\">&quot;hands&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;clap your </span><span style=\"color:#ff80f4;\">{{something}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ instead, *always* call use_state\n</span><span style=\"color:#8c8c8c;\">// You can put other stuff in the conditional though\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> something </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ffee99;\">&quot;hands&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> you_are_happy </span><span style=\"color:#f92672;\">&amp;&amp;</span><span style=\"color:#f8f8f2;\"> you_know_it {{\n</span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;clap your </span><span style=\"color:#ff80f4;\">{{something}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_bad.rs".to_string(),
        }
        h3 { id: "no-hooks-in-closures",
            Link {
                to: BookRoute::InteractivityHooks {
                    section: InteractivityHooksSection::NoHooksInClosures,
                },
                class: "header",
                "No Hooks in Closures"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ don&#39;t call hooks inside closures!\n</span><span style=\"color:#8c8c8c;\">// We can&#39;t guarantee that the closure, if used, will be called in the same order every time\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#a6e22e;\">_a </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">|| {{\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> b </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">b.</span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ instead, move hook `b` outside\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> b </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#a6e22e;\">_a </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">|| b.</span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">();</span></pre>\n",
            name: "hooks_bad.rs".to_string(),
        }
        h3 { id: "no-hooks-in-loops",
            Link {
                to: BookRoute::InteractivityHooks {
                    section: InteractivityHooksSection::NoHooksInLoops,
                },
                class: "header",
                "No Hooks in Loops"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// `names` is a Vec&lt;&amp;str&gt;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ❌ Do not use hooks in loops!\n</span><span style=\"color:#8c8c8c;\">// In this case, if the length of the Vec changes, we break rule 2\n</span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> _name </span><span style=\"color:#f92672;\">in &amp;</span><span style=\"color:#f8f8f2;\">names {{\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> is_selected </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">false</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;selected: </span><span style=\"color:#ff80f4;\">{{is_selected}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Instead, use a hashmap with use_ref\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> selection_map </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_ref</span><span style=\"color:#f8f8f2;\">(cx, HashMap::&lt;</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#66d9ef;\">bool</span><span style=\"color:#f8f8f2;\">&gt;::new);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">in &amp;</span><span style=\"color:#f8f8f2;\">names {{\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> is_selected </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> selection_map.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">()[name];\n</span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;selected: </span><span style=\"color:#ff80f4;\">{{is_selected}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_bad.rs".to_string(),
        }
        h2 { id: "use-ref-hook",
            Link {
                to: BookRoute::InteractivityHooks {
                    section: InteractivityHooksSection::UseRefHook,
                },
                class: "header",
                "use_ref Hook"
            }
        }
        p {
            code { "use_state" }
            " is great for tracking simple values. However, you may notice in the "
            Link { to: "https://docs.rs/dioxus/latest/dioxus/hooks/struct.UseState.html",
                code { "UseState" }
                " API"
            }
            " that the only way to modify its value is to replace it with something else (e.g., by calling "
            code { "set" }
            ", or through one of the "
            code { "+=" }
            ", "
            code { "-=" }
            " operators). This works well when it is cheap to construct a value (such as any primitive). But what if you want to maintain more complex data in the components state?"
        }
        p {
            "For example, suppose we want to maintain a  "
            code { "Vec" }
            " of values. If we stored it with  "
            code { "use_state" }
            ", the only way to add a new value to the list would be to create a new  "
            code { "Vec" }
            " with the additional value, and put it in the state. This is expensive! We want to modify the existing  "
            code { "Vec" }
            " instead."
        }
        p {
            "Thankfully, there is another hook for that,  "
            code { "use_ref" }
            "! It is similar to  "
            code { "use_state" }
            ", but it lets you get a mutable reference to the contained data."
        }
        p {
            "Here's a simple example that keeps a list of events in a  "
            code { "use_ref" }
            ". We can acquire write access to the state with  "
            code { ".with_mut()" }
            ", and then just  "
            code { ".push" }
            " a new value to the state:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> list </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_ref</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"font-style:italic;color:#66d9ef;\">Vec</span><span style=\"color:#f8f8f2;\">::new);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">        p {{ </span><span style=\"color:#ffee99;\">&quot;Current list: {{list.read():?}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                list.</span><span style=\"color:#66d9ef;\">with_mut</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">list</span><span style=\"color:#f8f8f2;\">| list.</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(event));\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Click me!&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    ))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_use_ref.rs".to_string(),
        }
        blockquote {
            p {
                "The return values of  "
                code { "use_state" }
                " and  "
                code { "use_ref" }
                " ( "
                code { "UseState" }
                " and  "
                code { "UseRef" }
                ", respectively) are in some ways similar to "
                Link { to: "https://doc.rust-lang.org/std/cell/",
                    code { "Cell" }
                }
                " and "
                Link { to: "https://doc.rust-lang.org/std/cell/struct.RefCell.html",
                    code { "RefCell" }
                }
                " – they provide interior mutability. However, these Dioxus wrappers also ensure that the component gets re-rendered whenever you change the state."
            }
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum InteractivityUserInputSection {
    #[default]
    Empty,
    UserInput,
    ControlledInputs,
    UncontrolledInputs,
}
impl std::str::FromStr for InteractivityUserInputSection {
    type Err = InteractivityUserInputSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "user-input" => Ok(Self::UserInput),
            "controlled-inputs" => Ok(Self::ControlledInputs),
            "uncontrolled-inputs" => Ok(Self::UncontrolledInputs),
            _ => Err(InteractivityUserInputSectionParseError),
        }
    }
}
impl std::fmt::Display for InteractivityUserInputSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::UserInput => f.write_str("user-input"),
            Self::ControlledInputs => f.write_str("controlled-inputs"),
            Self::UncontrolledInputs => f.write_str("uncontrolled-inputs"),
        }
    }
}
#[derive(Debug)]
pub struct InteractivityUserInputSectionParseError;
impl std::fmt::Display for InteractivityUserInputSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of InteractivityUserInputSectionuser-input, controlled-inputs, uncontrolled-inputs",
        )?;
        Ok(())
    }
}
impl std::error::Error for InteractivityUserInputSectionParseError {}
#[component(no_case_check)]
pub fn InteractivityUserInput(section: InteractivityUserInputSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "user-input",
            Link {
                to: BookRoute::InteractivityUserInput {
                    section: InteractivityUserInputSection::UserInput,
                },
                class: "header",
                "User Input"
            }
        }
        p {
            "Interfaces often need to provide a way to input data: e.g. text, numbers, checkboxes, etc. In Dioxus, there are two ways you can work with user input."
        }
        h2 { id: "controlled-inputs",
            Link {
                to: BookRoute::InteractivityUserInput {
                    section: InteractivityUserInputSection::ControlledInputs,
                },
                class: "header",
                "Controlled Inputs"
            }
        }
        p {
            "With controlled inputs, you are directly in charge of the state of the input. This gives you a lot of flexibility, and makes it easy to keep things in sync. For example, this is how you would create a controlled text input:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ffee99;\">&quot;bob&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        input {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// we tell the component what to render\n</span><span style=\"color:#f8f8f2;\">            value: </span><span style=\"color:#ffee99;\">&quot;{{name}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// and what to do when the value changes\n</span><span style=\"color:#f8f8f2;\">            oninput: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">evt</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> name.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(evt.value.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">()),\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "input_controlled.rs".to_string(),
        }
        p { "Notice the flexibility – you can:" }
        ul {
            li { "Also display the same contents in another element, and they will be in sync" }
            li { "Transform the input every time it is modified (e.g. to make sure it is upper case)" }
            li { "Validate the input every time it changes" }
            li {
                "Have custom logic happening when the input changes (e.g. network request for autocompletion)"
            }
            li {
                "Programmatically change the value (e.g. a \"randomize\" button that fills the input with nonsense)"
            }
        }
        h2 { id: "uncontrolled-inputs",
            Link {
                to: BookRoute::InteractivityUserInput {
                    section: InteractivityUserInputSection::UncontrolledInputs,
                },
                class: "header",
                "Uncontrolled Inputs"
            }
        }
        p {
            "As an alternative to controlled inputs, you can simply let the platform keep track of the input values. If we don't tell a HTML input what content it should have, it will be editable anyway (this is built into the browser). This approach can be more performant, but less flexible. For example, it's harder to keep the input in sync with another element."
        }
        p {
            "Since you don't necessarily have the current value of the uncontrolled input in state, you can access it either by listening to  "
            code { "oninput" }
            " events (similarly to controlled components), or, if the input is part of a form, you can access the form data in the form events (e.g.  "
            code { "oninput" }
            " or  "
            code { "onsubmit" }
            "):"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        form {{\n</span><span style=\"color:#f8f8f2;\">            onsubmit: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                println!(</span><span style=\"color:#ffee99;\">&quot;Submitted! </span><span style=\"color:#ff80f4;\">{{event:?}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            input {{ name: </span><span style=\"color:#ffee99;\">&quot;name&quot;</span><span style=\"color:#f8f8f2;\">, }},\n</span><span style=\"color:#f8f8f2;\">            input {{ name: </span><span style=\"color:#ffee99;\">&quot;age&quot;</span><span style=\"color:#f8f8f2;\">, }},\n</span><span style=\"color:#f8f8f2;\">            input {{ name: </span><span style=\"color:#ffee99;\">&quot;date&quot;</span><span style=\"color:#f8f8f2;\">, }},\n</span><span style=\"color:#f8f8f2;\">            input {{ r</span><span style=\"color:#f92672;\">#</span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;submit&quot;</span><span style=\"color:#f8f8f2;\">, }},\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "input_uncontrolled.rs".to_string(),
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">Submitted</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\"> UiEvent {{ data: FormData {{ value: </span><span style=\"color:#ffee99;\">&quot;&quot;</span><span style=\"color:#f8f8f2;\">, values: {{</span><span style=\"color:#ffee99;\">&quot;age&quot;</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;very old&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;date&quot;</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;1966&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;name&quot;</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;Fred&quot;</span><span style=\"color:#f8f8f2;\">}} }} }}</span></pre>\n" }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum InteractivitySharingStateSection {
    #[default]
    Empty,
    SharingState,
    LiftingState,
    UsingContext,
}
impl std::str::FromStr for InteractivitySharingStateSection {
    type Err = InteractivitySharingStateSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "sharing-state" => Ok(Self::SharingState),
            "lifting-state" => Ok(Self::LiftingState),
            "using-context" => Ok(Self::UsingContext),
            _ => Err(InteractivitySharingStateSectionParseError),
        }
    }
}
impl std::fmt::Display for InteractivitySharingStateSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::SharingState => f.write_str("sharing-state"),
            Self::LiftingState => f.write_str("lifting-state"),
            Self::UsingContext => f.write_str("using-context"),
        }
    }
}
#[derive(Debug)]
pub struct InteractivitySharingStateSectionParseError;
impl std::fmt::Display for InteractivitySharingStateSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of InteractivitySharingStateSectionsharing-state, lifting-state, using-context",
        )?;
        Ok(())
    }
}
impl std::error::Error for InteractivitySharingStateSectionParseError {}
#[component(no_case_check)]
pub fn InteractivitySharingState(
    section: InteractivitySharingStateSection,
) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "sharing-state",
            Link {
                to: BookRoute::InteractivitySharingState {
                    section: InteractivitySharingStateSection::SharingState,
                },
                class: "header",
                "Sharing State"
            }
        }
        p {
            "Often, multiple components need to access the same state. Depending on your needs, there are several ways to implement this."
        }
        h2 { id: "lifting-state",
            Link {
                to: BookRoute::InteractivitySharingState {
                    section: InteractivitySharingStateSection::LiftingState,
                },
                class: "header",
                "Lifting State"
            }
        }
        p {
            "One approach to share state between components is to \"lift\" it up to the nearest common ancestor. This means putting the  "
            code { "use_state" }
            " hook in a parent component, and passing the needed values down as props."
        }
        p {
            "Suppose we want to build a meme editor. We want to have an input to edit the meme caption, but also a preview of the meme with the caption. Logically, the meme and the input are 2 separate components, but they need access to the same state (the current caption)."
        }
        blockquote {
            p {
                "Of course, in this simple example, we could write everything in one component – but it is better to split everything out in smaller components to make the code more reusable, maintainable, and performant (this is even more important for larger, complex apps)."
            }
        }
        p {
            "We start with a  "
            code { "Meme" }
            " component, responsible for rendering a meme with a given caption:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[inline_props]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Meme</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;, </span><span style=\"font-style:italic;color:#fd971f;\">caption</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;&#39;a </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">) -&gt; Element&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> container_style </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;\n</span><span style=\"color:#ffee99;\">        position: relative;\n</span><span style=\"color:#ffee99;\">        width: fit-content;\n</span><span style=\"color:#ffee99;\">    &quot;#</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> caption_container_style </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;\n</span><span style=\"color:#ffee99;\">        position: absolute;\n</span><span style=\"color:#ffee99;\">        bottom: 0;\n</span><span style=\"color:#ffee99;\">        left: 0;\n</span><span style=\"color:#ffee99;\">        right: 0;\n</span><span style=\"color:#ffee99;\">        padding: 16px 8px;\n</span><span style=\"color:#ffee99;\">    &quot;#</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> caption_style </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">&quot;\n</span><span style=\"color:#ffee99;\">        font-size: 32px;\n</span><span style=\"color:#ffee99;\">        margin: 0;\n</span><span style=\"color:#ffee99;\">        color: white;\n</span><span style=\"color:#ffee99;\">        text-align: center;\n</span><span style=\"color:#ffee99;\">    &quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            style: </span><span style=\"color:#ffee99;\">&quot;{{container_style}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            img {{\n</span><span style=\"color:#f8f8f2;\">                src: </span><span style=\"color:#ffee99;\">&quot;https://i.imgflip.com/2zh47r.jpg&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                height: </span><span style=\"color:#ffee99;\">&quot;500px&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            div {{\n</span><span style=\"color:#f8f8f2;\">                style: </span><span style=\"color:#ffee99;\">&quot;{{caption_container_style}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                p {{\n</span><span style=\"color:#f8f8f2;\">                    style: </span><span style=\"color:#ffee99;\">&quot;{{caption_style}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#ffee99;\">&quot;{{caption}}&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    ))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "meme_editor.rs".to_string(),
        }
        blockquote {
            p {
                "Note that the  "
                code { "Meme" }
                " component is unaware where the caption is coming from – it could be stored in  "
                code { "use_state" }
                ",  "
                code { "use_ref" }
                ", or a constant. This ensures that it is very reusable – the same component can be used for a meme gallery without any changes!"
            }
        }
        p {
            "We also create a caption editor, completely decoupled from the meme. The caption editor must not store the caption itself – otherwise, how will we provide it to the  "
            code { "Meme" }
            " component? Instead, it should accept the current caption as a prop, as well as an event handler to delegate input events to:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[inline_props]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">CaptionEditor</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">caption</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;&#39;a </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">on_input</span><span style=\"color:#f8f8f2;\">: EventHandler&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">, FormEvent&gt;,\n</span><span style=\"color:#f8f8f2;\">) -&gt; Element&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> input_style </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">&quot;\n</span><span style=\"color:#ffee99;\">        border: none;\n</span><span style=\"color:#ffee99;\">        background: cornflowerblue;\n</span><span style=\"color:#ffee99;\">        padding: 8px 16px;\n</span><span style=\"color:#ffee99;\">        margin: 0;\n</span><span style=\"color:#ffee99;\">        border-radius: 4px;\n</span><span style=\"color:#ffee99;\">        color: white;\n</span><span style=\"color:#ffee99;\">    &quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(input {{\n</span><span style=\"color:#f8f8f2;\">        style: </span><span style=\"color:#ffee99;\">&quot;{{input_style}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        value: </span><span style=\"color:#ffee99;\">&quot;{{caption}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        oninput: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> on_input.</span><span style=\"color:#66d9ef;\">call</span><span style=\"color:#f8f8f2;\">(event),\n</span><span style=\"color:#f8f8f2;\">    }}))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "meme_editor.rs".to_string(),
        }
        p {
            "Finally, a third component will render the other two as children. It will be responsible for keeping the state and passing down the relevant props."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">MemeEditor</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> container_style </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">&quot;\n</span><span style=\"color:#ffee99;\">        display: flex;\n</span><span style=\"color:#ffee99;\">        flex-direction: column;\n</span><span style=\"color:#ffee99;\">        gap: 16px;\n</span><span style=\"color:#ffee99;\">        margin: 0 auto;\n</span><span style=\"color:#ffee99;\">        width: fit-content;\n</span><span style=\"color:#ffee99;\">    &quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> caption </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ffee99;\">&quot;me waiting for my rust code to compile&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            style: </span><span style=\"color:#ffee99;\">&quot;{{container_style}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            h1 {{ </span><span style=\"color:#ffee99;\">&quot;Meme Editor&quot; </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">            Meme {{\n</span><span style=\"color:#f8f8f2;\">                caption: caption,\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            CaptionEditor {{\n</span><span style=\"color:#f8f8f2;\">                caption: caption,\n</span><span style=\"color:#f8f8f2;\">                on_input: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event: FormEvent</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{caption.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(event.value.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">());}},\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "meme_editor.rs".to_string(),
        }
        p {
            img {
                src: asset!(
                    "/assets/static/images03/meme_editor_screenshot.png", ImageAssetOptions::new()
                    .with_avif()
                ),
                alt: "Meme Editor Screenshot: An old plastic skeleton sitting on a park bench. Caption: \"me waiting for a language feature\"",
                title: "",
            }
        }
        h2 { id: "using-context",
            Link {
                to: BookRoute::InteractivitySharingState {
                    section: InteractivitySharingStateSection::UsingContext,
                },
                class: "header",
                "Using Context"
            }
        }
        p {
            "Sometimes, some state needs to be shared between multiple components far down the tree, and passing it down through props is very inconvenient."
        }
        p {
            "Suppose now that we want to implement a dark mode toggle for our app. To achieve this, we will make every component select styling depending on whether dark mode is enabled or not."
        }
        blockquote {
            p {
                "Note: we're choosing this approach for the sake of an example. There are better ways to implement dark mode (e.g. using CSS variables). Let's pretend CSS variables don't exist – welcome to 2013!"
            }
        }
        p {
            "Now, we could write another  "
            code { "use_state" }
            " in the top component, and pass  "
            code { "is_dark_mode" }
            " down to every component through props. But think about what will happen as the app grows in complexity – almost every component that renders any CSS is going to need to know if dark mode is enabled or not – so they'll all need the same dark mode prop. And every parent component will need to pass it down to them. Imagine how messy and verbose that would get, especially if we had components several levels deep!"
        }
        p {
            "Dioxus offers a better solution than this \"prop drilling\" – providing context. The "
            Link { to: "https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context_provider.html",
                code { "use_context_provider" }
            }
            " hook is similar to "
            code { "use_ref" }
            ", but it makes it available through "
            Link { to: "https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context.html",
                code { "use_context" }
            }
            " for all children components."
        }
        p { "First, we have to create a struct for our dark mode configuration:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">DarkMode(</span><span style=\"font-style:italic;color:#66d9ef;\">bool</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "meme_editor_dark_mode.rs".to_string(),
        }
        p {
            "Now, in a top-level component (like  "
            code { "App" }
            "), we can provide the  "
            code { "DarkMode" }
            " context to all children components:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#66d9ef;\">use_shared_state_provider</span><span style=\"color:#f8f8f2;\">(cx, || DarkMode(</span><span style=\"color:#ff80f4;\">false</span><span style=\"color:#f8f8f2;\">));</span></pre>\n",
            name: "meme_editor_dark_mode.rs".to_string(),
        }
        p {
            "As a result, any child component of  "
            code { "App" }
            " (direct or not), can access the  "
            code { "DarkMode" }
            " context."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> dark_mode_context </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">use_shared_state::&lt;DarkMode&gt;(cx);</span></pre>\n",
            name: "meme_editor_dark_mode.rs".to_string(),
        }
        blockquote {
            p {
                code { "use_context" }
                " returns  "
                code { "Option<UseSharedState<DarkMode>>" }
                " here. If the context has been provided, the value is  "
                code { "Some(UseSharedState<DarkMode>)" }
                ", which you can call  "
                code { ".read" }
                " or  "
                code { ".write" }
                " on, similarly to  "
                code { "UseRef" }
                ". Otherwise, the value is  "
                code { "None" }
                "."
            }
        }
        p {
            "For example, here's how we would implement the dark mode toggle, which both reads the context (to determine what color it should render) and writes to it (to toggle dark mode):"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">DarkModeToggle</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> dark_mode </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">use_shared_state::&lt;DarkMode&gt;(cx).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> style </span><span style=\"color:#f92672;\">= if</span><span style=\"color:#f8f8f2;\"> dark_mode.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;color:white&quot;\n</span><span style=\"color:#f8f8f2;\">    }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;&quot;\n</span><span style=\"color:#f8f8f2;\">    }};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(label {{\n</span><span style=\"color:#f8f8f2;\">        style: </span><span style=\"color:#ffee99;\">&quot;{{style}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Dark Mode&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        input {{\n</span><span style=\"color:#f8f8f2;\">            r</span><span style=\"color:#f92672;\">#</span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;checkbox&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            oninput: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> is_enabled </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> event.value </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ffee99;\">&quot;true&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">                dark_mode.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> is_enabled;\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    }}))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "meme_editor_dark_mode.rs".to_string(),
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum InteractivityCustomHooksSection {
    #[default]
    Empty,
    CustomHooks,
    ComposingHooks,
    CustomHookLogic,
}
impl std::str::FromStr for InteractivityCustomHooksSection {
    type Err = InteractivityCustomHooksSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "custom-hooks" => Ok(Self::CustomHooks),
            "composing-hooks" => Ok(Self::ComposingHooks),
            "custom-hook-logic" => Ok(Self::CustomHookLogic),
            _ => Err(InteractivityCustomHooksSectionParseError),
        }
    }
}
impl std::fmt::Display for InteractivityCustomHooksSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::CustomHooks => f.write_str("custom-hooks"),
            Self::ComposingHooks => f.write_str("composing-hooks"),
            Self::CustomHookLogic => f.write_str("custom-hook-logic"),
        }
    }
}
#[derive(Debug)]
pub struct InteractivityCustomHooksSectionParseError;
impl std::fmt::Display for InteractivityCustomHooksSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of InteractivityCustomHooksSectioncustom-hooks, composing-hooks, custom-hook-logic",
        )?;
        Ok(())
    }
}
impl std::error::Error for InteractivityCustomHooksSectionParseError {}
#[component(no_case_check)]
pub fn InteractivityCustomHooks(
    section: InteractivityCustomHooksSection,
) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "custom-hooks",
            Link {
                to: BookRoute::InteractivityCustomHooks {
                    section: InteractivityCustomHooksSection::CustomHooks,
                },
                class: "header",
                "Custom Hooks"
            }
        }
        p {
            "Hooks are a great way to encapsulate business logic. If none of the existing hooks work for your problem, you can write your own."
        }
        h2 { id: "composing-hooks",
            Link {
                to: BookRoute::InteractivityCustomHooks {
                    section: InteractivityCustomHooksSection::ComposingHooks,
                },
                class: "header",
                "Composing Hooks"
            }
        }
        p {
            "To avoid repetition, you can encapsulate business logic based on existing hooks to create a new hook."
        }
        p {
            "For example, if many components need to access an  "
            code { "AppSettings" }
            " struct, you can create a \"shortcut\" hook:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">use_settings</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">ScopeState) -&gt; UseSharedState&lt;AppSettings&gt; {{\n</span><span style=\"color:#f8f8f2;\">    use_shared_state::&lt;AppSettings&gt;(cx).</span><span style=\"color:#66d9ef;\">expect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;App settings not provided&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_composed.rs".to_string(),
        }
        h2 { id: "custom-hook-logic",
            Link {
                to: BookRoute::InteractivityCustomHooks {
                    section: InteractivityCustomHooksSection::CustomHookLogic,
                },
                class: "header",
                "Custom Hook Logic"
            }
        }
        p {
            "You can use "
            Link { to: "https://docs.rs/dioxus/latest/dioxus/prelude/struct.Scope.html#method.use_hook",
                code { "cx.use_hook" }
            }
            " to build your own hooks. In fact, this is what all the standard hooks are built on!"
        }
        p {
            code { "use_hook" }
            " accepts a single closure for initializing the hook. It will be only run the first time the component is rendered. The return value of that closure will be used as the value of the hook – Dioxus will take it, and store it for as long as the component is alive. On every render (not just the first one!), you will get a reference to this value."
        }
        blockquote {
            p {
                "Note: You can implement "
                Link { to: "https://doc.rust-lang.org/std/ops/trait.Drop.html",
                    code { "Drop" }
                }
                " for your hook value – it will be dropped then the component is unmounted (no longer in the UI)"
            }
        }
        p {
            "Inside the initialization closure, you will typically make calls to other  "
            code { "cx" }
            " methods. For example:"
        }
        ul {
            li {
                "The "
                code { "use_state" }
                " hook tracks state in the hook value, and uses "
                Link { to: "https://docs.rs/dioxus/latest/dioxus/prelude/struct.Scope.html#method.schedule_update",
                    code { "cx.schedule_update" }
                }
                " to make Dioxus re-render the component whenever it changes."
            }
            li {
                "The "
                code { "use_context" }
                " hook calls "
                Link { to: "https://docs.rs/dioxus/latest/dioxus/prelude/struct.Scope.html#method.consume_context",
                    code { "cx.consume_context" }
                }
                " (which would be expensive to call on every render) to get some context from the scope"
            }
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum InteractivityDynamicRenderingSection {
    #[default]
    Empty,
    DynamicRendering,
    ConditionalRendering,
    ImprovingTheIfElseExample,
    InspectingElementProps,
    RenderingNothing,
    RenderingLists,
    InlineForLoops,
    TheKeyAttribute,
}
impl std::str::FromStr for InteractivityDynamicRenderingSection {
    type Err = InteractivityDynamicRenderingSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "dynamic-rendering" => Ok(Self::DynamicRendering),
            "conditional-rendering" => Ok(Self::ConditionalRendering),
            "improving-the-if-else-example" => Ok(Self::ImprovingTheIfElseExample),
            "inspecting-element-props" => Ok(Self::InspectingElementProps),
            "rendering-nothing" => Ok(Self::RenderingNothing),
            "rendering-lists" => Ok(Self::RenderingLists),
            "inline-for-loops" => Ok(Self::InlineForLoops),
            "the-key-attribute" => Ok(Self::TheKeyAttribute),
            _ => Err(InteractivityDynamicRenderingSectionParseError),
        }
    }
}
impl std::fmt::Display for InteractivityDynamicRenderingSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::DynamicRendering => f.write_str("dynamic-rendering"),
            Self::ConditionalRendering => f.write_str("conditional-rendering"),
            Self::ImprovingTheIfElseExample => f.write_str("improving-the-if-else-example"),
            Self::InspectingElementProps => f.write_str("inspecting-element-props"),
            Self::RenderingNothing => f.write_str("rendering-nothing"),
            Self::RenderingLists => f.write_str("rendering-lists"),
            Self::InlineForLoops => f.write_str("inline-for-loops"),
            Self::TheKeyAttribute => f.write_str("the-key-attribute"),
        }
    }
}
#[derive(Debug)]
pub struct InteractivityDynamicRenderingSectionParseError;
impl std::fmt::Display for InteractivityDynamicRenderingSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of InteractivityDynamicRenderingSectiondynamic-rendering, conditional-rendering, improving-the-if-else-example, inspecting-element-props, rendering-nothing, rendering-lists, inline-for-loops, the-key-attribute",
        )?;
        Ok(())
    }
}
impl std::error::Error for InteractivityDynamicRenderingSectionParseError {}
#[component(no_case_check)]
pub fn InteractivityDynamicRendering(
    section: InteractivityDynamicRenderingSection,
) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "dynamic-rendering",
            Link {
                to: BookRoute::InteractivityDynamicRendering {
                    section: InteractivityDynamicRenderingSection::DynamicRendering,
                },
                class: "header",
                "Dynamic Rendering"
            }
        }
        p {
            "Sometimes you want to render different things depending on the state/props. With Dioxus, just describe what you want to see using Rust control flow – the framework will take care of making the necessary changes on the fly if the state or props change!"
        }
        h2 { id: "conditional-rendering",
            Link {
                to: BookRoute::InteractivityDynamicRendering {
                    section: InteractivityDynamicRenderingSection::ConditionalRendering,
                },
                class: "header",
                "Conditional Rendering"
            }
        }
        p {
            "To render different elements based on a condition, you could use an  "
            code { "if-else" }
            " statement:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">if *</span><span style=\"color:#f8f8f2;\">is_logged_in {{\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;Welcome!&quot;\n</span><span style=\"color:#f8f8f2;\">    button {{\n</span><span style=\"color:#f8f8f2;\">        onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> on_log_out.</span><span style=\"color:#66d9ef;\">call</span><span style=\"color:#f8f8f2;\">(()),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Log Out&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}})\n</span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">    button {{\n</span><span style=\"color:#f8f8f2;\">        onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> on_log_in.</span><span style=\"color:#66d9ef;\">call</span><span style=\"color:#f8f8f2;\">(()),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Log In&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "conditional_rendering.rs".to_string(),
        }
        blockquote {
            p {
                "You could also use  "
                code { "match" }
                " statements, or any Rust function to conditionally render different things."
            }
        }
        h3 { id: "improving-the-if-else-example",
            Link {
                to: BookRoute::InteractivityDynamicRendering {
                    section: InteractivityDynamicRenderingSection::ImprovingTheIfElseExample,
                },
                class: "header",
                "Improving the if-else Example"
            }
        }
        p {
            "You may have noticed some repeated code in the  "
            code { "if-else" }
            " example above. Repeating code like this is both bad for maintainability and performance. Dioxus will skip diffing static elements like the button, but when switching between multiple  "
            code { "rsx" }
            " calls it cannot perform this optimization. For this example either approach is fine, but for components with large parts that are reused between conditionals, it can be more of an issue."
        }
        p {
            "We can improve this example by splitting up the dynamic parts and inserting them where they are needed."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#8c8c8c;\">// We only render the welcome message if we are logged in\n</span><span style=\"color:#8c8c8c;\">// You can use if statements in the middle of a render block to conditionally render elements\n</span><span style=\"color:#f92672;\">if *</span><span style=\"color:#f8f8f2;\">is_logged_in {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Notice the body of this if statment is rsx code, not an expression\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;Welcome!&quot;\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">button {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// depending on the value of `is_logged_in`, we will call a different event handler\n</span><span style=\"color:#f8f8f2;\">    onclick: </span><span style=\"color:#f92672;\">move |_| if *</span><span style=\"color:#f8f8f2;\">is_logged_in {{\n</span><span style=\"color:#f8f8f2;\">        on_log_in.</span><span style=\"color:#66d9ef;\">call</span><span style=\"color:#f8f8f2;\">(())\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">else</span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        on_log_out.</span><span style=\"color:#66d9ef;\">call</span><span style=\"color:#f8f8f2;\">(())\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if *</span><span style=\"color:#f8f8f2;\">is_logged_in {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// if we are logged in, the button should say &quot;Log Out&quot;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Log Out&quot;\n</span><span style=\"color:#f8f8f2;\">    }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// if we are not logged in, the button should say &quot;Log In&quot;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Log In&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "conditional_rendering.rs".to_string(),
        }
        h3 { id: "inspecting-element-props",
            Link {
                to: BookRoute::InteractivityDynamicRendering {
                    section: InteractivityDynamicRenderingSection::InspectingElementProps,
                },
                class: "header",
                "Inspecting Element props"
            }
        }
        p {
            "Since  "
            code { "Element" }
            " is a  "
            code { "Option<VNode>" }
            ", components accepting  "
            code { "Element" }
            " as a prop can inspect its contents, and render different things based on that. Example:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Clickable</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">, ClickableProps&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> cx.props.children {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(VNode {{ dynamic_nodes, </span><span style=\"color:#f92672;\">.. </span><span style=\"color:#f8f8f2;\">}}) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            todo!(</span><span style=\"color:#ffee99;\">&quot;render some stuff&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">_ =&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            todo!(</span><span style=\"color:#ffee99;\">&quot;render some other stuff&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_children_inspect.rs".to_string(),
        }
        p {
            "You can't mutate the  "
            code { "Element" }
            ", but if you need a modified version of it, you can construct a new one based on its attributes/children/etc."
        }
        h2 { id: "rendering-nothing",
            Link {
                to: BookRoute::InteractivityDynamicRendering {
                    section: InteractivityDynamicRenderingSection::RenderingNothing,
                },
                class: "header",
                "Rendering Nothing"
            }
        }
        p {
            "To render nothing, you can return  "
            code { "None" }
            " from a component. This is useful if you want to conditionally hide something:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">if *</span><span style=\"color:#f8f8f2;\">is_logged_in {{\n</span><span style=\"color:#f92672;\">return </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">a {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;You must be logged in to comment&quot;\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "conditional_rendering.rs".to_string(),
        }
        p {
            "This works because the  "
            code { "Element" }
            " type is just an alias for  "
            code { "Option<VNode>" }
        }
        blockquote {
            p {
                "Again, you may use a different method to conditionally return  "
                code { "None" }
                ". For example the boolean's "
                Link { to: "https://doc.rust-lang.org/std/primitive.bool.html#method.then",
                    code { "then()" }
                }
                " function could be used."
            }
        }
        h2 { id: "rendering-lists",
            Link {
                to: BookRoute::InteractivityDynamicRendering {
                    section: InteractivityDynamicRenderingSection::RenderingLists,
                },
                class: "header",
                "Rendering Lists"
            }
        }
        p {
            "Often, you'll want to render a collection of components. For example, you might want to render a list of all comments on a post."
        }
        p {
            "For this, Dioxus accepts iterators that produce  "
            code { "Element" }
            "s. So we need to:"
        }
        ul {
            li {
                "Get an iterator over all of our items (e.g., if you have a "
                code { "Vec" }
                " of comments, iterate over it with "
                code { "iter()" }
                ")"
            }
            li {
                code { ".map" }
                " the iterator to convert each item into a "
                code { "LazyNode" }
                " using "
                code { "rsx!(...)" }
                ul {
                    li {
                        "Add a unique "
                        code { "key" }
                        " attribute to each iterator item"
                    }
                }
            }
            li { "Include this iterator in the final RSX (or use it inline)" }
        }
        p {
            "Example: suppose you have a list of comments you want to render. Then, you can render them like this:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comment_field </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::new);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> next_id </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_ref</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"font-style:italic;color:#66d9ef;\">Vec</span><span style=\"color:#f8f8f2;\">::&lt;Comment&gt;::new);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments_lock </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> comments.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments_rendered </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> comments_lock.</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">comment</span><span style=\"color:#f8f8f2;\">| {{\n</span><span style=\"color:#f8f8f2;\">rsx!(CommentComponent {{\n</span><span style=\"color:#f8f8f2;\">    key: </span><span style=\"color:#ffee99;\">&quot;{{comment.id}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    comment: comment.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">}})\n</span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">form {{\n</span><span style=\"color:#f8f8f2;\">    onsubmit: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        comments.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(Comment {{\n</span><span style=\"color:#f8f8f2;\">            content: comment_field.</span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            id: </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">next_id.</span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        }});\n</span><span style=\"color:#f8f8f2;\">        next_id </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        comment_field.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::new());\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">    input {{\n</span><span style=\"color:#f8f8f2;\">        value: </span><span style=\"color:#ffee99;\">&quot;{{comment_field}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        oninput: |</span><span style=\"font-style:italic;color:#fd971f;\">event</span><span style=\"color:#f8f8f2;\">| comment_field.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(event.value.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">()),\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    input {{\n</span><span style=\"color:#f8f8f2;\">        r</span><span style=\"color:#f92672;\">#</span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;submit&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">comments_rendered,\n</span><span style=\"color:#f8f8f2;\">))</span></pre>\n",
            name: "rendering_lists.rs".to_string(),
        }
        h3 { id: "inline-for-loops",
            Link {
                to: BookRoute::InteractivityDynamicRendering {
                    section: InteractivityDynamicRenderingSection::InlineForLoops,
                },
                class: "header",
                "Inline for loops"
            }
        }
        p {
            "Because of how common it is to render a list of items, Dioxus provides a shorthand for this. Instead of using  "
            code { ".iter, " }
            ".map "
            code { ", and " }
            "rsx "
            code { ", you can use a " }
            "for"
            " "
            "`"
            " loop with a body of rsx code:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comment_field </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::new);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> next_id </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_ref</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"font-style:italic;color:#66d9ef;\">Vec</span><span style=\"color:#f8f8f2;\">::&lt;Comment&gt;::new);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">form {{\n</span><span style=\"color:#f8f8f2;\">    onsubmit: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        comments.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(Comment {{\n</span><span style=\"color:#f8f8f2;\">            content: comment_field.</span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            id: </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">next_id.</span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        }});\n</span><span style=\"color:#f8f8f2;\">        next_id </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        comment_field.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::new());\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">    input {{\n</span><span style=\"color:#f8f8f2;\">        value: </span><span style=\"color:#ffee99;\">&quot;{{comment_field}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        oninput: |</span><span style=\"font-style:italic;color:#fd971f;\">event</span><span style=\"color:#f8f8f2;\">| comment_field.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(event.value.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">()),\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    input {{\n</span><span style=\"color:#f8f8f2;\">        r</span><span style=\"color:#f92672;\">#</span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;submit&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> comment </span><span style=\"color:#f92672;\">in &amp;*</span><span style=\"color:#f8f8f2;\">comments.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Notice the body of this for loop is rsx code, not an expression\n</span><span style=\"color:#f8f8f2;\">    CommentComponent {{\n</span><span style=\"color:#f8f8f2;\">        key: </span><span style=\"color:#ffee99;\">&quot;{{comment.id}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        comment: comment.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">))</span></pre>\n",
            name: "rendering_lists.rs".to_string(),
        }
        h3 { id: "the-key-attribute",
            Link {
                to: BookRoute::InteractivityDynamicRendering {
                    section: InteractivityDynamicRenderingSection::TheKeyAttribute,
                },
                class: "header",
                "The key Attribute"
            }
        }
        p {
            "Every time you re-render your list, Dioxus needs to keep track of which items go where to determine what updates need to be made to the UI."
        }
        p {
            "For example, suppose the  "
            code { "CommentComponent" }
            " had some state – e.g. a field where the user typed in a reply. If the order of comments suddenly changes, Dioxus needs to correctly associate that state with the same comment – otherwise, the user will end up replying to a different comment!"
        }
        p {
            "To help Dioxus keep track of list items, we need to associate each item with a unique key. In the example above, we dynamically generated the unique key. In real applications, it's more likely that the key will come from e.g. a database ID. It doesn't matter where you get the key from, as long as it meets the requirements:"
        }
        ul {
            li { "Keys must be unique in a list" }
            li { "The same item should always get associated with the same key" }
            li {
                "Keys should be relatively small (i.e. converting the entire Comment structure to a String would be a pretty bad key) so they can be compared efficiently"
            }
        }
        p {
            "You might be tempted to use an item's index in the list as its key. That’s what Dioxus will use if you don’t specify a key at all. This is only acceptable if you can guarantee that the list is constant – i.e., no re-ordering, additions, or deletions."
        }
        blockquote {
            p {
                "Note that if you pass the key to a component you've made, it won't receive the key as a prop. It’s only used as a hint by Dioxus itself. If your component needs an ID, you have to pass it as a separate prop."
            }
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum InteractivityRouterSection {
    #[default]
    Empty,
    Router,
    WhatIsIt,
    UsingTheRouter,
    Links,
    MoreReading,
}
impl std::str::FromStr for InteractivityRouterSection {
    type Err = InteractivityRouterSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "router" => Ok(Self::Router),
            "what-is-it" => Ok(Self::WhatIsIt),
            "using-the-router" => Ok(Self::UsingTheRouter),
            "links" => Ok(Self::Links),
            "more-reading" => Ok(Self::MoreReading),
            _ => Err(InteractivityRouterSectionParseError),
        }
    }
}
impl std::fmt::Display for InteractivityRouterSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Router => f.write_str("router"),
            Self::WhatIsIt => f.write_str("what-is-it"),
            Self::UsingTheRouter => f.write_str("using-the-router"),
            Self::Links => f.write_str("links"),
            Self::MoreReading => f.write_str("more-reading"),
        }
    }
}
#[derive(Debug)]
pub struct InteractivityRouterSectionParseError;
impl std::fmt::Display for InteractivityRouterSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of InteractivityRouterSectionrouter, what-is-it, using-the-router, links, more-reading",
        )?;
        Ok(())
    }
}
impl std::error::Error for InteractivityRouterSectionParseError {}
#[component(no_case_check)]
pub fn InteractivityRouter(section: InteractivityRouterSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "router",
            Link {
                to: BookRoute::InteractivityRouter {
                    section: InteractivityRouterSection::Router,
                },
                class: "header",
                "Router"
            }
        }
        p {
            "In many of your apps, you'll want to have different \"scenes\". For a webpage, these scenes might be the different webpages with their own content. For a desktop app, these scenes might be different views in your app."
        }
        p {
            "To unify these platforms, Dioxus provides a first-party solution for scene management called Dioxus Router."
        }
        h2 { id: "what-is-it",
            Link {
                to: BookRoute::InteractivityRouter {
                    section: InteractivityRouterSection::WhatIsIt,
                },
                class: "header",
                "What is it?"
            }
        }
        p {
            "For an app like the Dioxus landing page (https://dioxuslabs.com), we want to have several different scenes:"
        }
        ul {
            li { "Homepage" }
            li { "Blog" }
        }
        p {
            "Each of these scenes is independent – we don't want to render both the homepage and blog at the same time."
        }
        p {
            "The Dioxus router makes it easy to create these scenes. To make sure we're using the router, add the  "
            code { "dioxus-router" }
            " package to your  "
            code { "Cargo.toml" }
            "."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">router</span></pre>\n" }
        h2 { id: "using-the-router",
            Link {
                to: BookRoute::InteractivityRouter {
                    section: InteractivityRouterSection::UsingTheRouter,
                },
                class: "header",
                "Using the router"
            }
        }
        p {
            "Unlike other routers in the Rust ecosystem, our router is built declaratively. This makes it possible to compose our app layout simply by arranging components."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx!{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// All of our routes will be rendered inside this Router component\n</span><span style=\"color:#f8f8f2;\">    Router {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// if the current location is &quot;/home&quot;, render the Home component\n</span><span style=\"color:#f8f8f2;\">        Route {{ to: </span><span style=\"color:#ffee99;\">&quot;/home&quot;</span><span style=\"color:#f8f8f2;\">, Home {{}} }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// if the current location is &quot;/blog&quot;, render the Blog component\n</span><span style=\"color:#f8f8f2;\">        Route {{ to: </span><span style=\"color:#ffee99;\">&quot;/blog&quot;</span><span style=\"color:#f8f8f2;\">, Blog {{}} }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Whenever we visit this app, we will get either the Home component or the Blog component rendered depending on which route we enter at. If neither of these routes match the current location, then nothing will render."
        }
        p { "We can fix this one of two ways:" }
        ul {
            li { "A fallback 404 page" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx!{{\n</span><span style=\"color:#f8f8f2;\">    Router {{\n</span><span style=\"color:#f8f8f2;\">        Route {{ to: </span><span style=\"color:#ffee99;\">&quot;/home&quot;</span><span style=\"color:#f8f8f2;\">, Home {{}} }}\n</span><span style=\"color:#f8f8f2;\">        Route {{ to: </span><span style=\"color:#ffee99;\">&quot;/blog&quot;</span><span style=\"color:#f8f8f2;\">, Blog {{}} }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">//  if the current location doesn&#39;t match any of the above routes, render the NotFound component\n</span><span style=\"color:#f8f8f2;\">        Route {{ to: </span><span style=\"color:#ffee99;\">&quot;&quot;</span><span style=\"color:#f8f8f2;\">, NotFound {{}} }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        ul {
            li { "Redirect 404 to home" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx!{{\n</span><span style=\"color:#f8f8f2;\">    Router {{\n</span><span style=\"color:#f8f8f2;\">        Route {{ to: </span><span style=\"color:#ffee99;\">&quot;/home&quot;</span><span style=\"color:#f8f8f2;\">, Home {{}} }}\n</span><span style=\"color:#f8f8f2;\">        Route {{ to: </span><span style=\"color:#ffee99;\">&quot;/blog&quot;</span><span style=\"color:#f8f8f2;\">, Blog {{}} }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">//  if the current location doesn&#39;t match any of the above routes, redirect to &quot;/home&quot;\n</span><span style=\"color:#f8f8f2;\">        Redirect {{ from: </span><span style=\"color:#ffee99;\">&quot;&quot;</span><span style=\"color:#f8f8f2;\">, to: </span><span style=\"color:#ffee99;\">&quot;/home&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "links",
            Link {
                to: BookRoute::InteractivityRouter {
                    section: InteractivityRouterSection::Links,
                },
                class: "header",
                "Links"
            }
        }
        p {
            "For our app to navigate these routes, we can provide clickable elements called Links. These simply wrap  "
            code { "<a>" }
            " elements that, when clicked, navigate the app to the given location."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx!{{\n</span><span style=\"color:#f8f8f2;\">    Link {{\n</span><span style=\"color:#f8f8f2;\">        to: </span><span style=\"color:#ffee99;\">&quot;/home&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Go home!&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        h2 { id: "more-reading",
            Link {
                to: BookRoute::InteractivityRouter {
                    section: InteractivityRouterSection::MoreReading,
                },
                class: "header",
                "More reading"
            }
        }
        p {
            "This page is just a very brief overview of the router. For more information, check out "
            Link { to: "https://dioxuslabs.com/router/guide/", "the router book" }
            " or some of "
            Link { to: "https://github.com/DioxusLabs/dioxus/blob/master/examples/router.rs",
                "the router examples"
            }
            "."
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum AsyncIndexSection {
    #[default]
    Empty,
    WorkingWithAsync,
    TheRuntime,
}
impl std::str::FromStr for AsyncIndexSection {
    type Err = AsyncIndexSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "working-with-async" => Ok(Self::WorkingWithAsync),
            "the-runtime" => Ok(Self::TheRuntime),
            _ => Err(AsyncIndexSectionParseError),
        }
    }
}
impl std::fmt::Display for AsyncIndexSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::WorkingWithAsync => f.write_str("working-with-async"),
            Self::TheRuntime => f.write_str("the-runtime"),
        }
    }
}
#[derive(Debug)]
pub struct AsyncIndexSectionParseError;
impl std::fmt::Display for AsyncIndexSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of AsyncIndexSectionworking-with-async, the-runtime",
        )?;
        Ok(())
    }
}
impl std::error::Error for AsyncIndexSectionParseError {}
#[component(no_case_check)]
pub fn AsyncIndex(section: AsyncIndexSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "working-with-async",
            Link {
                to: BookRoute::AsyncIndex {
                    section: AsyncIndexSection::WorkingWithAsync,
                },
                class: "header",
                "Working with Async"
            }
        }
        p {
            "Often, apps need to interact with file systems, network interfaces, hardware, or timers. This chapter provides an overview of using async code in Dioxus."
        }
        h2 { id: "the-runtime",
            Link {
                to: BookRoute::AsyncIndex {
                    section: AsyncIndexSection::TheRuntime,
                },
                class: "header",
                "The Runtime"
            }
        }
        p {
            "By default, Dioxus-Desktop ships with the  "
            code { "Tokio" }
            " runtime and automatically sets everything up for you. This is currently not configurable, though it would be easy to write an integration for Dioxus desktop that uses a different asynchronous runtime."
        }
        p {
            "Dioxus is not currently thread-safe, so any async code you write does "
            em { "not" }
            " need to be "
            code { "Send/Sync" }
            ". That means that you can use non-thread-safe structures like "
            code { "Cell" }
            ", "
            code { "Rc" }
            ", and "
            code { "RefCell" }
            "."
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum AsyncUseFutureSection {
    #[default]
    Empty,
    Usefuture,
    RestartingTheFuture,
    Dependencies,
}
impl std::str::FromStr for AsyncUseFutureSection {
    type Err = AsyncUseFutureSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "usefuture" => Ok(Self::Usefuture),
            "restarting-the-future" => Ok(Self::RestartingTheFuture),
            "dependencies" => Ok(Self::Dependencies),
            _ => Err(AsyncUseFutureSectionParseError),
        }
    }
}
impl std::fmt::Display for AsyncUseFutureSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Usefuture => f.write_str("usefuture"),
            Self::RestartingTheFuture => f.write_str("restarting-the-future"),
            Self::Dependencies => f.write_str("dependencies"),
        }
    }
}
#[derive(Debug)]
pub struct AsyncUseFutureSectionParseError;
impl std::fmt::Display for AsyncUseFutureSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of AsyncUseFutureSectionusefuture, restarting-the-future, dependencies",
        )?;
        Ok(())
    }
}
impl std::error::Error for AsyncUseFutureSectionParseError {}
#[component(no_case_check)]
pub fn AsyncUseFuture(section: AsyncUseFutureSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "usefuture",
            Link {
                to: BookRoute::AsyncUseFuture {
                    section: AsyncUseFutureSection::Usefuture,
                },
                class: "header",
                "UseFuture"
            }
        }
        p {
            Link { to: "https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_future.html",
                code { "use_future" }
            }
            " lets you run an async closure, and provides you with its result."
        }
        p {
            "For example, we can make an API request (using "
            Link { to: "https://docs.rs/reqwest/latest/reqwest/index.html", "reqwest" }
            ") inside "
            code { "use_future" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> future </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_future</span><span style=\"color:#f8f8f2;\">(cx, (), |_| async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    reqwest::get(</span><span style=\"color:#ffee99;\">&quot;https://dog.ceo/api/breeds/image/random&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .await\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .json::&lt;ApiResponse&gt;()\n</span><span style=\"color:#f8f8f2;\">        .await\n</span><span style=\"color:#f8f8f2;\">}});</span></pre>\n",
            name: "use_future.rs".to_string(),
        }
        p {
            "The code inside  "
            code { "use_future" }
            " will be submitted to the Dioxus scheduler once the component has rendered."
        }
        p {
            "We can use  "
            code { ".value()" }
            " to get the result of the future. On the first run, since there's no data ready when the component loads, its value will be  "
            code { "None" }
            ".  However, once the future is finished, the component will be re-rendered and the value will now be  "
            code { "Some(...)" }
            ", containing the return value of the closure."
        }
        p { "We can then render that result:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> future.</span><span style=\"color:#66d9ef;\">value</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(response)) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> future.</span><span style=\"color:#66d9ef;\">restart</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Click to fetch another doggo&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            img {{\n</span><span style=\"color:#f8f8f2;\">                max_width: </span><span style=\"color:#ffee99;\">&quot;500px&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                max_height: </span><span style=\"color:#ffee99;\">&quot;500px&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                src: </span><span style=\"color:#ffee99;\">&quot;{{response.image_url}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">)) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{ div {{ </span><span style=\"color:#ffee99;\">&quot;Loading dogs failed&quot; </span><span style=\"color:#f8f8f2;\">}} }},\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{ div {{ </span><span style=\"color:#ffee99;\">&quot;Loading dogs...&quot; </span><span style=\"color:#f8f8f2;\">}} }},\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "use_future.rs".to_string(),
        }
        h2 { id: "restarting-the-future",
            Link {
                to: BookRoute::AsyncUseFuture {
                    section: AsyncUseFutureSection::RestartingTheFuture,
                },
                class: "header",
                "Restarting the Future"
            }
        }
        p {
            "The  "
            code { "UseFuture" }
            " handle provides a  "
            code { "restart" }
            " method. It can be used to execute the future again, producing a new value."
        }
        h2 { id: "dependencies",
            Link {
                to: BookRoute::AsyncUseFuture {
                    section: AsyncUseFutureSection::Dependencies,
                },
                class: "header",
                "Dependencies"
            }
        }
        p {
            "Often, you will need to run the future again every time some value (e.g. a prop) changes. Rather than calling  "
            code { "restart" }
            " manually, you can provide a tuple of \"dependencies\" to the hook. It will automatically re-run the future when any of those dependencies change. Example:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> future </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_future</span><span style=\"color:#f8f8f2;\">(cx, (breed,), |(</span><span style=\"font-style:italic;color:#fd971f;\">breed</span><span style=\"color:#f8f8f2;\">,)| async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    reqwest::get(format!(</span><span style=\"color:#ffee99;\">&quot;https://dog.ceo/api/breed/</span><span style=\"color:#ff80f4;\">{{breed}}</span><span style=\"color:#ffee99;\">/images/random&quot;</span><span style=\"color:#f8f8f2;\">))\n</span><span style=\"color:#f8f8f2;\">        .await\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .json::&lt;ApiResponse&gt;()\n</span><span style=\"color:#f8f8f2;\">        .await\n</span><span style=\"color:#f8f8f2;\">}});</span></pre>\n",
            name: "use_future.rs".to_string(),
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum AsyncUseCoroutineSection {
    #[default]
    Empty,
    Coroutines,
    UseCoroutine,
    YieldingValues,
    SendingValues,
    AutomaticInjectionIntoTheContextApi,
}
impl std::str::FromStr for AsyncUseCoroutineSection {
    type Err = AsyncUseCoroutineSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "coroutines" => Ok(Self::Coroutines),
            "use-coroutine" => Ok(Self::UseCoroutine),
            "yielding-values" => Ok(Self::YieldingValues),
            "sending-values" => Ok(Self::SendingValues),
            "automatic-injection-into-the-context-api" => {
                Ok(Self::AutomaticInjectionIntoTheContextApi)
            }
            _ => Err(AsyncUseCoroutineSectionParseError),
        }
    }
}
impl std::fmt::Display for AsyncUseCoroutineSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Coroutines => f.write_str("coroutines"),
            Self::UseCoroutine => f.write_str("use-coroutine"),
            Self::YieldingValues => f.write_str("yielding-values"),
            Self::SendingValues => f.write_str("sending-values"),
            Self::AutomaticInjectionIntoTheContextApi => {
                f.write_str("automatic-injection-into-the-context-api")
            }
        }
    }
}
#[derive(Debug)]
pub struct AsyncUseCoroutineSectionParseError;
impl std::fmt::Display for AsyncUseCoroutineSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of AsyncUseCoroutineSectioncoroutines, use-coroutine, yielding-values, sending-values, automatic-injection-into-the-context-api",
        )?;
        Ok(())
    }
}
impl std::error::Error for AsyncUseCoroutineSectionParseError {}
#[component(no_case_check)]
pub fn AsyncUseCoroutine(section: AsyncUseCoroutineSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "coroutines",
            Link {
                to: BookRoute::AsyncUseCoroutine {
                    section: AsyncUseCoroutineSection::Coroutines,
                },
                class: "header",
                "Coroutines"
            }
        }
        p {
            "Another tool in your async toolbox are coroutines. Coroutines are futures that can be manually stopped, started, paused, and resumed."
        }
        p {
            "Like regular futures, code in a coroutine will run until the next  "
            code { "await" }
            " point before yielding. This low-level control over asynchronous tasks is quite powerful, allowing for infinitely looping tasks like WebSocket polling, background timers, and other periodic actions."
        }
        h2 { id: "use-coroutine",
            Link {
                to: BookRoute::AsyncUseCoroutine {
                    section: AsyncUseCoroutineSection::UseCoroutine,
                },
                class: "header",
                "use_coroutine"
            }
        }
        p {
            "The  "
            code { "use_coroutine" }
            " hook allows you to create a coroutine. Most coroutines we write will be polling loops using async/await."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> ws: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">UseCoroutine&lt;()&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_coroutine</span><span style=\"color:#f8f8f2;\">(cx, |</span><span style=\"font-style:italic;color:#fd971f;\">rx</span><span style=\"color:#f8f8f2;\">| async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Connect to some sort of service\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> conn </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">connect_to_ws_server</span><span style=\"color:#f8f8f2;\">().await;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Wait for data on the service\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">while </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(msg) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> conn.</span><span style=\"color:#66d9ef;\">next</span><span style=\"color:#f8f8f2;\">().await {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// handle messages\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "For many services, a simple async loop will handle the majority of use cases." }
        p {
            "However, if we want to temporarily disable the coroutine, we can \"pause\" it using the  "
            code { "pause" }
            " method, and \"resume\" it using the  "
            code { "resume" }
            " method:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> sync: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">UseCoroutine&lt;()&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_coroutine</span><span style=\"color:#f8f8f2;\">(cx, |</span><span style=\"font-style:italic;color:#fd971f;\">rx</span><span style=\"color:#f8f8f2;\">| async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// code for syncing\n</span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> sync.</span><span style=\"color:#66d9ef;\">is_running</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> sync.</span><span style=\"color:#66d9ef;\">pause</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Disable syncing&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> sync.</span><span style=\"color:#66d9ef;\">resume</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Enable syncing&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This pattern is where coroutines are extremely useful – instead of writing all the complicated logic for pausing our async tasks like we would with JavaScript promises, the Rust model allows us to just not poll our future."
        }
        h2 { id: "yielding-values",
            Link {
                to: BookRoute::AsyncUseCoroutine {
                    section: AsyncUseCoroutineSection::YieldingValues,
                },
                class: "header",
                "Yielding Values"
            }
        }
        p {
            "To yield values from a coroutine, simply bring in a  "
            code { "UseState" }
            " handle and set the value whenever your coroutine completes its work."
        }
        p {
            "The future must be  "
            code { "'static" }
            " – so any values captured by the task cannot carry any references to  "
            code { "cx" }
            ", such as a  "
            code { "UseState" }
            "."
        }
        p {
            "You can use "
            Link { to: "https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned",
                "to_owned"
            }
            " to create a clone of the hook handle which can be moved into the async closure."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> sync_status </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || Status::Launching);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> sync_task </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_coroutine</span><span style=\"color:#f8f8f2;\">(cx, |</span><span style=\"font-style:italic;color:#fd971f;\">rx</span><span style=\"color:#f8f8f2;\">: UnboundedReceiver&lt;SyncAction&gt;| {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> sync_status </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> sync_status.</span><span style=\"color:#66d9ef;\">to_owned</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">loop </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#66d9ef;\">delay_ms</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">1000</span><span style=\"color:#f8f8f2;\">).await;\n</span><span style=\"color:#f8f8f2;\">            sync_status.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(Status::Working);\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
        }
        p {
            "To make this a bit less verbose, Dioxus exports the  "
            code { "to_owned!" }
            " macro which will create a binding as shown above, which can be quite helpful when dealing with many values."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> sync_status </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || Status::Launching);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> load_status </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || Status::Launching);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> sync_task </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_coroutine</span><span style=\"color:#f8f8f2;\">(cx, |</span><span style=\"font-style:italic;color:#fd971f;\">rx</span><span style=\"color:#f8f8f2;\">: UnboundedReceiver&lt;SyncAction&gt;| {{\n</span><span style=\"color:#f8f8f2;\">    to_owned![sync_status, load_status];\n</span><span style=\"color:#f8f8f2;\">    async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// ...\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
        }
        h2 { id: "sending-values",
            Link {
                to: BookRoute::AsyncUseCoroutine {
                    section: AsyncUseCoroutineSection::SendingValues,
                },
                class: "header",
                "Sending Values"
            }
        }
        p {
            "You might've noticed the  "
            code { "use_coroutine" }
            " closure takes an argument called  "
            code { "rx" }
            ". What is that? Well, a common pattern in complex apps is to handle a bunch of async code at once. With libraries like Redux Toolkit, managing multiple promises at once can be challenging and a common source of bugs."
        }
        p {
            "With Coroutines, we can centralize our async logic. The  "
            code { "rx" }
            " parameter is an Channel that allows code external to the coroutine to send data "
            em { "into" }
            " the coroutine. Instead of looping on an external service, we can loop on the channel itself, processing messages from within our app without needing to spawn a new future. To send data into the coroutine, we would call \"send\" on the handle."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">ProfileUpdate {{\n</span><span style=\"color:#f8f8f2;\">    SetUsername(</span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    SetAge(</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> profile </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_coroutine</span><span style=\"color:#f8f8f2;\">(cx, |</span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">rx</span><span style=\"color:#f8f8f2;\">: UnboundedReciver&lt;ProfileUpdate&gt;| async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> server </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">connect_to_server</span><span style=\"color:#f8f8f2;\">().await;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">while </span><span style=\"font-style:italic;color:#66d9ef;\">let Ok</span><span style=\"color:#f8f8f2;\">(msg) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> rx.</span><span style=\"color:#66d9ef;\">next</span><span style=\"color:#f8f8f2;\">().await {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> msg {{\n</span><span style=\"color:#f8f8f2;\">            ProfileUpdate::SetUsername(name) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> server.</span><span style=\"color:#66d9ef;\">update_username</span><span style=\"color:#f8f8f2;\">(name).await,\n</span><span style=\"color:#f8f8f2;\">            ProfileUpdate::SetAge(age) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> server.</span><span style=\"color:#66d9ef;\">update_age</span><span style=\"color:#f8f8f2;\">(age).await,\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">    button {{\n</span><span style=\"color:#f8f8f2;\">        onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> profile.</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">(ProfileUpdate::SetUsername(</span><span style=\"color:#ffee99;\">&quot;Bob&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">())),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Update username&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
        }
        p {
            "For sufficiently complex apps, we could build a bunch of different useful \"services\" that loop on channels to update the app."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> profile </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_coroutine</span><span style=\"color:#f8f8f2;\">(cx, profile_service);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> editor </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_coroutine</span><span style=\"color:#f8f8f2;\">(cx, editor_service);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> sync </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_coroutine</span><span style=\"color:#f8f8f2;\">(cx, sync_service);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">profile_service</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">rx</span><span style=\"color:#f8f8f2;\">: UnboundedReceiver&lt;ProfileCommand&gt;) {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// do stuff\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">sync_service</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">rx</span><span style=\"color:#f8f8f2;\">: UnboundedReceiver&lt;SyncCommand&gt;) {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// do stuff\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">editor_service</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">rx</span><span style=\"color:#f8f8f2;\">: UnboundedReceiver&lt;EditorCommand&gt;) {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// do stuff\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "We can combine coroutines with "
            Link { to: "https://docs.rs/fermi/latest/fermi/index.html", "Fermi" }
            " to emulate Redux Toolkit's Thunk system with much less headache. This lets us store all of our app's state "
            em { "within" }
            " a task and then simply update the \"view\" values stored in Atoms. It cannot be understated how powerful this technique is: we get all the perks of native Rust tasks with the optimizations and ergonomics of global state. This means your "
            em { "actual" }
            " state does not need to be tied up in a system like Fermi or Redux – the only Atoms that need to exist are those that are used to drive the display/UI."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">USERNAME</span><span style=\"color:#f8f8f2;\">: Atom&lt;String&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">|_| </span><span style=\"color:#ffee99;\">&quot;default&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> atoms </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_atom_root</span><span style=\"color:#f8f8f2;\">(cx);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_coroutine</span><span style=\"color:#f8f8f2;\">(cx, |</span><span style=\"font-style:italic;color:#fd971f;\">rx</span><span style=\"color:#f8f8f2;\">| </span><span style=\"color:#66d9ef;\">sync_service</span><span style=\"color:#f8f8f2;\">(rx, atoms.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">()));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        Banner {{}}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Banner</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> username </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_read</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"color:#ff80f4;\">USERNAME</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;Welcome back, {{username}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Now, in our sync service, we can structure our state however we want. We only need to update the view values when ready."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">SyncAction {{\n</span><span style=\"color:#f8f8f2;\">    SetUsername(</span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">sync_service</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">rx</span><span style=\"color:#f8f8f2;\">: UnboundedReceiver&lt;SyncAction&gt;, </span><span style=\"font-style:italic;color:#fd971f;\">atoms</span><span style=\"color:#f8f8f2;\">: AtomRoot) {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> username </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> atoms.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">USERNAME</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> errors </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> atoms.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">ERRORS</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">while </span><span style=\"font-style:italic;color:#66d9ef;\">let Ok</span><span style=\"color:#f8f8f2;\">(msg) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> rx.</span><span style=\"color:#66d9ef;\">next</span><span style=\"color:#f8f8f2;\">().await {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> msg {{\n</span><span style=\"color:#f8f8f2;\">            SyncAction::SetUsername(name) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">set_name_on_server</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">name).await.</span><span style=\"color:#66d9ef;\">is_ok</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">                    username.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(name);\n</span><span style=\"color:#f8f8f2;\">                }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    errors.</span><span style=\"color:#66d9ef;\">make_mut</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;SetUsernameFailed&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "automatic-injection-into-the-context-api",
            Link {
                to: BookRoute::AsyncUseCoroutine {
                    section: AsyncUseCoroutineSection::AutomaticInjectionIntoTheContextApi,
                },
                class: "header",
                "Automatic injection into the Context API"
            }
        }
        p {
            "Coroutine handles are automatically injected through the context API. You can use the  "
            code { "use_coroutine_handle" }
            " hook with the message type as a generic to fetch a handle."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> sync_task </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">use_coroutine_handle::&lt;SyncAction&gt;(cx);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    sync_task.</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">(SyncAction::SetUsername);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum AsyncSpawnSection {
    #[default]
    Empty,
    SpawningFutures,
    SpawningTokioTasks,
}
impl std::str::FromStr for AsyncSpawnSection {
    type Err = AsyncSpawnSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "spawning-futures" => Ok(Self::SpawningFutures),
            "spawning-tokio-tasks" => Ok(Self::SpawningTokioTasks),
            _ => Err(AsyncSpawnSectionParseError),
        }
    }
}
impl std::fmt::Display for AsyncSpawnSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::SpawningFutures => f.write_str("spawning-futures"),
            Self::SpawningTokioTasks => f.write_str("spawning-tokio-tasks"),
        }
    }
}
#[derive(Debug)]
pub struct AsyncSpawnSectionParseError;
impl std::fmt::Display for AsyncSpawnSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of AsyncSpawnSectionspawning-futures, spawning-tokio-tasks",
        )?;
        Ok(())
    }
}
impl std::error::Error for AsyncSpawnSectionParseError {}
#[component(no_case_check)]
pub fn AsyncSpawn(section: AsyncSpawnSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "spawning-futures",
            Link {
                to: BookRoute::AsyncSpawn {
                    section: AsyncSpawnSection::SpawningFutures,
                },
                class: "header",
                "Spawning Futures"
            }
        }
        p {
            "The  "
            code { "use_future" }
            " and  "
            code { "use_coroutine" }
            " hooks are useful if you want to unconditionally spawn the future. Sometimes, though, you'll want to only spawn a future in response to an event, such as a mouse click. For example, suppose you need to send a request when the user clicks a \"log in\" button. For this, you can use  "
            code { "cx.spawn" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> logged_in </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">false</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> log_in </span><span style=\"color:#f92672;\">= move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">spawn</span><span style=\"color:#f8f8f2;\">({{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> logged_in </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> logged_in.</span><span style=\"color:#66d9ef;\">to_owned</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> resp </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">reqwest::Client::new()\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">post</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;http://example.com/login&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                .await;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> resp {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(_data) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    println!(</span><span style=\"color:#ffee99;\">&quot;Login successful!&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                    logged_in.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(_err) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    println!(\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"color:#ffee99;\">&quot;Login failed - you need a login server running on localhost:8080.&quot;\n</span><span style=\"color:#f8f8f2;\">                    )\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">    button {{\n</span><span style=\"color:#f8f8f2;\">        onclick: log_in,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Login&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "spawn.rs".to_string(),
        }
        blockquote {
            p {
                "Note:  "
                code { "spawn" }
                " will always spawn a "
                em { "new" }
                " future. You most likely don't want to call it on every render."
            }
        }
        p {
            "Calling  "
            code { "spawn" }
            " will give you a  "
            code { "JoinHandle" }
            " which lets you cancel or pause the future."
        }
        h2 { id: "spawning-tokio-tasks",
            Link {
                to: BookRoute::AsyncSpawn {
                    section: AsyncSpawnSection::SpawningTokioTasks,
                },
                class: "header",
                "Spawning Tokio Tasks"
            }
        }
        p {
            "Sometimes, you might want to spawn a background task that needs multiple threads or talk to hardware that might block your app code. In these cases, we can directly spawn a Tokio task from our future. For Dioxus-Desktop, your task will be spawned onto Tokio's Multithreaded runtime:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">spawn</span><span style=\"color:#f8f8f2;\">(async {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">_ = </span><span style=\"color:#f8f8f2;\">tokio::spawn(async {{}}).await;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">_ = </span><span style=\"color:#f8f8f2;\">tokio::task::spawn_local(async {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// some !Send work\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">    .await;\n</span><span style=\"color:#f8f8f2;\">}});</span></pre>\n",
            name: "spawn.rs".to_string(),
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum BestPracticesIndexSection {
    #[default]
    Empty,
    BestPractices,
    ReusableComponents,
    MinimizeStateDependencies,
}
impl std::str::FromStr for BestPracticesIndexSection {
    type Err = BestPracticesIndexSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "best-practices" => Ok(Self::BestPractices),
            "reusable-components" => Ok(Self::ReusableComponents),
            "minimize-state-dependencies" => Ok(Self::MinimizeStateDependencies),
            _ => Err(BestPracticesIndexSectionParseError),
        }
    }
}
impl std::fmt::Display for BestPracticesIndexSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::BestPractices => f.write_str("best-practices"),
            Self::ReusableComponents => f.write_str("reusable-components"),
            Self::MinimizeStateDependencies => f.write_str("minimize-state-dependencies"),
        }
    }
}
#[derive(Debug)]
pub struct BestPracticesIndexSectionParseError;
impl std::fmt::Display for BestPracticesIndexSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of BestPracticesIndexSectionbest-practices, reusable-components, minimize-state-dependencies",
        )?;
        Ok(())
    }
}
impl std::error::Error for BestPracticesIndexSectionParseError {}
#[component(no_case_check)]
pub fn BestPracticesIndex(section: BestPracticesIndexSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "best-practices",
            Link {
                to: BookRoute::BestPracticesIndex {
                    section: BestPracticesIndexSection::BestPractices,
                },
                class: "header",
                "Best Practices"
            }
        }
        h2 { id: "reusable-components",
            Link {
                to: BookRoute::BestPracticesIndex {
                    section: BestPracticesIndexSection::ReusableComponents,
                },
                class: "header",
                "Reusable Components"
            }
        }
        p {
            "As much as possible, break your code down into small, reusable components and hooks, instead of implementing large chunks of the UI in a single component. This will help you keep the code maintainable – it is much easier to e.g. add, remove or re-order parts of the UI if it is organized in components."
        }
        p { "Organize your components in modules to keep the codebase easy to navigate!" }
        h2 { id: "minimize-state-dependencies",
            Link {
                to: BookRoute::BestPracticesIndex {
                    section: BestPracticesIndexSection::MinimizeStateDependencies,
                },
                class: "header",
                "Minimize State Dependencies"
            }
        }
        p {
            "While it is possible to share state between components, this should only be done when necessary. Any component that is associated with a particular state object needs to be re-rendered when that state changes. For this reason:"
        }
        ul {
            li { "Keep state local to a component if possible" }
            li { "When sharing state through props, only pass down the specific data necessary" }
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum BestPracticesErrorHandlingSection {
    #[default]
    Empty,
    ErrorHandling,
    TheSimplestReturningNone,
    EarlyReturnOnResult,
    MatchResults,
    PassingErrorStatesThroughComponents,
    GoingGlobal,
}
impl std::str::FromStr for BestPracticesErrorHandlingSection {
    type Err = BestPracticesErrorHandlingSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "error-handling" => Ok(Self::ErrorHandling),
            "the-simplest--returning-none" => Ok(Self::TheSimplestReturningNone),
            "early-return-on-result" => Ok(Self::EarlyReturnOnResult),
            "match-results" => Ok(Self::MatchResults),
            "passing-error-states-through-components" => {
                Ok(Self::PassingErrorStatesThroughComponents)
            }
            "going-global" => Ok(Self::GoingGlobal),
            _ => Err(BestPracticesErrorHandlingSectionParseError),
        }
    }
}
impl std::fmt::Display for BestPracticesErrorHandlingSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::ErrorHandling => f.write_str("error-handling"),
            Self::TheSimplestReturningNone => f.write_str("the-simplest--returning-none"),
            Self::EarlyReturnOnResult => f.write_str("early-return-on-result"),
            Self::MatchResults => f.write_str("match-results"),
            Self::PassingErrorStatesThroughComponents => {
                f.write_str("passing-error-states-through-components")
            }
            Self::GoingGlobal => f.write_str("going-global"),
        }
    }
}
#[derive(Debug)]
pub struct BestPracticesErrorHandlingSectionParseError;
impl std::fmt::Display for BestPracticesErrorHandlingSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of BestPracticesErrorHandlingSectionerror-handling, the-simplest--returning-none, early-return-on-result, match-results, passing-error-states-through-components, going-global",
        )?;
        Ok(())
    }
}
impl std::error::Error for BestPracticesErrorHandlingSectionParseError {}
#[component(no_case_check)]
pub fn BestPracticesErrorHandling(
    section: BestPracticesErrorHandlingSection,
) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "error-handling",
            Link {
                to: BookRoute::BestPracticesErrorHandling {
                    section: BestPracticesErrorHandlingSection::ErrorHandling,
                },
                class: "header",
                "Error handling"
            }
        }
        p {
            "A selling point of Rust for web development is the reliability of always knowing where errors can occur and being forced to handle them"
        }
        p {
            "However, we haven't talked about error handling at all in this guide! In this chapter, we'll cover some strategies in handling errors to ensure your app never crashes."
        }
        h2 { id: "the-simplest--returning-none",
            Link {
                to: BookRoute::BestPracticesErrorHandling {
                    section: BestPracticesErrorHandlingSection::TheSimplestReturningNone,
                },
                class: "header",
                "The simplest – returning None"
            }
        }
        p {
            "Astute observers might have noticed that  "
            code { "Element" }
            " is actually a type alias for  "
            code { "Option<VNode>" }
            ". You don't need to know what a  "
            code { "VNode" }
            " is, but it's important to recognize that we could actually return nothing at all:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">None\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "This lets us add in some syntactic sugar for operations we think "
            em { "shouldn't" }
            " fail, but we're still not confident enough to \"unwrap\" on."
        }
        blockquote {
            p {
                "The nature of  "
                code { "Option<VNode>" }
                " might change in the future as the  "
                code { "try" }
                " trait gets upgraded."
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// immediately return &quot;None&quot;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> cx.</span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|_| </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;hi&quot;</span><span style=\"color:#f8f8f2;\">))</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "early-return-on-result",
            Link {
                to: BookRoute::BestPracticesErrorHandling {
                    section: BestPracticesErrorHandlingSection::EarlyReturnOnResult,
                },
                class: "header",
                "Early return on result"
            }
        }
        p {
            "Because Rust can't accept both Options and Results with the existing try infrastructure, you'll need to manually handle Results. This can be done by converting them into Options or by explicitly handling them."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Convert Result to Option\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> cx.</span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|_| </span><span style=\"color:#ffee99;\">&quot;1.234&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">ok</span><span style=\"color:#f8f8f2;\">()</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Early return\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> cx.</span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|_| </span><span style=\"color:#ffee99;\">&quot;1.234&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> val </span><span style=\"color:#f92672;\">= match</span><span style=\"color:#f8f8f2;\"> count.</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(val) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> val\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(err) </span><span style=\"color:#f92672;\">=&gt; return</span><span style=\"color:#f8f8f2;\"> cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{ </span><span style=\"color:#ffee99;\">&quot;Parsing failed&quot; </span><span style=\"color:#f8f8f2;\">}})\n</span><span style=\"color:#f8f8f2;\">    }};\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Notice that while hooks in Dioxus do not like being called in conditionals or loops, they "
            em { "are" }
            " okay with early returns. Returning an error state early is a completely valid way of handling errors."
        }
        h2 { id: "match-results",
            Link {
                to: BookRoute::BestPracticesErrorHandling {
                    section: BestPracticesErrorHandlingSection::MatchResults,
                },
                class: "header",
                "Match results"
            }
        }
        p {
            "The next \"best\" way of handling errors in Dioxus is to match on the error locally. This is the most robust way of handling errors, though it doesn't scale to architectures beyond a single component."
        }
        p { "To do this, we simply have an error state built into our component:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> err </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);</span></pre>\n" }
        p {
            "Whenever we perform an action that generates an error, we'll set that error state. We can then match on the error in a number of ways (early return, return Element, etc)."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Commandline</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">match *</span><span style=\"color:#f8f8f2;\">error {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(error) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx!(\n</span><span style=\"color:#f8f8f2;\">            h1 {{ </span><span style=\"color:#ffee99;\">&quot;An error occured&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        )\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx!(\n</span><span style=\"color:#f8f8f2;\">            input {{\n</span><span style=\"color:#f8f8f2;\">                oninput: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> error.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;bad thing happened!&quot;</span><span style=\"color:#f8f8f2;\">)),\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        )\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "passing-error-states-through-components",
            Link {
                to: BookRoute::BestPracticesErrorHandling {
                    section: BestPracticesErrorHandlingSection::PassingErrorStatesThroughComponents,
                },
                class: "header",
                "Passing error states through components"
            }
        }
        p {
            "If you're dealing with a handful of components with minimal nesting, you can just pass the error handle into child components."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Commandline</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(error) </span><span style=\"color:#f92672;\">= **</span><span style=\"color:#f8f8f2;\">error {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">return</span><span style=\"color:#f8f8f2;\"> cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{ </span><span style=\"color:#ffee99;\">&quot;An error occured&quot; </span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        Child {{ error: error.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">        Child {{ error: error.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">        Child {{ error: error.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">        Child {{ error: error.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Much like before, our child components can manually set the error during their own actions. The advantage to this pattern is that we can easily isolate error states to a few components at a time, making our app more predictable and robust."
        }
        h2 { id: "going-global",
            Link {
                to: BookRoute::BestPracticesErrorHandling {
                    section: BestPracticesErrorHandlingSection::GoingGlobal,
                },
                class: "header",
                "Going global"
            }
        }
        p {
            "A strategy for handling cascaded errors in larger apps is through signaling an error using global state. This particular pattern involves creating an \"error\" context, and then setting it wherever relevant. This particular method is not as \"sophisticated\" as React's error boundary, but it is more fitting for Rust."
        }
        p {
            "To get started, consider using a built-in hook like  "
            code { "use_context" }
            " and  "
            code { "use_context_provider" }
            " or Fermi. Of course, it's pretty easy to roll your own hook too."
        }
        p {
            "At the \"top\" of our architecture, we're going to want to explicitly declare a value that could be an error."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">InputError {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    TooLong,\n</span><span style=\"color:#f8f8f2;\">    TooShort,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">INPUT_ERROR</span><span style=\"color:#f8f8f2;\">: Atom&lt;InputError&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">|_| InputError::None;</span></pre>\n" }
        p {
            "Then, in our top level component, we want to explicitly handle the possible error state for this part of the tree."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">TopLevel</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_read</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"color:#ff80f4;\">INPUT_ERROR</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> error {{\n</span><span style=\"color:#f8f8f2;\">        TooLong </span><span style=\"color:#f92672;\">=&gt; return</span><span style=\"color:#f8f8f2;\"> cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{ </span><span style=\"color:#ffee99;\">&quot;FAILED: Too long!&quot; </span><span style=\"color:#f8f8f2;\">}}),\n</span><span style=\"color:#f8f8f2;\">        TooShort </span><span style=\"color:#f92672;\">=&gt; return</span><span style=\"color:#f8f8f2;\"> cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{ </span><span style=\"color:#ffee99;\">&quot;FAILED: Too Short!&quot; </span><span style=\"color:#f8f8f2;\">}}),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">_ =&gt; </span><span style=\"color:#f8f8f2;\">{{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Now, whenever a downstream component has an error in its actions, it can simply just set its own error state:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Commandline</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> set_error </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_set</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"color:#ff80f4;\">INPUT_ERROR</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">        input {{\n</span><span style=\"color:#f8f8f2;\">            oninput: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">evt</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> evt.value.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">&gt; </span><span style=\"color:#ff80f4;\">20 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#66d9ef;\">set_error</span><span style=\"color:#f8f8f2;\">(InputError::TooLong);\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This approach to error handling is best in apps that have \"well defined\" error states. Consider using a crate like  "
            code { "thiserror" }
            " or  "
            code { "anyhow" }
            " to simplify the generation of the error types."
        }
        p {
            "This pattern is widely popular in many contexts and is particularly helpful whenever your code generates a non-recoverable error. You can gracefully capture these \"global\" error states without panicking or mucking up state."
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum BestPracticesAntipatternsSection {
    #[default]
    Empty,
    Antipatterns,
    UnnecessarilyNestedFragments,
    IncorrectIteratorKeys,
    AvoidInteriorMutabilityInProps,
    AvoidUpdatingStateDuringRender,
}
impl std::str::FromStr for BestPracticesAntipatternsSection {
    type Err = BestPracticesAntipatternsSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "antipatterns" => Ok(Self::Antipatterns),
            "unnecessarily-nested-fragments" => Ok(Self::UnnecessarilyNestedFragments),
            "incorrect-iterator-keys" => Ok(Self::IncorrectIteratorKeys),
            "avoid-interior-mutability-in-props" => Ok(Self::AvoidInteriorMutabilityInProps),
            "avoid-updating-state-during-render" => Ok(Self::AvoidUpdatingStateDuringRender),
            _ => Err(BestPracticesAntipatternsSectionParseError),
        }
    }
}
impl std::fmt::Display for BestPracticesAntipatternsSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Antipatterns => f.write_str("antipatterns"),
            Self::UnnecessarilyNestedFragments => f.write_str("unnecessarily-nested-fragments"),
            Self::IncorrectIteratorKeys => f.write_str("incorrect-iterator-keys"),
            Self::AvoidInteriorMutabilityInProps => {
                f.write_str("avoid-interior-mutability-in-props")
            }
            Self::AvoidUpdatingStateDuringRender => {
                f.write_str("avoid-updating-state-during-render")
            }
        }
    }
}
#[derive(Debug)]
pub struct BestPracticesAntipatternsSectionParseError;
impl std::fmt::Display for BestPracticesAntipatternsSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of BestPracticesAntipatternsSectionantipatterns, unnecessarily-nested-fragments, incorrect-iterator-keys, avoid-interior-mutability-in-props, avoid-updating-state-during-render",
        )?;
        Ok(())
    }
}
impl std::error::Error for BestPracticesAntipatternsSectionParseError {}
#[component(no_case_check)]
pub fn BestPracticesAntipatterns(
    section: BestPracticesAntipatternsSection,
) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "antipatterns",
            Link {
                to: BookRoute::BestPracticesAntipatterns {
                    section: BestPracticesAntipatternsSection::Antipatterns,
                },
                class: "header",
                "Antipatterns"
            }
        }
        p {
            "This example shows what not to do and provides a reason why a given pattern is considered an \"AntiPattern\". Most anti-patterns are considered wrong for performance or code re-usability reasons."
        }
        h2 { id: "unnecessarily-nested-fragments",
            Link {
                to: BookRoute::BestPracticesAntipatterns {
                    section: BestPracticesAntipatternsSection::UnnecessarilyNestedFragments,
                },
                class: "header",
                "Unnecessarily Nested Fragments"
            }
        }
        p {
            "Fragments don't mount a physical element to the DOM immediately, so Dioxus must recurse into its children to find a physical DOM node. This process is called \"normalization\". This means that deeply nested fragments make Dioxus perform unnecessary work. Prefer one or two levels of fragments / nested components until presenting a true DOM element."
        }
        p {
            "Only Component and Fragment nodes are susceptible to this issue. Dioxus mitigates this with components by providing an API for registering shared state without the Context Provider pattern."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ Don&#39;t unnecessarily nest fragments\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">_ =</span><span style=\"color:#f8f8f2;\"> cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">    Fragment {{\n</span><span style=\"color:#f8f8f2;\">        Fragment {{\n</span><span style=\"color:#f8f8f2;\">            Fragment {{\n</span><span style=\"color:#f8f8f2;\">                Fragment {{\n</span><span style=\"color:#f8f8f2;\">                    Fragment {{\n</span><span style=\"color:#f8f8f2;\">                        div {{ </span><span style=\"color:#ffee99;\">&quot;Finally have a real node!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Render shallow structures\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!(\n</span><span style=\"color:#f8f8f2;\">    div {{ </span><span style=\"color:#ffee99;\">&quot;Finally have a real node!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">))</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "incorrect-iterator-keys",
            Link {
                to: BookRoute::BestPracticesAntipatterns {
                    section: BestPracticesAntipatternsSection::IncorrectIteratorKeys,
                },
                class: "header",
                "Incorrect Iterator Keys"
            }
        }
        p {
            "As described in the "
            Link {
                to: BookRoute::InteractivityDynamicRendering {
                    section: InteractivityDynamicRenderingSection::TheKeyAttribute,
                },
                "dynamic rendering chapter"
            }
            ", list items must have unique keys that are associated with the same items across renders. This helps Dioxus associate state with the contained components and ensures good diffing performance. Do not omit keys, unless you know that the list will never change."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> data: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">HashMap&lt;</span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">&gt; </span><span style=\"color:#f92672;\">= &amp;</span><span style=\"color:#f8f8f2;\">cx.props.data;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ❌ No keys\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ul {{\n</span><span style=\"color:#f8f8f2;\">        data.</span><span style=\"color:#66d9ef;\">values</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">value</span><span style=\"color:#f8f8f2;\">| rsx!(\n</span><span style=\"color:#f8f8f2;\">            li {{ </span><span style=\"color:#ffee99;\">&quot;List item: {{value}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        ))\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ❌ Using index as keys\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ul {{\n</span><span style=\"color:#f8f8f2;\">        cx.props.data.</span><span style=\"color:#66d9ef;\">values</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">enumerate</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|(</span><span style=\"font-style:italic;color:#fd971f;\">index</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">value</span><span style=\"color:#f8f8f2;\">)| rsx!(\n</span><span style=\"color:#f8f8f2;\">            li {{ key: </span><span style=\"color:#ffee99;\">&quot;{{index}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;List item: {{value}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        ))\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Using unique IDs as keys:\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ul {{\n</span><span style=\"color:#f8f8f2;\">        cx.props.data.</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|(</span><span style=\"font-style:italic;color:#fd971f;\">key</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">value</span><span style=\"color:#f8f8f2;\">)| rsx!(\n</span><span style=\"color:#f8f8f2;\">            li {{ key: </span><span style=\"color:#ffee99;\">&quot;{{key}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;List item: {{value}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        ))\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "avoid-interior-mutability-in-props",
            Link {
                to: BookRoute::BestPracticesAntipatterns {
                    section: BestPracticesAntipatternsSection::AvoidInteriorMutabilityInProps,
                },
                class: "header",
                "Avoid Interior Mutability in Props"
            }
        }
        p {
            "While it is technically acceptable to have a  "
            code { "Mutex" }
            " or a  "
            code { "RwLock" }
            " in the props, they will be difficult to use."
        }
        p {
            "Suppose you have a struct  "
            code { "User" }
            " containing the field  "
            code { "username: String" }
            ". If you pass a  "
            code { "Mutex<User>" }
            " prop to a  "
            code { "UserComponent" }
            " component, that component may wish to pass the username as a  "
            code { "&str" }
            " prop to a child component. However, it cannot pass that borrowed field down, since it only would live as long as the  "
            code { "Mutex" }
            "'s lock, which belongs to the  "
            code { "UserComponent" }
            " function. Therefore, the component will be forced to clone the  "
            code { "username" }
            " field."
        }
        h2 { id: "avoid-updating-state-during-render",
            Link {
                to: BookRoute::BestPracticesAntipatterns {
                    section: BestPracticesAntipatternsSection::AvoidUpdatingStateDuringRender,
                },
                class: "header",
                "Avoid Updating State During Render"
            }
        }
        p {
            "Every time you update the state, Dioxus needs to re-render the component – this is inefficient! Consider refactoring your code to avoid this."
        }
        p {
            "Also, if you unconditionally update the state during render, it will be re-rendered in an infinite loop."
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum PublishingIndexSection {
    #[default]
    Empty,
    Publishing,
}
impl std::str::FromStr for PublishingIndexSection {
    type Err = PublishingIndexSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "publishing" => Ok(Self::Publishing),
            _ => Err(PublishingIndexSectionParseError),
        }
    }
}
impl std::fmt::Display for PublishingIndexSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Publishing => f.write_str("publishing"),
        }
    }
}
#[derive(Debug)]
pub struct PublishingIndexSectionParseError;
impl std::fmt::Display for PublishingIndexSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid section name. Expected one of PublishingIndexSectionpublishing")?;
        Ok(())
    }
}
impl std::error::Error for PublishingIndexSectionParseError {}
#[component(no_case_check)]
pub fn PublishingIndex(section: PublishingIndexSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "publishing",
            Link {
                to: BookRoute::PublishingIndex {
                    section: PublishingIndexSection::Publishing,
                },
                class: "header",
                "Publishing"
            }
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum PublishingDesktopSection {
    #[default]
    Empty,
    Publishing,
    InstallCargoBundle,
    SettingUpYourProject,
    Building,
}
impl std::str::FromStr for PublishingDesktopSection {
    type Err = PublishingDesktopSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "publishing" => Ok(Self::Publishing),
            "install-cargo-bundle" => Ok(Self::InstallCargoBundle),
            "setting-up-your-project" => Ok(Self::SettingUpYourProject),
            "building" => Ok(Self::Building),
            _ => Err(PublishingDesktopSectionParseError),
        }
    }
}
impl std::fmt::Display for PublishingDesktopSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Publishing => f.write_str("publishing"),
            Self::InstallCargoBundle => f.write_str("install-cargo-bundle"),
            Self::SettingUpYourProject => f.write_str("setting-up-your-project"),
            Self::Building => f.write_str("building"),
        }
    }
}
#[derive(Debug)]
pub struct PublishingDesktopSectionParseError;
impl std::fmt::Display for PublishingDesktopSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of PublishingDesktopSectionpublishing, install-cargo-bundle, setting-up-your-project, building",
        )?;
        Ok(())
    }
}
impl std::error::Error for PublishingDesktopSectionParseError {}
#[component(no_case_check)]
pub fn PublishingDesktop(section: PublishingDesktopSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "publishing",
            Link {
                to: BookRoute::PublishingDesktop {
                    section: PublishingDesktopSection::Publishing,
                },
                class: "header",
                "Publishing"
            }
        }
        p {
            "Congrats! You've made your first Dioxus app that actually does some pretty cool stuff. This app uses your operating system's WebView library, so it's portable to be distributed for other platforms."
        }
        p { "In this section, we'll cover how to bundle your app for macOS, Windows, and Linux." }
        h2 { id: "install-cargo-bundle",
            Link {
                to: BookRoute::PublishingDesktop {
                    section: PublishingDesktopSection::InstallCargoBundle,
                },
                class: "header",
                "Install cargo-bundle"
            }
        }
        p {
            "The first thing we'll do is install "
            Link { to: "https://github.com/burtonageo/cargo-bundle",
                code { "cargo-bundle" }
            }
            ". This extension to cargo will make it very easy to package our app for the various platforms."
        }
        p {
            "According to the  "
            code { "cargo-bundle" }
            " github page,"
        }
        p {
            em {
                "\"cargo-bundle is a tool used to generate installers or app bundles for GUI  executables built with cargo. It can create .app bundles for Mac OS X and iOS, .deb packages for Linux, and .msi installers for Windows (note however that iOS and Windows support is still experimental). Support for creating .rpm packages (for Linux) and .apk packages (for Android) is still pending.\""
            }
        }
        p { "To install, simply run" }
        p {
            code { "cargo install cargo-bundle" }
        }
        h2 { id: "setting-up-your-project",
            Link {
                to: BookRoute::PublishingDesktop {
                    section: PublishingDesktopSection::SettingUpYourProject,
                },
                class: "header",
                "Setting up your project"
            }
        }
        p {
            "To get a project setup for bundling, we need to add some flags to our  "
            code { "Cargo.toml" }
            " file."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[package]\n</span><span style=\"color:#f8f8f2;\">name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;example&quot;\n</span><span style=\"color:#f92672;\"># ...</span><span style=\"color:#f8f8f2;\">other fields</span><span style=\"color:#f92672;\">...\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">[package.metadata.bundle]\n</span><span style=\"color:#f8f8f2;\">name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;DogSearch&quot;\n</span><span style=\"color:#f8f8f2;\">identifier </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;com.dogs.dogsearch&quot;\n</span><span style=\"color:#f8f8f2;\">version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;1.0.0&quot;\n</span><span style=\"color:#f8f8f2;\">copyright </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;Copyright (c) Jane Doe 2016. All rights reserved.&quot;\n</span><span style=\"color:#f8f8f2;\">category </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;Developer Tool&quot;\n</span><span style=\"color:#f8f8f2;\">short_description </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;Easily search for Dog photos&quot;\n</span><span style=\"color:#f8f8f2;\">long_description </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;&quot;&quot;\n</span><span style=\"color:#ffee99;\">This app makes it quick and easy to browse photos of dogs from over 200 bree\n</span><span style=\"color:#ffee99;\">&quot;&quot;&quot;</span></pre>\n",
        }
        h2 { id: "building",
            Link {
                to: BookRoute::PublishingDesktop {
                    section: PublishingDesktopSection::Building,
                },
                class: "header",
                "Building"
            }
        }
        p {
            "Following cargo-bundle's instructions, we simply  "
            code { "cargo-bundle --release" }
            " to produce a final app with all the optimizations and assets builtin."
        }
        p {
            "Once you've ran  "
            code { "cargo-bundle --release" }
            ", your app should be accessible in"
        }
        p {
            code { "target/release/bundle/<platform>/" }
            "."
        }
        p { "For example, a macOS app would look like this:" }
        p {
            img {
                src: asset!("/assets/static/publish.png", ImageAssetOptions::new().with_avif()),
                alt: "Published App",
                title: "",
            }
        }
        p {
            "Nice! And it's only 4.8 Mb – extremely lean!! Because Dioxus leverages your platform's native WebView, Dioxus apps are extremely memory efficient and won't waste your battery."
        }
        blockquote {
            p {
                "Note: not all CSS works the same on all platforms. Make sure to view your app's CSS on each platform – or web browser (Firefox, Chrome, Safari) before publishing."
            }
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum PublishingWebSection {
    #[default]
    Empty,
    PublishingWithGithubPages,
}
impl std::str::FromStr for PublishingWebSection {
    type Err = PublishingWebSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "publishing-with-github-pages" => Ok(Self::PublishingWithGithubPages),
            _ => Err(PublishingWebSectionParseError),
        }
    }
}
impl std::fmt::Display for PublishingWebSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::PublishingWithGithubPages => f.write_str("publishing-with-github-pages"),
        }
    }
}
#[derive(Debug)]
pub struct PublishingWebSectionParseError;
impl std::fmt::Display for PublishingWebSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of PublishingWebSectionpublishing-with-github-pages",
        )?;
        Ok(())
    }
}
impl std::error::Error for PublishingWebSectionParseError {}
#[component(no_case_check)]
pub fn PublishingWeb(section: PublishingWebSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h2 { id: "publishing-with-github-pages",
            Link {
                to: BookRoute::PublishingWeb {
                    section: PublishingWebSection::PublishingWithGithubPages,
                },
                class: "header",
                "Publishing with Github Pages"
            }
        }
        p { "To build our app and publish it to Github:" }
        ul {
            li { "Make sure GitHub Pages is set up for your repo" }
            li {
                "Build your app with "
                code { "trunk build --release" }
                " (include "
                code { "--public-url <repo-name>" }
                " to update asset prefixes if using a project site)"
            }
            li {
                "Move your generated HTML/CSS/JS/Wasm from "
                code { "dist" }
                " into the folder configured for Github Pages"
            }
            li { "Add and commit with git" }
            li { "Push to GitHub" }
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum CustomRendererIndexSection {
    #[default]
    Empty,
    CustomRenderer,
    TheSpecifics,
    Templates,
    Mutations,
    AnExample,
    EventLoop,
    CustomRawElements,
    NativeCore,
    Realdom,
    Example,
    Layout,
    Conclusion,
}
impl std::str::FromStr for CustomRendererIndexSection {
    type Err = CustomRendererIndexSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "custom-renderer" => Ok(Self::CustomRenderer),
            "the-specifics" => Ok(Self::TheSpecifics),
            "templates" => Ok(Self::Templates),
            "mutations" => Ok(Self::Mutations),
            "an-example" => Ok(Self::AnExample),
            "event-loop" => Ok(Self::EventLoop),
            "custom-raw-elements" => Ok(Self::CustomRawElements),
            "native-core" => Ok(Self::NativeCore),
            "realdom" => Ok(Self::Realdom),
            "example" => Ok(Self::Example),
            "layout" => Ok(Self::Layout),
            "conclusion" => Ok(Self::Conclusion),
            _ => Err(CustomRendererIndexSectionParseError),
        }
    }
}
impl std::fmt::Display for CustomRendererIndexSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::CustomRenderer => f.write_str("custom-renderer"),
            Self::TheSpecifics => f.write_str("the-specifics"),
            Self::Templates => f.write_str("templates"),
            Self::Mutations => f.write_str("mutations"),
            Self::AnExample => f.write_str("an-example"),
            Self::EventLoop => f.write_str("event-loop"),
            Self::CustomRawElements => f.write_str("custom-raw-elements"),
            Self::NativeCore => f.write_str("native-core"),
            Self::Realdom => f.write_str("realdom"),
            Self::Example => f.write_str("example"),
            Self::Layout => f.write_str("layout"),
            Self::Conclusion => f.write_str("conclusion"),
        }
    }
}
#[derive(Debug)]
pub struct CustomRendererIndexSectionParseError;
impl std::fmt::Display for CustomRendererIndexSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of CustomRendererIndexSectioncustom-renderer, the-specifics, templates, mutations, an-example, event-loop, custom-raw-elements, native-core, realdom, example, layout, conclusion",
        )?;
        Ok(())
    }
}
impl std::error::Error for CustomRendererIndexSectionParseError {}
#[component(no_case_check)]
pub fn CustomRendererIndex(section: CustomRendererIndexSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "custom-renderer",
            Link {
                to: BookRoute::CustomRendererIndex {
                    section: CustomRendererIndexSection::CustomRenderer,
                },
                class: "header",
                "Custom Renderer"
            }
        }
        p {
            "Dioxus is an incredibly portable framework for UI development. The lessons, knowledge, hooks, and components you acquire over time can always be used for future projects. However, sometimes those projects cannot leverage a supported renderer or you need to implement your own better renderer."
        }
        p {
            "Great news: the design of the renderer is entirely up to you! We provide suggestions and inspiration with the 1st party renderers, but only really require processing  "
            code { "DomEdits" }
            " and sending  "
            code { "UserEvents" }
            "."
        }
        h2 { id: "the-specifics",
            Link {
                to: BookRoute::CustomRendererIndex {
                    section: CustomRendererIndexSection::TheSpecifics,
                },
                class: "header",
                "The specifics:"
            }
        }
        p { "Implementing the renderer is fairly straightforward. The renderer needs to:" }
        ol {
            li { "Handle the stream of edits generated by updates to the virtual DOM" }
            li { "Register listeners and pass events into the virtual DOM's event system" }
        }
        p {
            "Essentially, your renderer needs to process edits and generate events to update the VirtualDOM. From there, you'll have everything needed to render the VirtualDOM to the screen."
        }
        p {
            "Internally, Dioxus handles the tree relationship, diffing, memory management, and the event system, leaving as little as possible required for renderers to implement themselves."
        }
        p {
            "For reference, check out the "
            Link { to: "https://github.com/DioxusLabs/dioxus/tree/master/packages/interpreter",
                "javascript interpreter"
            }
            " or "
            Link { to: "https://github.com/DioxusLabs/dioxus/tree/master/packages/tui",
                "tui renderer"
            }
            " as a starting point for your custom renderer."
        }
        h2 { id: "templates",
            Link {
                to: BookRoute::CustomRendererIndex {
                    section: CustomRendererIndexSection::Templates,
                },
                class: "header",
                "Templates"
            }
        }
        p {
            "Dioxus is built around the concept of "
            Link { to: "https://docs.rs/dioxus-core/latest/dioxus_core/prelude/struct.Template.html",
                "Templates"
            }
            ". Templates describe a UI tree known at compile time with dynamic parts filled at runtime. This is useful internally to make skip diffing static nodes, but it is also useful for the renderer to reuse parts of the UI tree. This can be useful for things like a list of items. Each item could contain some static parts and some dynamic parts. The renderer can use the template to create a static part of the UI once, clone it for each element in the list, and then fill in the dynamic parts."
        }
        h2 { id: "mutations",
            Link {
                to: BookRoute::CustomRendererIndex {
                    section: CustomRendererIndexSection::Mutations,
                },
                class: "header",
                "Mutations"
            }
        }
        p {
            "The  "
            code { "Mutation" }
            " type is a serialized enum that represents an operation that should be applied to update the UI. The variants roughly follow this set:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Mutation {{\n</span><span style=\"color:#f8f8f2;\">    AppendChildren,\n</span><span style=\"color:#f8f8f2;\">    AssignId,\n</span><span style=\"color:#f8f8f2;\">    CreatePlaceholder,\n</span><span style=\"color:#f8f8f2;\">    CreateTextNode,\n</span><span style=\"color:#f8f8f2;\">    HydrateText,\n</span><span style=\"color:#f8f8f2;\">    LoadTemplate,\n</span><span style=\"color:#f8f8f2;\">    ReplaceWith,\n</span><span style=\"color:#f8f8f2;\">    ReplacePlaceholder,\n</span><span style=\"color:#f8f8f2;\">    InsertAfter,\n</span><span style=\"color:#f8f8f2;\">    InsertBefore,\n</span><span style=\"color:#f8f8f2;\">    SetAttribute,\n</span><span style=\"color:#f8f8f2;\">    SetText,\n</span><span style=\"color:#f8f8f2;\">    NewEventListener,\n</span><span style=\"color:#f8f8f2;\">    RemoveEventListener,\n</span><span style=\"color:#f8f8f2;\">    Remove,\n</span><span style=\"color:#f8f8f2;\">    PushRoot,\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "The Dioxus diffing mechanism operates as a "
            Link { to: "https://en.wikipedia.org/wiki/Stack_machine", "stack machine" }
            " where the \"push_root\" method pushes a new \"real\" DOM node onto the stack and \"append_child\" and \"replace_with\" both remove nodes from the stack."
        }
        h3 { id: "an-example",
            Link {
                to: BookRoute::CustomRendererIndex {
                    section: CustomRendererIndexSection::AnExample,
                },
                class: "header",
                "An Example"
            }
        }
        p {
            "For the sake of understanding, let's consider this example – a very simple UI declaration:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx!( h1 {{</span><span style=\"color:#ffee99;\">&quot;count {{x}}&quot;</span><span style=\"color:#f8f8f2;\">}} )</span></pre>\n" }
        p {
            "To get things started, Dioxus must first navigate to the container of this h1 tag. To \"navigate\" here, the internal diffing algorithm generates the DomEdit  "
            code { "PushRoot" }
            " where the ID of the root is the container."
        }
        p {
            "When the renderer receives this instruction, it pushes the actual Node onto its own stack. The real renderer's stack will look like this:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">instructions: [\n</span><span style=\"color:#f8f8f2;\">    PushRoot(Container)\n</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">stack: [\n</span><span style=\"color:#f8f8f2;\">    ContainerNode,\n</span><span style=\"color:#f8f8f2;\">]</span></pre>\n" }
        p {
            "Next, Dioxus will encounter the h1 node. The diff algorithm decides that this node needs to be created, so Dioxus will generate the DomEdit  "
            code { "CreateElement" }
            ". When the renderer receives this instruction, it will create an unmounted node and push it into its own stack:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">instructions: [\n</span><span style=\"color:#f8f8f2;\">    PushRoot(Container),\n</span><span style=\"color:#f8f8f2;\">    CreateElement(h1),\n</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">stack: [\n</span><span style=\"color:#f8f8f2;\">    ContainerNode,\n</span><span style=\"color:#f8f8f2;\">    h1,\n</span><span style=\"color:#f8f8f2;\">]</span></pre>\n" }
        p {
            "Next, Dioxus sees the text node, and generates the  "
            code { "CreateTextNode" }
            " DomEdit:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">instructions: [\n</span><span style=\"color:#f8f8f2;\">    PushRoot(Container),\n</span><span style=\"color:#f8f8f2;\">    CreateElement(h1),\n</span><span style=\"color:#f8f8f2;\">    CreateTextNode(</span><span style=\"color:#ffee99;\">&quot;hello world&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">stack: [\n</span><span style=\"color:#f8f8f2;\">    ContainerNode,\n</span><span style=\"color:#f8f8f2;\">    h1,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;hello world&quot;\n</span><span style=\"color:#f8f8f2;\">]</span></pre>\n" }
        p {
            "Remember, the text node is not attached to anything (it is unmounted) so Dioxus needs to generate an Edit that connects the text node to the h1 element. It depends on the situation, but in this case, we use  "
            code { "AppendChildren" }
            ". This pops the text node off the stack, leaving the h1 element as the next element in line."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">instructions: [\n</span><span style=\"color:#f8f8f2;\">    PushRoot(Container),\n</span><span style=\"color:#f8f8f2;\">    CreateElement(h1),\n</span><span style=\"color:#f8f8f2;\">    CreateTextNode(</span><span style=\"color:#ffee99;\">&quot;hello world&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    AppendChildren(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">stack: [\n</span><span style=\"color:#f8f8f2;\">    ContainerNode,\n</span><span style=\"color:#f8f8f2;\">    h1\n</span><span style=\"color:#f8f8f2;\">]</span></pre>\n" }
        p {
            "We call  "
            code { "AppendChildren" }
            " again, popping off the h1 node and attaching it to the parent:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">instructions: [\n</span><span style=\"color:#f8f8f2;\">    PushRoot(Container),\n</span><span style=\"color:#f8f8f2;\">    CreateElement(h1),\n</span><span style=\"color:#f8f8f2;\">    CreateTextNode(</span><span style=\"color:#ffee99;\">&quot;hello world&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    AppendChildren(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    AppendChildren(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">stack: [\n</span><span style=\"color:#f8f8f2;\">    ContainerNode,\n</span><span style=\"color:#f8f8f2;\">]</span></pre>\n" }
        p { "Finally, the container is popped since we don't need it anymore." }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">instructions: [\n</span><span style=\"color:#f8f8f2;\">    PushRoot(Container),\n</span><span style=\"color:#f8f8f2;\">    CreateElement(h1),\n</span><span style=\"color:#f8f8f2;\">    CreateTextNode(</span><span style=\"color:#ffee99;\">&quot;hello world&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    AppendChildren(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    AppendChildren(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    PopRoot\n</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">stack: []</span></pre>\n" }
        p { "Over time, our stack looked like this:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[]\n</span><span style=\"color:#f8f8f2;\">[Container]\n</span><span style=\"color:#f8f8f2;\">[Container, h1]\n</span><span style=\"color:#f8f8f2;\">[Container, h1, </span><span style=\"color:#ffee99;\">&quot;hello world&quot;</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">[Container, h1]\n</span><span style=\"color:#f8f8f2;\">[Container]\n</span><span style=\"color:#f8f8f2;\">[]</span></pre>\n" }
        p {
            "Notice how our stack is empty once UI has been mounted. Conveniently, this approach completely separates the Virtual DOM and the Real DOM. Additionally, these edits are serializable, meaning we can even manage UIs across a network connection. This little stack machine and serialized edits make Dioxus independent of platform specifics."
        }
        p {
            "Dioxus is also really fast. Because Dioxus splits the diff and patch phase, it's able to make all the edits to the RealDOM in a very short amount of time (less than a single frame) making rendering very snappy. It also allows Dioxus to cancel large diffing operations if higher priority work comes in while it's diffing."
        }
        p {
            "It's important to note that there "
            em { "is" }
            " one layer of connectedness between Dioxus and the renderer. Dioxus saves and loads elements (the PushRoot edit) with an ID. Inside the VirtualDOM, this is just tracked as a u64."
        }
        p {
            "Whenever a  "
            code { "CreateElement" }
            " edit is generated during diffing, Dioxus increments its node counter and assigns that new element its current NodeCount. The RealDom is responsible for remembering this ID and pushing the correct node when PushRoot(ID) is generated. Dioxus reclaims the IDs of elements when removed. To stay in sync with Dioxus you can use a sparse Vec (Vec"
            " "
            "<"
            " "
            "Option"
            p { class: "inline-html-block", dangerous_inner_html: "<T>" }
            ">) with possibly unoccupied items. You can use the ids as indexes into the Vec for elements, and grow the Vec when an id does not exist."
        }
        p {
            "This little demo serves to show exactly how a Renderer would need to process an edit stream to build UIs. A set of serialized DomEditss for various demos is available for you to test your custom renderer against."
        }
        h2 { id: "event-loop",
            Link {
                to: BookRoute::CustomRendererIndex {
                    section: CustomRendererIndexSection::EventLoop,
                },
                class: "header",
                "Event loop"
            }
        }
        p {
            "Like most GUIs, Dioxus relies on an event loop to progress the VirtualDOM. The VirtualDOM itself can produce events as well, so it's important that your custom renderer can handle those too."
        }
        p {
            "The code for the WebSys implementation is straightforward, so we'll add it here to demonstrate how simple an event loop is:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">run</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;mut </span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) -&gt; dioxus_core::error::Result&lt;()&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Push the body element onto the WebsysDom&#39;s stack machine\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> websys_dom </span><span style=\"color:#f92672;\">= crate</span><span style=\"color:#f8f8f2;\">::new::WebsysDom::new(</span><span style=\"color:#66d9ef;\">prepare_websys_dom</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">    websys_dom.stack.</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(root_node);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Rebuild or hydrate the virtualdom\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> mutations </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">self.internal_dom.</span><span style=\"color:#66d9ef;\">rebuild</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    websys_dom.</span><span style=\"color:#66d9ef;\">apply_mutations</span><span style=\"color:#f8f8f2;\">(mutations);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Wait for updates from the real dom and progress the virtual dom\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">loop </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> user_input_future </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> websys_dom.</span><span style=\"color:#66d9ef;\">wait_for_event</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> internal_event_future </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">self.internal_dom.</span><span style=\"color:#66d9ef;\">wait_for_work</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">match </span><span style=\"color:#66d9ef;\">select</span><span style=\"color:#f8f8f2;\">(user_input_future, internal_event_future).await {{\n</span><span style=\"color:#f8f8f2;\">            Either::Left((</span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">)) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> mutations </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">self.internal_dom.</span><span style=\"color:#66d9ef;\">work_with_deadline</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">false</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                websys_dom.</span><span style=\"color:#66d9ef;\">apply_mutations</span><span style=\"color:#f8f8f2;\">(mutations);\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            Either::Right((event, </span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">)) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> websys_dom.</span><span style=\"color:#66d9ef;\">handle_event</span><span style=\"color:#f8f8f2;\">(event),\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// render\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "It's important that you decode the real events from your event system into Dioxus' synthetic event system (synthetic meaning abstracted). This simply means matching your event type and creating a Dioxus  "
            code { "UserEvent" }
            " type. Right now, the VirtualEvent system is modeled almost entirely around the HTML spec, but we are interested in slimming it down."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">virtual_event_from_websys_event</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">event</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">web_sys::Event) -&gt; VirtualEvent {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> event.</span><span style=\"color:#66d9ef;\">type_</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">as_str</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;keydown&quot; </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> event: web_sys::KeyboardEvent </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> event.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">dyn_into</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">            UserEvent::KeyboardEvent(UserEvent {{\n</span><span style=\"color:#f8f8f2;\">                scope_id: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                priority: EventPriority::Medium,\n</span><span style=\"color:#f8f8f2;\">                name: </span><span style=\"color:#ffee99;\">&quot;keydown&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// This should be whatever element is focused\n</span><span style=\"color:#f8f8f2;\">                element: </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(ElementId(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">)),\n</span><span style=\"color:#f8f8f2;\">                data: Arc::new(KeyboardData{{\n</span><span style=\"color:#f8f8f2;\">                    char_code: event.</span><span style=\"color:#66d9ef;\">char_code</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    key: event.</span><span style=\"color:#66d9ef;\">key</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    key_code: event.</span><span style=\"color:#66d9ef;\">key_code</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    alt_key: event.</span><span style=\"color:#66d9ef;\">alt_key</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    ctrl_key: event.</span><span style=\"color:#66d9ef;\">ctrl_key</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    meta_key: event.</span><span style=\"color:#66d9ef;\">meta_key</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    shift_key: event.</span><span style=\"color:#66d9ef;\">shift_key</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    location: event.</span><span style=\"color:#66d9ef;\">location</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    repeat: event.</span><span style=\"color:#66d9ef;\">repeat</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    which: event.</span><span style=\"color:#66d9ef;\">which</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                }})\n</span><span style=\"color:#f8f8f2;\">            }})\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">_ =&gt; </span><span style=\"color:#f8f8f2;\">todo!()\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "custom-raw-elements",
            Link {
                to: BookRoute::CustomRendererIndex {
                    section: CustomRendererIndexSection::CustomRawElements,
                },
                class: "header",
                "Custom raw elements"
            }
        }
        p {
            "If you need to go as far as relying on custom elements for your renderer – you totally can. This still enables you to use Dioxus' reactive nature, component system, shared state, and other features, but will ultimately generate different nodes. All attributes and listeners for the HTML and SVG namespace are shuttled through helper structs that essentially compile away (pose no runtime overhead). You can drop in your own elements any time you want, with little hassle. However, you must be absolutely sure your renderer can handle the new type, or it will crash and burn."
        }
        p { "These custom elements are defined as unit structs with trait implementations." }
        p {
            "For example, the  "
            code { "div" }
            " element is (approximately!) defined as such:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">div;\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">div {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">/// Some glorious documentation about the class property.\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">TAG_NAME</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;&#39;static </span><span style=\"font-style:italic;color:#66d9ef;\">str </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;div&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">NAME_SPACE</span><span style=\"color:#f8f8f2;\">: Option&lt;</span><span style=\"color:#f92672;\">&amp;&#39;static </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// define the class attribute\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">class</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: NodeFactory&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;, </span><span style=\"font-style:italic;color:#fd971f;\">val</span><span style=\"color:#f8f8f2;\">: Arguments) -&gt; Attribute&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">        cx.</span><span style=\"color:#66d9ef;\">attr</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;class&quot;</span><span style=\"color:#f8f8f2;\">, val, </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">false</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// more attributes\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "You've probably noticed that many elements in the  "
            code { "rsx!" }
            " macros support on-hover documentation. The approach we take to custom elements means that the unit struct is created immediately where the element is used in the macro. When the macro is expanded, the doc comments still apply to the unit struct, giving tons of in-editor feedback, even inside a proc macro."
        }
        h1 { id: "native-core",
            Link {
                to: BookRoute::CustomRendererIndex {
                    section: CustomRendererIndexSection::NativeCore,
                },
                class: "header",
                "Native Core"
            }
        }
        p {
            "If you are creating a renderer in rust, native-core provides some utilities to implement a renderer. It provides an abstraction over DomEdits and handles the layout for you."
        }
        h2 { id: "realdom",
            Link {
                to: BookRoute::CustomRendererIndex {
                    section: CustomRendererIndexSection::Realdom,
                },
                class: "header",
                "RealDom"
            }
        }
        p {
            "The  "
            code { "RealDom" }
            " is a higher-level abstraction over updating the Dom. It updates with  "
            code { "DomEdits" }
            " and provides a way to incrementally update the state of nodes based on what attributes change."
        }
        h3 { id: "example",
            Link {
                to: BookRoute::CustomRendererIndex {
                    section: CustomRendererIndexSection::Example,
                },
                class: "header",
                "Example"
            }
        }
        p {
            "Let's build a toy renderer with borders, size, and text color."
            " "
            "Before we start let's take a look at an example element we can render:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">    div{{\n</span><span style=\"color:#f8f8f2;\">        color: </span><span style=\"color:#ffee99;\">&quot;red&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        p{{\n</span><span style=\"color:#f8f8f2;\">            border: </span><span style=\"color:#ffee99;\">&quot;1px solid black&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;hello world&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n" }
        p {
            "In this tree, the color depends on the parent's color. The size depends on the children's size, the current text, and the text size. The border depends on only the current node."
        }
        p { "In the following diagram arrows represent dataflow:" }
        p {
            Link { to: "https://mermaid-js.github.io/mermaid-live-editor/edit#pako:eNqdVNFqgzAU_RXJXizUUZPJmIM-jO0LukdhpCbO0JhIGteW0n9fNK1Oa0brfUnu9VxyzzkXjyCVhIIYZFzu0hwr7X2-JcIzsa3W3wqXuZdKoele22oddfa1Y0Tnfn31muvMfqeCDNq3GmvaNROmaKqZFO1DPTRhP8MOd1fTWYNDvzlmQbBMJZcq9JtjNgY1mLVUhBqQPQeojl3wGCw5PsjqnIe-zXqEL8GZ2Kz0gVMPmoeU3ND4IcuiaLGY2zRouuKncv_qGKv3VodpJe0JVU6QCQ5kgqMyWQVr8hbk4hm1PBcmsuwmnrCVH94rP7xN_ucp8sOB_EPSfz9drYVrkpc_AmH8_yTjJueUc-ntpOJkgt2os9tKjcYlt-DLUiD3UsB2KZCLcwjv3Aq33-g2v0M0xXA0MBy5DUdXi-gcJZriuLmAOSioKjAj5ld8rMsJ0DktaAJicyVYbRKQiJPBVSUx438QpqUCcYb5ls4BrrRcHUTaFizqnWGzR8W5evoFI-bJdw",
                img {
                    src: "https://mermaid.ink/img/pako:eNqdVNFqgzAU_RXJXizUUZPJmIM-jO0LukdhpCbO0JhIGteW0n9fNK1Oa0brfUnu9VxyzzkXjyCVhIIYZFzu0hwr7X2-JcIzsa3W3wqXuZdKoele22oddfa1Y0Tnfn31muvMfqeCDNq3GmvaNROmaKqZFO1DPTRhP8MOd1fTWYNDvzlmQbBMJZcq9JtjNgY1mLVUhBqQPQeojl3wGCw5PsjqnIe-zXqEL8GZ2Kz0gVMPmoeU3ND4IcuiaLGY2zRouuKncv_qGKv3VodpJe0JVU6QCQ5kgqMyWQVr8hbk4hm1PBcmsuwmnrCVH94rP7xN_ucp8sOB_EPSfz9drYVrkpc_AmH8_yTjJueUc-ntpOJkgt2os9tKjcYlt-DLUiD3UsB2KZCLcwjv3Aq33-g2v0M0xXA0MBy5DUdXi-gcJZriuLmAOSioKjAj5ld8rMsJ0DktaAJicyVYbRKQiJPBVSUx438QpqUCcYb5ls4BrrRcHUTaFizqnWGzR8W5evoFI-bJdw",
                    alt: "",
                    title: "",
                }
            }
        }
        p {
            "To help in building a Dom, native-core provides four traits: State, ChildDepState, ParentDepState, NodeDepState, and a RealDom struct. The ChildDepState, ParentDepState, and NodeDepState provide a way to describe how some information in a node relates to that of its relatives. By providing how to build a single node from its relations, native-core will derive a way to update the state of all nodes for you with  "
            code { "#[derive(State)]" }
            ". Once you have a state you can provide it as a generic to RealDom. RealDom provides all of the methods to interact and update your new dom."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_native_core::node_ref::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_native_core::state::{{ChildDepState, NodeDepState, ParentDepState, State}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_native_core_macro::{{sorted_str_slice, State}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Default, Copy, Clone)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">Size(</span><span style=\"font-style:italic;color:#66d9ef;\">f32</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#66d9ef;\">f32</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#8c8c8c;\">// Size only depends on the current node and its children, so it implements ChildDepState\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">ChildDepState </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">Size {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Size accepts a font size context\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">type </span><span style=\"color:#f8f8f2;\">Ctx </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">f32</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Size depends on the Size part of each child\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">type </span><span style=\"color:#f8f8f2;\">DepState </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">Self</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Size only cares about the width, height, and text parts of the current node\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">NODE_MASK</span><span style=\"color:#f8f8f2;\">: NodeMask </span><span style=\"color:#f92672;\">=\n</span><span style=\"color:#f8f8f2;\">        NodeMask::new_with_attrs(AttributeMask::Static(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">sorted_str_slice!([</span><span style=\"color:#ffee99;\">&quot;width&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;height&quot;</span><span style=\"color:#f8f8f2;\">]))).</span><span style=\"color:#66d9ef;\">with_text</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">reduce</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">&amp;mut </span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#fd971f;\">node</span><span style=\"color:#f8f8f2;\">: NodeView,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#fd971f;\">children</span><span style=\"color:#f8f8f2;\">: impl Iterator&lt;Item = </span><span style=\"color:#f92672;\">&amp;&#39;a </span><span style=\"font-style:italic;color:#66d9ef;\">Self::</span><span style=\"color:#f8f8f2;\">DepState&gt;,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#fd971f;\">ctx</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">Self::</span><span style=\"color:#f8f8f2;\">Ctx,\n</span><span style=\"color:#f8f8f2;\">    ) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">bool\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">where\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Self::</span><span style=\"color:#f8f8f2;\">DepState: </span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> width;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> height;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(text) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> node.</span><span style=\"color:#66d9ef;\">text</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// if the node has text, use the text to size our object\n</span><span style=\"color:#f8f8f2;\">            width </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> text.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">as </span><span style=\"font-style:italic;color:#66d9ef;\">f32 </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\"> ctx;\n</span><span style=\"color:#f8f8f2;\">            height </span><span style=\"color:#f92672;\">= *</span><span style=\"color:#f8f8f2;\">ctx;\n</span><span style=\"color:#f8f8f2;\">        }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// otherwise, the size is the maximum size of the children\n</span><span style=\"color:#f8f8f2;\">            width </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> children\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">by_ref</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">item</span><span style=\"color:#f8f8f2;\">| item.</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">reduce</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">accum</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">item</span><span style=\"color:#f8f8f2;\">| </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> accum </span><span style=\"color:#f92672;\">&gt;=</span><span style=\"color:#f8f8f2;\"> item {{ accum }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ item }})\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">unwrap_or</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">0.0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">            height </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> children\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">item</span><span style=\"color:#f8f8f2;\">| item.</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">reduce</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">accum</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">item</span><span style=\"color:#f8f8f2;\">| </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> accum </span><span style=\"color:#f92672;\">&gt;=</span><span style=\"color:#f8f8f2;\"> item {{ accum }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ item }})\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">unwrap_or</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">0.0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// if the node contains a width or height attribute it overrides the other size\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> a </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> node.</span><span style=\"color:#66d9ef;\">attributes</span><span style=\"color:#f8f8f2;\">(){{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> a.name{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;width&quot; </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> width </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> a.value.</span><span style=\"color:#66d9ef;\">as_float32</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;height&quot; </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> height </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> a.value.</span><span style=\"color:#66d9ef;\">as_float32</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// because Size only depends on the width and height, no other attributes will be passed to the member\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">_ =&gt; </span><span style=\"color:#f8f8f2;\">panic!()\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// to determine what other parts of the dom need to be updated we return a boolean that marks if this member changed\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> changed </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">(width </span><span style=\"color:#f92672;\">!= </span><span style=\"color:#f8f8f2;\">self.</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">) </span><span style=\"color:#f92672;\">|| </span><span style=\"color:#f8f8f2;\">(height </span><span style=\"color:#f92672;\">!= </span><span style=\"color:#f8f8f2;\">self.</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">self </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">Self</span><span style=\"color:#f8f8f2;\">(width, height);\n</span><span style=\"color:#f8f8f2;\">        changed\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Debug, Clone, Copy, PartialEq, Default)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">TextColor {{\n</span><span style=\"color:#f8f8f2;\">    r: </span><span style=\"font-style:italic;color:#66d9ef;\">u8</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    g: </span><span style=\"font-style:italic;color:#66d9ef;\">u8</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    b: </span><span style=\"font-style:italic;color:#66d9ef;\">u8</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#8c8c8c;\">// TextColor only depends on the current node and its parent, so it implements ParentDepState\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">ParentDepState </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">TextColor {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">type </span><span style=\"color:#f8f8f2;\">Ctx </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// TextColor depends on the TextColor part of the parent\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">type </span><span style=\"color:#f8f8f2;\">DepState </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">Self</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// TextColor only cares about the color attribute of the current node\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">NODE_MASK</span><span style=\"color:#f8f8f2;\">: NodeMask </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">NodeMask::new_with_attrs(AttributeMask::Static(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;color&quot;</span><span style=\"color:#f8f8f2;\">]));\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">reduce</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">&amp;mut </span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#fd971f;\">node</span><span style=\"color:#f8f8f2;\">: NodeView,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#fd971f;\">parent</span><span style=\"color:#f8f8f2;\">: Option&lt;</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">Self::</span><span style=\"color:#f8f8f2;\">DepState&gt;,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#fd971f;\">_ctx</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">Self::</span><span style=\"color:#f8f8f2;\">Ctx,\n</span><span style=\"color:#f8f8f2;\">    ) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">bool </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// TextColor only depends on the color tag, so getting the first tag is equivilent to looking through all tags\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> new </span><span style=\"color:#f92672;\">= match</span><span style=\"color:#f8f8f2;\"> node.</span><span style=\"color:#66d9ef;\">attributes</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">next</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">attr</span><span style=\"color:#f8f8f2;\">| attr.name) {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// if there is a color tag, translate it\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;red&quot;</span><span style=\"color:#f8f8f2;\">) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> TextColor {{ r: </span><span style=\"color:#ff80f4;\">255</span><span style=\"color:#f8f8f2;\">, g: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">, b: </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;green&quot;</span><span style=\"color:#f8f8f2;\">) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> TextColor {{ r: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">, g: </span><span style=\"color:#ff80f4;\">255</span><span style=\"color:#f8f8f2;\">, b: </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;blue&quot;</span><span style=\"color:#f8f8f2;\">) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> TextColor {{ r: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">, g: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">, b: </span><span style=\"color:#ff80f4;\">255 </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">panic!(</span><span style=\"color:#ffee99;\">&quot;unknown color&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// otherwise check if the node has a parent and inherit that color\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f92672;\">=&gt; match</span><span style=\"color:#f8f8f2;\"> parent {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(parent) </span><span style=\"color:#f92672;\">=&gt; *</span><span style=\"color:#f8f8f2;\">parent,\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">Self</span><span style=\"color:#f8f8f2;\">::default(),\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">        }};\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// check if the member has changed\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> changed </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> new </span><span style=\"color:#f92672;\">!= *</span><span style=\"color:#f8f8f2;\">self;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">self </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> new;\n</span><span style=\"color:#f8f8f2;\">        changed\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Debug, Clone, PartialEq, Default)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">Border(</span><span style=\"font-style:italic;color:#66d9ef;\">bool</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#8c8c8c;\">// TextColor only depends on the current node, so it implements NodeDepState\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">NodeDepState&lt;()&gt; </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">Border {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">type </span><span style=\"color:#f8f8f2;\">Ctx </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">   \n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Border does not depended on any other member in the current node\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">NODE_MASK</span><span style=\"color:#f8f8f2;\">: NodeMask </span><span style=\"color:#f92672;\">=\n</span><span style=\"color:#f8f8f2;\">        NodeMask::new_with_attrs(AttributeMask::Static(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;border&quot;</span><span style=\"color:#f8f8f2;\">]));\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">reduce</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;mut </span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">node</span><span style=\"color:#f8f8f2;\">: NodeView, </span><span style=\"font-style:italic;color:#fd971f;\">_sibling</span><span style=\"color:#f8f8f2;\">: (), </span><span style=\"font-style:italic;color:#fd971f;\">_ctx</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">Self::</span><span style=\"color:#f8f8f2;\">Ctx) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">bool </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// check if the node contians a border attribute\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> new </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">Self</span><span style=\"color:#f8f8f2;\">(node.</span><span style=\"color:#66d9ef;\">attributes</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">next</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">a</span><span style=\"color:#f8f8f2;\">| a.name </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ffee99;\">&quot;border&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">is_some</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// check if the member has changed\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> changed </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> new </span><span style=\"color:#f92672;\">!= *</span><span style=\"color:#f8f8f2;\">self;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">self </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> new;\n</span><span style=\"color:#f8f8f2;\">        changed\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// State provides a derive macro, but anotations on the members are needed in the form #[dep_type(dep_member, CtxType)]\n</span><span style=\"color:#f8f8f2;\">#[derive(State, Default, Clone)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">ToyState {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// the color member of it&#39;s parent and no context\n</span><span style=\"color:#f8f8f2;\">    #[parent_dep_state(color)]\n</span><span style=\"color:#f8f8f2;\">    color: TextColor,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// depends on the node, and no context\n</span><span style=\"color:#f8f8f2;\">    #[node_dep_state()]\n</span><span style=\"color:#f8f8f2;\">    border: Border,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// depends on the layout_width member of children and f32 context (for text size)\n</span><span style=\"color:#f8f8f2;\">    #[child_dep_state(size, f32)]\n</span><span style=\"color:#f8f8f2;\">    size: Size,\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Now that we have our state, we can put it to use in our dom. Re can update the dom with update_state to update the structure of the dom (adding, removing, and changing properties of nodes) and then apply_mutations to update the ToyState for each of the nodes that changed."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">(){{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">        cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">            div{{\n</span><span style=\"color:#f8f8f2;\">                color: </span><span style=\"color:#ffee99;\">&quot;red&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;hello world&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }})\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> vdom </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">VirtualDom::new(app);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> rdom: RealDom&lt;ToyState&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">RealDom::new();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> mutations </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> dom.</span><span style=\"color:#66d9ef;\">rebuild</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// update the structure of the real_dom tree\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> to_update </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> rdom.</span><span style=\"color:#66d9ef;\">apply_mutations</span><span style=\"color:#f8f8f2;\">(vec![mutations]);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> ctx </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">AnyMap::new();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// set the font size to 3.3\n</span><span style=\"color:#f8f8f2;\">    ctx.</span><span style=\"color:#66d9ef;\">insert</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">3.3</span><span style=\"font-style:italic;color:#66d9ef;\">f32</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// update the ToyState for nodes in the real_dom tree\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> _to_rerender </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> rdom.</span><span style=\"color:#66d9ef;\">update_state</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">dom, to_update, ctx).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// we need to run the vdom in a async runtime\n</span><span style=\"color:#f8f8f2;\">    tokio::runtime::Builder::new_current_thread()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">enable_all</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">build</span><span style=\"color:#f8f8f2;\">()</span><span style=\"color:#f92672;\">?\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">block_on</span><span style=\"color:#f8f8f2;\">(async {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">loop</span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> wait </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> vdom.</span><span style=\"color:#66d9ef;\">wait_for_work</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> mutations </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> vdom.</span><span style=\"color:#66d9ef;\">work_with_deadline</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">false</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> to_update </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> rdom.</span><span style=\"color:#66d9ef;\">apply_mutations</span><span style=\"color:#f8f8f2;\">(mutations);\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> ctx </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">AnyMap::new();\n</span><span style=\"color:#f8f8f2;\">                ctx.</span><span style=\"color:#66d9ef;\">insert</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">3.3</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> _to_rerender </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> rdom.</span><span style=\"color:#66d9ef;\">update_state</span><span style=\"color:#f8f8f2;\">(vdom, to_update, ctx).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// render...\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "layout",
            Link {
                to: BookRoute::CustomRendererIndex {
                    section: CustomRendererIndexSection::Layout,
                },
                class: "header",
                "Layout"
            }
        }
        p {
            "For most platforms, the layout of the Elements will stay the same. The layout_attributes module provides a way to apply HTML attributes to a stretch layout style."
        }
        h2 { id: "conclusion",
            Link {
                to: BookRoute::CustomRendererIndex {
                    section: CustomRendererIndexSection::Conclusion,
                },
                class: "header",
                "Conclusion"
            }
        }
        p {
            "That should be it! You should have nearly all the knowledge required on how to implement your own renderer. We're super interested in seeing Dioxus apps brought to custom desktop renderers, mobile renderers, video game UI, and even augmented reality! If you're interested in contributing to any of these projects, don't be afraid to reach out or join the "
            Link { to: "https://discord.gg/XgGxMSkvUM", "community" }
            "."
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum RoadmapSection {
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
}
impl std::str::FromStr for RoadmapSection {
    type Err = RoadmapSectionParseError;
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
            _ => Err(RoadmapSectionParseError),
        }
    }
}
impl std::fmt::Display for RoadmapSection {
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
        }
    }
}
#[derive(Debug)]
pub struct RoadmapSectionParseError;
impl std::fmt::Display for RoadmapSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of RoadmapSectionroadmap--feature-set, features, roadmap, core, ssr, desktop, mobile, bundling-cli, essential-hooks, work-in-progress, build-tool, server-component-support, native-rendering",
        )?;
        Ok(())
    }
}
impl std::error::Error for RoadmapSectionParseError {}
#[component(no_case_check)]
pub fn Roadmap(section: RoadmapSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "roadmap--feature-set",
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::RoadmapFeatureSet,
                },
                class: "header",
                "Roadmap & Feature-set"
            }
        }
        p {
            "This feature set and roadmap can help you decide if what Dioxus can do today works for you."
        }
        p {
            "If a feature that you need doesn't exist or you want to contribute to projects on the roadmap, feel free to get involved by "
            Link { to: "https://discord.gg/XgGxMSkvUM", "joining the discord" }
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
        h2 { id: "features",
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::Features,
                },
                class: "header",
                "Features"
            }
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
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::Roadmap,
                },
                class: "header",
                "Roadmap"
            }
        }
        p { "These Features are planned for the future of Dioxus:" }
        h3 { id: "core",
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::Core,
                },
                class: "header",
                "Core"
            }
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
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::Ssr,
                },
                class: "header",
                "SSR"
            }
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
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::Desktop,
                },
                class: "header",
                "Desktop"
            }
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
                "Fully native renderer"
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
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::Mobile,
                },
                class: "header",
                "Mobile"
            }
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
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "false",
                }
                "Native Renderer"
            }
        }
        h3 { id: "bundling-cli",
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::BundlingCli,
                },
                class: "header",
                "Bundling (CLI)"
            }
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
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::EssentialHooks,
                },
                class: "header",
                "Essential hooks"
            }
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
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::WorkInProgress,
                },
                class: "header",
                "Work in Progress"
            }
        }
        h3 { id: "build-tool",
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::BuildTool,
                },
                class: "header",
                "Build Tool"
            }
        }
        p {
            "We are currently working on our own build tool called "
            Link { to: "https://github.com/DioxusLabs/cli", "Dioxus CLI" }
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
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::ServerComponentSupport,
                },
                class: "header",
                "Server Component Support"
            }
        }
        p {
            "While not currently fully implemented, the expectation is that LiveView apps can be a hybrid between Wasm and server-rendered where only portions of a page are \"live\" and the rest of the page is either server-rendered, statically generated, or handled by the host SPA."
        }
        h3 { id: "native-rendering",
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::NativeRendering,
                },
                class: "header",
                "Native rendering"
            }
        }
        p {
            "We are currently working on a native renderer for Dioxus using WGPU called "
            Link { to: "https://github.com/DioxusLabs/blitz/", "Blitz" }
            ". This will allow you to build apps that are rendered natively for iOS, Android, and Desktop."
        }
    }
}
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub enum ContributingSection {
    #[default]
    Empty,
    Contributing,
    ImprovingDocs,
    WorkingOnTheEcosystem,
    BugsFeatures,
}
impl std::str::FromStr for ContributingSection {
    type Err = ContributingSectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Empty),
            "contributing" => Ok(Self::Contributing),
            "improving-docs" => Ok(Self::ImprovingDocs),
            "working-on-the-ecosystem" => Ok(Self::WorkingOnTheEcosystem),
            "bugs--features" => Ok(Self::BugsFeatures),
            _ => Err(ContributingSectionParseError),
        }
    }
}
impl std::fmt::Display for ContributingSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Contributing => f.write_str("contributing"),
            Self::ImprovingDocs => f.write_str("improving-docs"),
            Self::WorkingOnTheEcosystem => f.write_str("working-on-the-ecosystem"),
            Self::BugsFeatures => f.write_str("bugs--features"),
        }
    }
}
#[derive(Debug)]
pub struct ContributingSectionParseError;
impl std::fmt::Display for ContributingSectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Invalid section name. Expected one of ContributingSectioncontributing, improving-docs, working-on-the-ecosystem, bugs--features",
        )?;
        Ok(())
    }
}
impl std::error::Error for ContributingSectionParseError {}
#[component(no_case_check)]
pub fn Contributing(section: ContributingSection) -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "contributing",
            Link {
                to: BookRoute::Contributing {
                    section: ContributingSection::Contributing,
                },
                class: "header",
                "Contributing"
            }
        }
        p {
            "Development happens in the "
            Link { to: "https://github.com/DioxusLabs/dioxus", "Dioxus GitHub repository" }
            ". If you've found a bug or have an idea for a feature, please submit an issue (but first check if someone hasn't "
            Link { to: "https://github.com/DioxusLabs/dioxus/issues", "done it already" }
            ")."
        }
        p {
            Link { to: "https://github.com/DioxusLabs/dioxus/discussions", "GitHub discussions" }
            " can be used as a place to ask for help or talk about features. You can also join "
            Link { to: "https://discord.gg/XgGxMSkvUM", "our Discord channel" }
            " where some development discussion happens."
        }
        h2 { id: "improving-docs",
            Link {
                to: BookRoute::Contributing {
                    section: ContributingSection::ImprovingDocs,
                },
                class: "header",
                "Improving Docs"
            }
        }
        p {
            "If you'd like to improve the docs, PRs are welcome! Both Rust docs ("
            Link { to: "https://github.com/DioxusLabs/dioxus/tree/master/packages", "source" }
            ") and this guide ("
            Link { to: "https://github.com/DioxusLabs/dioxus/tree/master/docs/guide",
                "source"
            }
            ") can be found in the GitHub repo."
        }
        h2 { id: "working-on-the-ecosystem",
            Link {
                to: BookRoute::Contributing {
                    section: ContributingSection::WorkingOnTheEcosystem,
                },
                class: "header",
                "Working on the Ecosystem"
            }
        }
        p {
            "Part of what makes React great is the rich ecosystem. We'd like the same for Dioxus! So if you have a library in mind that you'd like to write and many people would benefit from, it will be appreciated. You can "
            Link { to: "https://www.npmjs.com/search?q=keywords:react-component", "browse npm.js" }
            " for inspiration."
        }
        h2 { id: "bugs--features",
            Link {
                to: BookRoute::Contributing {
                    section: ContributingSection::BugsFeatures,
                },
                class: "header",
                "Bugs & Features"
            }
        }
        p {
            "If you've fixed "
            Link { to: "https://github.com/DioxusLabs/dioxus/issues", "an open issue" }
            ", feel free to submit a PR! You can also take a look at "
            Link {
                to: BookRoute::Roadmap {
                    section: RoadmapSection::Empty,
                },
                "the roadmap"
            }
            " and work on something in there. Consider "
            Link { to: "https://discord.gg/XgGxMSkvUM", "reaching out" }
            " to the team first to make sure everyone's on the same page, and you don't do useless work!"
        }
        p {
            "All pull requests (including those made by a team member) must be approved by at least one other team member."
            " "
            "Larger, more nuanced decisions about design, architecture, breaking changes, trade-offs, etc. are made by team consensus."
        }
    }
}

use super::*;
