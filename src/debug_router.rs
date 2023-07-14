// TODO: Outlets and Links do not work in nested route types. We need to make an adapter for the Router Context to handle this.
// Box<dyn RoutableAdapter>?

#![allow(non_snake_case)]
use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        Router {}
    }
}

mod doc {
    use dioxus::prelude::*;
    use dioxus_router::prelude::*;

    #[inline_props]
    fn About(cx: Scope) -> Element {
        render! {
            div {
                "About"
            }
        }
    }

    #[inline_props]
    fn Home(cx: Scope) -> Element {
        render! {
            div {
                "Home"
            }
        }
    }

    #[inline_props]
    fn Layout(cx: Scope) -> Element {
        render! {
            div {
                "layout"
                Outlet {}
            }
        }
    }

    #[derive(Routable, PartialEq, Clone, Debug)]
    pub enum Route {
        #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
    }

    #[inline_props]
    fn RouteLayout(cx: Scope) -> Element {
        render! {
            super::Layout::<Route> {}
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct DebugRouter<R> {
    inner: R,
}

impl<R> DebugRouter<R>
where
    R: Routable,
    <R as std::str::FromStr>::Err: std::fmt::Display,
{
    fn new(inner: R) -> Self {
        Self { inner }
    }
}

impl<R> FromStr for DebugRouter<R>
where
    R: Routable,
    <R as std::str::FromStr>::Err: std::fmt::Display,
{
    type Err = <R as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s.parse::<R>()?;
        Ok(Self::new(inner))
    }
}

impl<R> Display for DebugRouter<R>
where
    R: Routable,
    <R as std::str::FromStr>::Err: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<R> Routable for DebugRouter<R>
where
    R: Routable,
    <R as std::str::FromStr>::Err: std::fmt::Display,
{
    const SITE_MAP: &'static [SiteMapSegment] = R::SITE_MAP;

    fn render<'a>(&self, cx: &'a ScopeState, level: usize) -> Element<'a> {
        if level == 0 {
            render! {
                Layout::<DebugRouter<R>> {}
            }
        } else {
            self.inner.render(cx, level - 1)
        }
    }
}

#[derive(Props, PartialEq, Default)]
pub struct LayoutProps<R>
where
    R: Routable,
    <R as std::str::FromStr>::Err: std::fmt::Display,
{
    #[props(default)]
    phantom: std::marker::PhantomData<R>,
}

pub fn Layout<R: Routable>(cx: Scope<LayoutProps<R>>) -> Element
where
    <R as std::str::FromStr>::Err: std::fmt::Display,
{
    let current_url = use_ref(cx, || String::new());
    let route = current_url.read().parse::<R>();
    let navigator = use_generic_navigator::<R>(cx);

    render! {
        input {
            oninput: |evt| {
                current_url.set(evt.value.clone());
                if let Ok(new_route) = current_url.read().parse::<R>() {
                    navigator.push(new_route);
                }
            },
            value: "{current_url.read()}",
        }
        match route {
            Ok(_) => {
                rsx!{
                    GenericOutlet::<R> {}
                }
            }
            Err(err) => {
                rsx!{
                    pre {
                        color: "red",
                        "{err}"
                    }
                }
            }
        }
    }
}

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/:...segments")]
    NotFound { segments: Vec<String> },
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    render! {
        GenericRouter::<DebugRouter<doc::Route>> {
            config: || RouterConfig::default().history(MemoryHistory::default()),
        }
    }
}

#[inline_props]
#[allow(unused)]
fn NotFound(cx: Scope, segments: Vec<String>) -> Element {
    render! {
        div {
            "Not Found"
        }
    }
}
