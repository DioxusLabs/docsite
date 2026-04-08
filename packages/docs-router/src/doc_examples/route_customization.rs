#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

// ANCHOR: custom_component
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    // By default, the component rendered for a route matches the variant name.
    // You can specify a different component as the second argument:
    #[route("/", HomePage)]
    Index {},
}

// This component will be rendered for the Index route
#[component]
fn HomePage() -> Element {
    rsx! { "Welcome home!" }
}
// ANCHOR_END: custom_component

fn main() {}

mod custom_segment {
    use super::*;
    use std::fmt;
    use std::str::FromStr;

    // ANCHOR: custom_dynamic_segment
    /// A locale like "en", "fr", or "es" parsed from a URL segment.
    #[derive(Clone, PartialEq, Debug)]
    struct Locale {
        language: String,
    }

    /// Display is required so the router can serialize the type back into a URL.
    impl fmt::Display for Locale {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.language)
        }
    }

    /// Any type that implements FromStr can be used as a dynamic segment.
    /// If parsing fails, the route won't match and the router moves on.
    impl FromStr for Locale {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "en" | "fr" | "es" | "de" | "ja" => Ok(Locale {
                    language: s.to_string(),
                }),
                other => Err(format!("Unknown locale: {other}")),
            }
        }
    }

    #[derive(Routable, Clone)]
    #[rustfmt::skip]
    enum Route {
        // With this route, /en/about and /fr/about will match,
        // but /xyz/about will not.
        #[route("/:locale/about")]
        About { locale: Locale },
    }

    #[component]
    fn About(locale: Locale) -> Element {
        rsx! { "Viewing the about page in {locale}" }
    }
    // ANCHOR_END: custom_dynamic_segment
}

mod custom_catch_all {
    use super::*;
    use std::fmt;

    // ANCHOR: custom_catch_all
    /// A path like /docs/en/guide/intro parsed into structured data.
    #[derive(Clone, PartialEq, Debug)]
    struct DocPath {
        locale: String,
        sections: Vec<String>,
    }

    impl fmt::Display for DocPath {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.locale)?;
            for section in &self.sections {
                write!(f, "/{section}")?;
            }
            Ok(())
        }
    }

    /// For custom catch-all types, implement both FromRouteSegments (for parsing
    /// URLs into your type) and ToRouteSegments (for serializing back to a URL).
    impl dioxus::router::routable::FromRouteSegments for DocPath {
        type Err = String;

        fn from_route_segments(segments: &[&str]) -> Result<Self, Self::Err> {
            let mut iter = segments.iter();
            let locale = iter
                .next()
                .ok_or("Missing locale segment")?
                .to_string();
            let sections = iter.map(|s| s.to_string()).collect();
            Ok(DocPath { locale, sections })
        }
    }

    impl dioxus::router::routable::ToRouteSegments for DocPath {
        fn display_route_segments(
            &self,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "/{}", self.locale)?;
            for section in &self.sections {
                write!(f, "/{section}")?;
            }
            Ok(())
        }
    }

    #[derive(Routable, Clone)]
    #[rustfmt::skip]
    enum Route {
        #[route("/docs/:..path")]
        Docs { path: DocPath },
    }

    #[component]
    fn Docs(path: DocPath) -> Element {
        rsx! {
            div { "Locale: {path.locale}" }
            div { "Sections: {path.sections:?}" }
        }
    }
    // ANCHOR_END: custom_catch_all
}

mod custom_single_query {
    use super::*;
    use std::fmt;
    use std::str::FromStr;

    // ANCHOR: custom_single_query
    /// A sort order parsed from a query parameter like ?sort=asc or ?sort=desc.
    #[derive(Clone, Default, PartialEq, Debug)]
    enum SortOrder {
        #[default]
        Asc,
        Desc,
    }

    impl fmt::Display for SortOrder {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SortOrder::Asc => write!(f, "asc"),
                SortOrder::Desc => write!(f, "desc"),
            }
        }
    }

    /// Any type that implements FromStr + Default can be used as a query parameter.
    /// If the parameter is missing or fails to parse, Default::default() is used.
    impl FromStr for SortOrder {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "asc" => Ok(SortOrder::Asc),
                "desc" => Ok(SortOrder::Desc),
                other => Err(format!("Unknown sort order: {other}")),
            }
        }
    }

    #[derive(Routable, Clone)]
    #[rustfmt::skip]
    enum Route {
        #[route("/search?:query&:sort")]
        Search {
            query: String,
            sort: SortOrder,
        },
    }

    #[component]
    fn Search(query: String, sort: SortOrder) -> Element {
        rsx! {
            div { "Searching for: {query}" }
            div { "Sort order: {sort}" }
        }
    }
    // ANCHOR_END: custom_single_query
}

mod custom_spread_query {
    use super::*;
    use std::fmt;

    // ANCHOR: custom_spread_query
    /// A custom type that parses the entire query string at once.
    /// This is useful when you need full control over query parameter handling.
    #[derive(Clone, Default, PartialEq, Debug)]
    struct SearchParams {
        query: String,
        page: usize,
        sort: String,
    }

    impl fmt::Display for SearchParams {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "query={}&page={}&sort={}",
                self.query, self.page, self.sort
            )
        }
    }

    /// Implementing From<&str> gives you FromQuery automatically.
    impl From<&str> for SearchParams {
        fn from(query: &str) -> Self {
            let mut params = SearchParams::default();
            for pair in query.split('&') {
                if let Some((key, value)) = pair.split_once('=') {
                    match key {
                        "query" => params.query = value.to_string(),
                        "page" => params.page = value.parse().unwrap_or(0),
                        "sort" => params.sort = value.to_string(),
                        _ => {}
                    }
                }
            }
            params
        }
    }

    #[derive(Routable, Clone)]
    #[rustfmt::skip]
    enum Route {
        // Use ?:..field to capture the entire query string into a single type.
        #[route("/search?:..params")]
        Search { params: SearchParams },
    }

    #[component]
    fn Search(params: SearchParams) -> Element {
        rsx! {
            div { "Query: {params.query}" }
            div { "Page: {params.page}" }
            div { "Sort: {params.sort}" }
        }
    }
    // ANCHOR_END: custom_spread_query
}

mod custom_hash {
    use super::*;
    use std::str::FromStr;

    // ANCHOR: custom_hash
    /// A section anchor parsed from a hash fragment like #section-intro.
    #[derive(Clone, Default, PartialEq, Debug)]
    struct SectionAnchor {
        section: String,
        subsection: String,
    }

    impl std::fmt::Display for SectionAnchor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}-{}", self.section, self.subsection)
        }
    }

    /// Any type that implements FromStr + Default gets FromHashFragment
    /// automatically. Parsing failures return Default::default().
    impl FromStr for SectionAnchor {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.split_once('-') {
                Some((section, sub)) => Ok(SectionAnchor {
                    section: section.to_string(),
                    subsection: sub.to_string(),
                }),
                None => Ok(SectionAnchor {
                    section: s.to_string(),
                    subsection: String::new(),
                }),
            }
        }
    }

    #[derive(Routable, Clone)]
    #[rustfmt::skip]
    enum Route {
        #[route("/page#:anchor")]
        Page { anchor: SectionAnchor },
    }

    #[component]
    fn Page(anchor: SectionAnchor) -> Element {
        rsx! {
            div { "Section: {anchor.section}" }
            div { "Subsection: {anchor.subsection}" }
        }
    }
    // ANCHOR_END: custom_hash
}
