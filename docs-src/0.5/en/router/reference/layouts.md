# Layouts

Layouts allow you to wrap all child routes in a component. This can be useful when creating something like a header that will be used in many different routes.

[`Outlet`] tells the router where to render content in layouts. In the following example,
the Index will be rendered within the [`Outlet`].

This page is built with the Dioxus. It uses Layouts in several different places. Here is an outline of how layouts are used on the current page. Hover over different layouts to see what elements they are on the page.

```inject-dioxus
LayoutsExplanation {}
```

Here is a more complete example of a layout wrapping the body of a page.

```rust
{{#include src/doc_examples/outlet.rs:outlet}}
```

The example above will output the following HTML (line breaks added for
readability):

```html
<header>header</header>
<h1>Index</h1>
<footer>footer</footer>
```

## Layouts with dynamic segments

You can combine layouts with [nested routes](./routes/nested.md) to create dynamic layouts with content that changes based on the current route.

Just like routes, layouts components must accept a prop for each dynamic segment in the route. For example, if you have a route with a dynamic segment like `/:name`, your layout component must accept a `name` prop:

```rust
{{#include src/doc_examples/outlet.rs:outlet_with_params}}
```

Or to get the full route, you can use the `use_route` hook.

```rust
{{#include src/doc_examples/outlet.rs:outlet_route}}
```

[`Outlet`]: https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Outlet.html
[`use_route`]: https://docs.rs/dioxus-router/latest/dioxus_router/hooks/fn.use_route.html