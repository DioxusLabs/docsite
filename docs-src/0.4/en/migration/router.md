# Router

The router has been entirely rewritten in the `0.4` release to provide type safe routes. This guide serves to help you upgrade your project to the new router. For more information on the router, see the [router guide](../router/index.md).

## Defining Your Router

Previously, you defined your route with components:

```rust
rsx! {
    Router::<Route> {
        Route { to: "/home", Home {} }
        Route { to: "/blog", Blog {} }
        // BlogPost has a dynamic id
        Route { to: "/blog/:id", BlogPost {} }
    }
}
```

Now you must define your routes as an enum of possible routes:

```rust
use dioxus_router::prelude::*;
use dioxus::prelude::*;

#[derive(Routable, PartialEq, Debug, Clone)]
enum Route {
    #[route("/home")]
    // This route will render the Home component with the HomeProps props. (make sure you have the props imported)
    // You can modify the props by passing extra arguments to the macro. For example, if you want the Home variant to render a component called Homepage, you could use:
    // #[route("/home", Homepage)]
    Home {},
    #[route("/blog")]
    Blog {},
    // BlogPost has a dynamic id
    #[route("/blog/:id")]
    BlogPost {
        id: usize
    }
}

#[component]
fn Home(cx: Scope) -> Element {
    todo!()
}

#[component]
fn Blog(cx: Scope) -> Element {
    todo!()
}

#[component]
fn BlogPost(cx: Scope, id: usize) -> Element {
    // Note that you have access to id here in a type safe way without calling any extra functions!
    todo!()
}
```

## Linking to routes

Now that routes are enums, you should use the enum as the route in Links. If you try to link to a route that does not exist, you will get a compiler error.

```rust
use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn Component(cx: Scope) -> Element {
    render! {
        Link {
            to: Route::BlogPost { id: 123 },
            "blog post"
        }
    }
}
```

## External Links

To link to external routes, you can use a string:

```rust
use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn Component(cx: Scope) -> Element {
    render! {
        Link {
            to: "https://google.com",
            "google"
        }
    }
}
```

## use_router

The `use_router` hook has been split into two separate hooks: the `use_route` hook and the `use_navigator` hook.

### use_route

The new use_route hook lets you read the current route:

```rust
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone, Routable)]
enum Route {
    #[route("/")]
    Index {},
}

fn App(cx: Scope) -> Element {
    render! {
        h1 { "App" }
        Router::<Route> {}
    }
}

#[component]
fn Index(cx: Scope) -> Element {
    // Read from (and subscribe to the current route)
    let path = use_route(&cx).unwrap();
    render! {
        h2 { "Current Path" }
        p { "{path}" }
    }
}
```

### use_navigator

`use_navigator` lets you change the route programmatically:

```rust
{{#include src/doc_examples/untested_04/navigator.rs:nav}}
```

You can read more about programmatic navigation in the [Router Book](../router/reference/navigation/programmatic.md).

### New features

In addition to these changes, there have been many new features added to the router:

- [static generation support](../router/reference/static-generation.md)
- [Layouts](../router/reference/layouts.md)
- [Nesting](../router/reference/routes/nested.md)
