use crate::Route;
use dioxus::prelude::*;
use mdbook_shared::MdBook;
use std::hash::Hash;

pub use dioxus_docs_03 as router_03;
pub use dioxus_docs_04 as router_04;
pub use dioxus_docs_05 as router_05;
pub use dioxus_docs_06 as router_06;
pub use dioxus_docs_07 as router_07;
pub use dioxus_docs_blog as router_blog;
pub use dioxus_docs_examples::*;

pub enum CurrentDocsVersion {
    V07(router_07::BookRoute),
    V06(router_06::BookRoute),
    V05(router_05::BookRoute),
    V04(router_04::BookRoute),
    V03(router_03::BookRoute),
}

impl CurrentDocsVersion {
    pub fn full_version(&self) -> &'static str {
        match self {
            CurrentDocsVersion::V07(_) => router_07::BookRoute::full_version(),
            CurrentDocsVersion::V06(_) => router_06::BookRoute::full_version(),
            CurrentDocsVersion::V05(_) => router_05::BookRoute::full_version(),
            CurrentDocsVersion::V04(_) => router_04::BookRoute::full_version(),
            CurrentDocsVersion::V03(_) => router_03::BookRoute::full_version(),
        }
    }
}

pub fn use_try_current_docs_version() -> Option<CurrentDocsVersion> {
    let route = use_route();
    match route {
        Route::Docs07 { child } => Some(CurrentDocsVersion::V07(child)),
        Route::Docs06 { child } => Some(CurrentDocsVersion::V06(child)),
        Route::Docs05 { child } => Some(CurrentDocsVersion::V05(child)),
        Route::Docs04 { child } => Some(CurrentDocsVersion::V04(child)),
        Route::Docs03 { child } => Some(CurrentDocsVersion::V03(child)),
        Route::Homepage {} => None,
        Route::Components { .. } => None,
        Route::Awesome {} => None,
        Route::Deploy {} => None,
        Route::BlogList {} => None,
        Route::BlogPost { .. } => None,
        Route::Err404 { .. } => None,
    }
}

pub fn use_current_docs_version() -> CurrentDocsVersion {
    let route: Route = use_route();

    use_try_current_docs_version().unwrap_or_else(move || {
        // let r = dioxus::router::router().full_route_string();
        let r = web_sys::window()
            .and_then(|w| w.location().pathname().ok())
            .unwrap_or_else(|| "unknown".to_string());
        web_sys::console::log_1(&r.into());

        panic!(
            "No current docs version found. This should not happen. {:#?} ",
            route
        )
    })
}

pub trait AnyBookRoute: Routable + PartialEq + Hash + Eq + Clone + Copy {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section];
    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self>;
    fn global_route(&self) -> crate::Route;
    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId;
    fn book() -> &'static MdBook<Self>;
    fn use_current() -> Option<Self>;
    fn use_route() -> Self {
        Self::use_current().expect("current route to be the same as the route")
    }
    fn short_version() -> &'static str;
    fn full_version() -> &'static str;
    fn index() -> Self;
}

impl AnyBookRoute for router_blog::BookRoute {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        self.sections()
    }

    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        self.page()
    }

    fn global_route(&self) -> crate::Route {
        crate::Route::BlogPost { child: *self }
    }

    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        self.page_id()
    }
    fn book() -> &'static MdBook<Self> {
        &*router_blog::LAZY_BOOK
    }

    fn use_current() -> Option<Self> {
        let route = use_route();
        match route {
            Route::BlogPost { child } => Some(child),
            _ => None,
        }
    }
    fn short_version() -> &'static str {
        "blog"
    }
    fn full_version() -> &'static str {
        "blog"
    }
    fn index() -> Self {
        todo!()
    }
}

impl AnyBookRoute for router_03::BookRoute {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        self.sections()
    }

    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        self.page()
    }

    fn global_route(&self) -> crate::Route {
        crate::Route::Docs03 { child: *self }
    }

    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        self.page_id()
    }
    fn book() -> &'static MdBook<Self> {
        &*router_03::LAZY_BOOK
    }

    fn use_current() -> Option<Self> {
        let route = use_route();
        match route {
            Route::Docs03 { child } => Some(child),
            _ => None,
        }
    }
    fn short_version() -> &'static str {
        "0.3"
    }

    fn full_version() -> &'static str {
        "0.3.2"
    }

    fn index() -> Self {
        Self::Index {
            section: Default::default(),
        }
    }
}
impl AnyBookRoute for router_04::BookRoute {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        self.sections()
    }

    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        self.page()
    }

    fn global_route(&self) -> crate::Route {
        crate::Route::Docs04 { child: *self }
    }

    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        self.page_id()
    }
    fn book() -> &'static MdBook<Self> {
        &*router_04::LAZY_BOOK
    }

    fn use_current() -> Option<Self> {
        let route = use_route();
        match route {
            Route::Docs04 { child } => Some(child),
            _ => None,
        }
    }
    fn short_version() -> &'static str {
        "0.4"
    }

    fn full_version() -> &'static str {
        "0.4.3"
    }

    fn index() -> Self {
        Self::Index {
            section: Default::default(),
        }
    }
}
impl AnyBookRoute for router_05::BookRoute {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        self.sections()
    }

    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        self.page()
    }

    fn global_route(&self) -> crate::Route {
        crate::Route::Docs05 { child: *self }
    }

    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        self.page_id()
    }
    fn book() -> &'static MdBook<Self> {
        &*router_05::LAZY_BOOK
    }

    fn use_current() -> Option<Self> {
        let route = use_route();
        match route {
            Route::Docs05 { child } => Some(child),
            _ => None,
        }
    }
    fn short_version() -> &'static str {
        "0.5"
    }

    fn full_version() -> &'static str {
        "0.5.7"
    }

    fn index() -> Self {
        Self::Index {
            section: Default::default(),
        }
    }
}

impl AnyBookRoute for router_06::BookRoute {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        self.sections()
    }

    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        self.page()
    }

    fn global_route(&self) -> crate::Route {
        crate::Route::Docs06 { child: *self }
    }

    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        self.page_id()
    }
    fn book() -> &'static MdBook<Self> {
        &*router_06::LAZY_BOOK
    }

    fn use_current() -> Option<Self> {
        let route = use_route();
        match route {
            Route::Docs06 { child } => Some(child),
            _ => None,
        }
    }
    fn short_version() -> &'static str {
        "0.6"
    }
    fn full_version() -> &'static str {
        "0.6.3"
    }
    fn index() -> Self {
        Self::Index {
            section: Default::default(),
        }
    }
}

impl AnyBookRoute for router_07::BookRoute {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        self.sections()
    }

    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        self.page()
    }

    fn global_route(&self) -> crate::Route {
        crate::Route::Docs07 { child: *self }
    }

    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        self.page_id()
    }
    fn book() -> &'static MdBook<Self> {
        &*router_07::LAZY_BOOK
    }

    fn use_current() -> Option<Self> {
        let route = use_route();
        match route {
            Route::Docs07 { child } => Some(child),
            _ => None,
        }
    }
    fn short_version() -> &'static str {
        "0.7"
    }
    fn full_version() -> &'static str {
        "0.7.0"
    }
    fn index() -> Self {
        Self::Index {
            section: Default::default(),
        }
    }
}
