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
    #[route("/")]
    Index {},
    #[route("/introduction/roadmap")]
    IntroductionRoadmap {},
    #[route("/guide/")]
    GuideIndex {},
    #[route("/guide/your_first_component")]
    GuideYourFirstComponent {},
    #[route("/guide/state")]
    GuideState {},
    #[route("/guide/data_fetching")]
    GuideDataFetching {},
    #[route("/guide/full_code")]
    GuideFullCode {},
    #[route("/essentials/")]
    EssentialsIndex {},
    #[route("/essentials/rsx/")]
    EssentialsRsxIndex {},
    #[route("/essentials/lifecycle/")]
    EssentialsLifecycleIndex {},
    #[route("/essentials/state/")]
    EssentialsStateIndex {},
    #[route("/essentials/breaking/")]
    EssentialsBreakingIndex {},
    #[route("/reference/")]
    ReferenceIndex {},
    #[route("/router/")]
    RouterIndex {},
    #[route("/router/example/")]
    RouterExampleIndex {},
    #[route("/router/example/first-route")]
    RouterExampleFirstRoute {},
    #[route("/router/example/building-a-nest")]
    RouterExampleBuildingANest {},
    #[route("/router/example/navigation-targets")]
    RouterExampleNavigationTargets {},
    #[route("/router/example/redirection-perfection")]
    RouterExampleRedirectionPerfection {},
    #[route("/router/example/full-code")]
    RouterExampleFullCode {},
    #[route("/router/reference/")]
    RouterReferenceIndex {},
    #[route("/router/reference/routes/")]
    RouterReferenceRoutesIndex {},
    #[route("/router/reference/routes/nested")]
    RouterReferenceRoutesNested {},
    #[route("/router/reference/layouts")]
    RouterReferenceLayouts {},
    #[route("/router/reference/navigation/")]
    RouterReferenceNavigationIndex {},
    #[route("/router/reference/navigation/programmatic")]
    RouterReferenceNavigationProgrammatic {},
    #[route("/router/reference/history-providers")]
    RouterReferenceHistoryProviders {},
    #[route("/router/reference/history-buttons")]
    RouterReferenceHistoryButtons {},
    #[route("/router/reference/routing-update-callback")]
    RouterReferenceRoutingUpdateCallback {},
    #[route("/reference/assets")]
    ReferenceAssets {},
    #[route("/reference/web/")]
    ReferenceWebIndex {},
    #[route("/reference/desktop/")]
    ReferenceDesktopIndex {},
    #[route("/reference/mobile/")]
    ReferenceMobileIndex {},
    #[route("/reference/mobile/apis")]
    ReferenceMobileApis {},
    #[route("/reference/ssr")]
    ReferenceSsr {},
    #[route("/reference/fullstack/")]
    ReferenceFullstackIndex {},
    #[route("/reference/fullstack/server_functions")]
    ReferenceFullstackServerFunctions {},
    #[route("/reference/fullstack/extractors")]
    ReferenceFullstackExtractors {},
    #[route("/reference/fullstack/middleware")]
    ReferenceFullstackMiddleware {},
    #[route("/reference/fullstack/authentication")]
    ReferenceFullstackAuthentication {},
    #[route("/reference/fullstack/routing")]
    ReferenceFullstackRouting {},
    #[route("/cookbook/publishing")]
    CookbookPublishing {},
    #[route("/cookbook/antipatterns")]
    CookbookAntipatterns {},
    #[route("/cookbook/error_handling")]
    CookbookErrorHandling {},
    #[route("/cookbook/integrations/")]
    CookbookIntegrationsIndex {},
    #[route("/cookbook/integrations/logging")]
    CookbookIntegrationsLogging {},
    #[route("/cookbook/integrations/internationalization")]
    CookbookIntegrationsInternationalization {},
    #[route("/cookbook/state/")]
    CookbookStateIndex {},
    #[route("/cookbook/state/external/")]
    CookbookStateExternalIndex {},
    #[route("/cookbook/state/custom_hooks/")]
    CookbookStateCustomHooksIndex {},
    #[route("/cookbook/testing")]
    CookbookTesting {},
    #[route("/cookbook/tailwind")]
    CookbookTailwind {},
    #[route("/cookbook/optimizing")]
    CookbookOptimizing {},
    #[route("/migration/")]
    MigrationIndex {},
    #[route("/contributing/")]
    ContributingIndex {},
    #[route("/contributing/project_structure")]
    ContributingProjectStructure {},
    #[route("/contributing/guiding_principles")]
    ContributingGuidingPrinciples {},
    #[route("/contributing/roadmap")]
    ContributingRoadmap {},
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
            BookRoute::Index {} => use_mdbook::mdbook_shared::PageId(0usize),
            BookRoute::IntroductionRoadmap {} => use_mdbook::mdbook_shared::PageId(1usize),
            BookRoute::GuideIndex {} => use_mdbook::mdbook_shared::PageId(2usize),
            BookRoute::GuideYourFirstComponent {} => use_mdbook::mdbook_shared::PageId(3usize),
            BookRoute::GuideState {} => use_mdbook::mdbook_shared::PageId(4usize),
            BookRoute::GuideDataFetching {} => use_mdbook::mdbook_shared::PageId(5usize),
            BookRoute::GuideFullCode {} => use_mdbook::mdbook_shared::PageId(6usize),
            BookRoute::EssentialsIndex {} => use_mdbook::mdbook_shared::PageId(7usize),
            BookRoute::EssentialsRsxIndex {} => use_mdbook::mdbook_shared::PageId(8usize),
            BookRoute::EssentialsLifecycleIndex {} => use_mdbook::mdbook_shared::PageId(9usize),
            BookRoute::EssentialsStateIndex {} => use_mdbook::mdbook_shared::PageId(10usize),
            BookRoute::EssentialsBreakingIndex {} => use_mdbook::mdbook_shared::PageId(11usize),
            BookRoute::ReferenceIndex {} => use_mdbook::mdbook_shared::PageId(12usize),
            BookRoute::RouterIndex {} => use_mdbook::mdbook_shared::PageId(13usize),
            BookRoute::RouterExampleIndex {} => use_mdbook::mdbook_shared::PageId(14usize),
            BookRoute::RouterExampleFirstRoute {} => use_mdbook::mdbook_shared::PageId(15usize),
            BookRoute::RouterExampleBuildingANest {} => use_mdbook::mdbook_shared::PageId(16usize),
            BookRoute::RouterExampleNavigationTargets {} => {
                use_mdbook::mdbook_shared::PageId(17usize)
            }
            BookRoute::RouterExampleRedirectionPerfection {} => {
                use_mdbook::mdbook_shared::PageId(18usize)
            }
            BookRoute::RouterExampleFullCode {} => use_mdbook::mdbook_shared::PageId(19usize),
            BookRoute::RouterReferenceIndex {} => use_mdbook::mdbook_shared::PageId(20usize),
            BookRoute::RouterReferenceRoutesIndex {} => use_mdbook::mdbook_shared::PageId(21usize),
            BookRoute::RouterReferenceRoutesNested {} => use_mdbook::mdbook_shared::PageId(22usize),
            BookRoute::RouterReferenceLayouts {} => use_mdbook::mdbook_shared::PageId(23usize),
            BookRoute::RouterReferenceNavigationIndex {} => {
                use_mdbook::mdbook_shared::PageId(24usize)
            }
            BookRoute::RouterReferenceNavigationProgrammatic {} => {
                use_mdbook::mdbook_shared::PageId(25usize)
            }
            BookRoute::RouterReferenceHistoryProviders {} => {
                use_mdbook::mdbook_shared::PageId(26usize)
            }
            BookRoute::RouterReferenceHistoryButtons {} => {
                use_mdbook::mdbook_shared::PageId(27usize)
            }
            BookRoute::RouterReferenceRoutingUpdateCallback {} => {
                use_mdbook::mdbook_shared::PageId(28usize)
            }
            BookRoute::ReferenceAssets {} => use_mdbook::mdbook_shared::PageId(29usize),
            BookRoute::ReferenceWebIndex {} => use_mdbook::mdbook_shared::PageId(30usize),
            BookRoute::ReferenceDesktopIndex {} => use_mdbook::mdbook_shared::PageId(31usize),
            BookRoute::ReferenceMobileIndex {} => use_mdbook::mdbook_shared::PageId(32usize),
            BookRoute::ReferenceMobileApis {} => use_mdbook::mdbook_shared::PageId(33usize),
            BookRoute::ReferenceSsr {} => use_mdbook::mdbook_shared::PageId(34usize),
            BookRoute::ReferenceFullstackIndex {} => use_mdbook::mdbook_shared::PageId(35usize),
            BookRoute::ReferenceFullstackServerFunctions {} => {
                use_mdbook::mdbook_shared::PageId(36usize)
            }
            BookRoute::ReferenceFullstackExtractors {} => {
                use_mdbook::mdbook_shared::PageId(37usize)
            }
            BookRoute::ReferenceFullstackMiddleware {} => {
                use_mdbook::mdbook_shared::PageId(38usize)
            }
            BookRoute::ReferenceFullstackAuthentication {} => {
                use_mdbook::mdbook_shared::PageId(39usize)
            }
            BookRoute::ReferenceFullstackRouting {} => use_mdbook::mdbook_shared::PageId(40usize),
            BookRoute::CookbookPublishing {} => use_mdbook::mdbook_shared::PageId(41usize),
            BookRoute::CookbookAntipatterns {} => use_mdbook::mdbook_shared::PageId(42usize),
            BookRoute::CookbookErrorHandling {} => use_mdbook::mdbook_shared::PageId(43usize),
            BookRoute::CookbookIntegrationsIndex {} => use_mdbook::mdbook_shared::PageId(44usize),
            BookRoute::CookbookIntegrationsLogging {} => use_mdbook::mdbook_shared::PageId(45usize),
            BookRoute::CookbookIntegrationsInternationalization {} => {
                use_mdbook::mdbook_shared::PageId(46usize)
            }
            BookRoute::CookbookStateIndex {} => use_mdbook::mdbook_shared::PageId(47usize),
            BookRoute::CookbookStateExternalIndex {} => use_mdbook::mdbook_shared::PageId(48usize),
            BookRoute::CookbookStateCustomHooksIndex {} => {
                use_mdbook::mdbook_shared::PageId(49usize)
            }
            BookRoute::CookbookTesting {} => use_mdbook::mdbook_shared::PageId(50usize),
            BookRoute::CookbookTailwind {} => use_mdbook::mdbook_shared::PageId(51usize),
            BookRoute::CookbookOptimizing {} => use_mdbook::mdbook_shared::PageId(52usize),
            BookRoute::MigrationIndex {} => use_mdbook::mdbook_shared::PageId(53usize),
            BookRoute::ContributingIndex {} => use_mdbook::mdbook_shared::PageId(54usize),
            BookRoute::ContributingProjectStructure {} => {
                use_mdbook::mdbook_shared::PageId(55usize)
            }
            BookRoute::ContributingGuidingPrinciples {} => {
                use_mdbook::mdbook_shared::PageId(56usize)
            }
            BookRoute::ContributingRoadmap {} => use_mdbook::mdbook_shared::PageId(57usize),
        }
    }
}
impl Default for BookRoute {
    fn default() -> Self {
        BookRoute::Index {}
    }
}
pub static LAZY_BOOK: use_mdbook::Lazy<use_mdbook::mdbook_shared::MdBook<BookRoute>> =
    use_mdbook::Lazy::new(|| {
        let mut page_id_mapping = ::std::collections::HashMap::new();
        let mut pages = Vec::new();
        pages.push((0usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Introduction".to_string(),
                url: BookRoute::Index {},
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
            BookRoute::Index {},
            ::use_mdbook::mdbook_shared::PageId(0usize),
        );
        pages.push((1usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Roadmap".to_string(),
                url: BookRoute::IntroductionRoadmap {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Roadmap".to_string(),
                        id: "roadmap".to_string(),
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
            BookRoute::IntroductionRoadmap {},
            ::use_mdbook::mdbook_shared::PageId(1usize),
        );
        pages.push((2usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Tutorial".to_string(),
                url: BookRoute::GuideIndex {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Dioxus Guide".to_string(),
                        id: "dioxus-guide".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Introduction".to_string(),
                        id: "introduction".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(2usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::GuideIndex {},
            ::use_mdbook::mdbook_shared::PageId(2usize),
        );
        pages.push((3usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Your First Component".to_string(),
                url: BookRoute::GuideYourFirstComponent {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Your First Component".to_string(),
                        id: "your-first-component".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Setup".to_string(),
                        id: "setup".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Describing the UI".to_string(),
                        id: "describing-the-ui".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Dynamic Text".to_string(),
                        id: "dynamic-text".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Creating Elements".to_string(),
                        id: "creating-elements".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Setting Attributes".to_string(),
                        id: "setting-attributes".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Creating a Component".to_string(),
                        id: "creating-a-component".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Creating Props".to_string(),
                        id: "creating-props".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Cleaning Up Our Interface".to_string(),
                        id: "cleaning-up-our-interface".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(3usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::GuideYourFirstComponent {},
            ::use_mdbook::mdbook_shared::PageId(3usize),
        );
        pages.push((4usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "State".to_string(),
                url: BookRoute::GuideState {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Interactivity".to_string(),
                        id: "interactivity".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Creating a Preview".to_string(),
                        id: "creating-a-preview".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Event Handlers".to_string(),
                        id: "event-handlers".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "State".to_string(),
                        id: "state".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "The Rules of Hooks".to_string(),
                        id: "the-rules-of-hooks".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "No Hooks in Conditionals".to_string(),
                        id: "no-hooks-in-conditionals".to_string(),
                        level: 4usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "No Hooks in Closures".to_string(),
                        id: "no-hooks-in-closures".to_string(),
                        level: 4usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "No Hooks in Loops".to_string(),
                        id: "no-hooks-in-loops".to_string(),
                        level: 4usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(4usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::GuideState {},
            ::use_mdbook::mdbook_shared::PageId(4usize),
        );
        pages.push((5usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Data Fetching".to_string(),
                url: BookRoute::GuideDataFetching {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Fetching Data".to_string(),
                        id: "fetching-data".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Defining the API".to_string(),
                        id: "defining-the-api".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Working with Async".to_string(),
                        id: "working-with-async".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Lazily Fetching Data".to_string(),
                        id: "lazily-fetching-data".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(5usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::GuideDataFetching {},
            ::use_mdbook::mdbook_shared::PageId(5usize),
        );
        pages.push((6usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Full Code".to_string(),
                url: BookRoute::GuideFullCode {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Conclusion".to_string(),
                        id: "conclusion".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Challenges".to_string(),
                        id: "challenges".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "The full code for the hacker news project".to_string(),
                        id: "the-full-code-for-the-hacker-news-project".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(6usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::GuideFullCode {},
            ::use_mdbook::mdbook_shared::PageId(6usize),
        );
        pages.push((7usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Essential Concepts".to_string(),
                url: BookRoute::EssentialsIndex {},
                segments: vec![],
                sections: vec![],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(7usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::EssentialsIndex {},
            ::use_mdbook::mdbook_shared::PageId(7usize),
        );
        pages.push((8usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Building UIs with RSX".to_string(),
                url: BookRoute::EssentialsRsxIndex {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Building UIs with RSX".to_string(),
                        id: "building-uis-with-rsx".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Text Nodes".to_string(),
                        id: "text-nodes".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Elements".to_string(),
                        id: "elements".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Attributes".to_string(),
                        id: "attributes".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Conditional Attributes".to_string(),
                        id: "conditional-attributes".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Event Listeners".to_string(),
                        id: "event-listeners".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Children".to_string(),
                        id: "children".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Loops".to_string(),
                        id: "loops".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "If Statements".to_string(),
                        id: "if-statements".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(8usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::EssentialsRsxIndex {},
            ::use_mdbook::mdbook_shared::PageId(8usize),
        );
        pages.push((9usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Component Lifecycle".to_string(),
                url: BookRoute::EssentialsLifecycleIndex {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Component Lifecycle".to_string(),
                        id: "component-lifecycle".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Initializing State with ".to_string(),
                        id: "initializing-state-with".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Rerendering".to_string(),
                        id: "rerendering".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "⚠\u{fe0f} Don't mutate state in the body of a component"
                            .to_string(),
                        id: "⚠\u{fe0f}-don't-mutate-state-in-the-body-of-a-component".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Using Effects".to_string(),
                        id: "using-effects".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Cleaning Up Components with Drop".to_string(),
                        id: "cleaning-up-components-with-drop".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(9usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::EssentialsLifecycleIndex {},
            ::use_mdbook::mdbook_shared::PageId(9usize),
        );
        pages.push((10usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Managing State".to_string(),
                url: BookRoute::EssentialsStateIndex {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Managing State".to_string(),
                        id: "managing-state".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Creating State".to_string(),
                        id: "creating-state".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Reactive Scopes".to_string(),
                        id: "reactive-scopes".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Derived State".to_string(),
                        id: "derived-state".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Derived Async State".to_string(),
                        id: "derived-async-state".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Derived UI".to_string(),
                        id: "derived-ui".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Working with Untracked State".to_string(),
                        id: "working-with-untracked-state".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Making Props Reactive".to_string(),
                        id: "making-props-reactive".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Moving Around State".to_string(),
                        id: "moving-around-state".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Passing props".to_string(),
                        id: "passing-props".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Passing context".to_string(),
                        id: "passing-context".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Using globals".to_string(),
                        id: "using-globals".to_string(),
                        level: 3usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(10usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::EssentialsStateIndex {},
            ::use_mdbook::mdbook_shared::PageId(10usize),
        );
        pages.push((11usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Breaking Out".to_string(),
                url: BookRoute::EssentialsBreakingIndex {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Breaking Out of Dioxus".to_string(),
                        id: "breaking-out-of-dioxus".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Interacting with JavaScript with ".to_string(),
                        id: "interacting-with-javascript-with".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Synchronizing DOM updates with ".to_string(),
                        id: "synchronizing-dom-updates-with".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Getting access to elements with ".to_string(),
                        id: "getting-access-to-elements-with".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Down casting web sys events".to_string(),
                        id: "down-casting-web-sys-events".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(11usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::EssentialsBreakingIndex {},
            ::use_mdbook::mdbook_shared::PageId(11usize),
        );
        pages.push((12usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Guides".to_string(),
                url: BookRoute::ReferenceIndex {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Dioxus Reference".to_string(),
                        id: "dioxus-reference".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Rendering".to_string(),
                        id: "rendering".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "State".to_string(),
                        id: "state".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Platforms".to_string(),
                        id: "platforms".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(12usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ReferenceIndex {},
            ::use_mdbook::mdbook_shared::PageId(12usize),
        );
        pages.push((13usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Router".to_string(),
                url: BookRoute::RouterIndex {},
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "Introduction".to_string(),
                    id: "introduction".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(13usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterIndex {},
            ::use_mdbook::mdbook_shared::PageId(13usize),
        );
        pages.push((14usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Example Project".to_string(),
                url: BookRoute::RouterExampleIndex {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Overview".to_string(),
                        id: "overview".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "You'll learn how to".to_string(),
                        id: "you'll-learn-how-to".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(14usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterExampleIndex {},
            ::use_mdbook::mdbook_shared::PageId(14usize),
        );
        pages.push((15usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Creating Our First Route".to_string(),
                url: BookRoute::RouterExampleFirstRoute {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Creating Our First Route".to_string(),
                        id: "creating-our-first-route".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Fundamentals".to_string(),
                        id: "fundamentals".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Creating Routes".to_string(),
                        id: "creating-routes".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Fallback Route".to_string(),
                        id: "fallback-route".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Conclusion".to_string(),
                        id: "conclusion".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(15usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterExampleFirstRoute {},
            ::use_mdbook::mdbook_shared::PageId(15usize),
        );
        pages.push((16usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Building a Nest".to_string(),
                url: BookRoute::RouterExampleBuildingANest {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Building a Nest".to_string(),
                        id: "building-a-nest".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Site Navigation".to_string(),
                        id: "site-navigation".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "URL Parameters and Nested Routes".to_string(),
                        id: "url-parameters-and-nested-routes".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Conclusion".to_string(),
                        id: "conclusion".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(16usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterExampleBuildingANest {},
            ::use_mdbook::mdbook_shared::PageId(16usize),
        );
        pages.push((17usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Navigation Targets".to_string(),
                url: BookRoute::RouterExampleNavigationTargets {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Navigation Targets".to_string(),
                        id: "navigation-targets".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "What is a navigation target?".to_string(),
                        id: "what-is-a-navigation-target?".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "External navigation".to_string(),
                        id: "external-navigation".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(17usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterExampleNavigationTargets {},
            ::use_mdbook::mdbook_shared::PageId(17usize),
        );
        pages.push((18usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Redirection Perfection".to_string(),
                url: BookRoute::RouterExampleRedirectionPerfection {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Redirection Perfection".to_string(),
                        id: "redirection-perfection".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Creating Redirects".to_string(),
                        id: "creating-redirects".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Conclusion".to_string(),
                        id: "conclusion".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Challenges".to_string(),
                        id: "challenges".to_string(),
                        level: 3usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(18usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterExampleRedirectionPerfection {},
            ::use_mdbook::mdbook_shared::PageId(18usize),
        );
        pages.push((19usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Full Code".to_string(),
                url: BookRoute::RouterExampleFullCode {},
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "Full Code".to_string(),
                    id: "full-code".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(19usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterExampleFullCode {},
            ::use_mdbook::mdbook_shared::PageId(19usize),
        );
        pages.push((20usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Reference".to_string(),
                url: BookRoute::RouterReferenceIndex {},
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "Adding the router to your application".to_string(),
                    id: "adding-the-router-to-your-application".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(20usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterReferenceIndex {},
            ::use_mdbook::mdbook_shared::PageId(20usize),
        );
        pages.push((21usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Defining Routes".to_string(),
                url: BookRoute::RouterReferenceRoutesIndex {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Defining Routes".to_string(),
                        id: "defining-routes".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Route Segments".to_string(),
                        id: "route-segments".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Static segments".to_string(),
                        id: "static-segments".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Dynamic Segments".to_string(),
                        id: "dynamic-segments".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Catch All Segments".to_string(),
                        id: "catch-all-segments".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Query Segments".to_string(),
                        id: "query-segments".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(21usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterReferenceRoutesIndex {},
            ::use_mdbook::mdbook_shared::PageId(21usize),
        );
        pages.push((22usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Nested Routes".to_string(),
                url: BookRoute::RouterReferenceRoutesNested {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Nested Routes".to_string(),
                        id: "nested-routes".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Nesting".to_string(),
                        id: "nesting".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(22usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterReferenceRoutesNested {},
            ::use_mdbook::mdbook_shared::PageId(22usize),
        );
        pages.push((23usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Layouts".to_string(),
                url: BookRoute::RouterReferenceLayouts {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Layouts".to_string(),
                        id: "layouts".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Layouts with dynamic segments".to_string(),
                        id: "layouts-with-dynamic-segments".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(23usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterReferenceLayouts {},
            ::use_mdbook::mdbook_shared::PageId(23usize),
        );
        pages.push((24usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Navigation".to_string(),
                url: BookRoute::RouterReferenceNavigationIndex {},
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "Links & Navigation".to_string(),
                    id: "links-&-navigation".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(24usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterReferenceNavigationIndex {},
            ::use_mdbook::mdbook_shared::PageId(24usize),
        );
        pages.push((25usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Programmatic Navigation".to_string(),
                url: BookRoute::RouterReferenceNavigationProgrammatic {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Programmatic Navigation".to_string(),
                        id: "programmatic-navigation".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Using a Navigator".to_string(),
                        id: "using-a-navigator".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "External Navigation Targets".to_string(),
                        id: "external-navigation-targets".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(25usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterReferenceNavigationProgrammatic {},
            ::use_mdbook::mdbook_shared::PageId(25usize),
        );
        pages.push((26usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "History Providers".to_string(),
                url: BookRoute::RouterReferenceHistoryProviders {},
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "History Providers".to_string(),
                    id: "history-providers".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(26usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterReferenceHistoryProviders {},
            ::use_mdbook::mdbook_shared::PageId(26usize),
        );
        pages.push((27usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "History Buttons".to_string(),
                url: BookRoute::RouterReferenceHistoryButtons {},
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "History Buttons".to_string(),
                    id: "history-buttons".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(27usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterReferenceHistoryButtons {},
            ::use_mdbook::mdbook_shared::PageId(27usize),
        );
        pages.push((28usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Routing Update Callback".to_string(),
                url: BookRoute::RouterReferenceRoutingUpdateCallback {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Routing Update Callback".to_string(),
                        id: "routing-update-callback".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "How does the callback behave?".to_string(),
                        id: "how-does-the-callback-behave?".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Code Example".to_string(),
                        id: "code-example".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(28usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::RouterReferenceRoutingUpdateCallback {},
            ::use_mdbook::mdbook_shared::PageId(28usize),
        );
        pages.push((29usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Assets".to_string(),
                url: BookRoute::ReferenceAssets {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Assets".to_string(),
                        id: "assets".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Including images".to_string(),
                        id: "including-images".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Including arbitrary files".to_string(),
                        id: "including-arbitrary-files".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Including stylesheets".to_string(),
                        id: "including-stylesheets".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Conclusion".to_string(),
                        id: "conclusion".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(29usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ReferenceAssets {},
            ::use_mdbook::mdbook_shared::PageId(29usize),
        );
        pages.push((30usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Web".to_string(),
                url: BookRoute::ReferenceWebIndex {},
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
                        title: "Running Javascript".to_string(),
                        id: "running-javascript".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Customizing Index Template".to_string(),
                        id: "customizing-index-template".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(30usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ReferenceWebIndex {},
            ::use_mdbook::mdbook_shared::PageId(30usize),
        );
        pages.push((31usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Desktop".to_string(),
                url: BookRoute::ReferenceDesktopIndex {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Desktop".to_string(),
                        id: "desktop".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Examples".to_string(),
                        id: "examples".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Running Javascript".to_string(),
                        id: "running-javascript".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Custom Assets".to_string(),
                        id: "custom-assets".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Integrating with Wry".to_string(),
                        id: "integrating-with-wry".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(31usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ReferenceDesktopIndex {},
            ::use_mdbook::mdbook_shared::PageId(31usize),
        );
        pages.push((32usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Mobile".to_string(),
                url: BookRoute::ReferenceMobileIndex {},
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
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Setting up dependencies".to_string(),
                        id: "setting-up-dependencies".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Android".to_string(),
                        id: "android".to_string(),
                        level: 4usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "IOS".to_string(),
                        id: "ios".to_string(),
                        level: 4usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Setting up your project".to_string(),
                        id: "setting-up-your-project".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Running".to_string(),
                        id: "running".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Android".to_string(),
                        id: "android".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "IOS".to_string(),
                        id: "ios".to_string(),
                        level: 3usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(32usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ReferenceMobileIndex {},
            ::use_mdbook::mdbook_shared::PageId(32usize),
        );
        pages.push((33usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "APIs".to_string(),
                url: BookRoute::ReferenceMobileApis {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Mobile".to_string(),
                        id: "mobile".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Running Javascript".to_string(),
                        id: "running-javascript".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Custom Assets".to_string(),
                        id: "custom-assets".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Integrating with Wry".to_string(),
                        id: "integrating-with-wry".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(33usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ReferenceMobileApis {},
            ::use_mdbook::mdbook_shared::PageId(33usize),
        );
        pages.push((34usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Streaming and SSR".to_string(),
                url: BookRoute::ReferenceSsr {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Server-Side Rendering".to_string(),
                        id: "server-side-rendering".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Setup".to_string(),
                        id: "setup".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Multithreaded Support".to_string(),
                        id: "multithreaded-support".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(34usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ReferenceSsr {},
            ::use_mdbook::mdbook_shared::PageId(34usize),
        );
        pages.push((35usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Fullstack".to_string(),
                url: BookRoute::ReferenceFullstackIndex {},
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "Fullstack development".to_string(),
                    id: "fullstack-development".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(35usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ReferenceFullstackIndex {},
            ::use_mdbook::mdbook_shared::PageId(35usize),
        );
        pages.push((36usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Server Functions".to_string(),
                url: BookRoute::ReferenceFullstackServerFunctions {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Communicating with the server".to_string(),
                        id: "communicating-with-the-server".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Cached data fetching".to_string(),
                        id: "cached-data-fetching".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Running the client with dioxus-desktop".to_string(),
                        id: "running-the-client-with-dioxus-desktop".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Client code".to_string(),
                        id: "client-code".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Server code".to_string(),
                        id: "server-code".to_string(),
                        level: 3usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(36usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ReferenceFullstackServerFunctions {},
            ::use_mdbook::mdbook_shared::PageId(36usize),
        );
        pages.push((37usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Extractors".to_string(),
                url: BookRoute::ReferenceFullstackExtractors {},
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "Extractors".to_string(),
                    id: "extractors".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(37usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ReferenceFullstackExtractors {},
            ::use_mdbook::mdbook_shared::PageId(37usize),
        );
        pages.push((38usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Middleware".to_string(),
                url: BookRoute::ReferenceFullstackMiddleware {},
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "Middleware".to_string(),
                    id: "middleware".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(38usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ReferenceFullstackMiddleware {},
            ::use_mdbook::mdbook_shared::PageId(38usize),
        );
        pages.push((39usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Authentication".to_string(),
                url: BookRoute::ReferenceFullstackAuthentication {},
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "Authentication".to_string(),
                    id: "authentication".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(39usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ReferenceFullstackAuthentication {},
            ::use_mdbook::mdbook_shared::PageId(39usize),
        );
        pages.push((40usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Routing".to_string(),
                url: BookRoute::ReferenceFullstackRouting {},
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "Routing".to_string(),
                    id: "routing".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(40usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ReferenceFullstackRouting {},
            ::use_mdbook::mdbook_shared::PageId(40usize),
        );
        pages.push((41usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Publishing".to_string(),
                url: BookRoute::CookbookPublishing {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Publishing".to_string(),
                        id: "publishing".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Web: Publishing with GitHub Pages".to_string(),
                        id: "web:-publishing-with-github-pages".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Desktop: Creating an installer".to_string(),
                        id: "desktop:-creating-an-installer".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Preparing your application for bundling".to_string(),
                        id: "preparing-your-application-for-bundling".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Adding assets to your application".to_string(),
                        id: "adding-assets-to-your-application".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Install ".to_string(),
                        id: "install".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Building".to_string(),
                        id: "building".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(41usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::CookbookPublishing {},
            ::use_mdbook::mdbook_shared::PageId(41usize),
        );
        pages.push((42usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Anti-patterns".to_string(),
                url: BookRoute::CookbookAntipatterns {},
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
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Avoid Large Groups of State".to_string(),
                        id: "avoid-large-groups-of-state".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Running Non-Deterministic Code in the Body of a Component"
                            .to_string(),
                        id: "running-non-deterministic-code-in-the-body-of-a-component".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Overly Permissive PartialEq for Props".to_string(),
                        id: "overly-permissive-partialeq-for-props".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(42usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::CookbookAntipatterns {},
            ::use_mdbook::mdbook_shared::PageId(42usize),
        );
        pages.push((43usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Error Handling".to_string(),
                url: BookRoute::CookbookErrorHandling {},
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
                        title: "Throwing errors".to_string(),
                        id: "throwing-errors".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(43usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::CookbookErrorHandling {},
            ::use_mdbook::mdbook_shared::PageId(43usize),
        );
        pages.push((44usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Integrations".to_string(),
                url: BookRoute::CookbookIntegrationsIndex {},
                segments: vec![],
                sections: vec![],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(44usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::CookbookIntegrationsIndex {},
            ::use_mdbook::mdbook_shared::PageId(44usize),
        );
        pages.push((45usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Logging".to_string(),
                url: BookRoute::CookbookIntegrationsLogging {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Logging".to_string(),
                        id: "logging".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "The Tracing Crate".to_string(),
                        id: "the-tracing-crate".to_string(),
                        level: 4usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Dioxus Logger".to_string(),
                        id: "dioxus-logger".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Platform Intricacies".to_string(),
                        id: "platform-intricacies".to_string(),
                        level: 4usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Final Notes".to_string(),
                        id: "final-notes".to_string(),
                        level: 4usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Desktop and Server".to_string(),
                        id: "desktop-and-server".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Web".to_string(),
                        id: "web".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Mobile".to_string(),
                        id: "mobile".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Android".to_string(),
                        id: "android".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Viewing Logs".to_string(),
                        id: "viewing-logs".to_string(),
                        level: 4usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "iOS".to_string(),
                        id: "ios".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Viewing Logs".to_string(),
                        id: "viewing-logs".to_string(),
                        level: 4usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(45usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::CookbookIntegrationsLogging {},
            ::use_mdbook::mdbook_shared::PageId(45usize),
        );
        pages.push((46usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Internationalization".to_string(),
                url: BookRoute::CookbookIntegrationsInternationalization {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Internationalization".to_string(),
                        id: "internationalization".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "The full code for internationalization".to_string(),
                        id: "the-full-code-for-internationalization".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(46usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::CookbookIntegrationsInternationalization {},
            ::use_mdbook::mdbook_shared::PageId(46usize),
        );
        pages.push((47usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "State Management".to_string(),
                url: BookRoute::CookbookStateIndex {},
                segments: vec![],
                sections: vec![::use_mdbook::mdbook_shared::Section {
                    title: "State Cookbook".to_string(),
                    id: "state-cookbook".to_string(),
                    level: 1usize,
                }],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(47usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::CookbookStateIndex {},
            ::use_mdbook::mdbook_shared::PageId(47usize),
        );
        pages.push((48usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "External State".to_string(),
                url: BookRoute::CookbookStateExternalIndex {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Working with External State".to_string(),
                        id: "working-with-external-state".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Working with non-reactive State".to_string(),
                        id: "working-with-non-reactive-state".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Making Reactive State External".to_string(),
                        id: "making-reactive-state-external".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(48usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::CookbookStateExternalIndex {},
            ::use_mdbook::mdbook_shared::PageId(48usize),
        );
        pages.push((49usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Custom Hooks".to_string(),
                url: BookRoute::CookbookStateCustomHooksIndex {},
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
                id: ::use_mdbook::mdbook_shared::PageId(49usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::CookbookStateCustomHooksIndex {},
            ::use_mdbook::mdbook_shared::PageId(49usize),
        );
        pages.push((50usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Testing".to_string(),
                url: BookRoute::CookbookTesting {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Testing".to_string(),
                        id: "testing".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Component Testing".to_string(),
                        id: "component-testing".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Hook Testing".to_string(),
                        id: "hook-testing".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "End to End Testing".to_string(),
                        id: "end-to-end-testing".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(50usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::CookbookTesting {},
            ::use_mdbook::mdbook_shared::PageId(50usize),
        );
        pages.push((51usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Tailwind".to_string(),
                url: BookRoute::CookbookTailwind {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Tailwind".to_string(),
                        id: "tailwind".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Setup".to_string(),
                        id: "setup".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Bonus Steps".to_string(),
                        id: "bonus-steps".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Development".to_string(),
                        id: "development".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Web".to_string(),
                        id: "web".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Desktop".to_string(),
                        id: "desktop".to_string(),
                        level: 3usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(51usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::CookbookTailwind {},
            ::use_mdbook::mdbook_shared::PageId(51usize),
        );
        pages.push((52usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Optimizing".to_string(),
                url: BookRoute::CookbookOptimizing {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Optimizing".to_string(),
                        id: "optimizing".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Building in release mode".to_string(),
                        id: "building-in-release-mode".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "UPX".to_string(),
                        id: "upx".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Build configuration".to_string(),
                        id: "build-configuration".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Stable".to_string(),
                        id: "stable".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Unstable".to_string(),
                        id: "unstable".to_string(),
                        level: 3usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "wasm-opt".to_string(),
                        id: "wasm-opt".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Improving Dioxus code".to_string(),
                        id: "improving-dioxus-code".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Optimizing the size of assets".to_string(),
                        id: "optimizing-the-size-of-assets".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(52usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::CookbookOptimizing {},
            ::use_mdbook::mdbook_shared::PageId(52usize),
        );
        pages.push((53usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Migrating to v0.6".to_string(),
                url: BookRoute::MigrationIndex {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "How to Upgrade to Dioxus 0.6".to_string(),
                        id: "how-to-upgrade-to-dioxus-0.6".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Element".to_string(),
                        id: "element".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Prevent Default".to_string(),
                        id: "prevent-default".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Assets".to_string(),
                        id: "assets".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(53usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::MigrationIndex {},
            ::use_mdbook::mdbook_shared::PageId(53usize),
        );
        pages.push((54usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Contributing".to_string(),
                url: BookRoute::ContributingIndex {},
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
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Before you contribute".to_string(),
                        id: "before-you-contribute".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "How to test dioxus with local crate".to_string(),
                        id: "how-to-test-dioxus-with-local-crate".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(54usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ContributingIndex {},
            ::use_mdbook::mdbook_shared::PageId(54usize),
        );
        pages.push((55usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Project Structure".to_string(),
                url: BookRoute::ContributingProjectStructure {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Project Structure".to_string(),
                        id: "project-structure".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Renderers".to_string(),
                        id: "renderers".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "State Management/Hooks".to_string(),
                        id: "state-management/hooks".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Core utilities".to_string(),
                        id: "core-utilities".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Native Renderer Utilities".to_string(),
                        id: "native-renderer-utilities".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Web renderer tooling".to_string(),
                        id: "web-renderer-tooling".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Developer tooling".to_string(),
                        id: "developer-tooling".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(55usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ContributingProjectStructure {},
            ::use_mdbook::mdbook_shared::PageId(55usize),
        );
        pages.push((56usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Guiding Principles".to_string(),
                url: BookRoute::ContributingGuidingPrinciples {},
                segments: vec![],
                sections: vec![
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Overall Goals".to_string(),
                        id: "overall-goals".to_string(),
                        level: 1usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Cross-Platform".to_string(),
                        id: "cross-platform".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Performance".to_string(),
                        id: "performance".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Type Safety".to_string(),
                        id: "type-safety".to_string(),
                        level: 2usize,
                    },
                    ::use_mdbook::mdbook_shared::Section {
                        title: "Developer Experience".to_string(),
                        id: "developer-experience".to_string(),
                        level: 2usize,
                    },
                ],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(56usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ContributingGuidingPrinciples {},
            ::use_mdbook::mdbook_shared::PageId(56usize),
        );
        pages.push((57usize, {
            ::use_mdbook::mdbook_shared::Page {
                title: "Roadmap".to_string(),
                url: BookRoute::ContributingRoadmap {},
                segments: vec![],
                sections: vec![],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(57usize),
            }
        }));
        page_id_mapping.insert(
            BookRoute::ContributingRoadmap {},
            ::use_mdbook::mdbook_shared::PageId(57usize),
        );
        ::use_mdbook::mdbook_shared::MdBook {
            summary: ::use_mdbook::mdbook_shared::Summary {
                title: Some("Summary".to_string()),
                prefix_chapters: vec![],
                numbered_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Introduction".to_string(),
                        location: Some(BookRoute::Index {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Roadmap".to_string(),
                                location: Some(BookRoute::IntroductionRoadmap {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Separator,
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Tutorial".to_string(),
                        location: Some(BookRoute::GuideIndex {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Your First Component".to_string(),
                                location: Some(BookRoute::GuideYourFirstComponent {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "State".to_string(),
                                location: Some(BookRoute::GuideState {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 2u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Data Fetching".to_string(),
                                location: Some(BookRoute::GuideDataFetching {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 3u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Full Code".to_string(),
                                location: Some(BookRoute::GuideFullCode {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 4u32]),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Separator,
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Essential Concepts".to_string(),
                        location: Some(BookRoute::EssentialsIndex {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Building UIs with RSX".to_string(),
                                location: Some(BookRoute::EssentialsRsxIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Component Lifecycle".to_string(),
                                location: Some(BookRoute::EssentialsLifecycleIndex {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 2u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Managing State".to_string(),
                                location: Some(BookRoute::EssentialsStateIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 3u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Breaking Out".to_string(),
                                location: Some(BookRoute::EssentialsBreakingIndex {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 4u32]),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Separator,
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Guides".to_string(),
                        location: Some(BookRoute::ReferenceIndex {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Router".to_string(),
                                location: Some(BookRoute::RouterIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 1u32]),
                                ),
                                nested_items: vec![
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Example Project".to_string(),
                                        location: Some(BookRoute::RouterExampleIndex {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 1u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Creating Our First Route".to_string(),
                                        location: Some(BookRoute::RouterExampleFirstRoute {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 2u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Building a Nest".to_string(),
                                        location: Some(BookRoute::RouterExampleBuildingANest {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 3u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Navigation Targets".to_string(),
                                        location: Some(BookRoute::RouterExampleNavigationTargets {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 4u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Redirection Perfection".to_string(),
                                        location: Some(BookRoute::RouterExampleRedirectionPerfection {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 5u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Full Code".to_string(),
                                        location: Some(BookRoute::RouterExampleFullCode {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 6u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Reference".to_string(),
                                        location: Some(BookRoute::RouterReferenceIndex {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 7u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Defining Routes".to_string(),
                                        location: Some(BookRoute::RouterReferenceRoutesIndex {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 8u32],
                                            ),
                                        ),
                                        nested_items: vec![
                                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                                name: "Nested Routes".to_string(),
                                                location: Some(BookRoute::RouterReferenceRoutesNested {
                                                }),
                                                number: Some(
                                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                                        vec![4u32, 1u32, 8u32, 1u32],
                                                    ),
                                                ),
                                                nested_items: vec![],
                                            }),
                                        ],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Layouts".to_string(),
                                        location: Some(BookRoute::RouterReferenceLayouts {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 9u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Navigation".to_string(),
                                        location: Some(BookRoute::RouterReferenceNavigationIndex {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 10u32],
                                            ),
                                        ),
                                        nested_items: vec![
                                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                                name: "Programmatic Navigation".to_string(),
                                                location: Some(BookRoute::RouterReferenceNavigationProgrammatic {}),
                                                number: Some(
                                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                                        vec![4u32, 1u32, 10u32, 1u32],
                                                    ),
                                                ),
                                                nested_items: vec![],
                                            }),
                                        ],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "History Providers".to_string(),
                                        location: Some(BookRoute::RouterReferenceHistoryProviders {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 11u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "History Buttons".to_string(),
                                        location: Some(BookRoute::RouterReferenceHistoryButtons {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 12u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Routing Update Callback".to_string(),
                                        location: Some(BookRoute::RouterReferenceRoutingUpdateCallback {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 13u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                ],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Assets".to_string(),
                                location: Some(BookRoute::ReferenceAssets {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 2u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Web".to_string(),
                                location: Some(BookRoute::ReferenceWebIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 3u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Desktop".to_string(),
                                location: Some(BookRoute::ReferenceDesktopIndex {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 4u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Mobile".to_string(),
                                location: Some(BookRoute::ReferenceMobileIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 5u32]),
                                ),
                                nested_items: vec![
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "APIs".to_string(),
                                        location: Some(BookRoute::ReferenceMobileApis {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 5u32, 1u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                ],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Streaming and SSR".to_string(),
                                location: Some(BookRoute::ReferenceSsr {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 6u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Fullstack".to_string(),
                                location: Some(BookRoute::ReferenceFullstackIndex {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 7u32]),
                                ),
                                nested_items: vec![
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Server Functions".to_string(),
                                        location: Some(BookRoute::ReferenceFullstackServerFunctions {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 7u32, 1u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Extractors".to_string(),
                                        location: Some(BookRoute::ReferenceFullstackExtractors {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 7u32, 2u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Middleware".to_string(),
                                        location: Some(BookRoute::ReferenceFullstackMiddleware {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 7u32, 3u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Authentication".to_string(),
                                        location: Some(BookRoute::ReferenceFullstackAuthentication {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 7u32, 4u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Routing".to_string(),
                                        location: Some(BookRoute::ReferenceFullstackRouting {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 7u32, 5u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                ],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Publishing".to_string(),
                                location: Some(BookRoute::CookbookPublishing {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 8u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Anti-patterns".to_string(),
                                location: Some(BookRoute::CookbookAntipatterns {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 9u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Error Handling".to_string(),
                                location: Some(BookRoute::CookbookErrorHandling {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 10u32],
                                    ),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Integrations".to_string(),
                                location: Some(BookRoute::CookbookIntegrationsIndex {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 11u32],
                                    ),
                                ),
                                nested_items: vec![
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Logging".to_string(),
                                        location: Some(BookRoute::CookbookIntegrationsLogging {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 11u32, 1u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Internationalization".to_string(),
                                        location: Some(BookRoute::CookbookIntegrationsInternationalization {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 11u32, 2u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                ],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "State Management".to_string(),
                                location: Some(BookRoute::CookbookStateIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 12u32],
                                    ),
                                ),
                                nested_items: vec![
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "External State".to_string(),
                                        location: Some(BookRoute::CookbookStateExternalIndex {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 12u32, 1u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Custom Hooks".to_string(),
                                        location: Some(BookRoute::CookbookStateCustomHooksIndex {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 12u32, 2u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                ],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Testing".to_string(),
                                location: Some(BookRoute::CookbookTesting {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 13u32],
                                    ),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Tailwind".to_string(),
                                location: Some(BookRoute::CookbookTailwind {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 14u32],
                                    ),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Optimizing".to_string(),
                                location: Some(BookRoute::CookbookOptimizing {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 15u32],
                                    ),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Migrating to v0.6".to_string(),
                                location: Some(BookRoute::MigrationIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 16u32],
                                    ),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Separator,
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Contributing".to_string(),
                        location: Some(BookRoute::ContributingIndex {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Project Structure".to_string(),
                                location: Some(BookRoute::ContributingProjectStructure {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Guiding Principles".to_string(),
                                location: Some(BookRoute::ContributingGuidingPrinciples {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32, 2u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Roadmap".to_string(),
                                location: Some(BookRoute::ContributingRoadmap {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32, 3u32]),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                ],
                suffix_chapters: vec![],
            },
            pages: pages.into_iter().collect(),
            page_id_mapping,
        }
    });
#[component(no_case_check)]
pub fn Index() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "introduction",
            a { href: "#introduction", class: "header", "Introduction" }
        }
        p {
            "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust. This guide will help you get started with writing Dioxus apps for the Web, Desktop, Mobile, and more."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;High-Five counter: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Up high!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Down low!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "readme.rs".to_string(),
        }
        DemoFrame { readme::App {} }
        p {
            "Dioxus is heavily inspired by React. If you know React, getting started with Dioxus will be a breeze."
        }
        blockquote {
            p {
                "This guide assumes you already know some "
                a { href: "https://www.rust-lang.org/", "Rust" }
                "! If not, we recommend reading "
                a { href: "https://doc.rust-lang.org/book/ch01-00-getting-started.html",
                    em { "the book" }
                }
                " to learn Rust first."
            }
        }
        h2 { id: "features",
            a { href: "#features", class: "header", "Features" }
        }
        ul {
            li { "Cross platform apps in three lines of code. (Web, Desktop, Server, Mobile, and more)" }
            li {
                "Incredibly ergonomic and powerful state management that combines the best parts of react, solid and svelte."
            }
            li {
                "Comprehensive inline documentation – hover and guides for all HTML elements, listeners, and events."
            }
            li {
                "High performance applications "
                a { href: "https://dioxuslabs.com/blog/templates-diffing",
                    "approaching the fastest web frameworks on the web"
                }
                " and native speeds on desktop."
            }
            li { "First-class async support." }
        }
        h3 { id: "multiplatform",
            a { href: "#multiplatform", class: "header", "Multiplatform" }
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
            li { "WebSys/Sledgehammer (for WASM): Great support" }
            li { "Tao/Tokio (for Desktop apps): Good support" }
            li { "Tao/Tokio (for Mobile apps): Poor support" }
            li { "Fullstack (for SSR and server functions): Good support" }
            li { "TUI/Plasmo (for terminal-based apps): Experimental" }
        }
        h2 { id: "stability",
            a { href: "#stability", class: "header", "Stability" }
        }
        p { "Dioxus has not reached a stable release yet." }
        p {
            "Web: Since the web is a fairly mature platform, we expect there to be very little API churn for web-based features."
        }
        p {
            "Desktop: APIs will likely be in flux as we figure out better patterns than our ElectronJS counterpart."
        }
        p {
            "Fullstack: APIs will likely be in flux as we figure out the best API for server communication."
        }
    }
}
#[component(no_case_check)]
pub fn IntroductionRoadmap() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "roadmap",
            a { href: "#roadmap", class: "header", "Roadmap" }
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
                th { "x" }
                th { "if/then to hide/show component" }
            }
            tr {
                th { "Map, Iterator" }
                th { "x" }
                th { "map/filter/reduce to produce rsx!" }
            }
            tr {
                th { "Keyed Components" }
                th { "x" }
                th { "advanced diffing with keys" }
            }
            tr {
                th { "Web" }
                th { "x" }
                th { "renderer for web browser" }
            }
            tr {
                th { "Desktop (webview)" }
                th { "x" }
                th { "renderer for desktop" }
            }
            tr {
                th { "Shared State (Context)" }
                th { "x" }
                th { "share state through the tree" }
            }
            tr {
                th { "Hooks" }
                th { "x" }
                th { "memory cells in components" }
            }
            tr {
                th { "SSR" }
                th { "x" }
                th { "render directly to string" }
            }
            tr {
                th { "Component Children" }
                th { "x" }
                th { "cx.children() as a list of nodes" }
            }
            tr {
                th { "Headless components" }
                th { "x" }
                th { "components that don't return real elements" }
            }
            tr {
                th { "Fragments" }
                th { "x" }
                th { "multiple elements without a real root" }
            }
            tr {
                th { "Manual Props" }
                th { "x" }
                th { "Manually pass in props with spread syntax" }
            }
            tr {
                th { "Controlled Inputs" }
                th { "x" }
                th { "stateful wrappers around inputs" }
            }
            tr {
                th { "CSS/Inline Styles" }
                th { "x" }
                th { "syntax for inline styles/attribute groups" }
            }
            tr {
                th { "Custom elements" }
                th { "x" }
                th { "Define new element primitives" }
            }
            tr {
                th { "Suspense" }
                th { "x" }
                th { "schedule future render from future/promise" }
            }
            tr {
                th { "Integrated error handling" }
                th { "x" }
                th { "Gracefully handle errors with ? syntax" }
            }
            tr {
                th { "NodeRef" }
                th { "x" }
                th { "gain direct access to nodes" }
            }
            tr {
                th { "Re-hydration" }
                th { "x" }
                th { "Pre-render to HTML to speed up first contentful paint" }
            }
            tr {
                th { "Jank-Free Rendering" }
                th { "x" }
                th { "Large diffs are segmented across frames for silky-smooth transitions" }
            }
            tr {
                th { "Effects" }
                th { "x" }
                th { "Run effects after a component has been committed to render" }
            }
            tr {
                th { "Portals" }
                th { "*" }
                th { "Render nodes outside of the traditional tree structure" }
            }
            tr {
                th { "Cooperative Scheduling" }
                th { "*" }
                th { "Prioritize important events over non-important events" }
            }
            tr {
                th { "Server Components" }
                th { "*" }
                th { "Hybrid components for SPA and Server" }
            }
            tr {
                th { "Bundle Splitting" }
                th { "i" }
                th { "Efficiently and asynchronously load the app" }
            }
            tr {
                th { "Lazy Components" }
                th { "i" }
                th { "Dynamically load the new components as the page is loaded" }
            }
            tr {
                th { "1st class global state" }
                th { "x" }
                th { "redux/recoil/mobx on top of context" }
            }
            tr {
                th { "Runs natively" }
                th { "x" }
                th { "runs as a portable binary w/o a runtime (Node)" }
            }
            tr {
                th { "Subtree Memoization" }
                th { "x" }
                th { "skip diffing static element subtrees" }
            }
            tr {
                th { "High-efficiency templates" }
                th { "x" }
                th { "rsx! calls are translated to templates on the DOM's side" }
            }
            tr {
                th { "Compile-time correct" }
                th { "x" }
                th { "Throw errors on invalid template layouts" }
            }
            tr {
                th { "Heuristic Engine" }
                th { "x" }
                th { "track component memory usage to minimize future allocations" }
            }
            tr {
                th { "Fine-grained reactivity" }
                th { "i" }
                th { "Skip diffing for fine-grain updates" }
            }
        }
        ul {
            li { "x = implemented and working" }
            li { "* = actively being worked on" }
            li { "i = not yet implemented or being worked on" }
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
                    value: "true",
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
                    value: "true",
                }
                "Asset macros"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "true",
                }
                "Css pipeline"
            }
            li {
                input {
                    r#type: "checkbox",
                    readonly: true,
                    class: "mdbook-checkbox",
                    value: "true",
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
            a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/cli",
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
pub fn GuideIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "dioxus-guide",
            a { href: "#dioxus-guide", class: "header", "Dioxus Guide" }
        }
        h2 { id: "introduction",
            a { href: "#introduction", class: "header", "Introduction" }
        }
        p {
            "In this guide, you'll learn to use Dioxus to build user interfaces that run anywhere. We will recreate the hackernews homepage in Dioxus:"
        }
        DemoFrame { hackernews_complete::App {} }
        p {
            "This guide serves a very brief overview of Dioxus. Throughout the guide, there will be links to the "
            a { href: "../reference", "reference" }
            " with more details about specific concepts."
        }
    }
}
#[component(no_case_check)]
pub fn GuideYourFirstComponent() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "your-first-component",
            a { href: "#your-first-component", class: "header", "Your First Component" }
        }
        p {
            "This chapter will teach you how to create a "
            a { href: "../reference/components", "Component" }
            " that displays a link to a post on hackernews."
        }
        h2 { id: "setup",
            a { href: "#setup", class: "header", "Setup" }
        }
        blockquote {
            p {
                "Before you start the guide, make sure you have the dioxus CLI and any required dependencies for your platform as described in the "
                a { href: "../getting_started", "getting started" }
                " guide."
            }
        }
        p {
            "First, let's create a new project for our hacker news app. We can use the CLI to create a new project. You can select a platform of your choice or view the getting started guide for more information on each option. If you aren't sure what platform to try out, we recommend getting started with web or desktop:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dx new</span></pre>\n" }
        p {
            "The template contains some boilerplate to help you get started. For this guide, we will be rebuilding some of the code from scratch for learning purposes. You can clear the  "
            code { "src/main.rs" }
            " file. We will be adding new code in the next sections."
        }
        p {
            "Next, let's setup our dependencies. We need to set up a few dependencies to work with the hacker news API: "
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add chrono </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features serde\n</span><span style=\"color:#f8f8f2;\">cargo add futures\n</span><span style=\"color:#f8f8f2;\">cargo add reqwest </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features json\n</span><span style=\"color:#f8f8f2;\">cargo add serde </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features derive\n</span><span style=\"color:#f8f8f2;\">cargo add serde_json\n</span><span style=\"color:#f8f8f2;\">cargo add async_recursion</span></pre>\n" }
        h2 { id: "describing-the-ui",
            a { href: "#describing-the-ui", class: "header", "Describing the UI" }
        }
        p {
            "Now, we can define how to display a post. Dioxus is a "
            em { "declarative" }
            " framework. This means that instead of telling Dioxus what to do (e.g. to \"create an element\" or \"set the color to red\") we simply "
            em { "declare" }
            " how we want the UI to look. "
        }
        p {
            "To declare what you want your UI to look like, you will need to use the  "
            code { "rsx" }
            " macro. Let's create a  "
            code { "main" }
            " function and an  "
            code { "App" }
            " component to show information about our story:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(App);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{</span><span style=\"color:#ffee99;\">&quot;story&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_post.rs".to_string(),
        }
        p { "Now if you run your application you should see something like this:" }
        DemoFrame { hackernews_post::story_v1::App {} }
        blockquote {
            p { "RSX mirrors HTML. Because of this you will need to know some html to use Dioxus." }
            p { "Here are some resources to help get you started learning HTML:" }
            ul {
                li {
                    a { href: "https://developer.mozilla.org/en-US/docs/Learn/HTML",
                        "MDN HTML Guide"
                    }
                }
                li {
                    a { href: "https://www.w3schools.com/html/default.asp",
                        "W3 Schools HTML Tutorial"
                    }
                }
            }
            p {
                "In addition to HTML, Dioxus uses CSS to style applications. You can either use traditional CSS (what this guide uses) or use a tool like "
                a { href: "https://tailwindcss.com/docs/installation", "tailwind CSS" }
                ":"
            }
            ul {
                li {
                    a { href: "https://developer.mozilla.org/en-US/docs/Learn/HTML",
                        "MDN Traditional CSS Guide"
                    }
                }
                li {
                    a { href: "https://www.w3schools.com/css/default.asp",
                        "W3 Schools Traditional CSS Tutorial"
                    }
                }
                li {
                    a { href: "https://tailwindcss.com/docs/installation", "Tailwind tutorial" }
                    " (used with the "
                    a { href: "https://github.com/DioxusLabs/dioxus/tree/v0.5/examples/tailwind",
                        "Tailwind setup example"
                    }
                    ")"
                }
            }
            p {
                "If you have existing html code, you can use the "
                a { href: "../CLI/translate", "translate" }
                " command to convert it to RSX. Or if you prefer to write html, you can use the "
                a { href: "https://github.com/DioxusLabs/dioxus-html-macro", "html! macro" }
                " to write html directly in your code."
            }
        }
        h2 { id: "dynamic-text",
            a { href: "#dynamic-text", class: "header", "Dynamic Text" }
        }
        p {
            "Let's expand our  "
            code { "App" }
            " component to include the story title, author, score, time posted, and number of comments. We can insert dynamic text in the render macro by inserting variables inside  "
            code { "{{}}" }
            "s (this works similarly to the formatting in the "
            a { href: "https://doc.rust-lang.org/std/macro.println.html", "println!" }
            " macro):"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> title </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;title&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> by </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;author&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> score </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> time </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">chrono::Utc::now();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;comments&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{</span><span style=\"color:#ffee99;\">&quot;{{title}} by {{by}} ({{score}}) {{time}} {{comments}}&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_post.rs".to_string(),
        }
        DemoFrame { hackernews_post::story_v2::App {} }
        h2 { id: "creating-elements",
            a { href: "#creating-elements", class: "header", "Creating Elements" }
        }
        p {
            "Next, let's wrap our post description in a "
            a { href: "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div",
                code { "div" }
            }
            ". You can create HTML elements in Dioxus by putting a "
            code { "{{" }
            " after the element name and a "
            code { "}}" }
            " after the last child of the element:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> title </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;title&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> by </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;author&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> score </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> time </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">chrono::Utc::now();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;comments&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ div {{ </span><span style=\"color:#ffee99;\">&quot;{{title}} by {{by}} ({{score}}) {{time}} {{comments}}&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_post.rs".to_string(),
        }
        DemoFrame { hackernews_post::story_v3::App {} }
        blockquote {
            p {
                "You can read more about elements in the "
                a { href: "../reference/rsx", "rsx reference" }
                "."
            }
        }
        h2 { id: "setting-attributes",
            a { href: "#setting-attributes", class: "header", "Setting Attributes" }
        }
        p { "Next, let's add some padding around our post listing with an attribute." }
        p {
            "Attributes (and "
            a { href: "../reference/event_handlers", "listeners" }
            ") modify the behavior or appearance of the element they are attached to. They are specified inside the "
            code { "{{}}" }
            " brackets before any children, using the "
            code { "name: value" }
            " syntax. You can format the text in the attribute as you would with a text node:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> title </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;title&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> by </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;author&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> score </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> time </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">chrono::Utc::now();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;comments&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, position: </span><span style=\"color:#ffee99;\">&quot;relative&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;{{title}} by {{by}} ({{score}}) {{time}} {{comments}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_post.rs".to_string(),
        }
        DemoFrame { hackernews_post::story_v4::App {} }
        blockquote {
            p {
                "Note: All attributes defined in "
                a { href: "https://docs.rs/dioxus-html/latest/dioxus_html/",
                    code { "dioxus-html" }
                }
                " follow the snake_case naming convention. They transform their "
                code { "snake_case" }
                " names to HTML's "
                code { "camelCase" }
                " attributes."
            }
        }
        blockquote {
            p {
                "Note: Styles can be used directly outside of the  "
                code { "style:" }
                " attribute. In the above example,  "
                code { "padding: \"0.5rem\"" }
                " is turned into  "
                code { "style=\"padding: 0.5rem\"" }
                "."
            }
        }
        blockquote {
            p {
                "You can read more about elements in the "
                a { href: "../reference/rsx", "attribute reference" }
            }
        }
        h2 { id: "creating-a-component",
            a { href: "#creating-a-component", class: "header", "Creating a Component" }
        }
        p {
            "Just like you wouldn't want to write a complex program in a single, long,  "
            code { "main" }
            " function, you shouldn't build a complex UI in a single  "
            code { "App" }
            " function. Instead, you should break down the functionality of an app in logical parts called components."
        }
        p {
            "A component is a Rust function, named in UpperCamelCase, that takes a props parameter and returns an  "
            code { "Element" }
            " describing the UI it wants to render. In fact, our  "
            code { "App" }
            " function is a component!"
        }
        p { "Let's pull our story description into a new component:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">StoryListing</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> title </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;title&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> by </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;author&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> score </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> time </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">chrono::Utc::now();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;comments&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, position: </span><span style=\"color:#ffee99;\">&quot;relative&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;{{title}} by {{by}} ({{score}}) {{time}} {{comments}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_post.rs".to_string(),
        }
        p {
            "We can render our component like we would an element by putting  "
            code { "{{}}" }
            "s after the component name. Let's modify our  "
            code { "App" }
            " component to render our new StoryListing component:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ StoryListing {{}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_post.rs".to_string(),
        }
        DemoFrame { hackernews_post::story_v5::App {} }
        blockquote {
            p {
                "You can read more about elements in the "
                a { href: "../reference/components", "component reference" }
            }
        }
        h2 { id: "creating-props",
            a { href: "#creating-props", class: "header", "Creating Props" }
        }
        p {
            "Just like you can pass arguments to a function or attributes to an element, you can pass props to a component that customize its behavior!"
        }
        p {
            "We can define arguments that components can take when they are rendered (called  "
            code { "Props" }
            ") by adding the  "
            code { "#[component]" }
            " macro before our function definition and adding extra function arguments."
        }
        p {
            "Currently, our  "
            code { "StoryListing" }
            " component always renders the same story. We can modify it to accept a story to render as a prop."
        }
        p {
            "We will also define what a post is and include information for how to transform our post to and from a different format using "
            a { href: "https://serde.rs", "serde" }
            ". This will be used with the hackernews API in a later chapter:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">chrono::{{DateTime, Utc}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">serde::{{Deserialize, Serialize}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Define the Hackernews types\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">StoryPageData {{\n</span><span style=\"color:#f8f8f2;\">    #[serde(flatten)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">item: StoryItem,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">comments: Vec&lt;CommentData&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">CommentData {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">id: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">/// there will be no by field if the comment was deleted\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">by: String,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">text: String,\n</span><span style=\"color:#f8f8f2;\">    #[serde(with = &quot;chrono::serde::ts_seconds&quot;)]\n</span><span style=\"color:#f8f8f2;\">    pub time: DateTime&lt;Utc&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">kids: Vec&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">sub_comments: Vec&lt;CommentData&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> r#type: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">StoryItem {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">id: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">title: String,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">url: Option&lt;String&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">text: Option&lt;String&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">by: String,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">score: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">descendants: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    #[serde(with = &quot;chrono::serde::ts_seconds&quot;)]\n</span><span style=\"color:#f8f8f2;\">    pub time: DateTime&lt;Utc&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">kids: Vec&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> r#type: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">StoryListing</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">story</span><span style=\"color:#f8f8f2;\">: ReadOnlySignal&lt;StoryItem&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> StoryItem {{\n</span><span style=\"color:#f8f8f2;\">        title,\n</span><span style=\"color:#f8f8f2;\">        url,\n</span><span style=\"color:#f8f8f2;\">        by,\n</span><span style=\"color:#f8f8f2;\">        score,\n</span><span style=\"color:#f8f8f2;\">        time,\n</span><span style=\"color:#f8f8f2;\">        kids,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">..\n</span><span style=\"color:#f8f8f2;\">    }} </span><span style=\"color:#f92672;\">= &amp;*</span><span style=\"color:#f8f8f2;\">story.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> kids.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, position: </span><span style=\"color:#ffee99;\">&quot;relative&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;{{title}} by {{by}} ({{score}}) {{time}} {{comments}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_post.rs".to_string(),
        }
        p {
            "Make sure to also add "
            a { href: "https://serde.rs", "serde" }
            " as a dependency:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add serde </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features derive\n</span><span style=\"color:#f8f8f2;\">cargo add serde_json</span></pre>\n" }
        p {
            "We will also use the "
            a { href: "https://crates.io/crates/chrono", "chrono" }
            " crate to provide utilities for handling time data from the hackernews API:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add chrono </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features serde</span></pre>\n" }
        p {
            "Now, let's modify the  "
            code { "App" }
            " component to pass the story to our  "
            code { "StoryListing" }
            " component like we would set an attribute on an element:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        StoryListing {{\n</span><span style=\"color:#f8f8f2;\">            story: StoryItem {{\n</span><span style=\"color:#f8f8f2;\">                id: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                title: </span><span style=\"color:#ffee99;\">&quot;hello hackernews&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                url: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                text: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                by: </span><span style=\"color:#ffee99;\">&quot;Author&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                score: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                descendants: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                time: chrono::Utc::now(),\n</span><span style=\"color:#f8f8f2;\">                kids: vec![],\n</span><span style=\"color:#f8f8f2;\">                r</span><span style=\"color:#f92672;\">#</span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_post.rs".to_string(),
        }
        DemoFrame { hackernews_post::story_v6::App {} }
        blockquote {
            p {
                "You can read more about Props in the "
                a { href: "../reference/component_props", "Props reference" }
            }
        }
        h2 { id: "cleaning-up-our-interface",
            a { href: "#cleaning-up-our-interface", class: "header", "Cleaning Up Our Interface" }
        }
        p {
            "Finally, by combining elements and attributes, we can make our post listing much more appealing:"
        }
        p { "Full code up to this point:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Define the Hackernews types\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">chrono::{{DateTime, Utc}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">serde::{{Deserialize, Serialize}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">StoryPageData {{\n</span><span style=\"color:#f8f8f2;\">    #[serde(flatten)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">item: StoryItem,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">comments: Vec&lt;CommentData&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">CommentData {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">id: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">/// there will be no by field if the comment was deleted\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">by: String,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">text: String,\n</span><span style=\"color:#f8f8f2;\">    #[serde(with = &quot;chrono::serde::ts_seconds&quot;)]\n</span><span style=\"color:#f8f8f2;\">    pub time: DateTime&lt;Utc&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">kids: Vec&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">sub_comments: Vec&lt;CommentData&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> r#type: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">StoryItem {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">id: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">title: String,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">url: Option&lt;String&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">text: Option&lt;String&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">by: String,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">score: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">descendants: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    #[serde(with = &quot;chrono::serde::ts_seconds&quot;)]\n</span><span style=\"color:#f8f8f2;\">    pub time: DateTime&lt;Utc&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">kids: Vec&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> r#type: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(App);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        StoryListing {{\n</span><span style=\"color:#f8f8f2;\">            story: StoryItem {{\n</span><span style=\"color:#f8f8f2;\">                id: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                title: </span><span style=\"color:#ffee99;\">&quot;hello hackernews&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                url: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                text: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                by: </span><span style=\"color:#ffee99;\">&quot;Author&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                score: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                descendants: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                time: Utc::now(),\n</span><span style=\"color:#f8f8f2;\">                kids: vec![],\n</span><span style=\"color:#f8f8f2;\">                r</span><span style=\"color:#f92672;\">#</span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">StoryListing</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">story</span><span style=\"color:#f8f8f2;\">: ReadOnlySignal&lt;StoryItem&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> StoryItem {{\n</span><span style=\"color:#f8f8f2;\">        title,\n</span><span style=\"color:#f8f8f2;\">        url,\n</span><span style=\"color:#f8f8f2;\">        by,\n</span><span style=\"color:#f8f8f2;\">        score,\n</span><span style=\"color:#f8f8f2;\">        time,\n</span><span style=\"color:#f8f8f2;\">        kids,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">..\n</span><span style=\"color:#f8f8f2;\">    }} </span><span style=\"color:#f92672;\">= &amp;*</span><span style=\"color:#f8f8f2;\">story.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> url </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> url.</span><span style=\"color:#66d9ef;\">as_deref</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap_or_default</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> hostname </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> url\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;https://&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;http://&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;www.&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> score </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{score}} {{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">if *</span><span style=\"color:#f8f8f2;\">score </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#ffee99;\">&quot; point&quot; </span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#ffee99;\">&quot; points&quot; </span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}} {{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        kids.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> kids.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot; comment&quot;\n</span><span style=\"color:#f8f8f2;\">        }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot; comments&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> time </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> time.</span><span style=\"color:#66d9ef;\">format</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;%D %l:%M %p&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, position: </span><span style=\"color:#ffee99;\">&quot;relative&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            div {{ font_size: </span><span style=\"color:#ffee99;\">&quot;1.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                a {{ href: url, </span><span style=\"color:#ffee99;\">&quot;{{title}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                a {{\n</span><span style=\"color:#f8f8f2;\">                    color: </span><span style=\"color:#ffee99;\">&quot;gray&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    href: </span><span style=\"color:#ffee99;\">&quot;https://news.ycombinator.com/from?site={{hostname}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    text_decoration: </span><span style=\"color:#ffee99;\">&quot;none&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#ffee99;\">&quot; ({{hostname}})&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            div {{ display: </span><span style=\"color:#ffee99;\">&quot;flex&quot;</span><span style=\"color:#f8f8f2;\">, flex_direction: </span><span style=\"color:#ffee99;\">&quot;row&quot;</span><span style=\"color:#f8f8f2;\">, color: </span><span style=\"color:#ffee99;\">&quot;gray&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                div {{ </span><span style=\"color:#ffee99;\">&quot;{{score}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                div {{ padding_left: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;by {{by}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                div {{ padding_left: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;{{time}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                div {{ padding_left: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;{{comments}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_post.rs".to_string(),
        }
        DemoFrame { hackernews_post::story_final::App {} }
    }
}
#[component(no_case_check)]
pub fn GuideState() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "interactivity",
            a { href: "#interactivity", class: "header", "Interactivity" }
        }
        p { "In this chapter, we will add a preview for articles you hover over or links you focus on." }
        h2 { id: "creating-a-preview",
            a { href: "#creating-a-preview", class: "header", "Creating a Preview" }
        }
        p {
            "First, let's split our app into a Stories component on the left side of the screen, and a preview component on the right side of the screen:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ display: </span><span style=\"color:#ffee99;\">&quot;flex&quot;</span><span style=\"color:#f8f8f2;\">, flex_direction: </span><span style=\"color:#ffee99;\">&quot;row&quot;</span><span style=\"color:#f8f8f2;\">, width: </span><span style=\"color:#ffee99;\">&quot;100%&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            div {{ width: </span><span style=\"color:#ffee99;\">&quot;50%&quot;</span><span style=\"color:#f8f8f2;\">, Stories {{}} }}\n</span><span style=\"color:#f8f8f2;\">            div {{ width: </span><span style=\"color:#ffee99;\">&quot;50%&quot;</span><span style=\"color:#f8f8f2;\">, Preview {{}} }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// New\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Stories</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        StoryListing {{\n</span><span style=\"color:#f8f8f2;\">            story: StoryItem {{\n</span><span style=\"color:#f8f8f2;\">                id: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                title: </span><span style=\"color:#ffee99;\">&quot;hello hackernews&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                url: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                text: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                by: </span><span style=\"color:#ffee99;\">&quot;Author&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                score: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                descendants: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                time: chrono::Utc::now(),\n</span><span style=\"color:#f8f8f2;\">                kids: vec![],\n</span><span style=\"color:#f8f8f2;\">                r</span><span style=\"color:#f92672;\">#</span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// New\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Debug)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">PreviewState {{\n</span><span style=\"color:#f8f8f2;\">    Unset,\n</span><span style=\"color:#f8f8f2;\">    Loading,\n</span><span style=\"color:#f8f8f2;\">    Loaded(StoryPageData),\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// New\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Preview</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> preview_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">PreviewState::Unset;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> preview_state {{\n</span><span style=\"color:#f8f8f2;\">        PreviewState::Unset </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{</span><span style=\"color:#ffee99;\">&quot;Hover over a story to preview it here&quot;</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">        PreviewState::Loading </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{</span><span style=\"color:#ffee99;\">&quot;Loading...&quot;</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">        PreviewState::Loaded(story) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            rsx! {{\n</span><span style=\"color:#f8f8f2;\">                div {{ padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    div {{ font_size: </span><span style=\"color:#ffee99;\">&quot;1.5rem&quot;</span><span style=\"color:#f8f8f2;\">, a {{ href: story.item.url, </span><span style=\"color:#ffee99;\">&quot;{{story.item.title}}&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">                    div {{ dangerous_inner_html: story.item.text }}\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> comment </span><span style=\"color:#f92672;\">in &amp;</span><span style=\"color:#f8f8f2;\">story.comments {{\n</span><span style=\"color:#f8f8f2;\">                        Comment {{ comment: comment.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// NEW\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Comment</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">comment</span><span style=\"color:#f8f8f2;\">: CommentData) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            div {{ color: </span><span style=\"color:#ffee99;\">&quot;gray&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;by {{comment.by}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            div {{ dangerous_inner_html: </span><span style=\"color:#ffee99;\">&quot;{{comment.text}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> kid </span><span style=\"color:#f92672;\">in &amp;</span><span style=\"color:#f8f8f2;\">comment.sub_comments {{\n</span><span style=\"color:#f8f8f2;\">                Comment {{ comment: kid.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_state.rs".to_string(),
        }
        DemoFrame { hackernews_state::app_v1::App {} }
        h2 { id: "event-handlers",
            a { href: "#event-handlers", class: "header", "Event Handlers" }
        }
        p {
            "Next, we need to detect when the user hovers over a section or focuses a link. We can use an "
            a { href: "../reference/event_handlers", "event listener" }
            " to listen for the hover and focus events."
        }
        p {
            "Event handlers are similar to regular attributes, but their name usually starts with  "
            code { "on" }
            "- and they accept closures as values. The closure will be called whenever the event it listens for is triggered. When an event is triggered, information about the event is passed to the closure through the "
            a { href: "https://docs.rs/dioxus/latest/dioxus/prelude/struct.Event.html",
                "Event"
            }
            " structure."
        }
        p {
            "Let's create a "
            a { href: "https://docs.rs/dioxus/latest/dioxus/events/fn.onmouseenter.html",
                code { "onmouseenter" }
            }
            " event listener in the "
            code { "StoryListing" }
            " component:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    div {{\n</span><span style=\"color:#f8f8f2;\">        padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        position: </span><span style=\"color:#ffee99;\">&quot;relative&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        onmouseenter: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{}},\n</span><span style=\"color:#f8f8f2;\">        div {{ font_size: </span><span style=\"color:#ffee99;\">&quot;1.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            a {{ href: url, onfocus: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">_event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{}}, </span><span style=\"color:#ffee99;\">&quot;{{title}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            a {{\n</span><span style=\"color:#f8f8f2;\">                color: </span><span style=\"color:#ffee99;\">&quot;gray&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                href: </span><span style=\"color:#ffee99;\">&quot;https://news.ycombinator.com/from?site={{hostname}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                text_decoration: </span><span style=\"color:#ffee99;\">&quot;none&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot; ({{hostname}})&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        div {{ display: </span><span style=\"color:#ffee99;\">&quot;flex&quot;</span><span style=\"color:#f8f8f2;\">, flex_direction: </span><span style=\"color:#ffee99;\">&quot;row&quot;</span><span style=\"color:#f8f8f2;\">, color: </span><span style=\"color:#ffee99;\">&quot;gray&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            div {{ </span><span style=\"color:#ffee99;\">&quot;{{score}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            div {{ padding_left: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;by {{by}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            div {{ padding_left: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;{{time}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            div {{ padding_left: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;{{comments}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_state.rs".to_string(),
        }
        blockquote {
            p {
                "You can read more about Event Handlers in the "
                a { href: "../reference/event_handlers", "Event Handler reference" }
            }
        }
        h2 { id: "state",
            a { href: "#state", class: "header", "State" }
        }
        p {
            "So far our components have had no state like normal rust functions. To make our application change when we hover over a link we need state to store the currently hovered link in the root of the application."
        }
        p {
            "You can create state in dioxus using hooks. Hooks are Rust functions you call in a constant order in a component that add additional functionality to the component."
        }
        p {
            "In this case, we will use the  "
            code { "use_context_provider" }
            " and  "
            code { "use_context" }
            " hooks:"
        }
        ul {
            li {
                "You can provide a closure to "
                code { "use_context_provider" }
                " that determines the initial value of the shared state and provides the value to all child components"
            }
            li {
                "You can then use the "
                code { "use_context" }
                " hook to read and modify that state in the "
                code { "Preview" }
                " and "
                code { "StoryListing" }
                " components"
            }
            li {
                "When the value updates, the "
                code { "Signal" }
                " will cause the component to re-render, and provides you with the new value"
            }
        }
        blockquote {
            p {
                "Note: You should prefer local state hooks like use_signal or use_signal_sync when you only use state in one component. Because we use state in multiple components, we can use a "
                a { href: "../reference/context", "global state pattern" }
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_context_provider</span><span style=\"color:#f8f8f2;\">(|| Signal::new(PreviewState::Unset));</span></pre>\n",
            name: "hackernews_state.rs".to_string(),
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">StoryListing</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">story</span><span style=\"color:#f8f8f2;\">: ReadOnlySignal&lt;StoryItem&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> preview_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">consume_context::&lt;Signal&lt;PreviewState&gt;&gt;();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> StoryItem {{\n</span><span style=\"color:#f8f8f2;\">        title,\n</span><span style=\"color:#f8f8f2;\">        url,\n</span><span style=\"color:#f8f8f2;\">        by,\n</span><span style=\"color:#f8f8f2;\">        score,\n</span><span style=\"color:#f8f8f2;\">        time,\n</span><span style=\"color:#f8f8f2;\">        kids,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">..\n</span><span style=\"color:#f8f8f2;\">    }} </span><span style=\"color:#f92672;\">= &amp;*</span><span style=\"color:#f8f8f2;\">story.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> url </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> url.</span><span style=\"color:#66d9ef;\">as_deref</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap_or_default</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> hostname </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> url\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;https://&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;http://&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;www.&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> score </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{score}}</span><span style=\"color:#ffee99;\"> point</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">if *</span><span style=\"color:#f8f8f2;\">score </span><span style=\"color:#f92672;\">&gt; </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#ffee99;\">&quot;s&quot; </span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#ffee99;\">&quot;&quot; </span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}} {{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        kids.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> kids.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot; comment&quot;\n</span><span style=\"color:#f8f8f2;\">        }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot; comments&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> time </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> time.</span><span style=\"color:#66d9ef;\">format</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;%D %l:%M %p&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            position: </span><span style=\"color:#ffee99;\">&quot;relative&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            onmouseenter: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">_event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">preview_state\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">PreviewState::Loaded(StoryPageData {{\n</span><span style=\"color:#f8f8f2;\">                    item: </span><span style=\"color:#66d9ef;\">story</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    comments: vec![],\n</span><span style=\"color:#f8f8f2;\">                }});\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            div {{ font_size: </span><span style=\"color:#ffee99;\">&quot;1.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                a {{\n</span><span style=\"color:#f8f8f2;\">                    href: url,\n</span><span style=\"color:#f8f8f2;\">                    onfocus: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">_event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">preview_state\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">PreviewState::Loaded(StoryPageData {{\n</span><span style=\"color:#f8f8f2;\">                            item: </span><span style=\"color:#66d9ef;\">story</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                            comments: vec![],\n</span><span style=\"color:#f8f8f2;\">                        }});\n</span><span style=\"color:#f8f8f2;\">                    }},</span></pre>\n",
            name: "hackernews_state.rs".to_string(),
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Preview</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// New\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> preview_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">consume_context::&lt;Signal&lt;PreviewState&gt;&gt;();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// New\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match </span><span style=\"color:#66d9ef;\">preview_state</span><span style=\"color:#f8f8f2;\">() {{</span></pre>\n",
            name: "hackernews_state.rs".to_string(),
        }
        DemoFrame { hackernews_state::App {} }
        blockquote {
            p {
                "You can read more about Hooks in the "
                a { href: "../reference/hooks", "Hooks reference" }
            }
        }
        h3 { id: "the-rules-of-hooks",
            a { href: "#the-rules-of-hooks", class: "header", "The Rules of Hooks" }
        }
        p {
            "Hooks are a powerful way to manage state in Dioxus, but there are some rules you need to follow to insure they work as expected. Dioxus uses the order you call hooks to differentiate between hooks. Because the order you call hooks matters, you must follow these rules:"
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
        h4 { id: "no-hooks-in-conditionals",
            a { href: "#no-hooks-in-conditionals", class: "header", "No Hooks in Conditionals" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ don&#39;t call hooks in conditionals!\n</span><span style=\"color:#8c8c8c;\">// We must ensure that the same hooks will be called every time\n</span><span style=\"color:#8c8c8c;\">// But `if` statements only run if the conditional is true!\n</span><span style=\"color:#8c8c8c;\">// So we might violate rule 2.\n</span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> you_are_happy </span><span style=\"color:#f92672;\">&amp;&amp;</span><span style=\"color:#f8f8f2;\"> you_know_it {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> something </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;hands&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;clap your </span><span style=\"color:#ff80f4;\">{{something}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ instead, *always* call use_signal\n</span><span style=\"color:#8c8c8c;\">// You can put other stuff in the conditional though\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> something </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;hands&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> you_are_happy </span><span style=\"color:#f92672;\">&amp;&amp;</span><span style=\"color:#f8f8f2;\"> you_know_it {{\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;clap your </span><span style=\"color:#ff80f4;\">{{something}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_bad.rs".to_string(),
        }
        h4 { id: "no-hooks-in-closures",
            a { href: "#no-hooks-in-closures", class: "header", "No Hooks in Closures" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ don&#39;t call hooks inside closures!\n</span><span style=\"color:#8c8c8c;\">// We can&#39;t guarantee that the closure, if used, will be called in the same order every time\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#a6e22e;\">_a </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">|| {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> b </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">b</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ instead, move hook `b` outside\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> b </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#a6e22e;\">_a </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">|| </span><span style=\"color:#66d9ef;\">b</span><span style=\"color:#f8f8f2;\">();</span></pre>\n",
            name: "hooks_bad.rs".to_string(),
        }
        h4 { id: "no-hooks-in-loops",
            a { href: "#no-hooks-in-loops", class: "header", "No Hooks in Loops" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// `names` is a Vec&lt;&amp;str&gt;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ❌ Do not use hooks in loops!\n</span><span style=\"color:#8c8c8c;\">// In this case, if the length of the Vec changes, we break rule 2\n</span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> _name </span><span style=\"color:#f92672;\">in &amp;</span><span style=\"color:#f8f8f2;\">names {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> is_selected </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">false</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;selected: </span><span style=\"color:#ff80f4;\">{{is_selected}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Instead, use a hashmap with use_signal\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> selection_map </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(HashMap::&lt;</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#66d9ef;\">bool</span><span style=\"color:#f8f8f2;\">&gt;::new);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">in &amp;</span><span style=\"color:#f8f8f2;\">names {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> is_selected </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> selection_map.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">()[name];\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;selected: </span><span style=\"color:#ff80f4;\">{{is_selected}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_bad.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn GuideDataFetching() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "fetching-data",
            a { href: "#fetching-data", class: "header", "Fetching Data" }
        }
        p {
            "In this chapter, we will fetch data from the hacker news API and use it to render the list of top posts in our application."
        }
        h2 { id: "defining-the-api",
            a { href: "#defining-the-api", class: "header", "Defining the API" }
        }
        p {
            "First we need to create some utilities to fetch data from the hackernews API using "
            a { href: "https://docs.rs/reqwest/latest/reqwest/index.html", "reqwest" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Define the Hackernews API\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">futures::future::join_all;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">BASE_API_URL</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;https://hacker-news.firebaseio.com/v0/&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">ITEM_API</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;item/&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">USER_API</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;user/&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">COMMENT_DEPTH</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_story_preview</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">) -&gt; Result&lt;StoryItem, reqwest::Error&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> url </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}{{}}{{}}</span><span style=\"color:#ffee99;\">.json&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">BASE_API_URL</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">ITEM_API</span><span style=\"color:#f8f8f2;\">, id);\n</span><span style=\"color:#f8f8f2;\">    reqwest::get(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">url).await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">json</span><span style=\"color:#f8f8f2;\">().await\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_stories</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">) -&gt; Result&lt;Vec&lt;StoryItem&gt;, reqwest::Error&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> url </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">topstories.json&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">BASE_API_URL</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> stories_ids </span><span style=\"color:#f92672;\">= &amp;</span><span style=\"color:#f8f8f2;\">reqwest::get(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">url).await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">.json::&lt;Vec&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">&gt;&gt;().await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#f92672;\">..</span><span style=\"color:#f8f8f2;\">count];\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> story_futures </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> stories_ids[</span><span style=\"color:#f92672;\">..</span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">::min(stories_ids.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">(), count)]\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">story_id</span><span style=\"color:#f8f8f2;\">| </span><span style=\"color:#66d9ef;\">get_story_preview</span><span style=\"color:#f8f8f2;\">(story_id));\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> stories </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">join_all</span><span style=\"color:#f8f8f2;\">(story_futures)\n</span><span style=\"color:#f8f8f2;\">        .await\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">into_iter</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">filter_map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">story</span><span style=\"color:#f8f8f2;\">| story.</span><span style=\"color:#66d9ef;\">ok</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">collect</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(stories)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_story</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">) -&gt; Result&lt;StoryPageData, reqwest::Error&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> url </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}{{}}{{}}</span><span style=\"color:#ffee99;\">.json&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">BASE_API_URL</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">ITEM_API</span><span style=\"color:#f8f8f2;\">, id);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> story </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">reqwest::get(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">url).await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">.json::&lt;StoryPageData&gt;().await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comment_futures </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> story.item.kids.</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">| </span><span style=\"color:#66d9ef;\">get_comment</span><span style=\"color:#f8f8f2;\">(id));\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">join_all</span><span style=\"color:#f8f8f2;\">(comment_futures)\n</span><span style=\"color:#f8f8f2;\">        .await\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">into_iter</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">filter_map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">c</span><span style=\"color:#f8f8f2;\">| c.</span><span style=\"color:#66d9ef;\">ok</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">collect</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    story.comments </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> comments;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(story)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[async_recursion::async_recursion(?Send)]\n</span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_comment_with_depth</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">depth</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">) -&gt; Result&lt;CommentData, reqwest::Error&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> url </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}{{}}{{}}</span><span style=\"color:#ffee99;\">.json&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">BASE_API_URL</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">ITEM_API</span><span style=\"color:#f8f8f2;\">, id);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> comment </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">reqwest::get(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">url).await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">.json::&lt;CommentData&gt;().await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> depth </span><span style=\"color:#f92672;\">&gt; </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> sub_comments_futures </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> comment\n</span><span style=\"color:#f8f8f2;\">            .kids\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">story_id</span><span style=\"color:#f8f8f2;\">| </span><span style=\"color:#66d9ef;\">get_comment_with_depth</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">story_id, depth </span><span style=\"color:#f92672;\">- </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">        comment.sub_comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">join_all</span><span style=\"color:#f8f8f2;\">(sub_comments_futures)\n</span><span style=\"color:#f8f8f2;\">            .await\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">into_iter</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">filter_map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">c</span><span style=\"color:#f8f8f2;\">| c.</span><span style=\"color:#66d9ef;\">ok</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">collect</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(comment)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_comment</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">comment_id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">) -&gt; Result&lt;CommentData, reqwest::Error&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comment </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">get_comment_with_depth</span><span style=\"color:#f8f8f2;\">(comment_id, </span><span style=\"color:#ff80f4;\">COMMENT_DEPTH</span><span style=\"color:#f8f8f2;\">).await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(comment)\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_async.rs".to_string(),
        }
        p {
            "The code above requires you to add the "
            a { href: "https://crates.io/crates/reqwest", "reqwest" }
            ", "
            a { href: "https://crates.io/crates/async-recursion", "async_recursion" }
            ", and "
            a { href: "https://crates.io/crates/futures", "futures" }
            " crate:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add reqwest </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features json\n</span><span style=\"color:#f8f8f2;\">cargo add async_recursion\n</span><span style=\"color:#f8f8f2;\">cargo add futures</span></pre>\n" }
        p { "A quick overview of the supporting crates:" }
        ul {
            li {
                a { href: "https://crates.io/crates/reqwest", "reqwest" }
                " allows us to create HTTP calls to the hackernews API. "
            }
            li {
                a { href: "https://crates.io/crates/async-recursion", "async_recursion" }
                " provides a utility macro to allow us to recursively use an async function."
            }
            li {
                a { href: "https://crates.io/crates/futures", "futures" }
                " provides us with utilities all around Rust's futures."
            }
        }
        h2 { id: "working-with-async",
            a { href: "#working-with-async", class: "header", "Working with Async" }
        }
        p {
            a { href: "https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_resource.html",
                code { "use_resource" }
            }
            " is a "
            a { href: "./state", "hook" }
            " that lets you run an async closure, and provides you with its result."
        }
        p {
            "For example, we can make an API request (using "
            a { href: "https://docs.rs/reqwest/latest/reqwest/index.html", "reqwest" }
            ") inside "
            code { "use_resource" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Stories</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Fetch the top 10 stories on Hackernews\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> stories </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#66d9ef;\">get_stories</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">10</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// check if the future is resolved\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match &amp;*</span><span style=\"color:#f8f8f2;\">stories.</span><span style=\"color:#66d9ef;\">read_unchecked</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(list)) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// if it is, render the stories\n</span><span style=\"color:#f8f8f2;\">            rsx! {{\n</span><span style=\"color:#f8f8f2;\">                div {{\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#8c8c8c;\">// iterate over the stories with a for loop\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> story </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> list {{\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"color:#8c8c8c;\">// render every story with the StoryListing component\n</span><span style=\"color:#f8f8f2;\">                        StoryListing {{ story: story.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(err)) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// if there was an error, render the error\n</span><span style=\"color:#f8f8f2;\">            rsx! {{</span><span style=\"color:#ffee99;\">&quot;An error occurred while fetching stories {{err}}&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// if the future is not resolved yet, render a loading message\n</span><span style=\"color:#f8f8f2;\">            rsx! {{</span><span style=\"color:#ffee99;\">&quot;Loading items&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_async.rs".to_string(),
        }
        p {
            "The code inside  "
            code { "use_resource" }
            " will be submitted to the Dioxus scheduler once the component has rendered."
        }
        p {
            "We can use  "
            code { ".read()" }
            " to get the result of the future. On the first run, since there's no data ready when the component loads, its value will be  "
            code { "None" }
            ".  However, once the future is finished, the component will be re-rendered and the value will now be  "
            code { "Some(...)" }
            ", containing the return value of the closure."
        }
        p {
            "We can then render the result by looping over each of the posts and rendering them with the  "
            code { "StoryListing" }
            " component."
        }
        DemoFrame { hackernews_async::fetch::App {} }
        blockquote {
            p {
                "You can read more about working with Async in Dioxus in the "
                a { href: "../reference", "Async reference" }
            }
        }
        h2 { id: "lazily-fetching-data",
            a { href: "#lazily-fetching-data", class: "header", "Lazily Fetching Data" }
        }
        p { "Finally, we will lazily fetch the comments on each post as the user hovers over the post." }
        p {
            "We need to revisit the code that handles hovering over an item. Instead of passing an empty list of comments, we can fetch all the related comments when the user hovers over the item."
        }
        p {
            "We will cache the list of comments with a "
            a { href: "https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_signal.html",
                "use_signal"
            }
            " hook. This hook allows you to store some state in a single component. When the user triggers fetching the comments we will check if the response has already been cached before fetching the data from the hackernews API."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// New\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">resolve_story</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">full_story</span><span style=\"color:#f8f8f2;\">: Signal&lt;Option&lt;StoryPageData&gt;&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">preview_state</span><span style=\"color:#f8f8f2;\">: Signal&lt;PreviewState&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">story_id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(cached) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> full_story.</span><span style=\"color:#66d9ef;\">as_ref</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">preview_state.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">PreviewState::Loaded(cached.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">return</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">preview_state.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">PreviewState::Loading;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Ok</span><span style=\"color:#f8f8f2;\">(story) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">get_story</span><span style=\"color:#f8f8f2;\">(story_id).await {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">preview_state.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">PreviewState::Loaded(story.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">full_story.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(story);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">StoryListing</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">story</span><span style=\"color:#f8f8f2;\">: ReadOnlySignal&lt;StoryItem&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> preview_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">consume_context::&lt;Signal&lt;PreviewState&gt;&gt;();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> StoryItem {{\n</span><span style=\"color:#f8f8f2;\">        title,\n</span><span style=\"color:#f8f8f2;\">        url,\n</span><span style=\"color:#f8f8f2;\">        by,\n</span><span style=\"color:#f8f8f2;\">        score,\n</span><span style=\"color:#f8f8f2;\">        time,\n</span><span style=\"color:#f8f8f2;\">        kids,\n</span><span style=\"color:#f8f8f2;\">        id,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">..\n</span><span style=\"color:#f8f8f2;\">    }} </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">story</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// New\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> full_story </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> url </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> url.</span><span style=\"color:#66d9ef;\">as_deref</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap_or_default</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> hostname </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> url\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;https://&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;http://&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;www.&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> score </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{score}} {{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> score </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#ffee99;\">&quot; point&quot; </span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#ffee99;\">&quot; points&quot; </span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}} {{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        kids.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> kids.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot; comment&quot;\n</span><span style=\"color:#f8f8f2;\">        }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot; comments&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> time </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> time.</span><span style=\"color:#66d9ef;\">format</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;%D %l:%M %p&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            position: </span><span style=\"color:#ffee99;\">&quot;relative&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            onmouseenter: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">_event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#66d9ef;\">resolve_story</span><span style=\"color:#f8f8f2;\">(full_story, preview_state, id) }},\n</span><span style=\"color:#f8f8f2;\">            div {{ font_size: </span><span style=\"color:#ffee99;\">&quot;1.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                a {{\n</span><span style=\"color:#f8f8f2;\">                    href: url,\n</span><span style=\"color:#f8f8f2;\">                    onfocus: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">_event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#66d9ef;\">resolve_story</span><span style=\"color:#f8f8f2;\">(full_story, preview_state, id) }},\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#8c8c8c;\">// ...</span></pre>\n",
            name: "hackernews_async.rs".to_string(),
        }
        DemoFrame { hackernews_async::App {} }
    }
}
#[component(no_case_check)]
pub fn GuideFullCode() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "conclusion",
            a { href: "#conclusion", class: "header", "Conclusion" }
        }
        p {
            "Well done! You've completed the Dioxus guide and built a hackernews application in Dioxus. "
        }
        p {
            "To continue your journey, you can attempt a challenge listed below, or look at the "
            a { href: "../reference", "Dioxus reference" }
            "."
        }
        h2 { id: "challenges",
            a { href: "#challenges", class: "header", "Challenges" }
        }
        ul {
            li { "Organize your components into separate files for better maintainability." }
            li { "Give your app some style if you haven't already." }
            li {
                "Integrate your application with the "
                a { href: "../router", "Dioxus router" }
                "."
            }
        }
        h2 { id: "the-full-code-for-the-hacker-news-project",
            a {
                href: "#the-full-code-for-the-hacker-news-project",
                class: "header",
                "The full code for the hacker news project"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case)]\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(App);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_context_provider</span><span style=\"color:#f8f8f2;\">(|| Signal::new(PreviewState::Unset));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ display: </span><span style=\"color:#ffee99;\">&quot;flex&quot;</span><span style=\"color:#f8f8f2;\">, flex_direction: </span><span style=\"color:#ffee99;\">&quot;row&quot;</span><span style=\"color:#f8f8f2;\">, width: </span><span style=\"color:#ffee99;\">&quot;100%&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            div {{ width: </span><span style=\"color:#ffee99;\">&quot;50%&quot;</span><span style=\"color:#f8f8f2;\">, Stories {{}} }}\n</span><span style=\"color:#f8f8f2;\">            div {{ width: </span><span style=\"color:#ffee99;\">&quot;50%&quot;</span><span style=\"color:#f8f8f2;\">, Preview {{}} }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Stories</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> stories </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#66d9ef;\">get_stories</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">10</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match &amp;*</span><span style=\"color:#f8f8f2;\">stories.</span><span style=\"color:#66d9ef;\">read_unchecked</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(list)) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            div {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> story </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> list {{\n</span><span style=\"color:#f8f8f2;\">                    StoryListing {{ story: story.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(err)) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{</span><span style=\"color:#ffee99;\">&quot;An error occurred while fetching stories {{err}}&quot;</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{</span><span style=\"color:#ffee99;\">&quot;Loading items&quot;</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">resolve_story</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">full_story</span><span style=\"color:#f8f8f2;\">: Signal&lt;Option&lt;StoryPageData&gt;&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">preview_state</span><span style=\"color:#f8f8f2;\">: Signal&lt;PreviewState&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">story_id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(cached) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> full_story.</span><span style=\"color:#66d9ef;\">as_ref</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">preview_state.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">PreviewState::Loaded(cached.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">return</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">preview_state.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">PreviewState::Loading;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Ok</span><span style=\"color:#f8f8f2;\">(story) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">get_story</span><span style=\"color:#f8f8f2;\">(story_id).await {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">preview_state.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">PreviewState::Loaded(story.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">full_story.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(story);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">StoryListing</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">story</span><span style=\"color:#f8f8f2;\">: ReadOnlySignal&lt;StoryItem&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> preview_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">consume_context::&lt;Signal&lt;PreviewState&gt;&gt;();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> StoryItem {{\n</span><span style=\"color:#f8f8f2;\">        title,\n</span><span style=\"color:#f8f8f2;\">        url,\n</span><span style=\"color:#f8f8f2;\">        by,\n</span><span style=\"color:#f8f8f2;\">        score,\n</span><span style=\"color:#f8f8f2;\">        time,\n</span><span style=\"color:#f8f8f2;\">        kids,\n</span><span style=\"color:#f8f8f2;\">        id,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">..\n</span><span style=\"color:#f8f8f2;\">    }} </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">story</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> full_story </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> url </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> url.</span><span style=\"color:#66d9ef;\">as_deref</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap_or_default</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> hostname </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> url\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;https://&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;http://&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;www.&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> score </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{score}} {{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> score </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#ffee99;\">&quot; point&quot; </span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#ffee99;\">&quot; points&quot; </span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}} {{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        kids.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> kids.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot; comment&quot;\n</span><span style=\"color:#f8f8f2;\">        }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot; comments&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> time </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> time.</span><span style=\"color:#66d9ef;\">format</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;%D %l:%M %p&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            position: </span><span style=\"color:#ffee99;\">&quot;relative&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            onmouseenter: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">_event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#66d9ef;\">resolve_story</span><span style=\"color:#f8f8f2;\">(full_story, preview_state, id) }},\n</span><span style=\"color:#f8f8f2;\">            div {{ font_size: </span><span style=\"color:#ffee99;\">&quot;1.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                a {{\n</span><span style=\"color:#f8f8f2;\">                    href: url,\n</span><span style=\"color:#f8f8f2;\">                    onfocus: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">_event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#66d9ef;\">resolve_story</span><span style=\"color:#f8f8f2;\">(full_story, preview_state, id) }},\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#ffee99;\">&quot;{{title}}&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">                a {{\n</span><span style=\"color:#f8f8f2;\">                    color: </span><span style=\"color:#ffee99;\">&quot;gray&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    href: </span><span style=\"color:#ffee99;\">&quot;https://news.ycombinator.com/from?site={{hostname}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    text_decoration: </span><span style=\"color:#ffee99;\">&quot;none&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#ffee99;\">&quot; ({{hostname}})&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            div {{ display: </span><span style=\"color:#ffee99;\">&quot;flex&quot;</span><span style=\"color:#f8f8f2;\">, flex_direction: </span><span style=\"color:#ffee99;\">&quot;row&quot;</span><span style=\"color:#f8f8f2;\">, color: </span><span style=\"color:#ffee99;\">&quot;gray&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                div {{ </span><span style=\"color:#ffee99;\">&quot;{{score}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                div {{ padding_left: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;by {{by}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                div {{ padding_left: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;{{time}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                div {{ padding_left: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;{{comments}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Debug)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">PreviewState {{\n</span><span style=\"color:#f8f8f2;\">    Unset,\n</span><span style=\"color:#f8f8f2;\">    Loading,\n</span><span style=\"color:#f8f8f2;\">    Loaded(StoryPageData),\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Preview</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> preview_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">consume_context::&lt;Signal&lt;PreviewState&gt;&gt;();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match </span><span style=\"color:#66d9ef;\">preview_state</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        PreviewState::Unset </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{</span><span style=\"color:#ffee99;\">&quot;Hover over a story to preview it here&quot;</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">        PreviewState::Loading </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{</span><span style=\"color:#ffee99;\">&quot;Loading...&quot;</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">        PreviewState::Loaded(story) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            rsx! {{\n</span><span style=\"color:#f8f8f2;\">                div {{ padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    div {{ font_size: </span><span style=\"color:#ffee99;\">&quot;1.5rem&quot;</span><span style=\"color:#f8f8f2;\">, a {{ href: story.item.url, </span><span style=\"color:#ffee99;\">&quot;{{story.item.title}}&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">                    div {{ dangerous_inner_html: story.item.text }}\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> comment </span><span style=\"color:#f92672;\">in &amp;</span><span style=\"color:#f8f8f2;\">story.comments {{\n</span><span style=\"color:#f8f8f2;\">                        Comment {{ comment: comment.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Comment</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">comment</span><span style=\"color:#f8f8f2;\">: CommentData) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            div {{ color: </span><span style=\"color:#ffee99;\">&quot;gray&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;by {{comment.by}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            div {{ dangerous_inner_html: </span><span style=\"color:#ffee99;\">&quot;{{comment.text}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> kid </span><span style=\"color:#f92672;\">in &amp;</span><span style=\"color:#f8f8f2;\">comment.sub_comments {{\n</span><span style=\"color:#f8f8f2;\">                Comment {{ comment: kid.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Define the Hackernews API and types\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">chrono::{{DateTime, Utc}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">futures::future::join_all;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">serde::{{Deserialize, Serialize}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">BASE_API_URL</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;https://hacker-news.firebaseio.com/v0/&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">ITEM_API</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;item/&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">USER_API</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;user/&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">COMMENT_DEPTH</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_story_preview</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">) -&gt; Result&lt;StoryItem, reqwest::Error&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> url </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}{{}}{{}}</span><span style=\"color:#ffee99;\">.json&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">BASE_API_URL</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">ITEM_API</span><span style=\"color:#f8f8f2;\">, id);\n</span><span style=\"color:#f8f8f2;\">    reqwest::get(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">url).await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">json</span><span style=\"color:#f8f8f2;\">().await\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_stories</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">) -&gt; Result&lt;Vec&lt;StoryItem&gt;, reqwest::Error&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> url </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">topstories.json&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">BASE_API_URL</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> stories_ids </span><span style=\"color:#f92672;\">= &amp;</span><span style=\"color:#f8f8f2;\">reqwest::get(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">url).await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">.json::&lt;Vec&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">&gt;&gt;().await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#f92672;\">..</span><span style=\"color:#f8f8f2;\">count];\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> story_futures </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> stories_ids[</span><span style=\"color:#f92672;\">..</span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">::min(stories_ids.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">(), count)]\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">story_id</span><span style=\"color:#f8f8f2;\">| </span><span style=\"color:#66d9ef;\">get_story_preview</span><span style=\"color:#f8f8f2;\">(story_id));\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#66d9ef;\">join_all</span><span style=\"color:#f8f8f2;\">(story_futures)\n</span><span style=\"color:#f8f8f2;\">        .await\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">into_iter</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">filter_map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">story</span><span style=\"color:#f8f8f2;\">| story.</span><span style=\"color:#66d9ef;\">ok</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">collect</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">StoryPageData {{\n</span><span style=\"color:#f8f8f2;\">    #[serde(flatten)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">item: StoryItem,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">comments: Vec&lt;CommentData&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">CommentData {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">id: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">/// there will be no by field if the comment was deleted\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">by: String,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">text: String,\n</span><span style=\"color:#f8f8f2;\">    #[serde(with = &quot;chrono::serde::ts_seconds&quot;)]\n</span><span style=\"color:#f8f8f2;\">    pub time: DateTime&lt;Utc&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">kids: Vec&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">sub_comments: Vec&lt;CommentData&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> r#type: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">StoryItem {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">id: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">title: String,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">url: Option&lt;String&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">text: Option&lt;String&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">by: String,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">score: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">descendants: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    #[serde(with = &quot;chrono::serde::ts_seconds&quot;)]\n</span><span style=\"color:#f8f8f2;\">    pub time: DateTime&lt;Utc&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[serde(default)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">kids: Vec&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> r#type: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_story</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">) -&gt; Result&lt;StoryPageData, reqwest::Error&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> url </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}{{}}{{}}</span><span style=\"color:#ffee99;\">.json&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">BASE_API_URL</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">ITEM_API</span><span style=\"color:#f8f8f2;\">, id);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> story </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">reqwest::get(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">url).await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">.json::&lt;StoryPageData&gt;().await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comment_futures </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> story.item.kids.</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">| </span><span style=\"color:#66d9ef;\">get_comment</span><span style=\"color:#f8f8f2;\">(id));\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">join_all</span><span style=\"color:#f8f8f2;\">(comment_futures)\n</span><span style=\"color:#f8f8f2;\">        .await\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">into_iter</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">filter_map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">c</span><span style=\"color:#f8f8f2;\">| c.</span><span style=\"color:#66d9ef;\">ok</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">collect</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    story.comments </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> comments;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(story)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[async_recursion::async_recursion(?Send)]\n</span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_comment_with_depth</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">depth</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">) -&gt; Result&lt;CommentData, reqwest::Error&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> url </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}{{}}{{}}</span><span style=\"color:#ffee99;\">.json&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">BASE_API_URL</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">ITEM_API</span><span style=\"color:#f8f8f2;\">, id);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> comment </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">reqwest::get(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">url).await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">.json::&lt;CommentData&gt;().await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> depth </span><span style=\"color:#f92672;\">&gt; </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> sub_comments_futures </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> comment\n</span><span style=\"color:#f8f8f2;\">            .kids\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">story_id</span><span style=\"color:#f8f8f2;\">| </span><span style=\"color:#66d9ef;\">get_comment_with_depth</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">story_id, depth </span><span style=\"color:#f92672;\">- </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">        comment.sub_comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">join_all</span><span style=\"color:#f8f8f2;\">(sub_comments_futures)\n</span><span style=\"color:#f8f8f2;\">            .await\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">into_iter</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">filter_map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">c</span><span style=\"color:#f8f8f2;\">| c.</span><span style=\"color:#66d9ef;\">ok</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">collect</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(comment)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_comment</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">comment_id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">) -&gt; Result&lt;CommentData, reqwest::Error&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">get_comment_with_depth</span><span style=\"color:#f8f8f2;\">(comment_id, </span><span style=\"color:#ff80f4;\">COMMENT_DEPTH</span><span style=\"color:#f8f8f2;\">).await\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_complete.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn EssentialsIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "The essentials section will guide you through key concepts in Dioxus:" }
        ul {
            li {
                p {
                    a { href: "rsx", "Building UIs with RSX" }
                    " will teach you how to define html inside your Dioxus app with rsx."
                }
            }
            li {
                p {
                    a { href: "lifecycle", "Component Lifecycle" }
                    " teaches you about the lifecycle of components along with the hooks you need to run code when the component is first created, mounted, and removed."
                }
            }
            li {
                p {
                    a { href: "state", "Managing State" }
                    " guides you through how state works in Dioxus. It will teach you how to create state with "
                    code { "use_signal" }
                    ", derive state with "
                    code { "use_memo" }
                    ", and integrate state with asynchronous tasks with "
                    code { "use_resource" }
                    ". Along the way, you will learn about you can use reactivity to declaratively describe your UI."
                }
            }
            li {
                p {
                    a { href: "breaking", "Breaking Out" }
                    " will teach you how to break out of Dioxus' rendering model to run JavaScript or interact with the DOM directly with "
                    code { "web-sys" }
                    "."
                }
            }
        }
    }
}
#[component(no_case_check)]
pub fn EssentialsRsxIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "building-uis-with-rsx",
            a { href: "#building-uis-with-rsx", class: "header", "Building UIs with RSX" }
        }
        p {
            "Dioxus renders to HTML, if you are not familiar with HTML, this guide will help you get started with the basics. For more detail, the "
            a { href: "https://developer.mozilla.org/en-US/docs/Web/HTML", "MDN docs" }
            " are a great resource."
        }
        h2 { id: "text-nodes",
            a { href: "#text-nodes", class: "header", "Text Nodes" }
        }
        p { "Any content surrounded by quotes is rendered as a text node in rsx:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;Hello world&quot;\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::Text {} }
        p {
            "You can include formatted segments inside of the text just like the  "
            code { "format!" }
            " macro:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> user </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| User {{\n</span><span style=\"color:#f8f8f2;\">    name: </span><span style=\"color:#ffee99;\">&quot;Dioxus&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Unlike the format macro, you can include many expressions inline in the formatted text\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;Hello {{user.read().name}}&quot;\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::FormattedText {} }
        h2 { id: "elements",
            a { href: "#elements", class: "header", "Elements" }
        }
        p {
            "The most basic building block of HTML is an element. In rsx, you can create elements with the name and then curly braces. One of the most common elements is the  "
            code { "input" }
            " element. The input element creates an interactive input box:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    input {{}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::Input {} }
        blockquote {
            p { "Bonus: web components" }
            CodeBlock {
                contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    my</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">web</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">component {{}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
                name: "building_uis_with_rsx.rs".to_string(),
            }
        }
        h2 { id: "attributes",
            a { href: "#attributes", class: "header", "Attributes" }
        }
        p {
            "Attributes provide extra information about an element. You can specify attributes in dioxus inside an element's braces by typing the name of the attribute, a colon, and then the value (typically a formatted string). We can use an attribute to set the  "
            code { "type" }
            " of an input element. The default type is  "
            code { "text" }
            " which shows a text input box, but we can set it to  "
            code { "number" }
            " to only accept numbers:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    input {{ </span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;number&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::InputType {} }
        p {
            "Just like text nodes, attributes can include formatted segments. We can set the value of the input element to a signal to control it:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;Hello world&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    input {{ value: </span><span style=\"color:#ffee99;\">&quot;{{value}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::InputValue {} }
        h3 { id: "conditional-attributes",
            a { href: "#conditional-attributes", class: "header", "Conditional Attributes" }
        }
        p {
            "You can conditionally set an attribute by setting the attribute value to an unterminated if statement. If the if statement evaluates to true, the attribute will be set:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> number_type </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">false</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    input {{ </span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">number_type</span><span style=\"color:#f8f8f2;\">() {{ </span><span style=\"color:#ffee99;\">&quot;number&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::InputDisabled {} }
        h2 { id: "event-listeners",
            a { href: "#event-listeners", class: "header", "Event Listeners" }
        }
        p {
            "Event listeners allow you to respond to user input. In rsx, event handlers always start with  "
            code { "on" }
            ". The syntax is the same as normal attributes, but event handlers only accept a closure that responds to the event. We can attach an event listener to the  "
            code { "oninput" }
            " event of the input element to listen for changes to the input:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;Hello world&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    input {{\n</span><span style=\"color:#f8f8f2;\">        oninput: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> value.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(event.</span><span style=\"color:#66d9ef;\">value</span><span style=\"color:#f8f8f2;\">()),\n</span><span style=\"color:#f8f8f2;\">        value: </span><span style=\"color:#ffee99;\">&quot;{{value}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::OnInput {} }
        h2 { id: "children",
            a { href: "#children", class: "header", "Children" }
        }
        p {
            "You can add children to an element after all attributes and event listeners. Elements can accept text, components or other elements as children. We can add a  "
            code { "div" }
            " element around our input to center it:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    div {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// display sets the layout mode of the element\n</span><span style=\"color:#f8f8f2;\">        display: </span><span style=\"color:#ffee99;\">&quot;flex&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// justify-content centers the element horizontally\n</span><span style=\"color:#f8f8f2;\">        justify_content: </span><span style=\"color:#ffee99;\">&quot;center&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        input {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;number&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::InputChildren {} }
        h2 { id: "loops",
            a { href: "#loops", class: "header", "Loops" }
        }
        p {
            "You can insert for loops directly in rsx. The body of the loop accepts any number of children that will be rendered with each iteration of the loop. The  "
            code { "ul" }
            " element in html renders an unordered list with any number of  "
            code { "li" }
            " (list item) elements. We can use those two elements to render a list of items in a loop:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> items </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| vec![</span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Dioxus&quot;</span><span style=\"color:#f8f8f2;\">]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ul {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> item </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> items.</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            li {{ </span><span style=\"color:#ffee99;\">&quot;{{item}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::ForLoop {} }
        p {
            "Each item in your list should have unique value that is stable across rerenders called a key. Keys are used to identify how items move while diffing. Without keys, it is easy to accidentally lose or move state when you reorder items in a list. We can add keys to our list items by using the  "
            code { "key" }
            " attribute:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> items </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| vec![</span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Dioxus&quot;</span><span style=\"color:#f8f8f2;\">]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ul {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> item </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> items.</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            li {{ key: </span><span style=\"color:#ffee99;\">&quot;{{item}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;{{item}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::KeyedForLoop {} }
        h2 { id: "if-statements",
            a { href: "#if-statements", class: "header", "If Statements" }
        }
        p {
            "You can also use if/else statements in rsx. Each branch of the if statement accepts child nodes that will be rendered if the condition is true. We can use the  "
            code { "if" }
            " statement to conditionally render a login screen:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> logged_in </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">false</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    div {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">logged_in</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;You are logged in&quot;\n</span><span style=\"color:#f8f8f2;\">        }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;You are not logged in&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::IfStatement {} }
    }
}
#[component(no_case_check)]
pub fn EssentialsLifecycleIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "component-lifecycle",
            a { href: "#component-lifecycle", class: "header", "Component Lifecycle" }
        }
        h2 { id: "initializing-state-with",
            a { href: "#initializing-state-with", class: "header", "Initializing State with " }
            code { "use_hook" }
        }
        p {
            code { "use_hook" }
            " lets you create new state for your component. The closure you pass to  "
            code { "use_hook" }
            " will be called once the first time the component is rendered. Every time the component is re-rendered, the value that was created the first run will be re-used."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">UseHook</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// The closure that is passed to use_hook will be called once the first time the component is rendered\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> random_number </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> new_random_number </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">random_number</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        log!(</span><span style=\"color:#ffee99;\">&quot;{{new_random_number}}&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        new_random_number\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Random {{random_number}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_lifecycle.rs".to_string(),
        }
        DemoFrame { component_lifecycle::UseHookDemo {} }
        h2 { id: "rerendering",
            a { href: "#rerendering", class: "header", "Rerendering" }
        }
        p {
            "You can use "
            a { href: "../reference/reactivity", "tracked values" }
            " to re-render your component whenever a value changes. "
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Rerenders</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    log!(</span><span style=\"color:#ffee99;\">&quot;Rerendering parent component with {{}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">count.</span><span style=\"color:#66d9ef;\">peek</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Increment&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Since we read count here, the component will rerender when count changes\n</span><span style=\"color:#f8f8f2;\">        Count {{ current_count: </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// If the count prop changes, the component will rerender\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Count</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">current_count</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    log!(</span><span style=\"color:#ffee99;\">&quot;Rerendering child component with {{current_count}}&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;The count is {{current_count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_lifecycle.rs".to_string(),
        }
        DemoFrame { component_lifecycle::RerenderDemo {} }
        h3 { id: "-dont-mutate-state-in-the-body-of-a-component",
            a {
                href: "#-dont-mutate-state-in-the-body-of-a-component",
                class: "header",
                "⚠\u{fe0f} Don't mutate state in the body of a component"
            }
        }
        p {
            "You should avoid changing state in the body of a component. If you read and write to state in the body of a component, you can cause an infinite loop as the component tries to rerender because of the change which triggers another state change."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Bad</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// ❌ Don&#39;t mutate state in the body of the component.\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// It can easily cause an infinite loop!\n</span><span style=\"color:#f8f8f2;\">    count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ </span><span style=\"color:#ffee99;\">&quot;{{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_lifecycle.rs".to_string(),
        }
        p {
            "Instead, derive state with  "
            code { "use_memo" }
            ",  "
            code { "use_resource" }
            ", or mutate state in a effect."
        }
        h2 { id: "using-effects",
            a { href: "#using-effects", class: "header", "Using Effects" }
        }
        p {
            "You can use "
            a { href: "../reference/reactivity", "effects" }
            " to run code whenever a component is rendered."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Effect</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Effects run after the component is rendered\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// You can use them to read or modify the rendered component\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        log!(</span><span style=\"color:#ffee99;\">&quot;Effect ran&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">        document::eval(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">format!(\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;document.getElementById(&#39;effect-output&#39;).innerText = &#39;Effect ran&#39;&quot;\n</span><span style=\"color:#f8f8f2;\">        ));\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ id: </span><span style=\"color:#ffee99;\">&quot;effect-output&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;This will be changed by the effect&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_lifecycle.rs".to_string(),
        }
        DemoFrame { component_lifecycle::EffectDemo {} }
        h2 { id: "cleaning-up-components-with-drop",
            a { href: "#cleaning-up-components-with-drop", class: "header",
                "Cleaning Up Components with Drop"
            }
        }
        p {
            "Before a component is dropped, it will drop all of its hooks. You can use this drop behavior to clean up any resources that your component is using. If you just need the drop effect, you can use the "
            a { href: "https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_drop.html",
                code { "use_drop" }
            }
            " hook."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">TogglesChild</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> show </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> show.</span><span style=\"color:#66d9ef;\">toggle</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#ffee99;\">&quot;Toggle&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">show</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            Child {{}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// You can use the use_drop hook to clean up any resources\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_drop</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        log!(</span><span style=\"color:#ffee99;\">&quot;Child dropped&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Child&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_lifecycle.rs".to_string(),
        }
        DemoFrame { component_lifecycle::DropDemo {} }
    }
}
#[component(no_case_check)]
pub fn EssentialsStateIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "managing-state",
            a { href: "#managing-state", class: "header", "Managing State" }
        }
        p {
            "In Dioxus, your app is defined as a function of the current state. As the state changes, the parts of your app that depend on that state will automatically re-run. Reactivity automatically tracks state and updates derived state in your application."
        }
        h2 { id: "creating-state",
            a { href: "#creating-state", class: "header", "Creating State" }
        }
        p {
            "You can create mutable state in Dioxus with Signals. Signals are tracked values that automatically update your app when you change them. They form the skeleton of your app's state from which you can derive other state. Signals are often driven directly from user input through event handlers or async tasks."
        }
        p {
            "You can create a signal with the  "
            code { "use_signal" }
            " hook:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        p {
            "Once you have your signal, you can clone it by calling the signal like a function or get a reference to the inner value with the  "
            code { ".read()" }
            " method:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Call the signal like a function to clone the current value\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">signal</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#8c8c8c;\">// get a reference to the inner value with the .read() method\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= &amp;</span><span style=\"color:#f8f8f2;\">signal.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#8c8c8c;\">// or use one of the traits implemented for Signal like Display\n</span><span style=\"color:#f8f8f2;\">log!(</span><span style=\"color:#ffee99;\">&quot;{{signal}}&quot;</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        p {
            "Finally, you can set the value of the signal with the  "
            code { ".set()" }
            " method or get a mutable reference to the inner value with the  "
            code { ".write()" }
            " method:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Set the value from the signal\n</span><span style=\"color:#f8f8f2;\">signal.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#8c8c8c;\">// get a mutable reference to the inner value with the .write() method\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> value: </span><span style=\"color:#f92672;\">&amp;mut </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= &amp;mut</span><span style=\"color:#f8f8f2;\"> signal.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">value </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        h3 { id: "reactive-scopes",
            a { href: "#reactive-scopes", class: "header", "Reactive Scopes" }
        }
        p {
            "The simplest reactive primitive in Dioxus is the  "
            code { "use_effect" }
            " hook. It creates a closure that is run any time a tracked value that is run inside the closure changes."
        }
        p {
            "Any value you read inside the closure will become a dependency of the effect. If the value changes, the effect will rerun."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Effect</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// use_signal creates a tracked value called count\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// When we read count, it becomes a dependency of the effect\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> current_count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Whenever count changes, the effect will rerun\n</span><span style=\"color:#f8f8f2;\">        log!(</span><span style=\"color:#ffee99;\">&quot;{{current_count}}&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Increment&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Count is {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::EffectDemo {} }
        h3 { id: "derived-state",
            a { href: "#derived-state", class: "header", "Derived State" }
        }
        p {
            code { "use_memo" }
            " is a reactive primitive that lets you derive state from any tracked value. It takes a closure that computes the new state and returns a tracked value with the current state of the memo. Any time a dependency of the memo changes, the memo will rerun."
        }
        p {
            "The value you return from the closure will only change when the output of the closure changes ( "
            code { "PartialEq" }
            " between the old and new value returns false)."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Memo</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// use_memo creates a tracked value that is derived from count\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Since we read count inside the closure, it becomes a dependency of the memo\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Whenever count changes, the memo will rerun\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> half_count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">/ </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// half_count is itself a tracked value\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// When we read half_count, it becomes a dependency of the effect\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// and the effect will rerun when half_count changes\n</span><span style=\"color:#f8f8f2;\">        log!(</span><span style=\"color:#ffee99;\">&quot;{{half_count}}&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Increment&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Count is {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Half count is {{half_count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::MemoDemo {} }
        h3 { id: "derived-async-state",
            a { href: "#derived-async-state", class: "header", "Derived Async State" }
        }
        p {
            code { "use_resource" }
            " is a reactive primitive that lets you derive state from any async closure. It takes an async closure that computes the new state and returns a tracked value with the current state of the resource. Any time a dependency of the resource changes, the resource will rerun."
        }
        p {
            "The value you return from the closure will only change when the state of the future changes. Unlike  "
            code { "use_memo" }
            ", the resource's output is not memoized with  "
            code { "PartialEq" }
            "."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Resource</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// use_resource creates a tracked value that is derived from count\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Since we read count inside the closure, it becomes a dependency of the resource\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Whenever count changes, the resource will rerun\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> half_count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can do async work inside resources\n</span><span style=\"color:#f8f8f2;\">        gloo_timers::future::TimeoutFuture::new(</span><span style=\"color:#ff80f4;\">100</span><span style=\"color:#f8f8f2;\">).await;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">/ </span><span style=\"color:#ff80f4;\">2\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// half_count is itself a tracked value\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// When we read half_count, it becomes a dependency of the effect\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// and the effect will rerun when half_count changes\n</span><span style=\"color:#f8f8f2;\">        log!(</span><span style=\"color:#ffee99;\">&quot;{{:?}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#66d9ef;\">half_count</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Change Signal&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Count is {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Half count is {{half_count():?}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::ResourceDemo {} }
        h3 { id: "derived-ui",
            a { href: "#derived-ui", class: "header", "Derived UI" }
        }
        p {
            "Components are functions that return some UI. They memorize the output of the function just like memos. Components keep track of any dependencies you read inside the component and rerun when those dependencies change."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Component</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Change Signal&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Since we read count inside Component, it becomes a dependency of Component\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Whenever count changes, Component will rerun\n</span><span style=\"color:#f8f8f2;\">        Count {{ count: </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Components automatically memorize their props. If the props change, Count will rerun\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Count</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Count: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::ComponentDemo {} }
        h3 { id: "working-with-untracked-state",
            a { href: "#working-with-untracked-state", class: "header",
                "Working with Untracked State"
            }
        }
        p {
            "Most of the state in your app will be tracked values. All built in hooks return tracked values, and we encourage custom hooks to do the same. However, there are times when you need to work with untracked state. For example, you may receive a raw untracked value in props. When you read an untracked value inside a reactive context, it will not subscribe to the value:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Component</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Change Signal&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        Count {{ count: </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// The count reruns the component when it changes, but it is not a tracked value\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Count</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// When you read count inside the memo, it does not subscribe to the count signal\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// because the value is not reactive\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> double_count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">* </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Double count: {{double_count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::NonReactiveDemo {} }
        p {
            "You can start tracking raw state with the  "
            code { "use_reactive" }
            " hook. This hook takes a tuple of dependencies and returns a reactive closure. When the closure is called in a reactive context, it will track subscribe to the dependencies and rerun the closure when the dependencies change."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Count</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// You can manually track a non-reactive value with the use_reactive hook\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> double_count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Use reactive takes a tuple of dependencies and returns a reactive closure\n</span><span style=\"color:#f8f8f2;\">        use_reactive!(|(</span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">,)| count </span><span style=\"color:#f92672;\">* </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Double count: {{double_count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::UseReactiveDemo {} }
        h3 { id: "making-props-reactive",
            a { href: "#making-props-reactive", class: "header", "Making Props Reactive" }
        }
        p {
            "To avoid loosing reactivity with props, we recommend you wrap any props you want to track in a  "
            code { "ReadOnlySignal" }
            ". Dioxus will automatically convert  "
            code { "T" }
            " into  "
            code { "ReadOnlySignal<T>" }
            " when you pass props to the component. This will ensure your props are tracked and rerun any state you derive in the component:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// You can track props by wrapping the type in a ReadOnlySignal\n</span><span style=\"color:#8c8c8c;\">// Dioxus will automatically convert T into ReadOnlySignal&lt;T&gt; when you pass\n</span><span style=\"color:#8c8c8c;\">// props to the component\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Count</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">: ReadOnlySignal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Then when you read count inside the memo, it subscribes to the count signal\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> double_count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">* </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Double count: {{double_count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::MakingPropsReactiveDemo {} }
        h2 { id: "moving-around-state",
            a { href: "#moving-around-state", class: "header", "Moving Around State" }
        }
        p {
            "As you create signals and derived state in your app, you will need to move around that state between components. Dioxus provides three different ways to pass around state:"
        }
        h3 { id: "passing-props",
            a { href: "#passing-props", class: "header", "Passing props" }
        }
        p {
            "You can pass your values through component "
            a { href: "./component_props", "props" }
            ". This should be your default when passing around state. It is the most explicit and local to your component. Use this until it gets annoying to pass around the value:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ParentComponent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Count is {{count}}&quot;\n</span><span style=\"color:#f8f8f2;\">        IncrementButton {{\n</span><span style=\"color:#f8f8f2;\">            count\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">IncrementButton</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">: Signal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Increment&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "moving_state_around.rs".to_string(),
        }
        DemoFrame { moving_state_around::PassingProps {} }
        h3 { id: "passing-context",
            a { href: "#passing-context", class: "header", "Passing context" }
        }
        p {
            "If you need a slightly more powerful way to pass around state, you can use the context API."
        }
        p {
            "The context API lets you pass state from a parent component to all children. This is useful if you want to share state between many components. You can insert a unique type into the context with the "
            a { href: "https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context_provider.html",
                code { "use_context_provider" }
            }
            " hook in the parent component. Then you can access the context in any child component with the "
            a { href: "https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context.html",
                code { "use_context" }
            }
            " hook."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Clone, Copy)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">MyState {{\n</span><span style=\"color:#f8f8f2;\">    count: Signal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ParentComponent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Use context provider provides an unique type to all children of this component\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_context_provider</span><span style=\"color:#f8f8f2;\">(|| MyState {{\n</span><span style=\"color:#f8f8f2;\">        count: Signal::new(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Count is {{state.count}}&quot;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// IncrementButton will have access to the count without explicitly passing it through props\n</span><span style=\"color:#f8f8f2;\">        IncrementButton {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">IncrementButton</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Use context gets the value from a parent component\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">use_context::&lt;MyState&gt;().count;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Increment&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "moving_state_around.rs".to_string(),
        }
        DemoFrame { moving_state_around::PassingContext {} }
        p {
            "This is slightly less explicit than passing it as a prop, but it is still local to the component. This is really great if you want state that is global to part of your app. It lets you create multiple global-ish states while still making state different when you reuse components. If I create a new  "
            code { "ParentComponent" }
            ", it will have a new  "
            code { "MyState" }
            "."
        }
        h3 { id: "using-globals",
            a { href: "#using-globals", class: "header", "Using globals" }
        }
        p {
            "Finally, if you have truly global state, you can put your state in a  "
            code { "Global<T>" }
            " static. This is useful if you want to share state with your whole app:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#8c8c8c;\">// Globals are created the first time you access them with the closure you pass to Global::new\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">: GlobalSignal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Global::new(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ParentComponent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Count is {{COUNT}}&quot;\n</span><span style=\"color:#f8f8f2;\">        IncrementButton {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">IncrementButton</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// You don&#39;t need to pass anything around or get anything out of the context because COUNT is global\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| *</span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Increment&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "moving_state_around.rs".to_string(),
        }
        DemoFrame { moving_state_around::UsingGlobals {} }
        p {
            "Global state can be very ergonomic if your state is truly global, but you shouldn't use it if you need state to be different for different instances of your component. If I create another  "
            code { "IncrementButton" }
            " it will use the same  "
            code { "COUNT" }
            ". Libraries should generally avoid this to make components more reusable."
        }
        blockquote {
            p {
                "Note: Even though it is in a static,  "
                code { "COUNT" }
                " will be different for each app instance so you don't need to worry about state mangling when multiple instances of your app are running on the server"
            }
        }
    }
}
#[component(no_case_check)]
pub fn EssentialsBreakingIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "breaking-out-of-dioxus",
            a { href: "#breaking-out-of-dioxus", class: "header", "Breaking Out of Dioxus" }
        }
        p {
            "Dioxus is makes it easy to build reactive user interfaces. However, there are some cases where you may need to break out of the reactive paradigm to interact with the DOM directly."
        }
        h2 { id: "interacting-with-javascript-with",
            a { href: "#interacting-with-javascript-with", class: "header",
                "Interacting with JavaScript with "
            }
            code { "eval" }
            " and "
            code { "web-sys" }
        }
        p {
            "Dioxus exposes a limited number of "
            a { href: "https://developer.mozilla.org/en-US/docs/Web/API", "web apis" }
            " with a nicer interface. If you need access to more APIs, you can use the "
            code { "eval" }
            " function to run JavaScript in the browser."
        }
        p { "For example, you can use the eval function to read the domain of the current page:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Eval</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> domain </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::new);\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// When you click the button, some javascript will run in the browser\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// to read the domain and set the signal\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                domain.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(document::eval(</span><span style=\"color:#ffee99;\">&quot;return document.domain&quot;</span><span style=\"color:#f8f8f2;\">).await.</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Read Domain&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Current domain: {{domain}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "breaking_out.rs".to_string(),
        }
        DemoFrame { breaking_out::Eval {} }
        p {
            "If you are only targeting web, you can also use the "
            a { href: "https://crates.io/crates/web-sys",
                code { "web-sys" }
            }
            " crate for typed access to the web APIs. Here is what reading the domain looks like with web-sys:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">::web_sys::window;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">wasm_bindgen::JsCast;\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">WebSys</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> domain </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::new);\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// When you click the button, we use web-sys to read the domain and a signal\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                domain\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"color:#66d9ef;\">window</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">document</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                            .dyn_into::&lt;::web_sys::HtmlDocument&gt;()\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">domain</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    );\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Read Domain&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Current domain: {{domain}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "breaking_out.rs".to_string(),
        }
        DemoFrame { breaking_out::WebSys {} }
        h2 { id: "synchronizing-dom-updates-with",
            a { href: "#synchronizing-dom-updates-with", class: "header",
                "Synchronizing DOM updates with "
            }
            code { "use_effect" }
        }
        p {
            "If you do need to interact with the DOM directly, you should do so in a  "
            code { "use_effect" }
            " hook. This hook will run after the component is rendered and all of the Dioxus UI has been rendered. You can read or modify the DOM in this hook."
        }
        p {
            "For example, you can use the  "
            code { "use_effect" }
            " hook to write to a canvas element after it is created:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Canvas</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Effects are reactive like memos, and resources. If you read a value inside the effect, the effect will rerun when that value changes\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> count.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can use the count value to update the DOM manually\n</span><span style=\"color:#f8f8f2;\">        document::eval(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">format!(\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;var c = document.getElementById(&quot;dioxus-canvas&quot;);\n</span><span style=\"color:#ffee99;\">var ctx = c.getContext(&quot;2d&quot;);\n</span><span style=\"color:#ffee99;\">ctx.clearRect(0, 0, c.width, c.height);\n</span><span style=\"color:#ffee99;\">ctx.font = &quot;30px Arial&quot;;\n</span><span style=\"color:#ffee99;\">ctx.fillText(&quot;</span><span style=\"color:#ff80f4;\">{{count}}</span><span style=\"color:#ffee99;\">&quot;, 10, 50);&quot;#\n</span><span style=\"color:#f8f8f2;\">        ));\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// When you click the button, count will be incremented and the effect will rerun\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Increment&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        canvas {{ id: </span><span style=\"color:#ffee99;\">&quot;dioxus-canvas&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "breaking_out.rs".to_string(),
        }
        DemoFrame { breaking_out::Canvas {} }
        h2 { id: "getting-access-to-elements-with",
            a { href: "#getting-access-to-elements-with", class: "header",
                "Getting access to elements with "
            }
            code { "onmounted" }
        }
        p {
            "If you need a handle to an element that is rendered by dioxus, you can use the  "
            code { "onmounted" }
            " event. This event will fire after the element is first mounted to the DOM. It returns a live reference to the element with some methods to interact with it."
        }
        p {
            "You can use the onmounted event to do things like focus or scroll to an element after it is rendered:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">OnMounted</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> input_element </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ height: </span><span style=\"color:#ffee99;\">&quot;100px&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            button {{\n</span><span style=\"color:#f8f8f2;\">                class: </span><span style=\"color:#ffee99;\">&quot;focus:outline-2 focus:outline-blue-600 focus:outline-dashed&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// The onmounted event will run the first time the button element is mounted\n</span><span style=\"color:#f8f8f2;\">                onmounted: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">element</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> input_element.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(element.</span><span style=\"color:#66d9ef;\">data</span><span style=\"color:#f8f8f2;\">())),\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;First button&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">            button {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// When you click the button, if the button element has been mounted, we focus to that element\n</span><span style=\"color:#f8f8f2;\">                onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(header) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">input_element</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">_ =</span><span style=\"color:#f8f8f2;\"> header.</span><span style=\"color:#66d9ef;\">set_focus</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">).await;\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }},\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;Focus first button&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "breaking_out.rs".to_string(),
        }
        DemoFrame { breaking_out::OnMounted {} }
        h2 { id: "down-casting-web-sys-events",
            a { href: "#down-casting-web-sys-events", class: "header",
                "Down casting web sys events"
            }
        }
        p {
            "Dioxus provides platform agnostic wrappers over each event type. These wrappers are often nicer to interact with than the raw event types, but they can be more limited. If you are targeting web, you can downcast the event with the  "
            code { "as_web_event" }
            " method to get the underlying web-sys event:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Downcast</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> event_text </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            onmousemove: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                #[cfg(feature </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;web&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">                {{\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::web::WebEventExt;\n</span><span style=\"color:#f8f8f2;\">                    event_text.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(event.</span><span style=\"color:#66d9ef;\">as_web_event</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">movement_x</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;movement_x was {{event_text}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "breaking_out.rs".to_string(),
        }
        DemoFrame { breaking_out::Downcast {} }
    }
}
#[component(no_case_check)]
pub fn ReferenceIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "dioxus-reference",
            a { href: "#dioxus-reference", class: "header", "Dioxus Reference" }
        }
        p {
            "This Reference contains more detailed explanations for all concepts covered in the "
            a { href: "../guide",
                code { "guide" }
            }
            " and more."
        }
        h2 { id: "rendering",
            a { href: "#rendering", class: "header", "Rendering" }
        }
        ul {
            li {
                a { href: "rsx",
                    code { "RSX" }
                }
                " Rsx is a HTML-like macro that allows you to declare UI"
            }
            li {
                a { href: "components",
                    code { "Components" }
                }
                " Components are the building blocks of UI in Dioxus"
            }
            li {
                a { href: "component_props",
                    code { "Props" }
                }
                " Props allow you pass information to Components"
            }
            li {
                a { href: "event_handlers",
                    code { "Event Listeners" }
                }
                " Event listeners let you respond to user input"
            }
            li {
                a { href: "user_input",
                    code { "User Input" }
                }
                " How to handle User input in Dioxus"
            }
            li {
                a { href: "dynamic_rendering",
                    code { "Dynamic Rendering" }
                }
                " How to dynamically render data in Dioxus"
            }
        }
        h2 { id: "state",
            a { href: "#state", class: "header", "State" }
        }
        ul {
            li {
                a { href: "hooks",
                    code { "Hooks" }
                }
                " Hooks allow you to create components state"
            }
            li {
                a { href: "context",
                    code { "Context" }
                }
                " Context allows you to create state in a parent and consume it in children"
            }
            li {
                a { href: "router",
                    code { "Routing" }
                }
                " The router helps you manage the URL state"
            }
            li {
                a { href: "use_resource",
                    code { "Resource" }
                }
                " Use future allows you to create an async task and monitor it's state"
            }
            li {
                a { href: "use_coroutine",
                    code { "UseCoroutine" }
                }
                " Use coroutine helps you manage external state"
            }
            li {
                a { href: "spawn",
                    code { "Spawn" }
                }
                " Spawn creates an async task"
            }
        }
        h2 { id: "platforms",
            a { href: "#platforms", class: "header", "Platforms" }
        }
        ul {
            li {
                a { href: "choosing_a_web_renderer",
                    code { "Choosing a Web Renderer" }
                }
                " Overview of the different web renderers"
            }
            li {
                a { href: "desktop",
                    code { "Desktop" }
                }
                " Overview of desktop specific APIS"
            }
            li {
                a { href: "web",
                    code { "Web" }
                }
                " Overview of web specific APIS"
            }
            li {
                a { href: "fullstack",
                    code { "Fullstack" }
                }
                " Overview of Fullstack specific APIS"
                ul {
                    li {
                        a { href: "fullstack/server_functions",
                            code { "Server Functions" }
                        }
                        " Server functions make it easy to communicate between your server and client"
                    }
                    li {
                        a { href: "fullstack/extractors",
                            code { "Extractors" }
                        }
                        " Extractors allow you to get extra information out of the headers of a request"
                    }
                    li {
                        a { href: "fullstack/middleware",
                            code { "Middleware" }
                        }
                        " Middleware allows you to wrap a server function request or response"
                    }
                    li {
                        a { href: "fullstack/authentication",
                            code { "Authentication" }
                        }
                        " An overview of how to handle authentication with server functions"
                    }
                    li {
                        a { href: "fullstack/routing",
                            code { "Routing" }
                        }
                        " An overview of how to work with the router in the fullstack renderer"
                    }
                }
            }
            li {
                a { href: "ssr",
                    code { "SSR" }
                }
                " Overview of the SSR renderer"
            }
            li {
                a { href: "liveview",
                    code { "Liveview" }
                }
                " Overview of liveview specific APIS"
            }
        }
    }
}
#[component(no_case_check)]
pub fn RouterIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "introduction",
            a { href: "#introduction", class: "header", "Introduction" }
        }
        blockquote {
            p {
                "If you are not familiar with Dioxus itself, check out the "
                a { href: "../guide", "Dioxus guide" }
                " first."
            }
        }
        p {
            "Whether you are building a website, desktop app, or mobile app, splitting your app's views into \"pages\" can be an effective method for organization and maintainability."
        }
        p {
            "For this purpose, Dioxus provides a router. Use the  "
            code { "cargo add" }
            " command to add the dependency:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add dioxus</span><span style=\"color:#f92672;\">@</span><span style=\"color:#ff80f4;\">0.5</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features router</span></pre>\n" }
        p {
            "Then, add this to your  "
            code { "Dioxus.toml" }
            " (learn more about configuration "
            a { href: "../CLI/configure", "here" }
            "):"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[web.watcher]\n</span><span style=\"color:#f8f8f2;\">index_on_404 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">true</span></pre>\n" }
        blockquote {
            p {
                "This configuration only works when using  "
                code { "dx serve" }
                ". If you host your app in a different way (which you most likely do in production), you need to find out how to add a fallback 404 page to your app, and make it a copy of the generated  "
                code { "dist/index.html" }
                "."
            }
        }
        p {
            "This will instruct  "
            code { "dx serve" }
            " to redirect any unknown route to the index, to then be resolved by the router."
            code { "localhost:8080" }
            ", then click a link to go to "
            code { "localhost:8080/contact" }
            "), the app renders the new route without reloading."
            em { "before" }
            " going to the index (go straight to "
            code { "localhost:8080/contact" }
            "), we are trying to access a static route from the server, but the only static route on our server is the index (because the Dioxus frontend is a Single Page Application) and it will fail unless we redirect all missing routes to the index."
        }
        p { "This book is intended to get you up to speed with Dioxus Router. It is split" }
        ol {
            li {
                "The "
                a { href: "reference", "reference" }
                " section explains individual features in "
            }
            li {
                "If you prefer a learning-by-doing approach, you can check out the "
                em {
                    a { href: "example", "example project" }
                }
                ". It guides you through "
            }
        }
        blockquote {
            p {
                "Please note that this is not the only documentation for the Dioxus Router. You"
                a { href: "https://docs.rs/dioxus-router/", "API Docs" }
                "."
            }
        }
    }
}
#[component(no_case_check)]
pub fn RouterExampleIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "overview",
            a { href: "#overview", class: "header", "Overview" }
        }
        p { "In this guide, you'll learn to effectively use Dioxus Router whether you're" }
        blockquote {
            p {
                "To follow along with the router example, you'll need a working Dioxus app."
                a { href: "https://dioxuslabs.com/learn/0.5/getting_started", "Dioxus book" }
                " to get started."
            }
        }
        blockquote {
            p {
                "Make sure to add Dioxus Router as a dependency, as explained in the"
                a { href: "..", "introduction" }
                "."
            }
        }
        h2 { id: "youll-learn-how-to",
            a { href: "#youll-learn-how-to", class: "header", "You'll learn how to" }
        }
        ul {
            li { "Create routes and render \"pages\"." }
            li { "Utilize nested routes, create a navigation bar, and render content for a" }
            li { "Parse URL parameters to dynamically display content." }
            li { "Redirect visitors to different routes." }
        }
        blockquote {
            p {
                strong { "Disclaimer" }
            }
            p { "The example will only display the features of Dioxus Router. It will not" }
        }
        p {
            "You can find the complete application in the "
            a { href: "full-code", "full code" }
            " chapter."
        }
    }
}
#[component(no_case_check)]
pub fn RouterExampleFirstRoute() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "creating-our-first-route",
            a { href: "#creating-our-first-route", class: "header", "Creating Our First Route" }
        }
        p { "In this chapter, we will start utilizing Dioxus Router and add a homepage and a" }
        h2 { id: "fundamentals",
            a { href: "#fundamentals", class: "header", "Fundamentals" }
        }
        p {
            "The core of the Dioxus Router is the "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Routable.html",
                code { "Routable" }
            }
            " macro and the "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Router.html",
                code { "Router" }
            }
            " component."
        }
        p { "Routable is a trait for anything that can:" }
        ul {
            li { "Be parsed from a URL" }
            li { "Be turned into a URL" }
            li { "Be rendered as to a Element" }
        }
        p {
            "Let's create a new router. First, we need an actual page to route to! Let's add a homepage component:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Home</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ h1 {{ </span><span style=\"color:#ffee99;\">&quot;Welcome to the Dioxus Blog!&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "first_route.rs".to_string(),
        }
        h2 { id: "creating-routes",
            a { href: "#creating-routes", class: "header", "Creating Routes" }
        }
        p { "We want to use Dioxus Router to separate our application into different \"pages\"." }
        p {
            "To start using Dioxus Router, we need to use the "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Routable.html",
                code { "Routable" }
            }
            " macro."
        }
        p {
            "The "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Routable.html",
                code { "Routable" }
            }
            " macro takes an enum with all of the possible routes in our application. Each variant of the enum represents a route and must be annotated with the "
            code { "#[route(path)]" }
            " attribute."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case)]\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_router::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">/// An enum of all of the possible routes in the app.\n</span><span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// The home page is at the / route\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    Home {{}},\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "first_route.rs".to_string(),
        }
        p {
            "The "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Router.html",
                code { "Router" }
            }
            " component will provide a router context for all the inner components and hooks to use. You usually will want to place this at the top of your components tree."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ Router::&lt;Route&gt; {{}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "first_route.rs".to_string(),
        }
        p {
            "If you head to your application's browser tab, you should now see the text"
            code { "Welcome to Dioxus Blog!" }
            " when on the root URL ("
            code { "http://localhost:8080/" }
            "). If"
        }
        p {
            "This is because we told Dioxus Router to render the  "
            code { "Home" }
            " component only when"
            code { "/" }
            "."
        }
        h2 { id: "fallback-route",
            a { href: "#fallback-route", class: "header", "Fallback Route" }
        }
        p {
            "In our example, when a route doesn't exist Dioxus Router doesn't render anything. Many sites also have a \"404\" page when a path does not exist. Let's add one to our site."
        }
        p {
            "First, we create a new  "
            code { "PageNotFound" }
            " component."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">PageNotFound</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">route</span><span style=\"color:#f8f8f2;\">: Vec&lt;String&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;Page not found&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        p {{ </span><span style=\"color:#ffee99;\">&quot;We are terribly sorry, but the page you requested doesn&#39;t exist.&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        pre {{ color: </span><span style=\"color:#ffee99;\">&quot;red&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;log:</span><span style=\"color:#ff80f4;\">\\n</span><span style=\"color:#ffee99;\">attemped to navigate to: {{route:?}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "catch_all.rs".to_string(),
        }
        p { "Next, register the route in the Route enum to match if all other routes fail." }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    Home {{}},\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// PageNotFound is a catch all route that will match any route and placing the matched segments in the route field\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/:..route&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    PageNotFound {{ route: Vec&lt;String&gt; }},\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "catch_all.rs".to_string(),
        }
        p { "Now when you go to a route that doesn't exist, you should see the page not found" }
        h2 { id: "conclusion",
            a { href: "#conclusion", class: "header", "Conclusion" }
        }
        p {
            "In this chapter, we learned how to create a route and tell Dioxus Router what"
            code { "/" }
            ". We also created a 404 page to"
        }
    }
}
#[component(no_case_check)]
pub fn RouterExampleBuildingANest() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "building-a-nest",
            a { href: "#building-a-nest", class: "header", "Building a Nest" }
        }
        p { "In this chapter, we will begin to build the blog portion of our site which will" }
        h2 { id: "site-navigation",
            a { href: "#site-navigation", class: "header", "Site Navigation" }
        }
        p { "Our site visitors won't know all the available pages and blogs on our site so we" }
        p {
            "We want our navbar component to be rendered on several different pages on our site. Instead of duplicating the code, we can create a component that wraps all children routes. This is called a layout component. To tell the router where to render the child routes, we use the "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Outlet.html",
                code { "Outlet" }
            }
            " component."
        }
        p {
            "Let's create a new  "
            code { "NavBar" }
            " component:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">NavBar</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        nav {{\n</span><span style=\"color:#f8f8f2;\">            ul {{ li {{ </span><span style=\"color:#ffee99;\">&quot;links&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// The Outlet component will render child routes (In this case just the Home component) inside the Outlet component\n</span><span style=\"color:#f8f8f2;\">        Outlet::&lt;Route&gt; {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "nested_routes.rs".to_string(),
        }
        p {
            "Next, let's add our  "
            code { "NavBar" }
            " component as a layout to our Route enum:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"color:#f8f8f2;\">#[rustfmt::skip]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// All routes under the NavBar layout will be rendered inside of the NavBar Outlet\n</span><span style=\"color:#f8f8f2;\">    #[layout(NavBar)]\n</span><span style=\"color:#f8f8f2;\">        #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">        Home {{}},\n</span><span style=\"color:#f8f8f2;\">    #[end_layout]\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/:..route&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    PageNotFound {{ route: Vec&lt;String&gt; }},\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "nested_routes.rs".to_string(),
        }
        p {
            "To add links to our  "
            code { "NavBar" }
            ", we could always use an HTML anchor element but that has two issues:"
        }
        ol {
            li { "It causes a full-page reload" }
            li { "We can accidentally link to a page that doesn't exist" }
        }
        p {
            "Instead, we want to use the "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Link.html",
                code { "Link" }
            }
            " component provided by Dioxus Router."
        }
        p {
            "The "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Link.html",
                code { "Link" }
            }
            " is similar to a regular "
            code { "<a>" }
            " tag. It takes a target and children."
        }
        p {
            "Unlike a regular  "
            code { "<a>" }
            " tag, we can pass in our Route enum as the target. Because we annotated our routes with the  "
            code { "#[route(path)]" }
            " attribute, the "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Link.html",
                code { "Link" }
            }
            " will know how to generate the correct URL. If we use the Route enum, the rust compiler will prevent us from linking to a page that doesn't exist."
        }
        p { "Let's add our links:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">NavBar</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        nav {{\n</span><span style=\"color:#f8f8f2;\">            ul {{\n</span><span style=\"color:#f8f8f2;\">                li {{\n</span><span style=\"color:#f8f8f2;\">                    Link {{ to: Route::Home {{}}, </span><span style=\"color:#ffee99;\">&quot;Home&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        Outlet::&lt;Route&gt; {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "links.rs".to_string(),
        }
        blockquote {
            p {
                "Using this method, the "
                a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Link.html",
                    code { "Link" }
                }
                " component only works for links within our"
                a { href: "./navigation-targets", "here" }
                "."
            }
        }
        p { "Now you should see a list of links near the top of your page. Click on one and" }
        h2 { id: "url-parameters-and-nested-routes",
            a { href: "#url-parameters-and-nested-routes", class: "header",
                "URL Parameters and Nested Routes"
            }
        }
        p {
            "Many websites such as GitHub put parameters in their URL. For example,"
            code { "https://github.com/DioxusLabs" }
            " utilizes the text after the domain to"
        }
        p { "We want to store our blogs in a database and load them as needed. We also" }
        p { "We could utilize a search page that loads a blog when clicked but then our users" }
        p {
            "The path to our blog will look like  "
            code { "/blog/myBlogPage" }
            ",  "
            code { "myBlogPage" }
            " being the"
        }
        p {
            "First, let's create a layout component (similar to the navbar) that wraps the blog content. This allows us to add a heading that tells the user they are on the blog."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Blog</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;Blog&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        Outlet::&lt;Route&gt; {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "dynamic_route.rs".to_string(),
        }
        p { "Now we'll create another index component, that'll be displayed when no blog post" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BlogList</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h2 {{ </span><span style=\"color:#ffee99;\">&quot;Choose a post&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        ul {{\n</span><span style=\"color:#f8f8f2;\">            li {{\n</span><span style=\"color:#f8f8f2;\">                Link {{\n</span><span style=\"color:#f8f8f2;\">                    to: Route::BlogPost {{\n</span><span style=\"color:#f8f8f2;\">                        name: </span><span style=\"color:#ffee99;\">&quot;Blog post 1&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    }},\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#ffee99;\">&quot;Read the first blog post&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            li {{\n</span><span style=\"color:#f8f8f2;\">                Link {{\n</span><span style=\"color:#f8f8f2;\">                    to: Route::BlogPost {{\n</span><span style=\"color:#f8f8f2;\">                        name: </span><span style=\"color:#ffee99;\">&quot;Blog post 2&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    }},\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#ffee99;\">&quot;Read the second blog post&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "dynamic_route.rs".to_string(),
        }
        p {
            "We also need to create a component that displays an actual blog post. This component will accept the URL parameters as props:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// The name prop comes from the /:name route segment\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BlogPost</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ h2 {{ </span><span style=\"color:#ffee99;\">&quot;Blog Post: {{name}}&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "dynamic_route.rs".to_string(),
        }
        p { "Finally, let's tell our router about those components:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"color:#f8f8f2;\">#[rustfmt::skip]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    #[layout(NavBar)]\n</span><span style=\"color:#f8f8f2;\">        #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">        Home {{}},\n</span><span style=\"color:#f8f8f2;\">        #[nest(</span><span style=\"color:#ffee99;\">&quot;/blog&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">            #[layout(Blog)]\n</span><span style=\"color:#f8f8f2;\">            #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">            BlogList {{}},\n</span><span style=\"color:#f8f8f2;\">            #[route(</span><span style=\"color:#ffee99;\">&quot;/post/:name&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">            BlogPost {{ name: </span><span style=\"font-style:italic;color:#66d9ef;\">String </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">            #[end_layout]\n</span><span style=\"color:#f8f8f2;\">        #[end_nest]\n</span><span style=\"color:#f8f8f2;\">    #[end_layout]\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/:..route&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    PageNotFound {{\n</span><span style=\"color:#f8f8f2;\">        route: Vec&lt;String&gt;,\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "dynamic_route.rs".to_string(),
        }
        p {
            "That's it! If you head to  "
            code { "/blog/1" }
            " you should see our sample post."
        }
        h2 { id: "conclusion",
            a { href: "#conclusion", class: "header", "Conclusion" }
        }
        p { "In this chapter, we utilized Dioxus Router's Link, and Route Parameter" }
    }
}
#[component(no_case_check)]
pub fn RouterExampleNavigationTargets() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "navigation-targets",
            a { href: "#navigation-targets", class: "header", "Navigation Targets" }
        }
        p {
            "In the previous chapter, we learned how to create links to pages within our app."
            code { "target" }
            " property. This property takes something that can be converted to a "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/navigation/enum.NavigationTarget.html",
                code { "NavigationTarget" }
            }
            "."
        }
        h2 { id: "what-is-a-navigation-target",
            a { href: "#what-is-a-navigation-target", class: "header",
                "What is a navigation target?"
            }
        }
        p {
            "A "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/navigation/enum.NavigationTarget.html",
                code { "NavigationTarget" }
            }
            " is similar to the "
            code { "href" }
            " of an HTML anchor element. It"
        }
        ul {
            li {
                a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/navigation/enum.NavigationTarget.html#variant.Internal",
                    code { "Internal" }
                }
                ": We used internal links in the previous chapter. It's a link to a page within our"
            }
            li {
                a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/navigation/enum.NavigationTarget.html#variant.External",
                    code { "External" }
                }
                ": This works exactly like an HTML anchors' "
                code { "href" }
                ". Don't use this for in-app"
            }
        }
        h2 { id: "external-navigation",
            a { href: "#external-navigation", class: "header", "External navigation" }
        }
        p { "If we need a link to an external page we can do it like this:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">GoToDioxus</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Link {{ to: </span><span style=\"color:#ffee99;\">&quot;https://dioxuslabs.com&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;ExternalTarget target&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "external_link.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn RouterExampleRedirectionPerfection() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "redirection-perfection",
            a { href: "#redirection-perfection", class: "header", "Redirection Perfection" }
        }
        p { "You're well on your way to becoming a routing master!" }
        p { "In this chapter, we will cover creating redirects" }
        h2 { id: "creating-redirects",
            a { href: "#creating-redirects", class: "header", "Creating Redirects" }
        }
        p { "A redirect is very simple. When dioxus encounters a redirect while finding out" }
        p {
            "As a simple example, let's say you want user to still land on your blog, even"
            code { "/myblog" }
            " or "
            code { "/myblog/:name" }
            "."
        }
        p { "Redirects are special attributes in the router enum that accept a route and a closure" }
        p { "Let's add a redirect to our router enum:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"color:#f8f8f2;\">#[rustfmt::skip]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    #[layout(NavBar)]\n</span><span style=\"color:#f8f8f2;\">        #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">        Home {{}},\n</span><span style=\"color:#f8f8f2;\">        #[nest(</span><span style=\"color:#ffee99;\">&quot;/blog&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">            #[layout(Blog)]\n</span><span style=\"color:#f8f8f2;\">                #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">                BlogList {{}},\n</span><span style=\"color:#f8f8f2;\">                #[route(</span><span style=\"color:#ffee99;\">&quot;/post/:name&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">                BlogPost {{ name: </span><span style=\"font-style:italic;color:#66d9ef;\">String </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">            #[end_layout]\n</span><span style=\"color:#f8f8f2;\">        #[end_nest]\n</span><span style=\"color:#f8f8f2;\">    #[end_layout]\n</span><span style=\"color:#f8f8f2;\">    #[nest(</span><span style=\"color:#ffee99;\">&quot;/myblog&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">        #[redirect(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">, || Route::BlogList {{}})]\n</span><span style=\"color:#f8f8f2;\">        #[redirect(</span><span style=\"color:#ffee99;\">&quot;/:name&quot;</span><span style=\"color:#f8f8f2;\">, |name: String| Route::BlogPost {{ name }})]\n</span><span style=\"color:#f8f8f2;\">    #[end_nest]\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/:..route&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    PageNotFound {{\n</span><span style=\"color:#f8f8f2;\">        route: Vec&lt;String&gt;,\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "full_example.rs".to_string(),
        }
        p { "That's it! Now your users will be redirected to the blog." }
        h3 { id: "conclusion",
            a { href: "#conclusion", class: "header", "Conclusion" }
        }
        p {
            "Well done! You've completed the Dioxus Router guide. You've built a small "
            a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/router/examples",
                "router examples"
            }
            ", or "
            a { href: "https://docs.rs/dioxus-router/", "API reference" }
            "."
        }
        h3 { id: "challenges",
            a { href: "#challenges", class: "header", "Challenges" }
        }
        ul {
            li { "Organize your components into separate files for better maintainability." }
            li { "Give your app some style if you haven't already." }
            li { "Build an about page so your visitors know who you are." }
            li { "Add a user system that uses URL parameters." }
            li { "Create a simple admin system to create, delete, and edit blogs." }
            li { "If you want to go to the max, hook up your application to a rest API and database." }
        }
    }
}
#[component(no_case_check)]
pub fn RouterExampleFullCode() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "full-code",
            a { href: "#full-code", class: "header", "Full Code" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case)]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_router::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ANCHOR: router\n</span><span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"color:#f8f8f2;\">#[rustfmt::skip]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    #[layout(NavBar)]\n</span><span style=\"color:#f8f8f2;\">        #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">        Home {{}},\n</span><span style=\"color:#f8f8f2;\">        #[nest(</span><span style=\"color:#ffee99;\">&quot;/blog&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">            #[layout(Blog)]\n</span><span style=\"color:#f8f8f2;\">                #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">                BlogList {{}},\n</span><span style=\"color:#f8f8f2;\">                #[route(</span><span style=\"color:#ffee99;\">&quot;/post/:name&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">                BlogPost {{ name: </span><span style=\"font-style:italic;color:#66d9ef;\">String </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">            #[end_layout]\n</span><span style=\"color:#f8f8f2;\">        #[end_nest]\n</span><span style=\"color:#f8f8f2;\">    #[end_layout]\n</span><span style=\"color:#f8f8f2;\">    #[nest(</span><span style=\"color:#ffee99;\">&quot;/myblog&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">        #[redirect(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">, || Route::BlogList {{}})]\n</span><span style=\"color:#f8f8f2;\">        #[redirect(</span><span style=\"color:#ffee99;\">&quot;/:name&quot;</span><span style=\"color:#f8f8f2;\">, |name: String| Route::BlogPost {{ name }})]\n</span><span style=\"color:#f8f8f2;\">    #[end_nest]\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/:..route&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    PageNotFound {{\n</span><span style=\"color:#f8f8f2;\">        route: Vec&lt;String&gt;,\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#8c8c8c;\">// ANCHOR_END: router\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ Router::&lt;Route&gt; {{}} }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">NavBar</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        nav {{\n</span><span style=\"color:#f8f8f2;\">            ul {{\n</span><span style=\"color:#f8f8f2;\">                li {{\n</span><span style=\"color:#f8f8f2;\">                    Link {{ to: Route::Home {{}}, </span><span style=\"color:#ffee99;\">&quot;Home&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">                li {{\n</span><span style=\"color:#f8f8f2;\">                    Link {{ to: Route::BlogList {{}}, </span><span style=\"color:#ffee99;\">&quot;Blog&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        Outlet::&lt;Route&gt; {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Home</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ h1 {{ </span><span style=\"color:#ffee99;\">&quot;Welcome to the Dioxus Blog!&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Blog</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;Blog&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        Outlet::&lt;Route&gt; {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BlogList</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h2 {{ </span><span style=\"color:#ffee99;\">&quot;Choose a post&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        ul {{\n</span><span style=\"color:#f8f8f2;\">            li {{\n</span><span style=\"color:#f8f8f2;\">                Link {{\n</span><span style=\"color:#f8f8f2;\">                    to: Route::BlogPost {{\n</span><span style=\"color:#f8f8f2;\">                        name: </span><span style=\"color:#ffee99;\">&quot;Blog post 1&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    }},\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#ffee99;\">&quot;Read the first blog post&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            li {{\n</span><span style=\"color:#f8f8f2;\">                Link {{\n</span><span style=\"color:#f8f8f2;\">                    to: Route::BlogPost {{\n</span><span style=\"color:#f8f8f2;\">                        name: </span><span style=\"color:#ffee99;\">&quot;Blog post 2&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    }},\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#ffee99;\">&quot;Read the second blog post&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BlogPost</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ h2 {{ </span><span style=\"color:#ffee99;\">&quot;Blog Post: {{name}}&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">PageNotFound</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">route</span><span style=\"color:#f8f8f2;\">: Vec&lt;String&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;Page not found&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        p {{ </span><span style=\"color:#ffee99;\">&quot;We are terribly sorry, but the page you requested doesn&#39;t exist.&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        pre {{ color: </span><span style=\"color:#ffee99;\">&quot;red&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;log:</span><span style=\"color:#ff80f4;\">\\n</span><span style=\"color:#ffee99;\">attemped to navigate to: {{route:?}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "full_example.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn RouterReferenceIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "adding-the-router-to-your-application",
            a {
                href: "#adding-the-router-to-your-application",
                class: "header",
                "Adding the router to your application"
            }
        }
        p { "In this chapter, we will learn how to add the router to our app. By itself, this" }
        blockquote {
            p {
                "Make sure you added the  "
                code { "dioxus-router" }
                " dependency as explained in the"
                a { href: "..", "introduction" }
                "."
            }
        }
        p { "In most cases, we want to add the router to the root component of our app. This" }
        p { "First, we define the router with the router macro:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case)]\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_router::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">/// An enum of all of the possible routes in the app.\n</span><span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// The home page is at the / route\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    Home {{}},\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "first_route.rs".to_string(),
        }
        p {
            "Then we render the router with the "
            code { "Router" }
            "]"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ Router::&lt;Route&gt; {{}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "first_route.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn RouterReferenceRoutesIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "defining-routes",
            a { href: "#defining-routes", class: "header", "Defining Routes" }
        }
        p {
            "When creating a "
            code { "Routable" }
            "]"
            code { "route(\"path\")" }
            " attribute."
        }
        h2 { id: "route-segments",
            a { href: "#route-segments", class: "header", "Route Segments" }
        }
        p {
            "Each route is made up of segments. Most segments are separated by  "
            code { "/" }
            " characters in the path."
        }
        p { "There are four fundamental types of segments:" }
        ol {
            li {
                a { href: "#static-segments", "Static segments" }
                " are fixed strings that must be present in the path."
            }
            li {
                a { href: "#dynamic-segments", "Dynamic segments" }
                " are types that can be parsed from a segment."
            }
            li {
                a { href: "#catch-all-segments", "Catch-all segments" }
                " are types that can be parsed from multiple segments."
            }
            li {
                a { href: "#query-segments", "Query segments" }
                " are types that can be parsed from the query string."
            }
        }
        p { "Routes are matched:" }
        ul {
            li {
                "First, from most specific to least specific (Static then Dynamic then Catch All) (Query is always matched)"
            }
            li {
                "Then, if multiple routes match the same path, the order in which they are defined in the enum is followed."
            }
        }
        h2 { id: "static-segments",
            a { href: "#static-segments", class: "header", "Static segments" }
        }
        p {
            "Fixed routes match a specific path. For example, the route  "
            code { "#[route(\"/about\")]" }
            " will match the path  "
            code { "/about" }
            "."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"color:#f8f8f2;\">#[rustfmt::skip]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Routes always start with a slash\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    Home {{}},\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// You can have multiple segments in a route\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/hello/world&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    HelloWorld {{}},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Home</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">HelloWorld</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "static_segments.rs".to_string(),
        }
        h2 { id: "dynamic-segments",
            a { href: "#dynamic-segments", class: "header", "Dynamic Segments" }
        }
        p {
            "Dynamic segments are in the form of  "
            code { ":name" }
            " where  "
            code { "name" }
            " is"
        }
        p {
            "The segment can be of any type that implements  "
            code { "FromStr" }
            "."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"color:#f8f8f2;\">#[rustfmt::skip]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// segments that start with : are dynamic segments\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/post/:name&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    BlogPost {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You must include dynamic segments in child variants\n</span><span style=\"color:#f8f8f2;\">        name: </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/document/:id&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    Document {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can use any type that implements FromStr\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// If the segment can&#39;t be parsed, the route will not match\n</span><span style=\"color:#f8f8f2;\">        id: </span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Components must contain the same dynamic segments as their corresponding variant\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BlogPost</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Document</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "dynamic_segments.rs".to_string(),
        }
        h2 { id: "catch-all-segments",
            a { href: "#catch-all-segments", class: "header", "Catch All Segments" }
        }
        p {
            "Catch All segments are in the form of  "
            code { ":..name" }
            " where  "
            code { "name" }
            " is the name of the field in the route variant. If the segments are parsed successfully then the route matches, otherwise the matching continues."
        }
        p {
            "The segment can be of any type that implements  "
            code { "FromSegments" }
            ". (Vec"
            p { class: "inline-html-block", dangerous_inner_html: "<String>" }
            " implements this by default)"
        }
        p {
            "Catch All segments must be the "
            em { "last route segment" }
            " in the path (query segments are not counted) and cannot be included in nests."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"color:#f8f8f2;\">#[rustfmt::skip]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// segments that start with :.. are catch all segments\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/blog/:..segments&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    BlogPost {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You must include catch all segment in child variants\n</span><span style=\"color:#f8f8f2;\">        segments: Vec&lt;String&gt;,\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Components must contain the same catch all segments as their corresponding variant\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BlogPost</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">segments</span><span style=\"color:#f8f8f2;\">: Vec&lt;String&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "catch_all_segments.rs".to_string(),
        }
        h2 { id: "query-segments",
            a { href: "#query-segments", class: "header", "Query Segments" }
        }
        p {
            "Query segments are in the form of  "
            code { "?:name&:othername" }
            " where  "
            code { "name" }
            " and  "
            code { "othername" }
            " are the names of fields in the route variant."
        }
        p {
            "Unlike "
            a { href: "#dynamic-segments", "Dynamic Segments" }
            " and "
            a { href: "#catch-all-segments", "Catch All Segments" }
            ", parsing a Query segment must not fail."
        }
        p {
            "The segment can be of any type that implements  "
            code { "FromQueryArgument" }
            "."
        }
        p {
            "Query segments must be the "
            em { "after all route segments" }
            " and cannot be included in nests."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"color:#f8f8f2;\">#[rustfmt::skip]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// segments that start with ?: are query segments\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/blog?:name&amp;:surname&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    BlogPost {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You must include query segments in child variants\n</span><span style=\"color:#f8f8f2;\">        name: </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        surname: </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BlogPost</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String, </span><span style=\"font-style:italic;color:#fd971f;\">surname</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;This is your blogpost with a query segment:&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Name: {{name}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Surname: {{surname}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ Router::&lt;Route&gt; {{}} }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{}}</span></pre>\n",
            name: "query_segments.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn RouterReferenceRoutesNested() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "nested-routes",
            a { href: "#nested-routes", class: "header", "Nested Routes" }
        }
        p { "When developing bigger applications we often want to nest routes within each" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">└ Settings\n</span><span style=\"color:#f8f8f2;\">  ├ General Settings (displayed when opening the settings)\n</span><span style=\"color:#f8f8f2;\">  ├ Change Password\n</span><span style=\"color:#f8f8f2;\">  └ Privacy Settings</span></pre>\n" }
        p { "We might want to map this structure to these paths and components:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">settings\t\t  -&gt; Settings {{ GeneralSettings }}\n</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">settings</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">password -&gt; Settings {{ PWSettings }}\n</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">settings</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">privacy  -&gt; Settings {{ PrivacySettings }}</span></pre>\n" }
        p { "Nested routes allow us to do this without repeating /settings in every route." }
        h2 { id: "nesting",
            a { href: "#nesting", class: "header", "Nesting" }
        }
        p {
            "To nest routes, we use the  "
            code { "#[nest(\"path\")]" }
            " and  "
            code { "#[end_nest]" }
            " attributes."
        }
        p { "The path in nest must not:" }
        ol {
            li {
                "Contain a "
                a { href: "./#catch-all-segments", "Catch All Segment" }
            }
            li {
                "Contain a "
                a { href: "./#query-segments", "Query Segment" }
            }
        }
        p {
            "If you define a dynamic segment in a nest, it will be available to all child routes and layouts."
        }
        p {
            "To finish a nest, we use the  "
            code { "#[end_nest]" }
            " attribute or the end of the enum."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"color:#8c8c8c;\">// Skipping formatting allows you to indent nests\n</span><span style=\"color:#f8f8f2;\">#[rustfmt::skip]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Start the /blog nest\n</span><span style=\"color:#f8f8f2;\">    #[nest(</span><span style=\"color:#ffee99;\">&quot;/blog&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can nest as many times as you want\n</span><span style=\"color:#f8f8f2;\">        #[nest(</span><span style=\"color:#ffee99;\">&quot;/:id&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">            #[route(</span><span style=\"color:#ffee99;\">&quot;/post&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">            PostId {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// You must include parent dynamic segments in child variants\n</span><span style=\"color:#f8f8f2;\">                id: </span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// End nests manually with #[end_nest]\n</span><span style=\"color:#f8f8f2;\">        #[end_nest]\n</span><span style=\"color:#f8f8f2;\">        #[route(</span><span style=\"color:#ffee99;\">&quot;/:id&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// The absolute route of BlogPost is /blog/:name\n</span><span style=\"color:#f8f8f2;\">        BlogPost {{\n</span><span style=\"color:#f8f8f2;\">            id: </span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Or nests are ended automatically at the end of the enum\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BlogPost</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">PostId</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "nest.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn RouterReferenceLayouts() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "layouts",
            a { href: "#layouts", class: "header", "Layouts" }
        }
        p {
            "Layouts allow you to wrap all child routes in a component. This can be useful when creating something like a header that will be used in many different routes."
        }
        p {
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Outlet.html",
                code { "Outlet" }
            }
            " tells the router where to render content in layouts. In the following example,"
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Outlet.html",
                code { "Outlet" }
            }
            "."
        }
        p {
            "This page is built with the Dioxus. It uses Layouts in several different places. Here is an outline of how layouts are used on the current page. Hover over different layouts to see what elements they are on the page."
        }
        LayoutsExplanation {}
        p { "Here is a more complete example of a layout wrapping the body of a page." }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"color:#f8f8f2;\">#[rustfmt::skip]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    #[layout(Wrapper)]\n</span><span style=\"color:#f8f8f2;\">        #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">        Index {{}},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Wrapper</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        header {{ </span><span style=\"color:#ffee99;\">&quot;header&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// The index route will be rendered here\n</span><span style=\"color:#f8f8f2;\">        Outlet::&lt;Route&gt; {{}}\n</span><span style=\"color:#f8f8f2;\">        footer {{ </span><span style=\"color:#ffee99;\">&quot;footer&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Index</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ h1 {{ </span><span style=\"color:#ffee99;\">&quot;Index&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "outlet.rs".to_string(),
        }
        p { "The example above will output the following HTML (line breaks added for" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">header</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">header&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">header</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">h1</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">Index&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">h1</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">footer</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">footer&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">footer</span><span style=\"color:#f92672;\">&gt;</span></pre>\n",
        }
        h2 { id: "layouts-with-dynamic-segments",
            a { href: "#layouts-with-dynamic-segments", class: "header",
                "Layouts with dynamic segments"
            }
        }
        p {
            "You can combine layouts with "
            a { href: "./routes/nested", "nested routes" }
            " to create dynamic layouts with content that changes based on the current route."
        }
        p {
            "Just like routes, layouts components must accept a prop for each dynamic segment in the route. For example, if you have a route with a dynamic segment like  "
            code { "/:name" }
            ", your layout component must accept a  "
            code { "name" }
            " prop:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"color:#f8f8f2;\">#[rustfmt::skip]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    #[nest(</span><span style=\"color:#ffee99;\">&quot;/:name&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">        #[layout(Wrapper)]\n</span><span style=\"color:#f8f8f2;\">            #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">            Index {{\n</span><span style=\"color:#f8f8f2;\">                name: </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Wrapper</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        header {{ </span><span style=\"color:#ffee99;\">&quot;Welcome {{name}}!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// The index route will be rendered here\n</span><span style=\"color:#f8f8f2;\">        Outlet::&lt;Route&gt; {{}}\n</span><span style=\"color:#f8f8f2;\">        footer {{ </span><span style=\"color:#ffee99;\">&quot;footer&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Index</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ h1 {{ </span><span style=\"color:#ffee99;\">&quot;This is a homepage for {{name}}&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "outlet.rs".to_string(),
        }
        p {
            "Or to get the full route, you can use the  "
            code { "use_route" }
            " hook."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone)]\n</span><span style=\"color:#f8f8f2;\">#[rustfmt::skip]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    #[layout(Wrapper)]\n</span><span style=\"color:#f8f8f2;\">        #[route(</span><span style=\"color:#ffee99;\">&quot;/:name&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">        Index {{\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Wrapper</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> full_route </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">use_route::&lt;Route&gt;();\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        header {{ </span><span style=\"color:#ffee99;\">&quot;Welcome to {{full_route}}!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// The index route will be rendered here\n</span><span style=\"color:#f8f8f2;\">        Outlet::&lt;Route&gt; {{}}\n</span><span style=\"color:#f8f8f2;\">        footer {{ </span><span style=\"color:#ffee99;\">&quot;footer&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Index</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ h1 {{ </span><span style=\"color:#ffee99;\">&quot;This is a homepage for {{name}}&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "outlet.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn RouterReferenceNavigationIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "links--navigation",
            a { href: "#links--navigation", class: "header", "Links & Navigation" }
        }
        p { "When we split our app into pages, we need to provide our users with a way to" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">a href</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;/other&quot;</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">Link to an other page&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">/</span><span style=\"color:#f8f8f2;\">a</span><span style=\"color:#f92672;\">&gt;</span></pre>\n" }
        p { "However, we cannot do that when using the router for three reasons:" }
        ol {
            li { "Anchor tags make the browser load a new page from the server. This takes a" }
            li { "Navigation using anchor tags only works when the app is running inside a" }
            li { "Anchor tags cannot check if the target page exists. This means we cannot" }
        }
        p {
            "To solve these problems, the router provides us with a "
            code { "Link" }
            "]"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">NavBar</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        nav {{\n</span><span style=\"color:#f8f8f2;\">            ul {{\n</span><span style=\"color:#f8f8f2;\">                li {{\n</span><span style=\"color:#f8f8f2;\">                    Link {{ to: Route::Home {{}}, </span><span style=\"color:#ffee99;\">&quot;Home&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        Outlet::&lt;Route&gt; {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "links.rs".to_string(),
        }
        p {
            "The  "
            code { "target" }
            " in the example above is similar to the  "
            code { "href" }
            " of a regular anchor"
            code { "NavigationTarget" }
            "]"
        }
        ul {
            li {
                "The example uses a Internal route. This is the most common type of navigation."
                code { "Routable" }
                "]"
            }
            li {
                "["
                code { "External" }
                "]"
            }
        }
        blockquote {
            p {
                "The "
                code { "Link" }
                "]"
            }
        }
    }
}
#[component(no_case_check)]
pub fn RouterReferenceNavigationProgrammatic() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "programmatic-navigation",
            a { href: "#programmatic-navigation", class: "header", "Programmatic Navigation" }
        }
        p { "Sometimes we want our application to navigate to another page without having the" }
        h2 { id: "using-a-navigator",
            a { href: "#using-a-navigator", class: "header", "Using a Navigator" }
        }
        p {
            "We can get a navigator with the "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/prelude/struct.Navigator.html",
                code { "navigator" }
            }
            " function which returns a "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/prelude/struct.Navigator.html",
                code { "Navigator" }
            }
            "."
        }
        p {
            "We can use the "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/prelude/struct.Navigator.html",
                code { "Navigator" }
            }
            " to trigger four different kinds of navigation:"
        }
        ul {
            li {
                code { "push" }
                " will navigate to the target. It works like a regular anchor tag."
            }
            li {
                code { "replace" }
                " works like "
                code { "push" }
                ", except that it replaces the current history entry"
            }
            li {
                code { "Go back" }
                " works like the browser's back button."
            }
            li {
                code { "Go forward" }
                " works like the browser's forward button."
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Home</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> nav </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">navigator</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// push\n</span><span style=\"color:#f8f8f2;\">    nav.</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(Route::PageNotFound {{ route: vec![] }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// replace\n</span><span style=\"color:#f8f8f2;\">    nav.</span><span style=\"color:#66d9ef;\">replace</span><span style=\"color:#f8f8f2;\">(Route::Home {{}});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// go back\n</span><span style=\"color:#f8f8f2;\">    nav.</span><span style=\"color:#66d9ef;\">go_back</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// go forward\n</span><span style=\"color:#f8f8f2;\">    nav.</span><span style=\"color:#66d9ef;\">go_forward</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ h1 {{ </span><span style=\"color:#ffee99;\">&quot;Welcome to the Dioxus Blog!&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "navigator.rs".to_string(),
        }
        p {
            "You might have noticed that, like "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Link.html",
                code { "Link" }
            }
            ", the "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/prelude/struct.Navigator.html",
                code { "Navigator" }
            }
            "s "
            code { "push" }
            " and"
            code { "replace" }
            " functions take a "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/navigation/enum.NavigationTarget.html",
                code { "NavigationTarget" }
            }
            ". This means we can use either"
            code { "Internal" }
            ", or "
            code { "External" }
            " targets."
        }
        h2 { id: "external-navigation-targets",
            a { href: "#external-navigation-targets", class: "header",
                "External Navigation Targets"
            }
        }
        p {
            "Unlike a "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Link.html",
                code { "Link" }
            }
            ", the "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/prelude/struct.Navigator.html",
                code { "Navigator" }
            }
            " cannot rely on the browser (or webview) to"
        }
        p { "This means, that under certain conditions, navigation to external targets can" }
    }
}
#[component(no_case_check)]
pub fn RouterReferenceHistoryProviders() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "history-providers",
            a { href: "#history-providers", class: "header", "History Providers" }
        }
        p {
            "[ "
            code { "HistoryProvider" }
            "]"
        }
        p {
            "The router provides two "
            code { "HistoryProvider" }
            "]"
        }
        ul {
            li {
                "The "
                code { "MemoryHistory" }
                "]"
            }
            li {
                "The "
                code { "LiveviewHistory" }
                "]"
            }
            li {
                "The "
                code { "WebHistory" }
                "]"
            }
        }
        p {
            "By default, the router uses the "
            code { "MemoryHistory" }
            "]"
            code { "WebHistory" }
            "]"
            code { "web" }
            " feature is active, but that is not guaranteed."
        }
        p { "You can override the default history:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Router::&lt;Route&gt; {{ config: || RouterConfig::default() }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "history_provider.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn RouterReferenceHistoryButtons() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "history-buttons",
            a { href: "#history-buttons", class: "header", "History Buttons" }
        }
        p { "Some platforms, like web browsers, provide users with an easy way to navigate" }
        p { "However, native platforms usually don't provide such amenities, which means that" }
        ul {
            li {
                a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.GoBackButton.html",
                    code { "GoBackButton" }
                }
            }
            li {
                a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.GoForwardButton.html",
                    code { "GoForwardButton" }
                }
            }
        }
        blockquote {
            p {
                "If you want to navigate through the history programmatically, take a look at"
                a { href: "./navigation/programmatic",
                    code { "programmatic navigation" }
                }
                "."
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">HistoryNavigation</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        GoBackButton {{ </span><span style=\"color:#ffee99;\">&quot;Back to the Past&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        GoForwardButton {{ </span><span style=\"color:#ffee99;\">&quot;Back to the Future&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "history_buttons.rs".to_string(),
        }
        p { "As you might know, browsers usually disable the back and forward buttons if" }
        p {
            "Importantly, neither  "
            code { "WebHistory" }
            " supports that feature."
        }
        p { "However, in both cases, the router will just ignore button presses, if there is" }
        p {
            "Also, when using  "
            code { "WebHistory" }
            ", the history buttons might"
        }
    }
}
#[component(no_case_check)]
pub fn RouterReferenceRoutingUpdateCallback() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "routing-update-callback",
            a { href: "#routing-update-callback", class: "header", "Routing Update Callback" }
        }
        p {
            "In some cases, we might want to run custom code when the current route changes. For this reason, the "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/prelude/struct.RouterConfig.html",
                code { "RouterConfig" }
            }
            " exposes an "
            code { "on_update" }
            " field."
        }
        h2 { id: "how-does-the-callback-behave",
            a { href: "#how-does-the-callback-behave", class: "header",
                "How does the callback behave?"
            }
        }
        p {
            "The  "
            code { "on_update" }
            " is called whenever the current routing information changes. It is called after the router updated its internal state, but before dependent components and hooks are updated."
        }
        p {
            "If the callback returns a "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/navigation/enum.NavigationTarget.html",
                code { "NavigationTarget" }
            }
            ", the router will replace the current location with the specified target. It will not call the "
            code { "on_update" }
            " again."
        }
        p {
            "If at any point the router encounters a navigation failure, it will go to the appropriate state without calling the  "
            code { "on_update" }
            ". It doesn't matter if the invalid target initiated the navigation, was found as a redirect target, or was returned by the  "
            code { "on_update" }
            " itself."
        }
        h2 { id: "code-example",
            a { href: "#code-example", class: "header", "Code Example" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone, PartialEq)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    Index {{}},\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/home&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    Home {{}},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Home</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ p {{ </span><span style=\"color:#ffee99;\">&quot;Home&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Index</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ p {{ </span><span style=\"color:#ffee99;\">&quot;Index&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Router::&lt;Route&gt; {{\n</span><span style=\"color:#f8f8f2;\">            config: || {{\n</span><span style=\"color:#f8f8f2;\">                RouterConfig::default()\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">on_update</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">state</span><span style=\"color:#f8f8f2;\">| {{\n</span><span style=\"color:#f8f8f2;\">                        (state.</span><span style=\"color:#66d9ef;\">current</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">== </span><span style=\"color:#f8f8f2;\">Route::Index {{}})\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">then_some</span><span style=\"color:#f8f8f2;\">(NavigationTarget::Internal(Route::Home {{}}))\n</span><span style=\"color:#f8f8f2;\">                    }})\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "routing_update.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceAssets() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "assets",
            a { href: "#assets", class: "header", "Assets" }
        }
        blockquote {
            p {
                "⚠\u{fe0f} Support: Manganis is currently in alpha. API changes are planned and bugs are more likely"
            }
        }
        p {
            "Assets are files that are included in the final build of the application. They can be images, fonts, stylesheets, or any other file that is not a source file. Dioxus includes first class support for assets, and provides a simple way to include them in your application and automatically optimize them for production."
        }
        p {
            "Assets in dioxus are also compatible with libraries! If you are building a library, you can include assets in your library and they will be automatically included in the final build of any application that uses your library."
        }
        p {
            "First, you need to add the  "
            code { "manganis" }
            " crate to your  "
            code { "Cargo.toml" }
            " file:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add manganis</span></pre>\n" }
        h2 { id: "including-images",
            a { href: "#including-images", class: "header", "Including images" }
        }
        p {
            "To include an asset in your application, you can simply wrap the path to the asset in a  "
            code { "mg!" }
            " call. For example, to include an image in your application, you can use the following code:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// You can link to assets that are relative to the package root or even link to an asset from a url\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// These assets will automatically be picked up by the dioxus cli, optimized, and bundled with your final applications\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">ASSET</span><span style=\"color:#f8f8f2;\">: Asset </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">asset!(</span><span style=\"color:#ffee99;\">&quot;/assets/static/ferrous_wave.png&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        img {{ src: </span><span style=\"color:#ffee99;\">&quot;{{ASSET}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "assets.rs".to_string(),
        }
        p {
            "You can also optimize, resize, and preload images using the  "
            code { "mg!" }
            " macro. Choosing an optimized file type (like WebP) and a reasonable quality setting can significantly reduce the size of your images which helps your application load faster. For example, you can use the following code to include an optimized image in your application:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">ENUM_ROUTER_IMG</span><span style=\"color:#f8f8f2;\">: Asset </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">asset!(</span><span style=\"color:#ffee99;\">&quot;/assets/static/enum_router.png&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">EnumRouter</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        img {{ src: </span><span style=\"color:#ffee99;\">&quot;{{ENUM_ROUTER_IMG}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "assets.rs".to_string(),
        }
        h2 { id: "including-arbitrary-files",
            a { href: "#including-arbitrary-files", class: "header", "Including arbitrary files" }
        }
        p {
            "In dioxus desktop, you may want to include a file with data for your application. You can use the  "
            code { "file" }
            " function to include arbitrary files in your application. For example, you can use the following code to include a file in your application:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// You can also collect arbitrary files. Relative paths are resolved relative to the package root\n</span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">PATH_TO_BUNDLED_CARGO_TOML</span><span style=\"color:#f8f8f2;\">: Asset </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">asset!(</span><span style=\"color:#ffee99;\">&quot;/Cargo.toml&quot;</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "assets.rs".to_string(),
        }
        p {
            "These files will be automatically included in the final build of your application, and you can use them in your application as you would any other file."
        }
        h2 { id: "including-stylesheets",
            a { href: "#including-stylesheets", class: "header", "Including stylesheets" }
        }
        p {
            "You can include stylesheets in your application using the  "
            code { "mg!" }
            " macro. For example, you can use the following code to include a stylesheet in your application:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// You can also bundle stylesheets with your application\n</span><span style=\"color:#8c8c8c;\">// Any files that end with .css will be minified and bundled with your application even if you don&#39;t explicitly include them in your &lt;head&gt;\n</span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">: Asset </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">asset!(</span><span style=\"color:#ffee99;\">&quot;/tailwind.css&quot;</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "assets.rs".to_string(),
        }
        blockquote {
            p {
                "The "
                a { href: "../cookbook/tailwind", "tailwind guide" }
                " has more information on how to use tailwind with dioxus."
            }
        }
        h2 { id: "conclusion",
            a { href: "#conclusion", class: "header", "Conclusion" }
        }
        p {
            "Dioxus provides first class support for assets, and makes it easy to include them in your application. You can include images, arbitrary files, and stylesheets in your application, and dioxus will automatically optimize them for production. This makes it easy to include assets in your application and ensure that they are optimized for production."
        }
        p {
            "You can read more about assets and all the options available to optimize your assets in the "
            a { href: "https://docs.rs/manganis/0.2.2/manganis/", "manganis documentation" }
            "."
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceWebIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "web",
            a { href: "#web", class: "header", "Web" }
        }
        p {
            "To run on the Web, your app must be compiled to WebAssembly and depend on the  "
            code { "dioxus" }
            " and  "
            code { "dioxus-web" }
            " crates."
        }
        p {
            "A build of Dioxus for the web will be roughly equivalent to the size of a React build (70kb vs 65kb) but it will load significantly faster because "
            a { href: "https://hacks.mozilla.org/2018/01/making-webassembly-even-faster-firefoxs-new-streaming-and-tiering-compiler/",
                "WebAssembly can be compiled as it is streamed"
            }
            "."
        }
        p { "Examples:" }
        ul {
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/blob/main/examples/todomvc.rs",
                    "TodoMVC"
                }
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/examples/tailwind",
                    "Tailwind App"
                }
            }
        }
        p {
            a { href: "https://github.com/DioxusLabs/dioxus/blob/main/examples/todomvc.rs",
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
                a { href: "https://rustwasm.github.io/docs/book/reference/which-crates-work-with-wasm.html",
                    "not every crate will work"
                }
                " with your web apps, so you'll need to make sure that your crates work without native system calls (timers, IO, etc)."
            }
        }
        h2 { id: "support",
            a { href: "#support", class: "header", "Support" }
        }
        p { "The Web is the best-supported target platform for Dioxus." }
        ul {
            li {
                "Because your app will be compiled to WASM you have access to browser APIs through "
                a { href: "https://rustwasm.github.io/docs/wasm-bindgen/introduction.html",
                    "wasm-bindgen"
                }
                "."
            }
            li {
                "Dioxus provides hydration to resume apps that are rendered on the server. See the "
                a { href: "../fullstack", "fullstack" }
                " reference for more information."
            }
        }
        h2 { id: "running-javascript",
            a { href: "#running-javascript", class: "header", "Running Javascript" }
        }
        p {
            "Dioxus provides some ergonomic wrappers over the browser API, but in some cases you may need to access parts of the browser API Dioxus does not expose."
        }
        p {
            "For these cases, Dioxus web exposes the use_eval hook that allows you to run raw Javascript in the webview:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> future </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can create as many eval instances as you want\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> eval </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">document::eval(\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;\n</span><span style=\"color:#ffee99;\">            // You can send messages from JavaScript to Rust with the dioxus.send function\n</span><span style=\"color:#ffee99;\">            dioxus.send(&quot;Hi from JS!&quot;);\n</span><span style=\"color:#ffee99;\">            // You can receive messages from Rust to JavaScript with the dioxus.recv function\n</span><span style=\"color:#ffee99;\">            let msg = await dioxus.recv();\n</span><span style=\"color:#ffee99;\">            console.log(msg);\n</span><span style=\"color:#ffee99;\">            &quot;#</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        );\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can send messages to JavaScript with the send method\n</span><span style=\"color:#f8f8f2;\">        eval.</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hi from Rust!&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can receive any message from JavaScript with the recv method\n</span><span style=\"color:#f8f8f2;\">        eval.recv::&lt;String&gt;().await.</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> future.</span><span style=\"color:#66d9ef;\">read_unchecked</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">as_ref</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(v) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            p {{ </span><span style=\"color:#ffee99;\">&quot;{{v}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">_ =&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            p {{ </span><span style=\"color:#ffee99;\">&quot;hello&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "eval.rs".to_string(),
        }
        p {
            "If you are targeting web, but don't plan on targeting any other Dioxus renderer you can also use the generated wrappers in the "
            a { href: "https://rustwasm.github.io/wasm-bindgen/web-sys/index.html",
                "web-sys"
            }
            " and "
            a { href: "https://gloo-rs.web.app/", "gloo" }
            " crates."
        }
        h2 { id: "customizing-index-template",
            a { href: "#customizing-index-template", class: "header", "Customizing Index Template" }
        }
        p {
            "Dioxus supports providing custom index.html templates. The index.html must include a  "
            code { "div" }
            " with the id  "
            code { "main" }
            " to be used. Hot Reload is still supported. An example"
            a { href: "https://github.com/DioxusLabs/dioxus/blob/main/examples/PWA-example/index.html",
                "PWA-Example"
            }
            "."
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceDesktopIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "desktop",
            a { href: "#desktop", class: "header", "Desktop" }
        }
        p { "This guide will cover concepts specific to the Dioxus desktop renderer." }
        p {
            "Apps built with Dioxus desktop use the system WebView to render the page. This makes the final size of application much smaller than other WebView renderers (typically under 5MB)."
        }
        p {
            "Although desktop apps are rendered in a WebView, your Rust code runs natively. This means that browser APIs are "
            em { "not" }
            " available, so rendering WebGL, Canvas, etc is not as easy as the Web. However, native system APIs "
            em { "are" }
            " accessible, so streaming, WebSockets, filesystem, etc are all easily accessible though system APIs."
        }
        p {
            "Dioxus desktop is built off "
            a { href: "https://tauri.app/", "Tauri" }
            ". Right now there are limited Dioxus abstractions over the menubar, event handling, etc. In some places you may need to leverage Tauri directly – through "
            a { href: "http://github.com/tauri-apps/wry/", "Wry" }
            " and "
            a { href: "http://github.com/tauri-apps/tao", "Tao" }
            "."
        }
        blockquote {
            p {
                "In the future, we plan to move to a custom web renderer-based DOM renderer with WGPU integrations ("
                a { href: "https://github.com/DioxusLabs/blitz", "Blitz" }
                ")."
            }
        }
        h2 { id: "examples",
            a { href: "#examples", class: "header", "Examples" }
        }
        ul {
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/blob/main/examples/file_explorer.rs",
                    "File Explorer"
                }
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/examples/tailwind",
                    "Tailwind App"
                }
            }
        }
        p {
            a { href: "https://github.com/DioxusLabs/dioxus/tree/main/examples/tailwind",
                img {
                    src: asset!(
                        "/assets/static/tailwind_desktop_app.png", ImageAssetOptions::new().with_avif()
                    ),
                    alt: "Tailwind App screenshot",
                    title: "",
                }
            }
        }
        h2 { id: "running-javascript",
            a { href: "#running-javascript", class: "header", "Running Javascript" }
        }
        p {
            "Dioxus provides some ergonomic wrappers over the browser API, but in some cases you may need to access parts of the browser API Dioxus does not expose."
        }
        p {
            "For these cases, Dioxus desktop exposes the use_eval hook that allows you to run raw Javascript in the webview:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> future </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can create as many eval instances as you want\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> eval </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">document::eval(\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;\n</span><span style=\"color:#ffee99;\">            // You can send messages from JavaScript to Rust with the dioxus.send function\n</span><span style=\"color:#ffee99;\">            dioxus.send(&quot;Hi from JS!&quot;);\n</span><span style=\"color:#ffee99;\">            // You can receive messages from Rust to JavaScript with the dioxus.recv function\n</span><span style=\"color:#ffee99;\">            let msg = await dioxus.recv();\n</span><span style=\"color:#ffee99;\">            console.log(msg);\n</span><span style=\"color:#ffee99;\">            &quot;#</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        );\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can send messages to JavaScript with the send method\n</span><span style=\"color:#f8f8f2;\">        eval.</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hi from Rust!&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can receive any message from JavaScript with the recv method\n</span><span style=\"color:#f8f8f2;\">        eval.recv::&lt;String&gt;().await.</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> future.</span><span style=\"color:#66d9ef;\">read_unchecked</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">as_ref</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(v) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            p {{ </span><span style=\"color:#ffee99;\">&quot;{{v}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">_ =&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            p {{ </span><span style=\"color:#ffee99;\">&quot;hello&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "eval.rs".to_string(),
        }
        h2 { id: "custom-assets",
            a { href: "#custom-assets", class: "header", "Custom Assets" }
        }
        p { "You can link to local assets in dioxus desktop instead of using a url:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            img {{ src: </span><span style=\"color:#ffee99;\">&quot;/public/static/scanner.png&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "custom_assets.rs".to_string(),
        }
        p {
            "You can read more about assets in the "
            a { href: "./assets", "assets" }
            " reference."
        }
        h2 { id: "integrating-with-wry",
            a { href: "#integrating-with-wry", class: "header", "Integrating with Wry" }
        }
        p {
            "In cases where you need more low level control over your window, you can use wry APIs exposed through the "
            a { href: "https://docs.rs/dioxus-desktop/0.5.0/dioxus_desktop/struct.Config.html",
                "Desktop Config"
            }
            " and the "
            a { href: "https://docs.rs/dioxus-desktop/0.5.0/dioxus_desktop/fn.use_window.html",
                "use_window hook"
            }
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceMobileIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "mobile-app",
            a { href: "#mobile-app", class: "header", "Mobile App" }
        }
        p { "Build a mobile app with Dioxus!" }
        p {
            "Example: "
            a { href: "https://github.com/DioxusLabs/dioxus/tree/main/examples/mobile_demo",
                "Mobile Demo"
            }
        }
        h2 { id: "support",
            a { href: "#support", class: "header", "Support" }
        }
        p {
            "Mobile is currently the least-supported renderer target for Dioxus. Mobile apps are rendered with either the platform's WebView or experimentally with "
            a { href: "https://github.com/DioxusLabs/blitz", "WGPU" }
            ". WebView doesn't support animations, transparency, and native widgets."
        }
        p {
            "Mobile support is currently best suited for CRUD-style apps, ideally for internal teams who need to develop quickly but don't care much about animations or native widgets."
        }
        h2 { id: "getting-set-up",
            a { href: "#getting-set-up", class: "header", "Getting Set up" }
        }
        p {
            "Getting set up with mobile can be quite challenging. The tooling here isn't great (yet) and might take some hacking around to get things working."
        }
        h3 { id: "setting-up-dependencies",
            a { href: "#setting-up-dependencies", class: "header", "Setting up dependencies" }
        }
        h4 { id: "android",
            a { href: "#android", class: "header", "Android" }
        }
        p { "First, install the rust Android targets:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rustup target add aarch64</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">linux</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">android armv7</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">linux</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">androideabi i686</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">linux</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">android x86_64</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">linux</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">android</span></pre>\n" }
        p {
            "To develop on Android, you will need to "
            a { href: "https://developer.android.com/studio", "install Android Studio" }
            "."
        }
        p { "Once you have installed Android Studio, you will need to install the Android SDK and NDK:" }
        ol {
            li { "Create a blank Android project" }
            li {
                "Select "
                code { "Tools > SDK manager" }
            }
            li {
                "Navigate to the "
                code { "SDK tools" }
                " window:"
            }
        }
        p {
            img {
                src: asset!(
                    "/assets/static/android_ndk_install.png", ImageAssetOptions::new().with_avif()
                ),
                alt: "NDK install window",
                title: "",
            }
        }
        p { "Then select:" }
        ul {
            li { "The SDK" }
            li { "The SDK Command line tools" }
            li { "The NDK (side by side)" }
            li { "CMAKE" }
        }
        ol {
            li {
                "Select "
                code { "apply" }
                " and follow the prompts"
            }
        }
        blockquote {
            p {
                "More details that could be useful for debugging any errors you encounter are available "
                a { href: "https://developer.android.com/studio/intro/update#sdk-manager",
                    "in the official android docs"
                }
            }
        }
        p { "Next set the Java, Android and NDK home variables:" }
        p { "Mac:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">export </span><span style=\"color:#ff80f4;\">JAVA_HOME</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;/Applications/Android Studio.app/Contents/jbr/Contents/Home&quot;\n</span><span style=\"color:#f8f8f2;\">export </span><span style=\"color:#ff80f4;\">ANDROID_HOME</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;$HOME/Library/Android/sdk&quot;\n</span><span style=\"color:#f8f8f2;\">export </span><span style=\"color:#ff80f4;\">NDK_HOME</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;$ANDROID_HOME/ndk/25.2.9519653&quot;</span></pre>\n" }
        p { "Windows:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[System.Environment]::SetEnvironmentVariable(</span><span style=\"color:#ffee99;\">&quot;JAVA_HOME&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;C:\\Program Files\\Android\\Android Studio\\jbr&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;User&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">[System.Environment]::SetEnvironmentVariable(</span><span style=\"color:#ffee99;\">&quot;ANDROID_HOME&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;$env:LocalAppData\\Android\\Sdk&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;User&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">[System.Environment]::SetEnvironmentVariable(</span><span style=\"color:#ffee99;\">&quot;NDK_HOME&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;$env:LocalAppData\\Android\\Sdk</span><span style=\"color:#ff80f4;\">\\n</span><span style=\"color:#ffee99;\">dk\\25.2.9519653&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;User&quot;</span><span style=\"color:#f8f8f2;\">)</span></pre>\n",
        }
        blockquote {
            p { "The NDK version in the paths should match the version you installed in the last step" }
        }
        h4 { id: "ios",
            a { href: "#ios", class: "header", "IOS" }
        }
        p {
            "To develop on IOS, you will need to "
            a { href: "https://apps.apple.com/us/app/xcode/id497799835", "install XCode" }
            "."
        }
        blockquote {
            p {
                "If you are using M1, you will have to run  "
                code { "cargo build --target x86_64-apple-ios" }
                " instead of  "
                code { "cargo apple build" }
                " if you want to run in simulator."
            }
        }
        h3 { id: "setting-up-your-project",
            a { href: "#setting-up-your-project", class: "header", "Setting up your project" }
        }
        p { "First, we need to create a rust project:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo new dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">mobile</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">test </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">lib\n</span><span style=\"color:#f8f8f2;\">cd dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">mobile</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">test</span></pre>\n" }
        p {
            "Next, we can use  "
            code { "cargo-mobile2" }
            " to create a project for mobile:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo install </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">git https:</span><span style=\"color:#8c8c8c;\">//github.com/tauri-apps/cargo-mobile2\n</span><span style=\"color:#f8f8f2;\">cargo mobile init</span></pre>\n" }
        p {
            "When you run  "
            code { "cargo mobile init" }
            ", you will be asked a series of questions about your project. One of those questions is what template you should use. Dioxus currently doesn't have a template in Tauri mobile, instead you can use the  "
            code { "wry" }
            " template."
        }
        blockquote {
            p {
                "You may also be asked to input your team ID for IOS. You can find your team id "
                a { href: "https://developer.apple.com/help/account/manage-your-team/locate-your-team-id/",
                    "here"
                }
                " or create a team id by creating a developer account "
                a { href: "https://developer.apple.com/help/account/get-started/about-your-developer-account",
                    "here"
                }
            }
        }
        p {
            "Next, we need to modify our dependencies to include dioxus and ensure the right version of wry is installed. Change the  "
            code { "[dependencies]" }
            " section of your  "
            code { "Cargo.toml" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[dependencies]\n</span><span style=\"color:#f8f8f2;\">anyhow </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;1.0&quot;\n</span><span style=\"color:#f8f8f2;\">log </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.4&quot;\n</span><span style=\"color:#f8f8f2;\">wry </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.37&quot;\n</span><span style=\"color:#f8f8f2;\">tao </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.26&quot;\n</span><span style=\"color:#f8f8f2;\">dioxus </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.6&quot;</span><span style=\"color:#f8f8f2;\">, features </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;mobile&quot;</span><span style=\"color:#f8f8f2;\">] }}</span></pre>\n",
        }
        p {
            "Finally, we need to add a component to renderer. Replace the wry template in your  "
            code { "lib.rs" }
            " file with this code:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">anyhow::Result;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[cfg(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;android&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">init_logging</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    android_logger::init_once(\n</span><span style=\"color:#f8f8f2;\">        android_logger::Config::default()\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">with_max_level</span><span style=\"color:#f8f8f2;\">(log::LevelFilter::Trace)\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[cfg(not(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;android&quot;</span><span style=\"color:#f8f8f2;\">))]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">init_logging</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    env_logger::init();\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[cfg(any(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;android&quot;</span><span style=\"color:#f8f8f2;\">, target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;ios&quot;</span><span style=\"color:#f8f8f2;\">))]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">stop_unwind</span><span style=\"color:#f8f8f2;\">&lt;F: FnOnce() -&gt; T, T&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">f</span><span style=\"color:#f8f8f2;\">: F) -&gt; T {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match </span><span style=\"color:#f8f8f2;\">std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(t) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> t,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(err) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            eprintln!(</span><span style=\"color:#ffee99;\">&quot;attempt to unwind out of `rust` with err: {{:?}}&quot;</span><span style=\"color:#f8f8f2;\">, err);\n</span><span style=\"color:#f8f8f2;\">            std::process::abort()\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[no_mangle]\n</span><span style=\"color:#f8f8f2;\">#[inline(never)]\n</span><span style=\"color:#f8f8f2;\">#[cfg(any(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;android&quot;</span><span style=\"color:#f8f8f2;\">, target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;ios&quot;</span><span style=\"color:#f8f8f2;\">))]\n</span><span style=\"color:#f92672;\">pub extern </span><span style=\"color:#ffee99;\">&quot;C&quot; </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">start_app</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">_start_app</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#66d9ef;\">stop_unwind</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#66d9ef;\">main</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    #[cfg(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;android&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    {{\n</span><span style=\"color:#f8f8f2;\">        tao::android_binding</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">            com_example,\n</span><span style=\"color:#f8f8f2;\">            dioxus_mobile_test,\n</span><span style=\"color:#f8f8f2;\">            WryActivity,\n</span><span style=\"color:#f8f8f2;\">            wry::android_setup, </span><span style=\"color:#8c8c8c;\">// pass the wry::android_setup function to tao which will invoke when the event loop is created\n</span><span style=\"color:#f8f8f2;\">            _start_app\n</span><span style=\"color:#f8f8f2;\">        );\n</span><span style=\"color:#f8f8f2;\">        wry::android_binding</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(com_example, dioxus_mobile_test);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    #[cfg(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;ios&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">_start_app</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() -&gt; Result&lt;()&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">init_logging</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(())\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> items </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| vec![</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">3</span><span style=\"color:#f8f8f2;\">]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    log::debug</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hello from the app&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            h1 {{ </span><span style=\"color:#ffee99;\">&quot;Hello, Mobile&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            div {{ margin_left: </span><span style=\"color:#ffee99;\">&quot;auto&quot;</span><span style=\"color:#f8f8f2;\">, margin_right: </span><span style=\"color:#ffee99;\">&quot;auto&quot;</span><span style=\"color:#f8f8f2;\">, width: </span><span style=\"color:#ffee99;\">&quot;200px&quot;</span><span style=\"color:#f8f8f2;\">, padding: </span><span style=\"color:#ffee99;\">&quot;10px&quot;</span><span style=\"color:#f8f8f2;\">, border: </span><span style=\"color:#ffee99;\">&quot;1px solid black&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                button {{\n</span><span style=\"color:#f8f8f2;\">                    onclick: </span><span style=\"color:#f92672;\">move|_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                        println!(</span><span style=\"color:#ffee99;\">&quot;Clicked!&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> items_mut </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> items.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> new_item </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> items_mut.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">                        items_mut.</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(new_item);\n</span><span style=\"color:#f8f8f2;\">                        println!(</span><span style=\"color:#ffee99;\">&quot;Requested update&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                    }},\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#ffee99;\">&quot;Add item&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> item </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> items.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">                    div {{ </span><span style=\"color:#ffee99;\">&quot;- {{item}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "running",
            a { href: "#running", class: "header", "Running" }
        }
        p {
            "From there, you'll want to get a build of the crate using whichever platform you're targeting (simulator or actual hardware). For now, we'll just stick with the simulator."
        }
        p { "First, you need to make sure that the build variant is correct in Android Studio:" }
        ol {
            li { "Click \"Build\" in the top menu bar." }
            li { "Click \"Select Build Variant...\" in the dropdown." }
            li {
                "Find the \"Build Variants\" panel and use the dropdown to change the selected build variant."
            }
        }
        p {
            img {
                src: asset!("/assets/static/as-build-dropdown.png", ImageAssetOptions::new().with_avif()),
                alt: "android studio build dropdown",
                title: "",
            }
            img {
                src: asset!(
                    "/assets/static/as-build-variant-menu.png", ImageAssetOptions::new().with_avif()
                ),
                alt: "android studio build variants",
                title: "",
            }
        }
        h3 { id: "android",
            a { href: "#android", class: "header", "Android" }
        }
        p { "To build your project on Android you can run:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo android build</span></pre>\n" }
        p { "Next, open Android studio:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo android open</span></pre>\n" }
        p { "This will open an android studio project for this application." }
        p {
            "Next we need to create a simulator in Android studio to run our app in. To create a simulator click on the phone icon in the top right of Android studio:"
        }
        p {
            img {
                src: asset!(
                    "/assets/static/android-studio-simulator.png", ImageAssetOptions::new()
                    .with_avif()
                ),
                alt: "android studio manage devices",
                title: "",
            }
        }
        p {
            "Then click the  "
            code { "create a virtual device" }
            " button and follow the prompts:"
        }
        p {
            img {
                src: asset!(
                    "/assets/static/android-studio-devices.png", ImageAssetOptions::new().with_avif()
                ),
                alt: "android studio devices",
                title: "",
            }
        }
        p { "Finally, launch your device by clicking the play button on the device you created:" }
        p {
            img {
                src: asset!(
                    "/assets/static/android-studio-device.png", ImageAssetOptions::new().with_avif()
                ),
                alt: "android studio device",
                title: "",
            }
        }
        p { "Now you can start your application from your terminal by running:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo android run</span></pre>\n" }
        p {
            img {
                src: asset!(
                    "/assets/static/Android-Dioxus-demo.png", ImageAssetOptions::new().with_avif()
                ),
                alt: "android_demo",
                title: "",
            }
        }
        blockquote {
            p { "More information is available in the Android docs:" }
            ul {
                li { "https://developer.android.com/ndk/guides" }
                li { "https://developer.android.com/studio/projects/install-ndk" }
                li { "https://source.android.com/docs/setup/build/rust/building-rust-modules/overview" }
            }
        }
        h3 { id: "ios",
            a { href: "#ios", class: "header", "IOS" }
        }
        p { "To build your project for IOS, you can run:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo build </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">target aarch64</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">apple</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">ios</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">sim</span></pre>\n" }
        p { "Next, open XCode (this might take awhile if you've never opened XCode before):" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo apple open</span></pre>\n" }
        p { "This will open XCode with this particular project." }
        p {
            "From there, just click the \"play\" button with the right target and the app should be running!"
        }
        p {
            img {
                src: asset!("/assets/static/IOS-dioxus-demo.png", ImageAssetOptions::new().with_avif()),
                alt: "ios_demo",
                title: "",
            }
        }
        p {
            "Note that clicking play doesn't cause a new build, so you'll need to keep rebuilding the app between changes. The tooling here is very young, so please be patient. If you want to contribute to make things easier, please do! We'll be happy to help."
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceMobileApis() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "mobile",
            a { href: "#mobile", class: "header", "Mobile" }
        }
        p { "This guide will cover concepts specific to the Dioxus mobile renderer." }
        h2 { id: "running-javascript",
            a { href: "#running-javascript", class: "header", "Running Javascript" }
        }
        p {
            "Dioxus provides some ergonomic wrappers over the browser API, but in some cases you may need to access parts of the browser API Dioxus does not expose."
        }
        p {
            "For these cases, Dioxus desktop exposes the use_eval hook that allows you to run raw Javascript in the webview:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> future </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can create as many eval instances as you want\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> eval </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">document::eval(\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;\n</span><span style=\"color:#ffee99;\">            // You can send messages from JavaScript to Rust with the dioxus.send function\n</span><span style=\"color:#ffee99;\">            dioxus.send(&quot;Hi from JS!&quot;);\n</span><span style=\"color:#ffee99;\">            // You can receive messages from Rust to JavaScript with the dioxus.recv function\n</span><span style=\"color:#ffee99;\">            let msg = await dioxus.recv();\n</span><span style=\"color:#ffee99;\">            console.log(msg);\n</span><span style=\"color:#ffee99;\">            &quot;#</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        );\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can send messages to JavaScript with the send method\n</span><span style=\"color:#f8f8f2;\">        eval.</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hi from Rust!&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can receive any message from JavaScript with the recv method\n</span><span style=\"color:#f8f8f2;\">        eval.recv::&lt;String&gt;().await.</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> future.</span><span style=\"color:#66d9ef;\">read_unchecked</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">as_ref</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(v) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            p {{ </span><span style=\"color:#ffee99;\">&quot;{{v}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">_ =&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            p {{ </span><span style=\"color:#ffee99;\">&quot;hello&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "eval.rs".to_string(),
        }
        h2 { id: "custom-assets",
            a { href: "#custom-assets", class: "header", "Custom Assets" }
        }
        p { "You can link to local assets in dioxus mobile instead of using a url:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            img {{ src: </span><span style=\"color:#ffee99;\">&quot;/public/static/scanner.png&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "custom_assets.rs".to_string(),
        }
        h2 { id: "integrating-with-wry",
            a { href: "#integrating-with-wry", class: "header", "Integrating with Wry" }
        }
        p {
            "In cases where you need more low level control over your window, you can use wry APIs exposed through the "
            a { href: "https://docs.rs/dioxus-desktop/0.5.0/dioxus_desktop/struct.Config.html",
                "Desktop Config"
            }
            " and the "
            a { href: "https://docs.rs/dioxus-desktop/0.5.0/dioxus_desktop/struct.DesktopContext.html",
                "use_window hook"
            }
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceSsr() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "server-side-rendering",
            a { href: "#server-side-rendering", class: "header", "Server-Side Rendering" }
        }
        p {
            "For lower-level control over the rendering process, you can use the  "
            code { "dioxus-ssr" }
            " crate directly. This can be useful when integrating with a web framework that  "
            code { "dioxus-fullstack" }
            " does not support, or pre-rendering pages."
        }
        h2 { id: "setup",
            a { href: "#setup", class: "header", "Setup" }
        }
        p {
            "For this guide, we're going to show how to use Dioxus SSR with "
            a { href: "https://docs.rs/axum/latest/axum/", "Axum" }
            "."
        }
        p { "Make sure you have Rust and Cargo installed, and then create a new project:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo new </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">bin demo\n</span><span style=\"color:#f8f8f2;\">cd demo</span></pre>\n" }
        p { "Add Dioxus and the ssr renderer as dependencies:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add dioxus</span><span style=\"color:#f92672;\">@</span><span style=\"color:#ff80f4;\">0.5</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#ff80f4;\">0\n</span><span style=\"color:#f8f8f2;\">cargo add dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">ssr</span><span style=\"color:#f92672;\">@</span><span style=\"color:#ff80f4;\">0.5</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#ff80f4;\">0</span></pre>\n" }
        p {
            "Next, add all the Axum dependencies. This will be different if you're using a different Web Framework"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add tokio </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features full\n</span><span style=\"color:#f8f8f2;\">cargo add axum</span></pre>\n" }
        p { "Your dependencies should look roughly like this:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[dependencies]\n</span><span style=\"color:#f8f8f2;\">axum </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.7&quot;\n</span><span style=\"color:#f8f8f2;\">dioxus </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;*&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">ssr </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;*&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">tokio </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;1.15.0&quot;</span><span style=\"color:#f8f8f2;\">, features </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;full&quot;</span><span style=\"color:#f8f8f2;\">] }}</span></pre>\n",
        }
        p { "Now, set up your Axum app to respond on an endpoint." }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">axum::{{response::Html, routing::get, Router}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[tokio::main]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> listener </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">tokio::net::TcpListener::bind(</span><span style=\"color:#ffee99;\">&quot;127.0.0.1:3000&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .await\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;listening on http://127.0.0.1:3000&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    axum::serve(\n</span><span style=\"color:#f8f8f2;\">        listener,\n</span><span style=\"color:#f8f8f2;\">        Router::new()\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">route</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">(app_endpoint))\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">into_make_service</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">    )\n</span><span style=\"color:#f8f8f2;\">    .await\n</span><span style=\"color:#f8f8f2;\">    .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "ssr.rs".to_string(),
        }
        p {
            "And then add our endpoint. We can either render  "
            code { "rsx!" }
            " directly:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app_endpoint</span><span style=\"color:#f8f8f2;\">() -&gt; Html&lt;String&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// render the rsx! macro to HTML\n</span><span style=\"color:#f8f8f2;\">    Html(dioxus_ssr::render_element(rsx! {{ div {{ </span><span style=\"color:#ffee99;\">&quot;hello world!&quot; </span><span style=\"color:#f8f8f2;\">}} }}))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "ssr.rs".to_string(),
        }
        p { "Or we can render VirtualDoms." }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app_endpoint</span><span style=\"color:#f8f8f2;\">() -&gt; Html&lt;String&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// create a component that renders a div with the text &quot;hello world&quot;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">        rsx! {{ div {{ </span><span style=\"color:#ffee99;\">&quot;hello world&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// create a VirtualDom with the app component\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> app </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">VirtualDom::new(app);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// rebuild the VirtualDom before rendering\n</span><span style=\"color:#f8f8f2;\">    app.</span><span style=\"color:#66d9ef;\">rebuild_in_place</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// render the VirtualDom to HTML\n</span><span style=\"color:#f8f8f2;\">    Html(dioxus_ssr::render(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">app))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "ssr.rs".to_string(),
        }
        p {
            "Finally, you can run it using  "
            code { "cargo run" }
            " rather than  "
            code { "dx serve" }
            "."
        }
        h2 { id: "multithreaded-support",
            a { href: "#multithreaded-support", class: "header", "Multithreaded Support" }
        }
        p {
            "The Dioxus VirtualDom, sadly, is not currently  "
            code { "Send" }
            ". Internally, we use quite a bit of interior mutability which is not thread-safe."
            code { "Send" }
            ", it is possible to render a VirtualDom immediately to a String – but you cannot hold the VirtualDom across an await point. For retained-state SSR (essentially LiveView), you'll need to spawn a VirtualDom on its own thread and communicate with it via channels or create a pool of VirtualDoms."
            em { "must" }
            " remain on the thread it started. We are working on loosening this requirement."
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceFullstackIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "fullstack-development",
            a { href: "#fullstack-development", class: "header", "Fullstack development" }
        }
        p { "Dioxus Fullstack contains helpers for:" }
        ul {
            li { "Incremental, static, and server side rendering" }
            li { "Hydrating your application on the Client" }
            li { "Communicating between a server and a client" }
        }
        p {
            "This guide will teach you everything you need to know about how to use the utilities in Dioxus fullstack to create amazing fullstack applications."
        }
        blockquote {
            p {
                "In addition to this guide, you can find more examples of full-stack apps and information about how to integrate with other frameworks and desktop renderers in the "
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/fullstack/examples",
                    "dioxus-fullstack examples directory"
                }
                "."
            }
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceFullstackServerFunctions() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "communicating-with-the-server",
            a { href: "#communicating-with-the-server", class: "header",
                "Communicating with the server"
            }
        }
        p {
            code { "dioxus-fullstack" }
            " provides server functions that allow you to call an automatically generated API on the server from the client as if it were a local function."
        }
        p {
            "To make a server function, simply add the  "
            code { "#[server(YourUniqueType)]" }
            " attribute to a function. The function must:"
        }
        ul {
            li { "Be an async function" }
            li {
                "Have arguments and a return type that both implement serialize and deserialize (with "
                a { href: "https://serde.rs/", "serde" }
                ")."
            }
            li {
                "Return a "
                code { "Result" }
                " with an error type of ServerFnError"
            }
        }
        blockquote {
            p {
                "If you are targeting WASM on the server with WASI, you must call  "
                code { "register" }
                " on the type you passed into the server macro in your main function before starting your server to tell Dioxus about the server function. For all other targets, the server function will be registered automatically."
            }
        }
        p {
            "Let's continue building on the app we made in the "
            a { href: "../../getting_started/fullstack", "getting started" }
            " guide. We will add a server function to our app that allows us to double the count on the server."
        }
        p { "First, add serde as a dependency:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add serde</span></pre>\n" }
        p {
            "Next, add the server function to your  "
            code { "main.rs" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case)]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(App)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;High-Five counter: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Up high!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Down low!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Ok</span><span style=\"color:#f8f8f2;\">(new_count) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">double_server</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">()).await {{\n</span><span style=\"color:#f8f8f2;\">                        count.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(new_count);\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Double&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[server]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">double_server</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">number</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; Result&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">, ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Perform some expensive computation or access a database on the server\n</span><span style=\"color:#f8f8f2;\">    tokio::time::sleep(std::time::Duration::from_secs(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">)).await;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> result </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> number </span><span style=\"color:#f92672;\">* </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;server calculated </span><span style=\"color:#ff80f4;\">{{result}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(result)\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "server_function.rs".to_string(),
        }
        p {
            "Now, build your client-side bundle with  "
            code { "dx build --features web" }
            " and run your server with  "
            code { "cargo run --features ssr" }
            ". You should see a new button that multiplies the count by 2."
        }
        h2 { id: "cached-data-fetching",
            a { href: "#cached-data-fetching", class: "header", "Cached data fetching" }
        }
        p { "One common use case for server functions is fetching data from the server:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case, unused)]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(get_server_data);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{</span><span style=\"color:#ffee99;\">&quot;server data is {{count.value():?}}&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[server]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_server_data</span><span style=\"color:#f8f8f2;\">() -&gt; Result&lt;String, ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Access a database\n</span><span style=\"color:#f8f8f2;\">    tokio::time::sleep(std::time::Duration::from_millis(</span><span style=\"color:#ff80f4;\">100</span><span style=\"color:#f8f8f2;\">)).await;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hello from the server!&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "server_data_fetch.rs".to_string(),
        }
        p {
            "If you navigate to the site above, you will first see  "
            code { "server data is None" }
            ", then after the  "
            code { "WASM" }
            " has loaded and the request to the server has finished, you will see  "
            code { "server data is Some(Ok(\"Hello from the server!\"))" }
            "."
        }
        p {
            "This approach works, but it can be slow. Instead of waiting for the client to load and send a request to the server, what if we could get all of the data we needed for the page on the server and send it down to the client with the initial HTML page?"
        }
        p {
            "This is exactly what the  "
            code { "use_server_future" }
            " hook allows us to do!  "
            code { "use_server_future" }
            " is similar to the  "
            code { "use_resource" }
            " hook, but it allows you to wait for a future on the server and send the result of the future down to the client."
        }
        p {
            "Let's change our data fetching to use  "
            code { "use_server_future" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case, unused)]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_server_future</span><span style=\"color:#f8f8f2;\">(get_server_data)</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{</span><span style=\"color:#ffee99;\">&quot;server data is {{count.value():?}}&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[server]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_server_data</span><span style=\"color:#f8f8f2;\">() -&gt; Result&lt;String, ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Access a database\n</span><span style=\"color:#f8f8f2;\">    tokio::time::sleep(std::time::Duration::from_millis(</span><span style=\"color:#ff80f4;\">100</span><span style=\"color:#f8f8f2;\">)).await;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hello from the server!&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "server_data_prefetch.rs".to_string(),
        }
        blockquote {
            p {
                "Notice the  "
                code { "?" }
                " after  "
                code { "use_server_future" }
                ". This is what tells Dioxus fullstack to wait for the future to resolve before continuing rendering. If you want to not wait for a specific future, you can just remove the ? and deal with the  "
                code { "Option" }
                " manually."
            }
        }
        p {
            "Now when you load the page, you should see  "
            code { "server data is Ok(\"Hello from the server!\")" }
            ". No need to wait for the  "
            code { "WASM" }
            " to load or wait for the request to finish!"
        }
        SandBoxFrame { url: "https://codesandbox.io/p/sandbox/dioxus-fullstack-server-future-qwpp4p?file=/src/main.rs:3,24" }
        h2 { id: "running-the-client-with-dioxus-desktop",
            a {
                href: "#running-the-client-with-dioxus-desktop",
                class: "header",
                "Running the client with dioxus-desktop"
            }
        }
        p {
            "The project presented so far makes a web browser interact with the server, but it is also possible to make a desktop program interact with the server in a similar fashion. (The full example code is available in the "
            a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/fullstack/examples/axum-desktop",
                "Dioxus repo"
            }
            ")"
        }
        p {
            "First, we need to make two binary targets, one for the desktop program (the  "
            code { "client.rs" }
            " file), one for the server (the  "
            code { "server.rs" }
            " file). The client app and the server functions are written in a shared  "
            code { "lib.rs" }
            " file."
        }
        p {
            "The desktop and server targets have slightly different build configuration to enable additional dependencies or features. "
        }
        ul {
            li {
                "the client.rs has to be run with the "
                code { "desktop" }
                " feature, so that the optional "
                code { "dioxus-desktop" }
                " dependency is included"
            }
            li {
                "the server.rs has to be run with the "
                code { "ssr" }
                " features; this will generate the server part of the server functions and will run our backend server."
            }
        }
        p { "Once you create your project, you can run the server executable with:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo run </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">bin server </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features ssr</span></pre>\n" }
        p { "and the client desktop executable with:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo run </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">bin client </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features desktop</span></pre>\n" }
        h3 { id: "client-code",
            a { href: "#client-code", class: "header", "Client code" }
        }
        p {
            "The client file is pretty straightforward. You only need to set the server url in the client code, so it knows where to send the network requests. Then, dioxus_desktop launches the app."
        }
        p {
            "For development, the example project runs the server on  "
            code { "localhost:8080" }
            ". "
            strong { "Before you release remember to update the url to your production url." }
        }
        h3 { id: "server-code",
            a { href: "#server-code", class: "header", "Server code" }
        }
        p {
            "In the server code, first you have to set the network address and port where the server will listen to."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> listener </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">tokio::net::TcpListener::bind(</span><span style=\"color:#ffee99;\">&quot;127.0.0.1:3000&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">    .await\n</span><span style=\"color:#f8f8f2;\">    .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;listening on http://127.0.0.1:3000&quot;</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "server_function_desktop_client.rs".to_string(),
        }
        p {
            "Then, you have to register the types declared in the server function macros into the server."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[server(GetServerData)]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_server_data</span><span style=\"color:#f8f8f2;\">() -&gt; Result&lt;String, ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hello from the server!&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "server_function_desktop_client.rs".to_string(),
        }
        p {
            "The  "
            code { "GetServerData" }
            " type has to be registered in the server, which will add the corresponding route to the server."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n</pre>\n",
            name: "server_function_desktop_client.rs".to_string(),
        }
        p { "Finally, the server is started and it begins responding to requests." }
    }
}
#[component(no_case_check)]
pub fn ReferenceFullstackExtractors() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "extractors",
            a { href: "#extractors", class: "header", "Extractors" }
        }
        p {
            "Server functions are an ergonomic way to call a function on the server. Server function work by registering an endpoint on the server and using requests on the client. Most of the time, you shouldn't need to worry about how server functions operate, but there are some times when you need to get some value from the request other than the data passed in the server function."
        }
        p {
            "For example, requests contain information about the user's browser (called the "
            a { href: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent",
                "user agent"
            }
            "). We can use an extractor to retrieve that information."
        }
        p {
            "You can use the  "
            code { "extract" }
            " method within a server function to extract something from the request. You can extract any type that implements  "
            code { "FromServerContext" }
            " (or when axum is enabled, you can use axum extractors directly):"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// #[server]\n</span><span style=\"color:#8c8c8c;\">// pub async fn log_headers() -&gt; Result&lt;(), ServerFnError&gt; {{\n</span><span style=\"color:#8c8c8c;\">//     let headers: http::HeaderMap = extract().await?;\n</span><span style=\"color:#8c8c8c;\">//     log::info!(&quot;{{:?}}&quot;, headers[http::header::USER_AGENT]);\n</span><span style=\"color:#8c8c8c;\">//     Ok(())\n</span><span style=\"color:#8c8c8c;\">// }}</span></pre>\n",
            name: "server_function_extract.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceFullstackMiddleware() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "middleware",
            a { href: "#middleware", class: "header", "Middleware" }
        }
        p {
            "Extractors allow you to wrap your server function in some code that changes either the request or the response. Dioxus fullstack integrates with "
            a { href: "https://docs.rs/tower/latest/tower/index.html", "Tower" }
            " to allow you to wrap your server functions in middleware."
        }
        p {
            "You can use the  "
            code { "#[middleware]" }
            " attribute to add a layer of middleware to your server function. Let's add a timeout middleware to a server function. This middleware will stop running the server function if it reaches a certain timeout:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// #[server]\n</span><span style=\"color:#8c8c8c;\">// // Add a timeout middleware to the server function that will return an error if the function takes longer than 1 second to execute\n</span><span style=\"color:#8c8c8c;\">// #[middleware(tower_http::timeout::TimeoutLayer::new(std::time::Duration::from_secs(1)))]\n</span><span style=\"color:#8c8c8c;\">// pub async fn timeout() -&gt; Result&lt;(), ServerFnError&gt; {{\n</span><span style=\"color:#8c8c8c;\">//     tokio::time::sleep(std::time::Duration::from_secs(2)).await;\n</span><span style=\"color:#8c8c8c;\">//     Ok(())\n</span><span style=\"color:#8c8c8c;\">// }}</span></pre>\n",
            name: "server_function_middleware.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceFullstackAuthentication() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "authentication",
            a { href: "#authentication", class: "header", "Authentication" }
        }
        p {
            "You can use "
            a { href: "./extractors", "extractors" }
            " to integrate auth with your Fullstack application."
        }
        p {
            "You can create a custom extractors that extracts the auth session from the request. From that auth session, you can check if the user has the required privileges before returning the private data."
        }
        p {
            "A "
            a { href: "https://github.com/DioxusLabs/dioxus/blob/v0.5/packages/fullstack/examples/axum-auth/src/main.rs",
                "full auth example"
            }
            " with the complete implementation is available in the fullstack examples."
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceFullstackRouting() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "routing",
            a { href: "#routing", class: "header", "Routing" }
        }
        p {
            "You can easily integrate your fullstack application with a client side router using Dioxus Router. This allows you to create different scenes in your app and navigate between them. You can read more about the router in the "
            a { href: "../router", "router reference" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case)]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">axum::Router;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_router::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">serde::{{Deserialize, Serialize}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(|| rsx! {{ Router::&lt;Route&gt; {{}} }});\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    Home {{}},\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/blog/:id&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    Blog {{ id: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Blog</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Link {{ to: Route::Home {{}}, </span><span style=\"color:#ffee99;\">&quot;Go to counter&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        table {{\n</span><span style=\"color:#f8f8f2;\">            tbody {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">for _ in </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">..</span><span style=\"color:#f8f8f2;\">id {{\n</span><span style=\"color:#f8f8f2;\">                    tr {{\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"color:#f92672;\">for _ in </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">..</span><span style=\"color:#f8f8f2;\">id {{\n</span><span style=\"color:#f8f8f2;\">                            td {{ </span><span style=\"color:#ffee99;\">&quot;hello world!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                        }}\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Home</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> text </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;...&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Link {{ to: Route::Blog {{ id: </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() }}, </span><span style=\"color:#ffee99;\">&quot;Go to blog&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            h1 {{ </span><span style=\"color:#ffee99;\">&quot;High-Five counter: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Up high!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Down low!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            button {{\n</span><span style=\"color:#f8f8f2;\">                onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Ok</span><span style=\"color:#f8f8f2;\">(data) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">get_server_data</span><span style=\"color:#f8f8f2;\">().await {{\n</span><span style=\"color:#f8f8f2;\">                            println!(</span><span style=\"color:#ffee99;\">&quot;Client received: </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, data);\n</span><span style=\"color:#f8f8f2;\">                            text.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(data.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">                            </span><span style=\"color:#66d9ef;\">post_server_data</span><span style=\"color:#f8f8f2;\">(data).await.</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">                        }}\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }},\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;Run server function!&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Server said: {{text}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[server(PostServerData)]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">post_server_data</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">data</span><span style=\"color:#f8f8f2;\">: String) -&gt; Result&lt;(), ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;Server received: </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, data);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(())\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[server(GetServerData)]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_server_data</span><span style=\"color:#f8f8f2;\">() -&gt; Result&lt;String, ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hello from the server!&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "server_router.rs".to_string(),
        }
        SandBoxFrame { url: "https://codesandbox.io/p/sandbox/dioxus-fullstack-router-s75v5q?file=%2Fsrc%2Fmain.rs%3A7%2C1" }
    }
}
#[component(no_case_check)]
pub fn CookbookPublishing() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "publishing",
            a { href: "#publishing", class: "header", "Publishing" }
        }
        p {
            "After you have build your application, you will need to publish it somewhere. This reference will outline different methods of publishing your desktop or web application."
        }
        h2 { id: "web-publishing-with-github-pages",
            a { href: "#web-publishing-with-github-pages", class: "header",
                "Web: Publishing with GitHub Pages"
            }
        }
        p {
            "Edit your  "
            code { "Dioxus.toml" }
            " to point your  "
            code { "out_dir" }
            " to the  "
            code { "docs" }
            " folder and the  "
            code { "base_path" }
            " to the name of your repo:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[application]\n</span><span style=\"color:#f92672;\"># ...\n</span><span style=\"color:#f8f8f2;\">out_dir </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;docs&quot;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">[web.app]\n</span><span style=\"color:#f8f8f2;\">base_path </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;your_repo&quot;</span></pre>\n" }
        p { "Then build your app and publish it to Github:" }
        ul {
            li {
                "Make sure GitHub Pages is set up for your repo to publish any static files in the docs directory"
            }
            li { "Build your app with:" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dx build </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">release</span></pre>\n" }
        ul {
            li {
                "Make a copy of your "
                code { "docs/index.html" }
                " file and rename the copy to "
                code { "docs/404.html" }
                " so that your app will work with client-side routing"
            }
            li { "Add and commit with git" }
            li { "Push to GitHub" }
        }
        h2 { id: "desktop-creating-an-installer",
            a { href: "#desktop-creating-an-installer", class: "header",
                "Desktop: Creating an installer"
            }
        }
        p {
            "Dioxus desktop app uses your operating system's WebView library, so it's portable to be distributed for other platforms."
        }
        p { "In this section, we'll cover how to bundle your app for macOS, Windows, and Linux." }
        h2 { id: "preparing-your-application-for-bundling",
            a {
                href: "#preparing-your-application-for-bundling",
                class: "header",
                "Preparing your application for bundling"
            }
        }
        p {
            "Depending on your platform, you may need to add some additional code to your  "
            code { "main.rs" }
            " file to make sure your app is ready for bundling. On Windows, you'll need to add the  "
            code { "#![windows_subsystem = \"windows\"]" }
            " attribute to your  "
            code { "main.rs" }
            " file to hide the terminal window that pops up when you run your app. "
            strong { "If you're developing on Windows, only use this when bundling." }
            " It will disable the terminal, so you will not get logs of any kind. You can gate it behind a feature, like so:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> Cargo.toml\n</span><span style=\"color:#f8f8f2;\">[features]\n</span><span style=\"color:#f8f8f2;\">bundle </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[]</span></pre>\n" }
        p {
            "And then your  "
            code { "main.rs" }
            ":"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![cfg_attr(feature </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;bundle&quot;</span><span style=\"color:#f8f8f2;\">, windows_subsystem </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;windows&quot;</span><span style=\"color:#f8f8f2;\">)]</span></pre>\n" }
        h2 { id: "adding-assets-to-your-application",
            a { href: "#adding-assets-to-your-application", class: "header",
                "Adding assets to your application"
            }
        }
        p {
            "If you want to bundle assets with your application, you can either use them with the  "
            code { "manganis" }
            " crate (covered more in the "
            a { href: "../reference/assets", "assets" }
            " page), or you can include them in your "
            code { "Dioxus.toml" }
            " file:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[bundle]\n</span><span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> The list of files to include </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> the bundle. These can contain globs.\n</span><span style=\"color:#f8f8f2;\">resources </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;main.css&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;header.svg&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;**/*.png&quot;</span><span style=\"color:#f8f8f2;\">]</span></pre>\n" }
        h2 { id: "install",
            a { href: "#install", class: "header", "Install " }
            code { "dioxus CLI" }
        }
        p {
            "The first thing we'll do is install the "
            a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/cli",
                "dioxus-cli"
            }
            ". This extension to cargo will make it very easy to package our app for the various platforms."
        }
        p { "To install, simply run" }
        p {
            code { "cargo install dioxus-cli" }
        }
        h2 { id: "building",
            a { href: "#building", class: "header", "Building" }
        }
        p {
            "To bundle your application you can simply run  "
            code { "dx bundle --release" }
            " (also add  "
            code { "--features bundle" }
            " if you're using that, see the "
            a { href: "#preparing-your-application-for-bundling", "this" }
            " for more) to produce a final app with all the optimizations and assets builtin."
        }
        p {
            "Once you've ran the command, your app should be accessible in  "
            code { "dist/bundle/" }
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
#[component(no_case_check)]
pub fn CookbookAntipatterns() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "antipatterns",
            a { href: "#antipatterns", class: "header", "Antipatterns" }
        }
        p {
            "This example shows what not to do and provides a reason why a given pattern is considered an \"AntiPattern\". Most anti-patterns are considered wrong for performance or code re-usability reasons."
        }
        h2 { id: "unnecessarily-nested-fragments",
            a { href: "#unnecessarily-nested-fragments", class: "header",
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
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ Don&#39;t unnecessarily nest fragments\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">_ = </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    Fragment {{\n</span><span style=\"color:#f8f8f2;\">        Fragment {{\n</span><span style=\"color:#f8f8f2;\">            Fragment {{\n</span><span style=\"color:#f8f8f2;\">                Fragment {{\n</span><span style=\"color:#f8f8f2;\">                    Fragment {{ div {{ </span><span style=\"color:#ffee99;\">&quot;Finally have a real node!&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Render shallow structures\n</span><span style=\"color:#f8f8f2;\">rsx! {{ div {{ </span><span style=\"color:#ffee99;\">&quot;Finally have a real node!&quot; </span><span style=\"color:#f8f8f2;\">}} }}</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "incorrect-iterator-keys",
            a { href: "#incorrect-iterator-keys", class: "header", "Incorrect Iterator Keys" }
        }
        p {
            "As described in the "
            a { href: "../reference/dynamic_rendering#the", "dynamic rendering chapter" }
            ", list items must have unique keys that are associated with the same items across renders. This helps Dioxus associate state with the contained components and ensures good diffing performance. Do not omit keys, unless you know that the list will never change."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> data: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">HashMap&lt;</span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">&gt; </span><span style=\"color:#f92672;\">= &amp;</span><span style=\"color:#f8f8f2;\">props.data;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ❌ No keys\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ul {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> data.</span><span style=\"color:#66d9ef;\">values</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            li {{ </span><span style=\"color:#ffee99;\">&quot;List item: {{value}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ❌ Using index as keys\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ul {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">(index , value) </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> data.</span><span style=\"color:#66d9ef;\">values</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">enumerate</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            li {{ key: </span><span style=\"color:#ffee99;\">&quot;{{index}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;List item: {{value}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Using unique IDs as keys:\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ul {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">(key , value) </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> props.data.</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            li {{ key: </span><span style=\"color:#ffee99;\">&quot;{{key}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;List item: {{value}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "avoid-interior-mutability-in-props",
            a { href: "#avoid-interior-mutability-in-props", class: "header",
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
            " component, that component may wish to write to the  "
            code { "username" }
            " field. However, when it does, the parent component will not be aware of the change, and the component will not re-render which causes the UI to be out of sync with the state. Instead, consider passing down a reactive value like a  "
            code { "Signal" }
            " or immutable data."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ Mutex/RwLock/RefCell in props\n</span><span style=\"color:#f8f8f2;\">#[derive(Props, Clone)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">AntipatternInteriorMutability {{\n</span><span style=\"color:#f8f8f2;\">    map: Rc&lt;RefCell&lt;HashMap&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u32</span><span style=\"color:#f8f8f2;\">, String&gt;&gt;&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">PartialEq </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">AntipatternInteriorMutability {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">eq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">other</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">Self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">bool </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        std::rc::Rc::ptr_eq(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">self.map, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">other.map)\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">AntipatternInteriorMutability</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">map</span><span style=\"color:#f8f8f2;\">: Rc&lt;RefCell&lt;HashMap&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u32</span><span style=\"color:#f8f8f2;\">, String&gt;&gt;&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> map </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> map.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#8c8c8c;\">// Writing to map will not rerun any components\n</span><span style=\"color:#f8f8f2;\">                    map.</span><span style=\"color:#66d9ef;\">borrow_mut</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">insert</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Mutate map&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Since writing to map will not rerun any components, this will get out of date\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;{{map.borrow().get(&amp;0).unwrap()}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Use a signal to pass mutable state\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">AntipatternInteriorMutabilitySignal</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">map</span><span style=\"color:#f8f8f2;\">: Signal&lt;HashMap&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u32</span><span style=\"color:#f8f8f2;\">, String&gt;&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// Writing to map will rerun any components that read the map\n</span><span style=\"color:#f8f8f2;\">                map.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">insert</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Mutate map&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Since writing to map will rerun subscribers, this will get updated\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;{{map.read().get(&amp;0).unwrap()}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "avoid-updating-state-during-render",
            a { href: "#avoid-updating-state-during-render", class: "header",
                "Avoid Updating State During Render"
            }
        }
        p {
            "Every time you update the state, Dioxus needs to re-render the component – this is inefficient! Consider refactoring your code to avoid this."
        }
        p {
            "Also, if you unconditionally update the state during render, it will be re-rendered in an infinite loop."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ Updating state in render\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> first_signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> second_signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Updating the state during a render can easily lead to infinite loops\n</span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">first_signal</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f92672;\">!= </span><span style=\"color:#66d9ef;\">second_signal</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    second_signal.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#66d9ef;\">first_signal</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Update state in an effect\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> first_signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> second_signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// The closure you pass to use_effect will be rerun whenever any of the dependencies change without re-rendering the component\n</span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">first_signal</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f92672;\">!= </span><span style=\"color:#66d9ef;\">second_signal</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        second_signal.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#66d9ef;\">first_signal</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Deriving state with use_memo\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> first_signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#8c8c8c;\">// Memos are specifically designed for derived state. If your state fits this pattern, use it.\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> second_signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#66d9ef;\">first_signal</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "avoid-large-groups-of-state",
            a { href: "#avoid-large-groups-of-state", class: "header",
                "Avoid Large Groups of State"
            }
        }
        p {
            "It can be tempting to have a single large state struct that contains all of your application's state. However, this can lead to issues:"
        }
        ul {
            li { "It can be easy to accidentally mutate the state in a way that causes an infinite loop" }
            li { "It can be difficult to reason about when and how the state is updated" }
            li {
                "It can lead to performance issues because many components will need to re-render when the state changes"
            }
        }
        p {
            "Instead, consider breaking your state into smaller, more manageable pieces. This will make it easier to reason about the state, avoid update loops, and improve performance."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// ❌ Large state struct\n</span><span style=\"color:#f8f8f2;\">    #[derive(Props, Clone, PartialEq)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">LargeState {{\n</span><span style=\"color:#f8f8f2;\">        users: Vec&lt;User&gt;,\n</span><span style=\"color:#f8f8f2;\">        logged_in: </span><span style=\"font-style:italic;color:#66d9ef;\">bool</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        warnings: Vec&lt;String&gt;,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    #[derive(Props, Clone, PartialEq)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">User {{\n</span><span style=\"color:#f8f8f2;\">        name: String,\n</span><span style=\"color:#f8f8f2;\">        email: String,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> all_my_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| LargeState {{\n</span><span style=\"color:#f8f8f2;\">        users: vec![User {{\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#ffee99;\">&quot;Alice&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            email: </span><span style=\"color:#ffee99;\">&quot;alice@example.com&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        }}],\n</span><span style=\"color:#f8f8f2;\">        logged_in: </span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        warnings: vec![],\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// It is very easy to accidentally read and write to the state object if it contains all your state\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> read </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> all_my_state.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> logged_in </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> read.logged_in;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if !</span><span style=\"color:#f8f8f2;\">logged_in {{\n</span><span style=\"color:#f8f8f2;\">            all_my_state\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">write_unchecked</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                .warnings\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;You are not logged in&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// ✅ Use multiple signals to manage state\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> users </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        vec![User {{\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#ffee99;\">&quot;Alice&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            email: </span><span style=\"color:#ffee99;\">&quot;alice@example.com&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        }}]\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> logged_in </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> warnings </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| vec![]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Now you can read and write to separate signals which will not cause issues\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if !</span><span style=\"color:#66d9ef;\">logged_in</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            warnings.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;You are not logged in&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// ✅ Use memos to create derived state when larger states are unavoidable\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Notice we didn&#39;t split everything into separate signals. Users still make sense as a vec of data\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> users </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        vec![User {{\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#ffee99;\">&quot;Alice&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            email: </span><span style=\"color:#ffee99;\">&quot;alice@example.com&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        }}]\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> logged_in </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> warnings: Signal&lt;Vec&lt;String&gt;&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| vec![]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// In child components, you can use the memo to create derived that will only update when a specific part of the state changes\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// This will help you avoid unnecessary re-renders and infinite loops\n</span><span style=\"color:#f8f8f2;\">    #[component]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">FirstUser</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">users</span><span style=\"color:#f8f8f2;\">: Signal&lt;Vec&lt;User&gt;&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> first_user </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> users.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">first</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        rsx! {{\n</span><span style=\"color:#f8f8f2;\">            div {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;First user: {{first_user().name}}&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        FirstUser {{\n</span><span style=\"color:#f8f8f2;\">            users\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "running-non-deterministic-code-in-the-body-of-a-component",
            a {
                href: "#running-non-deterministic-code-in-the-body-of-a-component",
                class: "header",
                "Running Non-Deterministic Code in the Body of a Component"
            }
        }
        p {
            "If you have a component that contains non-deterministic code, that code should generally not be run in the body of the component. If it is put in the body of the component, it will be executed every time the component is re-rendered which can lead to performance issues."
        }
        p {
            "Instead, consider moving the non-deterministic code into a hook that only runs when the component is first created or an effect that reruns when dependencies change."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ Non-deterministic code in the body of a component\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">NonDeterministic</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> my_random_id </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">rand::random::&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u64</span><span style=\"color:#f8f8f2;\">&gt;();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// Id will change every single time the component is re-rendered\n</span><span style=\"color:#f8f8f2;\">            id: </span><span style=\"color:#ffee99;\">&quot;{{my_random_id}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello {{name}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Use a hook to run non-deterministic code\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">NonDeterministicHook</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// If you store the result of the non-deterministic code in a hook, it will stay the same between renders\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> my_random_id </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| rand::random::&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u64</span><span style=\"color:#f8f8f2;\">&gt;());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            id: </span><span style=\"color:#ffee99;\">&quot;{{my_random_id}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello {{name}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "overly-permissive-partialeq-for-props",
            a {
                href: "#overly-permissive-partialeq-for-props",
                class: "header",
                "Overly Permissive PartialEq for Props"
            }
        }
        p {
            "You may have noticed that  "
            code { "Props" }
            " requires a  "
            code { "PartialEq" }
            " implementation. That  "
            code { "PartialEq" }
            " is very important for Dioxus to work correctly. It is used to determine if a component should re-render or not when the parent component re-renders."
        }
        p {
            "If you cannot derive  "
            code { "PartialEq" }
            " for your  "
            code { "Props" }
            ", you will need to implement it yourself. If you do implement  "
            code { "PartialEq" }
            ", make sure to return  "
            code { "false" }
            " any time the props change in a way that would cause the UI in the child component to change."
        }
        p {
            "In general, returning  "
            code { "false" }
            " from  "
            code { "PartialEq" }
            " if you aren't sure if the props have changed or not is better than returning  "
            code { "true" }
            ". This will help you avoid out of date UI in your child components."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ Permissive PartialEq for Props\n</span><span style=\"color:#f8f8f2;\">#[derive(Props, Clone)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">PermissivePartialEqProps {{\n</span><span style=\"color:#f8f8f2;\">    name: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// This will cause the component to **never** re-render when the parent component re-renders\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">PartialEq </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">PermissivePartialEqProps {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">eq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">Self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">bool </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">PermissivePartialEq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: PermissivePartialEqProps) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello {{name.name}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">PermissivePartialEqParent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;Alice&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        PermissivePartialEq {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// The PermissivePartialEq component will not get the updated value of name because the PartialEq implementation says that the props are the same\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#66d9ef;\">name</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Derive PartialEq for Props\n</span><span style=\"color:#f8f8f2;\">#[derive(Props, Clone, PartialEq)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">DerivePartialEqProps {{\n</span><span style=\"color:#f8f8f2;\">    name: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">DerivePartialEq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: DerivePartialEqProps) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello {{name.name}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">DerivePartialEqParent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;Alice&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        DerivePartialEq {{\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#66d9ef;\">name</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Return false from PartialEq if you are unsure if the props have changed\n</span><span style=\"color:#f8f8f2;\">#[derive(Debug)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">NonPartialEq;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Props, Clone)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">RcPartialEqProps {{\n</span><span style=\"color:#f8f8f2;\">    name: Rc&lt;NonPartialEq&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">PartialEq </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">RcPartialEqProps {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">eq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">other</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">Self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">bool </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// This will almost always return false because the Rc will likely point to a different value\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Implementing PartialEq for NonPartialEq would be better, but if it is controlled by another library, it may not be possible\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// **Always** return false if you are unsure if the props have changed\n</span><span style=\"color:#f8f8f2;\">        std::rc::Rc::ptr_eq(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">self.name, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">other.name)\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">RcPartialEq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: RcPartialEqProps) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello {{name.name:?}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">RcPartialEqParent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| Rc::new(NonPartialEq));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        RcPartialEq {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// Generally, RcPartialEq will rerun even if the value of name hasn&#39;t actually changed because the Rc will point to a different value\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#66d9ef;\">name</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn CookbookErrorHandling() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "error-handling",
            a { href: "#error-handling", class: "header", "Error handling" }
        }
        p {
            "A selling point of Rust for web development is the reliability of always knowing where errors can occur and being forced to handle them"
        }
        p {
            "However, we haven't talked about error handling at all in this guide! In this chapter, we'll cover some strategies in handling errors to ensure your app never crashes."
        }
        h2 { id: "the-simplest--returning-none",
            a { href: "#the-simplest--returning-none", class: "header",
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
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{  }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
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
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// immediately return &quot;None&quot;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| dioxus::Result::Ok(</span><span style=\"color:#ffee99;\">&quot;hi&quot;</span><span style=\"color:#f8f8f2;\">))</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        h2 { id: "early-return-on-result",
            a { href: "#early-return-on-result", class: "header", "Early return on result" }
        }
        p {
            "Because Rust can't accept both Options and Results with the existing try infrastructure, you'll need to manually handle Results. This can be done by converting them into Options or by explicitly handling them. If you choose to convert your Result into an Option and bubble it with a  "
            code { "?" }
            ", keep in mind that if you do hit an error you will lose error information and nothing will be rendered for that component."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Convert Result to Option\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;1.234&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">context</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Failed to parse&quot;</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Early return\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;1.234&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> val: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= match</span><span style=\"color:#f8f8f2;\"> count.</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(val) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> val,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(err) </span><span style=\"color:#f92672;\">=&gt; return </span><span style=\"color:#f8f8f2;\">rsx! {{ </span><span style=\"color:#ffee99;\">&quot;Parsing failed&quot; </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">    }};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        p {
            "Notice that while hooks in Dioxus do not like being called in conditionals or loops, they "
            em { "are" }
            " okay with early returns. Returning an error state early is a completely valid way of handling errors."
        }
        h2 { id: "match-results",
            a { href: "#match-results", class: "header", "Match results" }
        }
        p {
            "The next \"best\" way of handling errors in Dioxus is to match on the error locally. This is the most robust way of handling errors, but it doesn't scale to architectures beyond a single component."
        }
        p { "To do this, we simply have an error state built into our component:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        p {
            "Whenever we perform an action that generates an error, we'll set that error state. We can then match on the error in a number of ways (early return, return Element, etc)."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Commandline</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match </span><span style=\"color:#66d9ef;\">error</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(error) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            h1 {{ </span><span style=\"color:#ffee99;\">&quot;An error occurred&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            input {{ oninput: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> error.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;bad thing happened!&quot;</span><span style=\"color:#f8f8f2;\">)) }}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        h2 { id: "passing-error-states-through-components",
            a {
                href: "#passing-error-states-through-components",
                class: "header",
                "Passing error states through components"
            }
        }
        p {
            "If you're dealing with a handful of components with minimal nesting, you can just pass the error handle into child components."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Commandline</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(error) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">error</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">return </span><span style=\"color:#f8f8f2;\">rsx! {{ </span><span style=\"color:#ffee99;\">&quot;An error occurred&quot; </span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Child {{ error }}\n</span><span style=\"color:#f8f8f2;\">        Child {{ error }}\n</span><span style=\"color:#f8f8f2;\">        Child {{ error }}\n</span><span style=\"color:#f8f8f2;\">        Child {{ error }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">error</span><span style=\"color:#f8f8f2;\">: Signal&lt;Option&lt;</span><span style=\"color:#f92672;\">&amp;&#39;static </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">&gt;&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        input {{ oninput: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> error.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;bad thing happened!&quot;</span><span style=\"color:#f8f8f2;\">)) }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        p {
            "Much like before, our child components can manually set the error during their own actions. The advantage to this pattern is that we can easily isolate error states to a few components at a time, making our app more predictable and robust."
        }
        h2 { id: "throwing-errors",
            a { href: "#throwing-errors", class: "header", "Throwing errors" }
        }
        p {
            "Dioxus provides a much easier way to handle errors: throwing them. Throwing errors combines the best parts of an error state and early return: you can easily throw and error with  "
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
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        ErrorBoundary {{\n</span><span style=\"color:#f8f8f2;\">            handle_error: |</span><span style=\"font-style:italic;color:#fd971f;\">ctx</span><span style=\"color:#f8f8f2;\">: ErrorContext| {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= &amp;</span><span style=\"color:#f8f8f2;\">ctx.</span><span style=\"color:#66d9ef;\">errors</span><span style=\"color:#f8f8f2;\">()[</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">];\n</span><span style=\"color:#f8f8f2;\">                rsx! {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;Oops, we encountered an error. Please report {{error}} to the developer of this application&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            ThrowsError {{}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ThrowsError</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;1.234&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">context</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Failed to parse&quot;</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        p {
            "You can even nest  "
            code { "ErrorBoundary" }
            " components to capture errors at different levels of your app."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        ErrorBoundary {{\n</span><span style=\"color:#f8f8f2;\">            handle_error: |</span><span style=\"font-style:italic;color:#fd971f;\">ctx</span><span style=\"color:#f8f8f2;\">: ErrorContext| {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= &amp;</span><span style=\"color:#f8f8f2;\">ctx.</span><span style=\"color:#66d9ef;\">errors</span><span style=\"color:#f8f8f2;\">()[</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">];\n</span><span style=\"color:#f8f8f2;\">                rsx! {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;Hmm, something went wrong. Please report {{error}} to the developer of this application&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            Parent {{}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        ErrorBoundary {{\n</span><span style=\"color:#f8f8f2;\">            handle_error: |</span><span style=\"font-style:italic;color:#fd971f;\">ctx</span><span style=\"color:#f8f8f2;\">: ErrorContext| {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= &amp;</span><span style=\"color:#f8f8f2;\">ctx.</span><span style=\"color:#66d9ef;\">errors</span><span style=\"color:#f8f8f2;\">()[</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">];\n</span><span style=\"color:#f8f8f2;\">                rsx! {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;The child component encountered an error: {{error}}&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            ThrowsError {{}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ThrowsError</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;1.234&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">context</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Failed to parse&quot;</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        p {
            "This pattern is particularly helpful whenever your code generates a non-recoverable error. You can gracefully capture these \"global\" error states without panicking or handling state for each error yourself."
        }
    }
}
#[component(no_case_check)]
pub fn CookbookIntegrationsIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p {
            "This section of the guide provides getting started guides for common tools used with Dioxus."
        }
        ul {
            li {
                a { href: "./logging", "Logging" }
            }
            li {
                a { href: "./internationalization", "Internationalization" }
            }
        }
    }
}
#[component(no_case_check)]
pub fn CookbookIntegrationsLogging() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "logging",
            a { href: "#logging", class: "header", "Logging" }
        }
        p {
            "Dioxus has a wide range of supported platforms, each with their own logging requirements. We'll discuss the different options available for your projects."
        }
        h4 { id: "the-tracing-crate",
            a { href: "#the-tracing-crate", class: "header", "The Tracing Crate" }
        }
        p {
            "The "
            a { href: "https://crates.io/crates/tracing", "Tracing" }
            " crate is the logging interface that the Dioxus library uses. It is not required to use the Tracing crate, but you will not recieve logs from the Dioxus library."
        }
        p {
            "The Tracing crate provides a variety of simple  "
            code { "println" }
            "-like macros with varying levels of severity. "
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    tracing::trace</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;trace&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    tracing::debug</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;debug&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    tracing::info</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;info&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    tracing::warn</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;warn&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    tracing::error</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;error&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "All the loggers provided on this page are, besides configuration and initialization, interfaced using these macros. Often you will also utilize the Tracing crate's  "
            code { "Level" }
            " enum. This enum usually represents the maximum log severity you want your application to emit and can be loaded from a variety of sources such as configuration file, environment variable, and more."
        }
        p {
            "For more information, visit the Tracing crate's "
            a { href: "https://docs.rs/tracing/latest/tracing/", "docs" }
            "."
        }
        h2 { id: "dioxus-logger",
            a { href: "#dioxus-logger", class: "header", "Dioxus Logger" }
        }
        p {
            a { href: "https://crates.io/crates/dioxus-logger", "Dioxus Logger" }
            " is a logging utility that will start the appropriate logger for the platform. Currently every platform except mobile is supported."
        }
        p {
            "To use Dioxus Logger, call the  "
            code { "init()" }
            " function:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">tracing::Level;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Init logger\n</span><span style=\"color:#f8f8f2;\">    dioxus_logger::init(Level::</span><span style=\"color:#ff80f4;\">INFO</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">expect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;failed to init logger&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Dioxus launch code\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "The  "
            code { "dioxus_logger::init()" }
            " function initializes Dioxus Logger with the appropriate tracing logger using the default configuration and provided  "
            code { "Level" }
            "."
        }
        h4 { id: "platform-intricacies",
            a { href: "#platform-intricacies", class: "header", "Platform Intricacies" }
        }
        p {
            "On web, Dioxus Logger will use "
            a { href: "https://crates.io/crates/tracing-wasm", "tracing-wasm" }
            ". On Desktop and server-based targets, Dioxus Logger will use "
            a { href: "https://crates.io/crates/tracing-subscriber", "tracing-subscriber" }
            "'s "
            code { "FmtSubscriber" }
            "."
        }
        h4 { id: "final-notes",
            a { href: "#final-notes", class: "header", "Final Notes" }
        }
        p {
            "Dioxus Logger is the preferred logger to use with Dioxus if it suites your needs. There are more features to come and Dioxus Logger is planned to become an integral part of Dioxus. If there are any feature suggestions or issues with Dioxus Logger, feel free to reach out on the "
            a { href: "https://discord.gg/XgGxMSkvUM", "Dioxus Discord Server" }
            "!"
        }
        p {
            "For more information, visit Dioxus Logger's "
            a { href: "https://docs.rs/dioxus-logger/latest/dioxus_logger/", "docs" }
            "."
        }
        h2 { id: "desktop-and-server",
            a { href: "#desktop-and-server", class: "header", "Desktop and Server" }
        }
        p { "For Dioxus' desktop and server targets, you can generally use the logger of your choice." }
        p { "Some popular options are:" }
        ul {
            li {
                a { href: "https://crates.io/crates/tracing-subscriber", "tracing-subscriber" }
                "'s "
                code { "FmtSubscriber" }
                " for console output."
            }
            li {
                a { href: "https://crates.io/crates/tracing-appender", "tracing-appender" }
                " for logging to files."
            }
            li {
                a { href: "https://crates.io/crates/tracing-bunyan-formatter",
                    "tracing-bunyan-formatter"
                }
                " for the Bunyan format."
            }
        }
        p { "To keep this guide short, we will not be covering the usage of these crates." }
        p {
            "For a full list of popular tracing-based logging crates, visit "
            a { href: "https://docs.rs/tracing/latest/tracing/#related-crates", "this" }
            " list in the Tracing crate's docs."
        }
        h2 { id: "web",
            a { href: "#web", class: "header", "Web" }
        }
        p {
            a { href: "https://crates.io/crates/tracing-wasm", "tracing-wasm" }
            " is a logging interface that can be used with Dioxus' web platform."
        }
        p {
            "The easiest way to use WASM Logger is with the  "
            code { "set_as_global_default" }
            " function:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Init logger\n</span><span style=\"color:#f8f8f2;\">    tracing_wasm::set_as_global_default();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Dioxus code\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "This starts tracing with a  "
            code { "Level" }
            " of  "
            code { "Trace" }
            ". "
        }
        p {
            "Using a custom  "
            code { "level" }
            " is a little trickier. We need to use the  "
            code { "WasmLayerConfigBuilder" }
            " and start the logger with  "
            code { "set_as_global_default_with_config()" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">tracing::Level;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Init logger\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> tracing_config </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">tracing_wasm::WASMLayerConfigBuilder::new().</span><span style=\"color:#66d9ef;\">set_max_level</span><span style=\"color:#f8f8f2;\">(Level::</span><span style=\"color:#ff80f4;\">INFO</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">build</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    tracing_wasm::set_as_global_default_with_config(tracing_config);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Dioxus code\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h1 { id: "mobile",
            a { href: "#mobile", class: "header", "Mobile" }
        }
        p {
            "Unfortunately there are no tracing crates that work with mobile targets. As an alternative you can use the "
            a { href: "https://crates.io/crates/log", "log" }
            " crate."
        }
        h2 { id: "android",
            a { href: "#android", class: "header", "Android" }
        }
        p {
            a { href: "https://crates.io/crates/android_logger", "Android Logger" }
            " is a logging interface that can be used when targeting Android. Android Logger runs whenever an event "
            code { "native_activity_create" }
            " is called by the Android system:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">log::LevelFilter;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">android_logger::Config;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">native_activity_create</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    android_logger::init_once(\n</span><span style=\"color:#f8f8f2;\">        Config::default()\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">with_max_level</span><span style=\"color:#f8f8f2;\">(LevelFilter::Info)\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">with_tag</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;myapp&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "The  "
            code { "with_tag()" }
            " is what your app's logs will show as."
        }
        h4 { id: "viewing-logs",
            a { href: "#viewing-logs", class: "header", "Viewing Logs" }
        }
        p { "Android logs are sent to logcat. To use logcat through the Android debugger, run:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">adb </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">d logcat</span></pre>\n" }
        p { "Your Android device will need developer options/usb debugging enabled." }
        p {
            "For more information, visit android_logger's "
            a { href: "https://docs.rs/android_logger/latest/android_logger/", "docs" }
            "."
        }
        h2 { id: "ios",
            a { href: "#ios", class: "header", "iOS" }
        }
        p {
            "The current option for iOS is the "
            a { href: "https://crates.io/crates/oslog", "oslog" }
            " crate. "
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Init logger\n</span><span style=\"color:#f8f8f2;\">    OsLogger::new(</span><span style=\"color:#ffee99;\">&quot;com.example.test&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">level_filter</span><span style=\"color:#f8f8f2;\">(LevelFilter::Debug)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">init</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">expect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;failed to init logger&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Dioxus code\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h4 { id: "viewing-logs",
            a { href: "#viewing-logs", class: "header", "Viewing Logs" }
        }
        p { "You can view the emitted logs in Xcode. " }
        p {
            "For more information, visit "
            a { href: "https://crates.io/crates/oslog", "oslog" }
            ". "
        }
    }
}
#[component(no_case_check)]
pub fn CookbookIntegrationsInternationalization() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "internationalization",
            a { href: "#internationalization", class: "header", "Internationalization" }
        }
        p {
            "If you application supports multiple languages, the "
            a { href: "https://github.com/DioxusLabs/sdk", "Dioxus SDK" }
            " crate contains helpers to make working with translations in your application easier."
        }
        h2 { id: "the-full-code-for-internationalization",
            a {
                href: "#the-full-code-for-internationalization",
                class: "header",
                "The full code for internationalization"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_sdk::i18n::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_sdk::translate;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::str::FromStr;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">EN_US</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;{{\n</span><span style=\"color:#ffee99;\">    &quot;id&quot;: &quot;en-US&quot;,\n</span><span style=\"color:#ffee99;\">    &quot;texts&quot;: {{\n</span><span style=\"color:#ffee99;\">        &quot;messages&quot;: {{\n</span><span style=\"color:#ffee99;\">            &quot;hello_world&quot;: &quot;Hello World!&quot;\n</span><span style=\"color:#ffee99;\">        }},\n</span><span style=\"color:#ffee99;\">        &quot;messages.hello&quot;: &quot;Hello {{name}}&quot;\n</span><span style=\"color:#ffee99;\">    }}\n</span><span style=\"color:#ffee99;\">}}&quot;#</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">ES_ES</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;{{\n</span><span style=\"color:#ffee99;\">    &quot;id&quot;: &quot;es-ES&quot;,\n</span><span style=\"color:#ffee99;\">    &quot;texts&quot;: {{\n</span><span style=\"color:#ffee99;\">        &quot;messages&quot;: {{\n</span><span style=\"color:#ffee99;\">            &quot;hello_world&quot;: &quot;Hola Mundo!&quot;\n</span><span style=\"color:#ffee99;\">        }},\n</span><span style=\"color:#ffee99;\">        &quot;messages.hello&quot;: &quot;Hola {{name}}&quot;\n</span><span style=\"color:#ffee99;\">    }}\n</span><span style=\"color:#ffee99;\">}}&quot;#</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[allow(non_snake_case)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Body</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> i18 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_i18</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> change_to_english </span><span style=\"color:#f92672;\">= move |_|</span><span style=\"color:#f8f8f2;\"> i18.</span><span style=\"color:#66d9ef;\">set_language</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;en-US&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> change_to_spanish </span><span style=\"color:#f92672;\">= move |_|</span><span style=\"color:#f8f8f2;\"> i18.</span><span style=\"color:#66d9ef;\">set_language</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;es-ES&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: change_to_english, label {{ </span><span style=\"color:#ffee99;\">&quot;English&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: change_to_spanish, label {{ </span><span style=\"color:#ffee99;\">&quot;Spanish&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">        p {{ {{translate!(i18, </span><span style=\"color:#ffee99;\">&quot;messages.hello_world&quot;</span><span style=\"color:#f8f8f2;\">)}} }}\n</span><span style=\"color:#f8f8f2;\">        p {{ {{translate!(i18, </span><span style=\"color:#ffee99;\">&quot;messages.hello&quot;</span><span style=\"color:#f8f8f2;\">, name: </span><span style=\"color:#ffee99;\">&quot;Dioxus&quot;</span><span style=\"color:#f8f8f2;\">)}} }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_init_i18n</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;en-US&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#ffee99;\">&quot;en-US&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">(), || {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> en_us </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Language::from_str(</span><span style=\"color:#ff80f4;\">EN_US</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> es_es </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Language::from_str(</span><span style=\"color:#ff80f4;\">ES_ES</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        vec![en_us, es_es]\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ Body {{}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "i18n.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn CookbookStateIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "state-cookbook",
            a { href: "#state-cookbook", class: "header", "State Cookbook" }
        }
        ul {
            li {
                a { href: "external", "External State" }
            }
            li {
                a { href: "custom_hooks", "Custom Hook" }
            }
        }
    }
}
#[component(no_case_check)]
pub fn CookbookStateExternalIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "working-with-external-state",
            a { href: "#working-with-external-state", class: "header",
                "Working with External State"
            }
        }
        p {
            "This guide will help you integrate your Dioxus application with some external state like a different thread or a websocket connection."
        }
        h2 { id: "working-with-non-reactive-state",
            a { href: "#working-with-non-reactive-state", class: "header",
                "Working with non-reactive State"
            }
        }
        p {
            a { href: "../../reference/use_coroutine", "Coroutines" }
            " are great tool for dealing with non-reactive (state you don't render directly) state within your application."
        }
        p {
            "You can store your state inside the coroutine async block and communicate with the coroutine with messages from any child components."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// import futures::StreamExt to use the next() method\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">futures::StreamExt;\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> response_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> tx </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_coroutine</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move |mut</span><span style=\"color:#f8f8f2;\"> rx</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Define your state before the loop\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">reqwest::Client::new();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> cache: HashMap&lt;String, String&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">HashMap::new();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">loop </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Loop and wait for the next message\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(request) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> rx.</span><span style=\"color:#66d9ef;\">next</span><span style=\"color:#f8f8f2;\">().await {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// Resolve the message\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> response </span><span style=\"color:#f92672;\">= if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(response) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> cache.</span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">request) {{\n</span><span style=\"color:#f8f8f2;\">                response.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">            }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> response </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> state\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">request)\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                    .await\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">text</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                    .await\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">                cache.</span><span style=\"color:#66d9ef;\">insert</span><span style=\"color:#f8f8f2;\">(request, response.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">                response\n</span><span style=\"color:#f8f8f2;\">            }};\n</span><span style=\"color:#f8f8f2;\">            response_state.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(response));\n</span><span style=\"color:#f8f8f2;\">        }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">break</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#8c8c8c;\">// Send a message to the coroutine\n</span><span style=\"color:#f8f8f2;\">tx.</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;https://example.com&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#8c8c8c;\">// Get the current state of the coroutine\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> response </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> response_state.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();</span></pre>\n",
            name: "use_coroutine.rs".to_string(),
        }
        h2 { id: "making-reactive-state-external",
            a { href: "#making-reactive-state-external", class: "header",
                "Making Reactive State External"
            }
        }
        p {
            "If you have some reactive state (state that is rendered), that you want to modify from another thread, you can use a signal that is sync. Signals take an optional second generic value with information about syncness. Sync signals have a slightly higher overhead than thread local signals, but they can be used in a multithreaded environment."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal_sync</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        std::thread::spawn(</span><span style=\"color:#f92672;\">move || loop </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            std::thread::sleep(std::time::Duration::from_secs(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// You can easily update the signal from a different thread\n</span><span style=\"color:#f8f8f2;\">            signal </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">        }});\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> signal </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Increase&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;{{signal}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "sync_signal.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn CookbookStateCustomHooksIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "custom-hooks",
            a { href: "#custom-hooks", class: "header", "Custom Hooks" }
        }
        p {
            "Hooks are a great way to encapsulate business logic. If none of the existing hooks work for your problem, you can write your own."
        }
        p {
            "When writing your hook, you can make a function that starts with  "
            code { "use_" }
            " and takes any arguments you need. You can then use the  "
            code { "use_hook" }
            " method to create a hook that will be called the first time the component is rendered."
        }
        h2 { id: "composing-hooks",
            a { href: "#composing-hooks", class: "header", "Composing Hooks" }
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
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">use_settings</span><span style=\"color:#f8f8f2;\">() -&gt; Signal&lt;AppSettings&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">consume_context</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_composed.rs".to_string(),
        }
        p {
            "Or if you want to wrap a hook that persists reloads with the storage API, you can build on top of the use_signal hook to work with mutable state:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">gloo_storage::{{LocalStorage, Storage}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">serde::{{de::DeserializeOwned, Serialize}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">/// A persistent storage hook that can be used to store data across application reloads.\n</span><span style=\"color:#f8f8f2;\">#[allow(clippy::needless_return)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">use_persistent</span><span style=\"color:#f8f8f2;\">&lt;T: Serialize </span><span style=\"color:#f92672;\">+</span><span style=\"color:#f8f8f2;\"> DeserializeOwned </span><span style=\"color:#f92672;\">+</span><span style=\"color:#f8f8f2;\"> Default </span><span style=\"color:#f92672;\">+ &#39;static</span><span style=\"color:#f8f8f2;\">&gt;(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// A unique key for the storage entry\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">key</span><span style=\"color:#f8f8f2;\">: impl ToString,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// A function that returns the initial value if the storage entry is empty\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">init</span><span style=\"color:#f8f8f2;\">: impl FnOnce() -&gt; T,\n</span><span style=\"color:#f8f8f2;\">) -&gt; UsePersistent&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Use the use_signal hook to create a mutable state for the storage entry\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// This closure will run when the hook is created\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> key </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> key.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">LocalStorage::get(key.</span><span style=\"color:#66d9ef;\">as_str</span><span style=\"color:#f8f8f2;\">()).</span><span style=\"color:#66d9ef;\">ok</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap_or_else</span><span style=\"color:#f8f8f2;\">(init);\n</span><span style=\"color:#f8f8f2;\">        StorageEntry {{ key, value }}\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Wrap the state in a new struct with a custom API\n</span><span style=\"color:#f8f8f2;\">    UsePersistent {{ inner: state }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">StorageEntry&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    key: String,\n</span><span style=\"color:#f8f8f2;\">    value: T,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">/// Storage that persists across application reloads\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">UsePersistent&lt;T: </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    inner: Signal&lt;StorageEntry&lt;T&gt;&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;T&gt; Clone </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">UsePersistent&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">clone</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">self\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;T&gt; Copy </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">UsePersistent&lt;T&gt; {{}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;T: Serialize </span><span style=\"color:#f92672;\">+</span><span style=\"color:#f8f8f2;\"> DeserializeOwned </span><span style=\"color:#f92672;\">+</span><span style=\"color:#f8f8f2;\"> Clone </span><span style=\"color:#f92672;\">+ &#39;static</span><span style=\"color:#f8f8f2;\">&gt; UsePersistent&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">/// Returns a reference to the value\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) -&gt; T {{\n</span><span style=\"color:#f8f8f2;\">        self.inner.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">().value.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">/// Sets the value\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;mut </span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">value</span><span style=\"color:#f8f8f2;\">: T) {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> inner </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">self.inner.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Write the new value to local storage\n</span><span style=\"color:#f8f8f2;\">        LocalStorage::set(inner.key.</span><span style=\"color:#66d9ef;\">as_str</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">value);\n</span><span style=\"color:#f8f8f2;\">        inner.value </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> value;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_composed.rs".to_string(),
        }
        h2 { id: "custom-hook-logic",
            a { href: "#custom-hook-logic", class: "header", "Custom Hook Logic" }
        }
        p {
            "You can use "
            a { href: "https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_hook.html",
                code { "use_hook" }
            }
            " to build your own hooks. In fact, this is what all the standard hooks are built on!"
        }
        p {
            code { "use_hook" }
            " accepts a single closure for initializing the hook. It will be only run the first time the component is rendered. The return value of that closure will be used as the value of the hook – Dioxus will take it, and store it for as long as the component is alive. On every render (not just the first one!), you will get a reference to this value."
        }
        blockquote {
            p {
                "Note: You can use the  "
                code { "use_on_destroy" }
                " hook to clean up any resources the hook uses when the component is destroyed."
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
                code { "use_signal" }
                " hook tracks state in the hook value, and uses "
                a { href: "https://docs.rs/dioxus/latest/dioxus/prelude/fn.schedule_update.html",
                    code { "schedule_update" }
                }
                " to make Dioxus re-render the component whenever it changes."
            }
        }
        p {
            "Here is a simplified implementation of the  "
            code { "use_signal" }
            " hook:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::cell::RefCell;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::rc::Rc;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::sync::Arc;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">Signal&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    value: Rc&lt;RefCell&lt;T&gt;&gt;,\n</span><span style=\"color:#f8f8f2;\">    update: Arc&lt;dyn Fn()&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;T&gt; Clone </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">Signal&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">clone</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            value: self.value.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            update: self.update.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">my_use_signal</span><span style=\"color:#f8f8f2;\">&lt;T: </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">init</span><span style=\"color:#f8f8f2;\">: impl FnOnce() -&gt; T) -&gt; Signal&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// The update function will trigger a re-render in the component cx is attached to\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> update </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">schedule_update</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Create the initial state\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Rc::new(RefCell::new(</span><span style=\"color:#66d9ef;\">init</span><span style=\"color:#f8f8f2;\">()));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        Signal {{ value, update }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;T: Clone&gt; Signal&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) -&gt; T {{\n</span><span style=\"color:#f8f8f2;\">        self.value.</span><span style=\"color:#66d9ef;\">borrow</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">value</span><span style=\"color:#f8f8f2;\">: T) {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Update the state\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">self.value.</span><span style=\"color:#66d9ef;\">borrow_mut</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> value;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Trigger a re-render on the component the state is from\n</span><span style=\"color:#f8f8f2;\">        (self.update)();\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_custom_logic.rs".to_string(),
        }
        ul {
            li {
                "The "
                code { "use_context" }
                " hook calls "
                a { href: "https://docs.rs/dioxus/latest/dioxus/prelude/fn.consume_context.html",
                    code { "consume_context" }
                }
                " (which would be expensive to call on every render) to get some context from the component"
            }
        }
        p {
            "Here is an implementation of the  "
            code { "use_context" }
            " and  "
            code { "use_context_provider" }
            " hooks:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">use_context</span><span style=\"color:#f8f8f2;\">&lt;T: </span><span style=\"color:#f92672;\">&#39;static +</span><span style=\"color:#f8f8f2;\"> Clone&gt;() -&gt; T {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#66d9ef;\">consume_context</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">use_context_provider</span><span style=\"color:#f8f8f2;\">&lt;T: </span><span style=\"color:#f92672;\">&#39;static +</span><span style=\"color:#f8f8f2;\"> Clone&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">f</span><span style=\"color:#f8f8f2;\">: impl FnOnce() -&gt; T) -&gt; T {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> val </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">f</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Provide the context state to the component\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#66d9ef;\">provide_context</span><span style=\"color:#f8f8f2;\">(val.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">        val\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_custom_logic.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn CookbookTesting() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "testing",
            a { href: "#testing", class: "header", "Testing" }
        }
        p {
            "When building application or libraries with Dioxus, you may want to include some tests to check the behavior of parts of your application. This guide will teach you how to test different parts of your Dioxus application."
        }
        h2 { id: "component-testing",
            a { href: "#component-testing", class: "header", "Component Testing" }
        }
        p {
            "You can use a combination of "
            a { href: "https://docs.rs/pretty_assertions/latest/pretty_assertions/",
                "pretty-assertions"
            }
            " and "
            a { href: "", "dioxus-ssr" }
            " to check that two snippets of rsx are equal:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">futures::FutureExt;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::{{cell::RefCell, sync::Arc}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[test]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">test</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">assert_rsx_eq</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">        rsx! {{\n</span><span style=\"color:#f8f8f2;\">            div {{ </span><span style=\"color:#ffee99;\">&quot;Hello world&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            div {{ </span><span style=\"color:#ffee99;\">&quot;Hello world&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        rsx! {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">for _ in </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">..</span><span style=\"color:#ff80f4;\">2 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                div {{ </span><span style=\"color:#ffee99;\">&quot;Hello world&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    )\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">assert_rsx_eq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">first</span><span style=\"color:#f8f8f2;\">: Element, </span><span style=\"font-style:italic;color:#fd971f;\">second</span><span style=\"color:#f8f8f2;\">: Element) {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> first </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">dioxus_ssr::render_element(first);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> second </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">dioxus_ssr::render_element(second);\n</span><span style=\"color:#f8f8f2;\">    pretty_assertions::assert_str_eq</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(first, second);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_test.rs".to_string(),
        }
        h2 { id: "hook-testing",
            a { href: "#hook-testing", class: "header", "Hook Testing" }
        }
        p {
            "When creating libraries around Dioxus, it can be helpful to make tests for your "
            a { href: "./state/custom_hooks", "custom hooks" }
            "."
        }
        p {
            "Dioxus does not currently have a full hook testing library, but you can build a bespoke testing framework by manually driving the virtual dom."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">futures::FutureExt;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::{{cell::RefCell, rc::Rc, sync::Arc, thread::Scope}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::{{dioxus_core::NoOpMutations, prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[test]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">test</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">test_hook</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">        || </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">        |</span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">value</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">proxy</span><span style=\"color:#f8f8f2;\">| </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> proxy.generation {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                value.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                assert_eq!(</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">value.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                value.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ff80f4;\">2 </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                proxy.</span><span style=\"color:#66d9ef;\">rerun</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ff80f4;\">3 </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{}}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">_ =&gt; </span><span style=\"color:#f8f8f2;\">todo!(),\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        |</span><span style=\"font-style:italic;color:#fd971f;\">proxy</span><span style=\"color:#f8f8f2;\">| assert_eq!(proxy.generation, </span><span style=\"color:#ff80f4;\">4</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">test_hook</span><span style=\"color:#f8f8f2;\">&lt;V: </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">&gt;(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">initialize</span><span style=\"color:#f8f8f2;\">: impl FnMut() -&gt; V + </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">check</span><span style=\"color:#f8f8f2;\">: impl FnMut(</span><span style=\"font-style:italic;color:#fd971f;\">V</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">MockProxy</span><span style=\"color:#f8f8f2;\">) + </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">final_check</span><span style=\"color:#f8f8f2;\">: impl FnMut(</span><span style=\"font-style:italic;color:#fd971f;\">MockProxy</span><span style=\"color:#f8f8f2;\">) + </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">    #[derive(Props)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">MockAppComponent&lt;I: </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">, C: </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">        hook: Rc&lt;RefCell&lt;I&gt;&gt;,\n</span><span style=\"color:#f8f8f2;\">        check: Rc&lt;RefCell&lt;C&gt;&gt;,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;I, C&gt; PartialEq </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">MockAppComponent&lt;I, C&gt; {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">eq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">Self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">bool </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;I, C&gt; Clone </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">MockAppComponent&lt;I, C&gt; {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">clone</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                hook: self.hook.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                check: self.check.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">mock_app</span><span style=\"color:#f8f8f2;\">&lt;I: FnMut() -&gt; V, C: FnMut(V, MockProxy), V&gt;(\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#fd971f;\">props</span><span style=\"color:#f8f8f2;\">: MockAppComponent&lt;I, C&gt;,\n</span><span style=\"color:#f8f8f2;\">    ) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> props.hook.</span><span style=\"color:#66d9ef;\">borrow_mut</span><span style=\"color:#f8f8f2;\">()();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        props.check.</span><span style=\"color:#66d9ef;\">borrow_mut</span><span style=\"color:#f8f8f2;\">()(value, MockProxy::new());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        rsx! {{ div {{}} }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> vdom </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">VirtualDom::new_with_props(\n</span><span style=\"color:#f8f8f2;\">        mock_app,\n</span><span style=\"color:#f8f8f2;\">        MockAppComponent {{\n</span><span style=\"color:#f8f8f2;\">            hook: Rc::new(RefCell::new(initialize)),\n</span><span style=\"color:#f8f8f2;\">            check: Rc::new(RefCell::new(check)),\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    vdom.</span><span style=\"color:#66d9ef;\">rebuild_in_place</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">while</span><span style=\"color:#f8f8f2;\"> vdom.</span><span style=\"color:#66d9ef;\">wait_for_work</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">now_or_never</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">is_some</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        vdom.</span><span style=\"color:#66d9ef;\">render_immediate</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;mut</span><span style=\"color:#f8f8f2;\"> NoOpMutations);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    vdom.</span><span style=\"color:#66d9ef;\">in_runtime</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        ScopeId::</span><span style=\"color:#ff80f4;\">ROOT</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">in_runtime</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#66d9ef;\">final_check</span><span style=\"color:#f8f8f2;\">(MockProxy::new());\n</span><span style=\"color:#f8f8f2;\">        }})\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">MockProxy {{\n</span><span style=\"color:#f8f8f2;\">    rerender: Arc&lt;dyn Fn()&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">generation: </span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">MockProxy {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">new</span><span style=\"color:#f8f8f2;\">() -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> generation </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">generation</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> rerender </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">schedule_update</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            rerender,\n</span><span style=\"color:#f8f8f2;\">            generation,\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">rerun</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;mut </span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">        (self.rerender)();\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hook_test.rs".to_string(),
        }
        h2 { id: "end-to-end-testing",
            a { href: "#end-to-end-testing", class: "header", "End to End Testing" }
        }
        p {
            "You can use "
            a { href: "https://playwright.dev/", "Playwright" }
            " to create end to end tests for your dioxus application."
        }
        p {
            "In your  "
            code { "playwright.config.js" }
            ", you will need to run cargo run or dx serve instead of the default build command. Here is a snippet from the end to end web example:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">//...\n</span><span style=\"color:#f8f8f2;\">webServer: [\n</span><span style=\"color:#f8f8f2;\">    {{\n</span><span style=\"color:#f8f8f2;\">        cwd: path.</span><span style=\"color:#66d9ef;\">join</span><span style=\"color:#f8f8f2;\">(process.</span><span style=\"color:#66d9ef;\">cwd</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#f92672;\">&#39;playwright-</span><span style=\"color:#f8f8f2;\">tests</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">web</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">        command: </span><span style=\"color:#f92672;\">&#39;dx</span><span style=\"color:#f8f8f2;\"> serve</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        port: </span><span style=\"color:#ff80f4;\">8080</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        timeout: </span><span style=\"color:#ff80f4;\">10 </span><span style=\"color:#f92672;\">* </span><span style=\"color:#ff80f4;\">60 </span><span style=\"color:#f92672;\">* </span><span style=\"color:#ff80f4;\">1000</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        reuseExistingServer: </span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">process.env.</span><span style=\"color:#ff80f4;\">CI</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        stdout: </span><span style=\"color:#ffee99;\">&quot;pipe&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">],</span></pre>\n",
        }
        ul {
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/playwright-tests/web",
                    "Web example"
                }
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/playwright-tests/liveview",
                    "Liveview example"
                }
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/playwright-tests/fullstack",
                    "Fullstack example"
                }
            }
        }
    }
}
#[component(no_case_check)]
pub fn CookbookTailwind() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "tailwind",
            a { href: "#tailwind", class: "header", "Tailwind" }
        }
        p {
            "You can style your Dioxus application with whatever CSS framework you choose, or just write vanilla CSS."
        }
        p {
            "One popular option for styling your Dioxus application is "
            a { href: "https://tailwindcss.com/", "Tailwind" }
            ". Tailwind allows you to style your elements with CSS utility classes. This guide will show you how to setup tailwind CSS with your Dioxus application."
        }
        h2 { id: "setup",
            a { href: "#setup", class: "header", "Setup" }
        }
        ol {
            li { "Install the Dioxus CLI:" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo install dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">cli</span></pre>\n" }
        ol {
            li {
                "Install npm: "
                a { href: "https://docs.npmjs.com/downloading-and-installing-node-js-and-npm",
                    "https://docs.npmjs.com/downloading-and-installing-node-js-and-npm"
                }
            }
            li {
                "Install the tailwind css cli: "
                a { href: "https://tailwindcss.com/docs/installation",
                    "https://tailwindcss.com/docs/installation"
                }
            }
            li { "Initialize the tailwind css project:" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">npx tailwindcss init</span></pre>\n" }
        p {
            "This should create a  "
            code { "tailwind.config.js" }
            " file in the root of the project."
        }
        ol {
            li {
                "Edit the "
                code { "tailwind.config.js" }
                " file to include rust files:"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">module.exports </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    mode: </span><span style=\"color:#ffee99;\">&quot;all&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    content: [\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// include all rust, html and css files in the src directory\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;./src/**/*.{{rs,html,css}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// include all html files in the output (dist) directory\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;./dist/**/*.html&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    ],\n</span><span style=\"color:#f8f8f2;\">    theme: {{\n</span><span style=\"color:#f8f8f2;\">        extend: {{}},\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">    plugins: [],\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        ol {
            li {
                "Create a "
                code { "input.css" }
                " file in the root of your project with the following content:"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">@</span><span style=\"color:#f8f8f2;\">tailwind base;\n</span><span style=\"color:#f92672;\">@</span><span style=\"color:#f8f8f2;\">tailwind components;\n</span><span style=\"color:#f92672;\">@</span><span style=\"color:#f8f8f2;\">tailwind utilities;</span></pre>\n" }
        ol {
            li {
                "Add "
                a { href: "https://github.com/DioxusLabs/manganis", "Manganis" }
                " to your project to handle asset collection."
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add manganis</span></pre>\n" }
        ol {
            li {
                "Create a link to the "
                code { "tailwind.css" }
                " file using manganis somewhere in your rust code:"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Urls are relative to your Cargo.toml file\n</span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">TAILWIND_URL</span><span style=\"color:#f8f8f2;\">: Asset </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">asset!(</span><span style=\"color:#ffee99;\">&quot;/assets/tailwind.css&quot;</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "tailwind.rs".to_string(),
        }
        h3 { id: "bonus-steps",
            a { href: "#bonus-steps", class: "header", "Bonus Steps" }
        }
        ol {
            li { "Install the tailwind css vs code extension" }
            li {
                "Go to the settings for the extension and find the experimental regex support section. Edit the setting.json file to look like this:"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#ffee99;\">&quot;tailwindCSS.experimental.classRegex&quot;</span><span style=\"color:#f8f8f2;\">: [</span><span style=\"color:#ffee99;\">&quot;class: </span><span style=\"color:#ff80f4;\">\\&quot;</span><span style=\"color:#ffee99;\">(.*)</span><span style=\"color:#ff80f4;\">\\&quot;</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">],\n</span><span style=\"color:#ffee99;\">&quot;tailwindCSS.includeLanguages&quot;</span><span style=\"color:#f8f8f2;\">: {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;rust&quot;</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;html&quot;\n</span><span style=\"color:#f8f8f2;\">}},</span></pre>\n" }
        h2 { id: "development",
            a { href: "#development", class: "header", "Development" }
        }
        ul {
            li {
                "Run the following command in the root of the project to start the tailwind css compiler:"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">npx tailwindcss </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">i .</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">input.css </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">o .</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">public</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">tailwind.css </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">watch</span></pre>\n" }
        h3 { id: "web",
            a { href: "#web", class: "header", "Web" }
        }
        ul {
            li { "Run the following command in the root of the project to start the dioxus dev server:" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dx serve</span></pre>\n" }
        ul {
            li {
                "Open the browser to "
                a { href: "http://localhost:8080", "http://localhost:8080" }
                "."
            }
        }
        h3 { id: "desktop",
            a { href: "#desktop", class: "header", "Desktop" }
        }
        ul {
            li { "Launch the dioxus desktop app:" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dx serve </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">platform desktop</span></pre>\n" }
    }
}
#[component(no_case_check)]
pub fn CookbookOptimizing() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "optimizing",
            a { href: "#optimizing", class: "header", "Optimizing" }
        }
        p {
            em {
                "Note: This is written primarily for the web, but the main optimizations will work on other platforms too."
            }
        }
        p {
            "You might have noticed that Dioxus binaries are pretty big."
            a { href: "https://github.com/tigerros/dioxus-todo-app", "TodoMVC app" }
            " weighs in at 2.36mb!"
        }
        p { "We will also discuss ways to optimize your app for increased speed." }
        p {
            "However, certain optimizations will sacrifice speed for decreased binary size or the other way around."
        }
        p {
            "To test binary sizes, we will use "
            a { href: "https://github.com/tigerros/dioxus-todo-app", "this" }
            " repository as a sample app."
            code { "no-optimizations" }
            " package will serve as the base, which weighs 2.36mb as of right now."
        }
        p { "Additional resources:" }
        ul {
            li {
                a { href: "https://rustwasm.github.io/docs/book/reference/code-size.html",
                    "WASM book - Shrinking  "
                    code { ".wasm" }
                    " code size"
                }
            }
            li {
                a { href: "https://github.com/johnthagen/min-sized-rust", "min-sized-rust" }
            }
        }
        h2 { id: "building-in-release-mode",
            a { href: "#building-in-release-mode", class: "header", "Building in release mode" }
        }
        p {
            "This is the best way to optimize. In fact, the 2.36mb figure at the start of the guide is with release mode."
        }
        p {
            "Thankfully, no matter what tool you're using to build your app, it will probably have a  "
            code { "--release" }
            " flag to do this."
        }
        p {
            "Using the "
            a { href: "https://dioxuslabs.com/learn/0.5/CLI", "Dioxus CLI" }
            " or "
            a { href: "https://trunkrs.dev/", "Trunk" }
            ":"
        }
        ul {
            li {
                "Dioxus CLI: "
                code { "dx build --release" }
            }
            li {
                "Trunk: "
                code { "trunk build --release" }
            }
        }
        h2 { id: "upx",
            a { href: "#upx", class: "header", "UPX" }
        }
        p {
            "If you're not targeting web, you can use the "
            a { href: "https://github.com/upx/upx", "UPX" }
            " CLI tool to compress your executables."
        }
        p { "Setup:" }
        ul {
            li {
                "Download a "
                a { href: "https://github.com/upx/upx/releases", "release" }
                " and extract the directory inside to a sensible location."
            }
            li { "Add the executable located in the directory to your path variable." }
        }
        p {
            "You can run  "
            code { "upx --help" }
            " to get the CLI options, but you should also view  "
            code { "upx-doc.html" }
            " for more detailed information."
        }
        p {
            "An example command might be:  "
            code { "upx --best -o target/release/compressed.exe target/release/your-executable.exe" }
            "."
        }
        h2 { id: "build-configuration",
            a { href: "#build-configuration", class: "header", "Build configuration" }
        }
        p {
            em {
                "Note: Settings defined in "
                code { ".cargo/config.toml" }
                " will override settings in "
                code { "Cargo.toml" }
                "."
            }
        }
        p {
            "Other than the  "
            code { "--release" }
            " flag, this is the easiest way to optimize your projects, and also the most effective way,"
        }
        h3 { id: "stable",
            a { href: "#stable", class: "header", "Stable" }
        }
        p {
            "This configuration is 100% stable and decreases the binary size from 2.36mb to 310kb."
            code { ".cargo/config.toml" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[profile.release]\n</span><span style=\"color:#f8f8f2;\">opt</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">level </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;z&quot;\n</span><span style=\"color:#f8f8f2;\">debug </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">false\n</span><span style=\"color:#f8f8f2;\">lto </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">codegen</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">units </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">1\n</span><span style=\"color:#f8f8f2;\">panic </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;abort&quot;\n</span><span style=\"color:#f8f8f2;\">strip </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">incremental </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">false</span></pre>\n",
        }
        p { "Links to the documentation of each value:" }
        ul {
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#opt-level",
                    code { "opt-level" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#debuginfo",
                    code { "debug" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#lto",
                    code { "lto" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#codegen-units",
                    code { "codegen-units" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#panic",
                    code { "panic" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#strip",
                    code { "strip" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#incremental",
                    code { "incremental" }
                }
            }
        }
        h3 { id: "unstable",
            a { href: "#unstable", class: "header", "Unstable" }
        }
        p {
            "This configuration contains some unstable features, but it should work just fine."
            code { ".cargo/config.toml" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[unstable]\n</span><span style=\"color:#f8f8f2;\">build</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">std </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;std&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;panic_abort&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;core&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;alloc&quot;</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">build</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">std</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">features </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;panic_immediate_abort&quot;</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">[build]\n</span><span style=\"color:#f8f8f2;\">rustflags </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;-Clto&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;-Zvirtual-function-elimination&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;-Zlocation-detail=none&quot;\n</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> Same </span><span style=\"color:#f92672;\">as in</span><span style=\"color:#f8f8f2;\"> the Stable section\n</span><span style=\"color:#f8f8f2;\">[profile.release]\n</span><span style=\"color:#f8f8f2;\">opt</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">level </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;z&quot;\n</span><span style=\"color:#f8f8f2;\">debug </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">false\n</span><span style=\"color:#f8f8f2;\">lto </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">codegen</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">units </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">1\n</span><span style=\"color:#f8f8f2;\">panic </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;abort&quot;\n</span><span style=\"color:#f8f8f2;\">strip </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">incremental </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">false</span></pre>\n",
        }
        p {
            em {
                "Note: The omitted space in each flag (e.g., "
                code { "-C<no space here>lto" }
                ") is intentional. It is not a typo."
            }
        }
        p {
            "The values in  "
            code { "[profile.release]" }
            " are documented in the "
            a { href: "#stable", "Stable" }
            " section. Links to the documentation of each value:"
        }
        ul {
            li {
                a { href: "https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags",
                    code { "[build.rustflags]" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#lto",
                    code { "-C lto" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/stable/unstable-book/compiler-flags/virtual-function-elimination.html",
                    code { "-Z virtual-function-elimination" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/stable/unstable-book/compiler-flags/location-detail.html",
                    code { "-Z location-detail" }
                }
            }
        }
        h2 { id: "wasm-opt",
            a { href: "#wasm-opt", class: "header", "wasm-opt" }
        }
        p {
            em {
                "Note: In the future, "
                code { "wasm-opt" }
                " will be supported natively through the "
                a { href: "https://crates.io/crates/dioxus-cli", "Dioxus CLI" }
                "."
            }
        }
        p {
            code { "wasm-opt" }
            " is a tool from the "
            a { href: "https://github.com/WebAssembly/binaryen", "binaryen" }
            " library that optimizes your WASM files."
            a { href: "https://github.com/WebAssembly/binaryen/releases", "binaryen release" }
            " and run this command from the package directory:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">wasm</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">opt dist</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">assets</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">dioxus</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">APP_NAME_bg.wasm </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">o dist</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">assets</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">dioxus</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">APP_NAME_bg.wasm </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">Oz</span></pre>\n" }
        p {
            "The  "
            code { "-Oz" }
            " flag specifies that  "
            code { "wasm-opt" }
            " should optimize for size. For speed, use  "
            code { "-O4" }
            "."
        }
        h2 { id: "improving-dioxus-code",
            a { href: "#improving-dioxus-code", class: "header", "Improving Dioxus code" }
        }
        p { "Let's talk about how you can improve your Dioxus code to be more performant." }
        p {
            "It's important to minimize the number of dynamic parts in your  "
            code { "rsx" }
            ", like conditional rendering."
            a { href: "../reference/dynamic_rendering", "Dynamic Rendering" }
            "."
        }
        p {
            "Also check out "
            a { href: "antipatterns", "Anti-patterns" }
            " for patterns that you should avoid."
        }
        h2 { id: "optimizing-the-size-of-assets",
            a { href: "#optimizing-the-size-of-assets", class: "header",
                "Optimizing the size of assets"
            }
        }
        p {
            "Assets can be a significant part of your app's size. Dioxus includes alpha support for first party "
            a { href: "../reference/assets", "assets" }
            ". Any assets you include with the "
            code { "mg!" }
            " macro will be optimized for production in release builds."
        }
    }
}
#[component(no_case_check)]
pub fn MigrationIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "how-to-upgrade-to-dioxus-06",
            a { href: "#how-to-upgrade-to-dioxus-06", class: "header",
                "How to Upgrade to Dioxus 0.6"
            }
        }
        p {
            "This guide will outline the API changes between the  "
            code { "0.5" }
            " and  "
            code { "0.6" }
            " releases. The  "
            code { "0.6" }
            " release contains a few breaking changes to:"
        }
        ul {
            li {
                "The "
                code { "Element" }
                " type"
            }
            li { "Prevent default" }
            li { "Assets with Manganis" }
        }
        h2 { id: "element",
            a { href: "#element", class: "header", "Element" }
        }
        p {
            "The element type has changed from  "
            code { "Option<VNode>" }
            " to  "
            code { "Result<VNode, RenderError>" }
            ". This makes it possible to bubble up errors while rendering with the  "
            code { "?" }
            " operator, but it does remove the ability to return  "
            code { "None" }
            " from a component. Instead of returning  "
            code { "None" }
            ", you can return  "
            code { "VNode::empty()" }
            " or an empty  "
            code { "rsx!" }
            " macro."
        }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> number </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    \n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">number</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">&lt; </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// ❌ In dioxus 0.6, the element type is a result, so None values cannot be returned directly\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">return </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Positive number: {{number}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Dioxus 0.6:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> number </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">number</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">&lt; </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// ✅ You can return VNode::empty() instead\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">return </span><span style=\"color:#f8f8f2;\">VNode::empty();\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">number</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">&lt; </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// ✅ Or an empty rsx! macro\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">return </span><span style=\"color:#f8f8f2;\">rsx! {{}};\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Positive number: {{number}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration.rs".to_string(),
        }
        h2 { id: "prevent-default",
            a { href: "#prevent-default", class: "header", "Prevent Default" }
        }
        p {
            "Dioxus 0.1-0.5 used the  "
            code { "prevent_default" }
            " attribute to prevent default behavior of event handlers for every event. Dioxus 0.6 introduces more fine-grained control over preventing default behavior with the  "
            code { "prevent_default" }
            " function on the event type. Instead of setting the  "
            code { "prevent_default" }
            " attribute for all events you want to prevent, you can create event handlers that call  "
            code { "event.prevent_default()" }
            "."
        }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        a {{\n</span><span style=\"color:#f8f8f2;\">            href: </span><span style=\"color:#ffee99;\">&quot;https://dioxuslabs.com&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// ❌ The prevent default attribute is deprecated in dioxus 0.6\n</span><span style=\"color:#f8f8f2;\">            prevent_default: </span><span style=\"color:#ffee99;\">&quot;onclick&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Don&#39;t navigate to dioxuslabs.com&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration.rs".to_string(),
        }
        p { "Dioxus 0.6:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        a {{\n</span><span style=\"color:#f8f8f2;\">            href: </span><span style=\"color:#ffee99;\">&quot;https://dioxuslabs.com&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// ✅ Instead, you can call event.prevent_default() inside the event handler\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> event.</span><span style=\"color:#66d9ef;\">prevent_default</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Don&#39;t navigate to dioxuslabs.com&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration.rs".to_string(),
        }
        blockquote {
            p {
                "Note: Since event handlers run on the server in Liveview, events cannot be prevented quickly inside the event handler. Because of this, the new  "
                code { "prevent_default" }
                " method does not prevent default behavior in Liveview."
            }
            p {
                "Instead you can use javascript inside the  "
                code { "onclick" }
                " handler to prevent default behavior."
            }
            CodeBlock {
                contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        a {{\n</span><span style=\"color:#f8f8f2;\">            href: </span><span style=\"color:#ffee99;\">&quot;https://dioxuslabs.com&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// ✅ In liveview, you can use javascript to prevent default behavior\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;onclick&quot;</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;event.preventDefault()&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Don&#39;t navigate to dioxuslabs.com&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
                name: "migration.rs".to_string(),
            }
        }
        h2 { id: "assets",
            a { href: "#assets", class: "header", "Assets" }
        }
        p {
            "The syntax of the  "
            code { "asset!" }
            " macro has changed in Dioxus 0.6. Instead of accepting a single argument with both the path and the configuration for the asset, you can now pass in the path as the first argument and the configuration as a optional second argument."
        }
        p {
            "The path the  "
            code { "asset!" }
            " macro accepts has also changed. Previously, the macro used to accept absolute and relative paths where relative paths were relative to the current crate directory. Now the macro only accepts absolute paths which are resolved relative to the root of the crate."
        }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        img {{\n</span><span style=\"color:#f8f8f2;\">            src: asset!(</span><span style=\"color:#66d9ef;\">image</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;./assets/static/bundle.png&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">size</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">100</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">100</span><span style=\"color:#f8f8f2;\">))\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Dioxus 0.6:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        img {{\n</span><span style=\"color:#f8f8f2;\">            src: asset!(</span><span style=\"color:#ffee99;\">&quot;/assets/static/bundle.png&quot;</span><span style=\"color:#f8f8f2;\">, ImageAssetOptions::new().</span><span style=\"color:#66d9ef;\">with_size</span><span style=\"color:#f8f8f2;\">(ImageSize::Manual {{ width: </span><span style=\"color:#ff80f4;\">100</span><span style=\"color:#f8f8f2;\">, height: </span><span style=\"color:#ff80f4;\">100 </span><span style=\"color:#f8f8f2;\">}}))\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn ContributingIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "contributing",
            a { href: "#contributing", class: "header", "Contributing" }
        }
        p {
            "Development happens in the "
            a { href: "https://github.com/DioxusLabs/dioxus", "Dioxus GitHub repository" }
            ". If you've found a bug or have an idea for a feature, please submit an issue (but first check if someone hasn't "
            a { href: "https://github.com/DioxusLabs/dioxus/issues", "done it already" }
            ")."
        }
        p {
            a { href: "https://github.com/DioxusLabs/dioxus/discussions", "GitHub discussions" }
            " can be used as a place to ask for help or talk about features. You can also join "
            a { href: "https://discord.gg/XgGxMSkvUM", "our Discord channel" }
            " where some development discussion happens."
        }
        h2 { id: "improving-docs",
            a { href: "#improving-docs", class: "header", "Improving Docs" }
        }
        p {
            "If you'd like to improve the docs, PRs are welcome! The Rust docs ("
            a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages", "source" }
            ") and this guide ("
            a { href: "https://github.com/DioxusLabs/docsite/tree/main/docs-src/0.5/en",
                "source"
            }
            ") can be found in their respective GitHub repos."
        }
        h2 { id: "working-on-the-ecosystem",
            a { href: "#working-on-the-ecosystem", class: "header", "Working on the Ecosystem" }
        }
        p {
            "Part of what makes React great is the rich ecosystem. We'd like the same for Dioxus! So if you have a library in mind that you'd like to write and many people would benefit from, it will be appreciated. You can "
            a { href: "https://www.npmjs.com/search?q=keywords:react-component", "browse npm.js" }
            " for inspiration. Once you are done, add your library to the "
            a { href: "https://github.com/DioxusLabs/awesome-dioxus", "awesome dioxus" }
            " list or share it in the "
            code { "#I-made-a-thing" }
            " channel on "
            a { href: "https://discord.gg/XgGxMSkvUM", "Discord" }
            "."
        }
        h2 { id: "bugs--features",
            a { href: "#bugs--features", class: "header", "Bugs & Features" }
        }
        p {
            "If you've fixed "
            a { href: "https://github.com/DioxusLabs/dioxus/issues", "an open issue" }
            ", feel free to submit a PR! You can also take a look at "
            a { href: "./roadmap", "the roadmap" }
            " and work on something in there. Consider "
            a { href: "https://discord.gg/XgGxMSkvUM", "reaching out" }
            " to the team first to make sure everyone's on the same page, and you don't do useless work!"
        }
        p {
            "All pull requests (including those made by a team member) must be approved by at least one other team member."
        }
        h2 { id: "before-you-contribute",
            a { href: "#before-you-contribute", class: "header", "Before you contribute" }
        }
        p {
            "You might be surprised that a lot of checks fail when making your first PR."
            em { "lots" }
            " of time, because the"
        }
        ul {
            li {
                "Format code with "
                a { href: "https://github.com/rust-lang/rustfmt", "rustfmt" }
                ":"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo fmt </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\"> src</span><span style=\"color:#8c8c8c;\">/**/</span><span style=\"color:#f92672;\">**</span><span style=\"color:#f8f8f2;\">.rs</span></pre>\n" }
        ul {
            li {
                "You might need to install some packages on Linux (Ubuntu/deb) before the following commands will complete successfully (there is also a Nix flake in the repo root):"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">sudo apt install libgdk3.</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">cil libatk1.</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libcairo2</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libpango1.</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libgdk</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">pixbuf2.</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libsoup</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">3.0</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libjavascriptcoregtk</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">4.1</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libwebkit2gtk</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">4.1</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev</span></pre>\n",
        }
        ul {
            li {
                "Check all code "
                a { href: "https://doc.rust-lang.org/cargo/commands/cargo-check.html",
                    "cargo check"
                }
                ":"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo check </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">workspace </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">examples </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">tests</span></pre>\n" }
        ul {
            li {
                "Check if "
                a { href: "https://doc.rust-lang.org/clippy/", "Clippy" }
                " generates any warnings. Please fix these!"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo clippy </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">workspace </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">examples </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">tests </span><span style=\"color:#f92672;\">-- -</span><span style=\"color:#f8f8f2;\">D warnings</span></pre>\n" }
        ul {
            li {
                "Test all code with "
                a { href: "https://doc.rust-lang.org/cargo/commands/cargo-test.html",
                    "cargo-test"
                }
                ":"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo test </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">all </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">tests</span></pre>\n" }
        ul {
            li {
                "More tests, this time with "
                a { href: "https://sagiegurari.github.io/cargo-make/", "cargo-make" }
                ". Here are all steps, including installation:"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo install </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">force cargo</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">make\n</span><span style=\"color:#f8f8f2;\">cargo make tests</span></pre>\n" }
        ul {
            li {
                "Test with Playwright. This tests the UI itself, right in a browser. Here are all steps, including installation:"
                strong {
                    "Disclaimer: This might inexplicably fail on your machine without it being your fault."
                }
                " Make that PR anyway!"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cd playwright</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">tests\n</span><span style=\"color:#f8f8f2;\">npm ci\n</span><span style=\"color:#f8f8f2;\">npm install </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">D </span><span style=\"color:#f92672;\">@</span><span style=\"color:#f8f8f2;\">playwright</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">test\n</span><span style=\"color:#f8f8f2;\">npx playwright install </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">with</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">deps\n</span><span style=\"color:#f8f8f2;\">npx playwright test</span></pre>\n" }
        h2 { id: "how-to-test-dioxus-with-local-crate",
            a { href: "#how-to-test-dioxus-with-local-crate", class: "header",
                "How to test dioxus with local crate"
            }
        }
        p {
            "If you are developing a feature, you should test it in your local setup before raising a PR. This process makes sure you are aware of your code functionality before being reviewed by peers."
        }
        ul {
            li { "Fork the following github repo (DioxusLabs/dioxus):" }
        }
        p {
            code { "https://github.com/DioxusLabs/dioxus" }
        }
        ul {
            li {
                "Create a new or use an existing rust crate (ignore this step if you will use an existing rust crate):"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo new </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">bin demo</span></pre>\n" }
        ul {
            li { "Add the dioxus dependency to your rust crate (new/existing) in Cargo.toml:" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dioxus </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ path </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;&lt;path to forked dioxus project&gt;/dioxus/packages/dioxus&quot;</span><span style=\"color:#f8f8f2;\">, features </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;web&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;router&quot;</span><span style=\"color:#f8f8f2;\">] }}</span></pre>\n" }
        p {
            "This above example is for dioxus-web, with dioxus-router. To know about the dependencies for different renderer visit "
            a { href: "https://dioxuslabs.com/learn/0.5/getting_started", "here" }
            "."
        }
        ul {
            li { "Run and test your feature" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dx serve</span></pre>\n" }
        p {
            "If this is your first time with dioxus, please read "
            a { href: "https://dioxuslabs.com/learn/0.5/guide", "the guide" }
            " to get familiar with dioxus."
        }
    }
}
#[component(no_case_check)]
pub fn ContributingProjectStructure() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "project-structure",
            a { href: "#project-structure", class: "header", "Project Structure" }
        }
        p {
            "There are many packages in the Dioxus organization. This document will help you understand the purpose of each package and how they fit together."
        }
        h2 { id: "renderers",
            a { href: "#renderers", class: "header", "Renderers" }
        }
        ul {
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/desktop",
                    "Desktop"
                }
                ": A Render that Runs Dioxus applications natively, but renders them with the system webview"
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/mobile",
                    "Mobile"
                }
                ": A Render that Runs Dioxus applications natively, but renders them with the system webview. This is currently a copy of the desktop render"
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/web",
                    "Web"
                }
                ": Renders Dioxus applications in the browser by compiling to WASM and manipulating the DOM"
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/liveview",
                    "Liveview"
                }
                ": A Render that Runs on the server, and renders using a websocket proxy in the browser"
            }
            li {
                a { href: "https://github.com/DioxusLabs/blitz/tree/master/packages/plasmo",
                    "Plasmo"
                }
                ": A Renderer that renders a HTML-like tree into a terminal"
            }
            li {
                a { href: "https://github.com/DioxusLabs/blitz/tree/master/packages/dioxus-tui",
                    "TUI"
                }
                ": A Renderer that uses Plasmo to render a Dioxus application in a terminal"
            }
            li {
                a { href: "https://github.com/DioxusLabs/blitz/tree/master/packages/blitz-core",
                    "Blitz-Core"
                }
                ": An experimental native renderer that renders a HTML-like tree using WGPU."
            }
            li {
                a { href: "https://github.com/DioxusLabs/blitz", "Blitz" }
                ": An experimental native renderer that uses Blitz-Core to render a Dioxus application using WGPU."
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/ssr",
                    "SSR"
                }
                ": A Render that Runs Dioxus applications on the server, and renders them to HTML"
            }
        }
        h2 { id: "state-managementhooks",
            a { href: "#state-managementhooks", class: "header", "State Management/Hooks" }
        }
        ul {
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/hooks",
                    "Hooks"
                }
                ": A collection of common hooks for Dioxus applications"
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/signals",
                    "Signals"
                }
                ": A experimental state management library for Dioxus applications. This currently contains a "
                code { "Copy" }
                " version of Signal"
            }
            li {
                a { href: "https://github.com/DioxusLabs/sdk", "SDK" }
                ": A collection of platform agnostic hooks to interact with system interfaces (The clipboard, camera, etc.)."
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/fermi",
                    "Fermi"
                }
                ": A global state management library for Dioxus applications."
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/router",
                    "Router"
                }
                ": A client-side router for Dioxus applications"
            }
        }
        h2 { id: "core-utilities",
            a { href: "#core-utilities", class: "header", "Core utilities" }
        }
        ul {
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/core",
                    "core"
                }
                ": The core virtual dom implementation every Dioxus application uses"
                ul {
                    li {
                        "You can read more about the architecture of the core "
                        a { href: "https://dioxuslabs.com/blog/templates-diffing/",
                            "in this blog post"
                        }
                    }
                }
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/rsx",
                    "RSX"
                }
                ": The core parsing for RSX used for hot reloading, autoformatting, and the macro"
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/core-macro",
                    "core-macro"
                }
                ": The rsx! macro used to write Dioxus applications. (This is a wrapper over the RSX crate)"
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus-html-macro", "HTML macro" }
                ": A html-like alternative to the RSX macro"
            }
        }
        h2 { id: "native-renderer-utilities",
            a { href: "#native-renderer-utilities", class: "header", "Native Renderer Utilities" }
        }
        ul {
            li {
                a { href: "https://github.com/DioxusLabs/taffy", "Taffy" }
                ": Layout engine powering Blitz-Core, Plasmo, and Bevy UI"
            }
            li {
                a { href: "https://github.com/DioxusLabs/blitz", "Blitz" }
                ": An experimental native renderer for HTML+CSS"
            }
        }
        h2 { id: "web-renderer-tooling",
            a { href: "#web-renderer-tooling", class: "header", "Web renderer tooling" }
        }
        ul {
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/html",
                    "HTML"
                }
                ": defines html specific elements, events, and attributes"
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/interpreter",
                    "Interpreter"
                }
                ": defines browser bindings used by the web and desktop renderers"
            }
        }
        h2 { id: "developer-tooling",
            a { href: "#developer-tooling", class: "header", "Developer tooling" }
        }
        ul {
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/hot-reload",
                    "hot-reload"
                }
                ": Macro that uses the RSX crate to hot reload static parts of any rsx! macro. This macro works with any non-web renderer with an "
                a { href: "https://crates.io/crates/dioxus-hot-reload", "integration" }
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/autofmt",
                    "autofmt"
                }
                ": Formats RSX code"
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/rsx-rosetta",
                    "rsx-rosetta"
                }
                ": Handles conversion between HTML and RSX"
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/cli",
                    "CLI"
                }
                ": A Command Line Interface and VSCode extension to assist with Dioxus usage"
            }
        }
    }
}
#[component(no_case_check)]
pub fn ContributingGuidingPrinciples() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "overall-goals",
            a { href: "#overall-goals", class: "header", "Overall Goals" }
        }
        p {
            "This document outlines some of the overall goals for Dioxus. These goals are not set in stone, but they represent general guidelines for the project."
        }
        p {
            "The goal of Dioxus is to make it easy to build "
            strong { "cross-platform applications that scale" }
            "."
        }
        h2 { id: "cross-platform",
            a { href: "#cross-platform", class: "header", "Cross-Platform" }
        }
        p {
            "Dioxus is designed to be cross-platform by default. This means that it should be easy to build applications that run on the web, desktop, and mobile. However, Dioxus should also be flexible enough to allow users to opt into platform-specific features when needed. The  "
            code { "use_eval" }
            " is one example of this. By default, Dioxus does not assume that the platform supports JavaScript, but it does provide a hook that allows users to opt into JavaScript when needed."
        }
        h2 { id: "performance",
            a { href: "#performance", class: "header", "Performance" }
        }
        p {
            "As Dioxus applications grow, they should remain relatively performant without the need for manual optimizations. There will be cases where manual optimizations are needed, but Dioxus should try to make these cases as rare as possible."
        }
        p {
            "One of the benefits of the core architecture of Dioxus is that it delivers reasonable performance even when components are rerendered often. It is based on a Virtual Dom which performs diffing which should prevent unnecessary re-renders even when large parts of the component tree are rerun. On top of this, Dioxus groups static parts of the RSX tree together to skip diffing them entirely."
        }
        h2 { id: "type-safety",
            a { href: "#type-safety", class: "header", "Type Safety" }
        }
        p {
            "As teams grow, the Type safety of Rust is a huge advantage. Dioxus should leverage this advantage to make it easy to build applications with large teams."
        }
        p {
            "To take full advantage of Rust's type system, Dioxus should try to avoid exposing public  "
            code { "Any" }
            " types and string-ly typed APIs where possible."
        }
        h2 { id: "developer-experience",
            a { href: "#developer-experience", class: "header", "Developer Experience" }
        }
        p { "Dioxus should be easy to learn and ergonomic to use." }
        ul {
            li {
                p {
                    "The API of Dioxus attempts to remain close to React's API where possible. This makes it easier for people to learn Dioxus if they already know React"
                }
            }
            li {
                p {
                    "We can avoid the tradeoff between simplicity and flexibility by providing multiple layers of API: One for the very common use case, one for low-level control"
                }
                ul {
                    li {
                        "Hooks: the hooks crate has the most common use cases, but "
                        code { "use_hook" }
                        " provides a way to access the underlying persistent value if needed."
                    }
                    li {
                        "The builder pattern in platform Configs: The builder pattern is used to default to the most common use case, but users can change the defaults if needed."
                    }
                }
            }
            li {
                p { "Documentation:" }
                ul {
                    li { "All public APIs should have rust documentation" }
                    li {
                        "Examples should be provided for all public features. These examples both serve as documentation and testing. They are checked by CI to ensure that they continue to compile"
                    }
                    li { "The most common workflows should be documented in the guide" }
                }
            }
        }
    }
}
#[component(no_case_check)]
pub fn ContributingRoadmap() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {}
}

use super::*;
