#![allow(non_snake_case, unused)]
use std::fmt::Display;

use dioxus::prelude::*;

// ANCHOR: route
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    // segments that start with ?: are query segments
    #[route("/blog?:query_params")]
    BlogPost {
        // You must include query segments in child variants
        query_params: BlogQuerySegments,
    },
}

#[derive(Debug, Clone, PartialEq)]
struct BlogQuerySegments {
    name: String,
    surname: String,
}

/// The display impl needs to display the query in a way that can be parsed:
impl Display for BlogQuerySegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name={}&surname={}", self.name, self.surname)
    }
}

/// The query segment is anything that implements https://docs.rs/dioxus-router/latest/dioxus_router/routable/trait.FromQuery.html. You can implement that trait for a struct if you want to parse multiple query parameters.
impl FromQuery for BlogQuerySegments {
    fn from_query(query: &str) -> Self {
        let mut name = None;
        let mut surname = None;
        let pairs = form_urlencoded::parse(query.as_bytes());
        pairs.for_each(|(key, value)| {
            if key == "name" {
                name = Some(value.clone().into());
            }
            if key == "surname" {
                surname = Some(value.clone().into());
            }
        });
        Self {
            name: name.unwrap(),
            surname: surname.unwrap(),
        }
    }
}

#[component]
fn BlogPost(cx: Scope, query_params: BlogQuerySegments) -> Element {
    render! {
        div{"This is your blogpost with a query segment:"}
        div{format!("{:?}", query_params)}
    }
}

fn App(cx: Scope) -> Element {
    render! { Router::<Route>{} }
}

fn main() {}
