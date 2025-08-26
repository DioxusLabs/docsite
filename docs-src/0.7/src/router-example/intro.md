# Intro to the Dioxus Router

Dioxus provides a first party type safe router that works across all platforms and the server. Routes are type checked at compile time to prevent runtime errors. Each route is defined in a single variant in a `Routable` enum that can be used throughout your application.

## Installing the router

To add the router, you can add the `router` feature to your `dioxus` dependency in your `Cargo.toml`:

```toml
[dependencies]
dioxus = { version = "0.7", features = ["router"] }
```

## Creating a Routable enum

The core of the router is your `Routable` enum. You will use this enum throughout your application to navigate to different pages. Each variant of the enum defines three things:

1. How to parse your route
2. How to display your route
3. How to render your route

To create a `Routable` enum, you will need to derive the `Routable` with a `#[route(..)]` attribute on each variant which describes the format of the route:

```rust
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[route("/")]
    Home,
    #[route("/about")]
    About,
    #[route("/user/:id")]
    User { id: u32 },
}

#[component]
fn Home() -> Element {
    rsx!("Welcome to the home page!")
}

#[component]
fn About() -> Element {
    rsx!("This is the about page.")
}

#[component]
fn UserPage(id: u32) -> Element {
    rsx!("User page for user with id: {id}")
}
```

## Linking to your first route

To navigate between routes, you can use the `Link` component provided by the router. The `Link` component takes a `to` prop which can be either a unchecked string route or a variant of your `Routable` enum. We can use the link component to add links to our `About` page from our `Home` page:

```rust
// ...

#[component]
fn Home() -> Element {
    rsx! {
        div {
            "Welcome to the home page!"
            Link { to: Route::About, "Go to About Page" }
        }
    }
}
```
