# Creating Our First Route

In this chapter, we will start utilizing Dioxus Router and add a homepage and a
404 page to our project.

## Fundamentals

The core of the Dioxus Router is the [`Routable`] macro and the [`Router`] component.


Routable is a trait for anything that can:
- Be parsed from a URL
- Be turned into a URL
- Be rendered as to a Element


Let's create a new router. First, we need an actual page to route to! Let's add a homepage component:

```rust
{{#include src/doc_examples/first_route.rs:home}}
```

## Creating Routes

We want to use Dioxus Router to separate our application into different "pages".
Dioxus Router will then determine which page to render based on the URL path.

To start using Dioxus Router, we need to use the [`Routable`] macro.

The [`Routable`] macro takes an enum with all of the possible routes in our application. Each variant of the enum represents a route and must be annotated with the `#[route(path)]` attribute.

```rust
{{#include src/doc_examples/first_route.rs:router}}
```

The [`Router`] component will provide a router context for all the inner components and hooks to use. You usually will want to place this at the top of your components tree.

```rust
{{#include src/doc_examples/first_route.rs:app}}
```

If you head to your application's browser tab, you should now see the text
`Welcome to Dioxus Blog!` when on the root URL (`http://localhost:8080/`). If
you enter a different path for the URL, nothing should be displayed.

This is because we told Dioxus Router to render the `Home` component only when
the URL path is `/`.

## Fallback Route

In our example, when a route doesn't exist Dioxus Router doesn't render anything. Many sites also have a "404" page when a path does not exist. Let's add one to our site.

First, we create a new `PageNotFound` component.

```rust
{{#include src/doc_examples/catch_all.rs:fallback}}
```

Next, register the route in the Route enum to match if all other routes fail.

```rust
{{#include src/doc_examples/catch_all.rs:router}}
```

Now when you go to a route that doesn't exist, you should see the page not found
text.

## Conclusion

In this chapter, we learned how to create a route and tell Dioxus Router what
component to render when the URL path is `/`. We also created a 404 page to
handle when a route doesn't exist. Next, we'll create the blog portion of our
site. We will utilize nested routes and URL parameters.

[`Router`]: https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Router.html
[`Routable`]: https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Routable.html
