# Introduction

As your app grows, it can be helpful to organize your app into multiple pages or views you can switch between. In web application, each view has an associated url that can be saved and shared. The Dioxus router helps you manage the URL state of your application. It provides a type safe interface that checks all routes at compile time to prevent runtime errors.

## Installing the router

To get started, you can add the `router` feature to your `dioxus` dependency in your `Cargo.toml`:

```toml
[dependencies]
dioxus = { version = "0.7", features = ["router"] }
```

## Creating a Routable enum

The core of the router is your `Routable` enum. You will use this enum throughout your application to navigate to different pages. Each variant of the enum is a single view page in your app handles:

1. Parsing your route from a URL
2. Displaying your route as a URL
3. Rendering your route as a component

To create a `Routable` enum, you will need to derive the `Routable` with a `#[route(..)]` attribute on each variant which describes the format of the route. You must have a component in scope that matches the name of each variant to render the route:

```rust
{{#include ../docs-router/src/doc_examples/router_introduction.rs:first_router}}
```

## Rendering the router

Now that you have defined your routes, you can use the `Router` component to render them. The `Router` component takes your `Routable` enum as a generic argument to define handle parsing, and rendering routes.

```rust
{{#include ../docs-router/src/doc_examples/router_introduction.rs:launch_router}}
```

## Linking to your first route

To navigate between routes, you can use the `Link` component provided by the router. The `Link` component takes a `to` prop which can be either a unchecked string route or a variant of your `Routable` enum:

```rust
// ...

{{#include ../docs-router/src/doc_examples/router_introduction.rs:first_link}}
```
