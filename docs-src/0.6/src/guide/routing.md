# Adding More Routes

So far, our app has only had a single page. Let's change that!

In this chapter, we'll be adding a Navbar, a welcome screen, and a "favorites" page where we can revisit our favorite dogs.

## Organizing our Project

Before we get too far with adding new pages to our app, let's organize our codebase a bit better. For larger projects you might want to break your app into different smaller crates. For HotDog, we'll keep it simple.

> The `dx new` Jumpstart and Workspace templates provide great scaffolding for new apps!

We generally recommend splitting your components, models, and backend functionality into different files. For HotDog, we're going to use a simple directory structure:

```sh
├── Cargo.toml
├── assets
│   └── main.css
└── src
    ├── backend.rs
    ├── components
    │   ├── favorites.rs
    │   ├── mod.rs
    │   ├── nav.rs
    │   └── view.rs
    └── main.rs
```

We'll have a `backend.rs` that contains our server functions and a `components` folder that contains our components. We don't have a `NavBar` or a `Favorites` component yet, but we'll still create the relevant files before adding them. By splitting out our server functions into a `backend.rs` file, we'll make it easier to extract our backend functionality as a shared library for different apps in the future.

Our `components/mod.rs` file will simply import and re-export the components in `view.rs`, `nav.rs`, and `favorites.rs`:

```rust
mod favorites;
mod nav;
mod view;

pub use favorites::*;
pub use nav::*;
pub use view::*;
```

Finally, we need to bring `backend` and `components` into scope in our `main.rs` file:

```rust
mod components;
mod backend;

use crate::components::*;
```

For more information on organizing Rust projects with modules, see the [Modules section](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) of the Rust Book.

## Creating a Route

Most Dioxus apps you'll build will have different screens. This could include pages like *Login*, *Settings*, and *Profile*. Our HotDog app will have two screens: a *DogView* page and a *Favorites* page.

Dioxus provides a first-party router that natively integrates with web, desktop, and mobile. For example, on web, whenever you visit the `/favorites` url in your browser, the corresponding *Favorites* page will load. The Dioxus router is very powerful, and most importantly, type-safe. You can rest easy knowing that users will never be sent to an invalid route. To achieve this, we first need to add the "Router" feature to the Cargo.toml file:

```toml
[dependencies]
dioxus = { version = "0.6.0", features = ["fullstack", "router"] } # <----- add "router"
```

Next, the Dioxus router is defined as an enum with the `Routable` derive attribute:

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    DogView,
}
```

With the Dioxus router, every route is an enum variant with a `#[route]` attribute that specifics the route's URL. Whenever the router renders our route, the component of the same name will be rendered.

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    DogView,     // <---- a DogView component must be in scope
}

fn DogView() -> Element { /* */ }
```


## Rendering the Route

Now that we have our app's `Route` defined, we need to render it. Let's change our `app` component to render the `Route {}` component instead of the `DogView`.

```rust
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }

        // 📣 delete Title and DogView and replace it with the Router component.
        Router::<Route> {}
    }
}
```

When the `Router {}` component renders, it will parse the document's current URL into a `Route` variant. If the url doesn't parse properly, the router will render nothing unless you add a "catch-all" route:

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    // ...
    // We can collect the segments of the URL into a Vec<String>
    #[route("/:..segments")]
    PageNotFound {
        segments: Vec<String>,
    },
}
```

Note here that the `PageNotFound` route takes the "segments" parameter. Dioxus routes are not only type-safe as variants, but also type-safe with URL parameters. For more information on how this works, [check the router guide](../router/index.md).

At this point, we should see our app, but this time without its Title.

![No Navbar](/assets/06_docs/no_navbar.png)


## Rendering the NavBar with a Layout

We're rendering our DogView component, but unfortunately we no longer see our Title. Let's add that back and turn it into a NavBar!

In our `src/components/nav.rs` file, we'll add back our Title code, but rename it to NavBar and modify it with two new items: the `Link {}` and `Outlet` components.

```rust
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "title",
            Link { to: Route::DogView,
                h1 { "🌭 HotDog! " }
            }
        }
        Outlet::<Route> {}
    }
}
```

The `Link {}` component wraps the anchor `<a>` element with a type-safe interface. This means any struct that implements `Routable` - anything that can `.to_string()` - is a valid navigation target.

```rust
// Using the Link with Route
Link { to: Route::DogView }

// Or passing in a "/" route directly
Link { to: "/" }
```

The Link component takes many different arguments, making it possible to extend and customize for your use-case.

In `NavBar`, we also added an `Outlet::<Route> {}` component. When the Router component renders, it first looks for any child `Outlet` components. If one is present, it renders the current route *under the outlet*. This lets us wrap the current page in extra elements - in this case, the NavBar. If no Outlet is present, then the current route is simply rendered where the `Router {}` is declared.

To actually add the NavBar component to our app, we need to update our `Route` enum with the `#[layout]` attribute. This forces the router to render the `NavBar` component *first* so it can expose its `Outlet {}`.

```rust
#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[layout(NavBar)] // <---- add the #[layout] attribute
    #[route("/")]
    DogView,
}
```

The `layout` attribute instructs the Router to wrap the following enum variants in the given component.
```rust
Router  {
    NavBar {
        Outlet {
            if route == “/” {
                DogView {}
            }
        }
    }
}
```

Visually, this should be straight-forward to understand. Note that the Router and Outlet share the same `Route` generic type.

![RouterLayout](/assets/06_docs/routeroutlet.png)

## Adding a Favorites Route

Now that we understand the fundamentals of routing, let's finally add our *Favorites* page so we can view the dog photos we saved.

We'll start by creating an empty component `src/components/favorites.rs`:

```rust
use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    rsx! { "favorites!" }
}
```

And then let's make sure to add a new variant in our `Route` enum:

```rust
#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,     // <------ add this new variant
}
```

To make sure the user can reach this page, let's also add a button in the nav that points to it.

```rust
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "title",
            Link { to: Route::DogView,
                h1 { "🌭 HotDog! " }
            }
            Link { to: Route::Favorites, id: "heart", "♥️" } // <------- add this Link
        }
        Outlet::<Route> {}
    }
}
```

## Our Favorites Page

Finally, we can build our favorites page. Let's add a new `list_dogs` server function that fetches the 10 most recently saved dog photos:

```rust
// Query the database and return the last 10 dogs and their url
#[server]
pub async fn list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(dogs)
}
```

Now, we can fill in our component. We're going to use the same `use_resource` hook from earlier. Resolving the request from the server might take some time, so we'll use the `.suspend()?` method on `Resource` to wait for the request to finish before mapping the contents to a list.

```rust
use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    // Create a pending resource that resolves to the list of dogs from the backend
    // Wait for the favorites list to resolve with `.suspend()`
    let mut favorites = use_resource(crate::backend::list_dogs).suspend()?;

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id, url) in favorites().unwrap() {
                    // Render a div for each photo using the dog's ID as the list key
                    div {
                        key: id,
                        class: "favorite-dog",
                        img { src: "{url}" }
                    }
                }
            }
        }
    }
}
```

As a stretch goal, try adding a button that lets the user also delete items from the database.

![FullDemo](/assets/06_docs/hotdogfull.mp4)
