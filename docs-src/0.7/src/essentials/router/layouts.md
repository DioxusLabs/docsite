# Layouts

Layouts allow you to wrap all child routes in a component. This can be useful when creating something like a header that will be used in many different routes.

[`Outlet`] tells the router where to render content in layouts. In the following example,
the Index will be rendered within the [`Outlet`].

Here is a more complete example of a layout wrapping the body of a page.

```rust
{{#include ../docs-router/src/doc_examples/outlet.rs:outlet}}
```

The example above will output the following HTML (line breaks added for
readability):

```html
<header>header</header>
<h1>Index</h1>
<footer>footer</footer>
```

## Multiple routes sharing a layout

When you want multiple routes to share the same layout, you need to understand that **`#[layout()]` creates a stateful scope** that remains active for all subsequent routes until explicitly closed.

### Correct Pattern

Use indentation to show hierarchy and close the layout scope with `#[end_layout]`:

```rust
{{#include ../docs-router/src/doc_examples/outlet.rs:multiple_routes}}
```

Both the `Home` and `About` routes will render inside the same `Wrapper` layout. The layout scope is explicitly closed with `#[end_layout]`, making it clear which routes are affected.

### Common Mistake: Nested Layouts

**⚠️ Warning:** Adding `#[layout()]` multiple times without closing the scope creates nested layouts:

```rust
{{#include ../docs-router/src/doc_examples/outlet.rs:wrong_pattern}}
```

In this example, the `About` route will render with **two** `Wrapper` components (double header, double footer) because:
1. The first `#[layout(Wrapper)]` opens a layout scope
2. The second `#[layout(Wrapper)]` **nests inside** the first scope (instead of replacing it)
3. No `#[end_layout]` was used to close the first scope

This produces duplicate UI elements with no compile-time warning.

### Layout Scoping Rules

- `#[layout(Component)]` opens a layout scope for all subsequent routes
- The scope remains active until:
  - `#[end_layout]` explicitly closes it, OR
  - The end of the enum is reached
- Subsequent `#[layout()]` attributes without closing the previous scope create **nested** layouts
- Use proper indentation to visually represent the scope hierarchy

### Debugging Layout Issues

If you see duplicate layouts in your rendered UI:

1. **Check HTML output**: Use browser DevTools or `curl http://localhost:8080/route | grep 'class="your-layout-class"' | wc -l`
2. **Verify nesting levels**: Run `cargo expand --package your-crate --lib routes` to see the macro-generated code
3. **Add `#[end_layout]`**: Explicitly close layout scopes to prevent accidental nesting

## Layouts with dynamic segments

You can combine layouts with nested routes to create dynamic layouts with content that changes based on the current route.

Just like routes, layouts components must accept a prop for each dynamic segment in the route. For example, if you have a route with a dynamic segment like `/:name`, your layout component must accept a `name` prop:

```rust
{{#include ../docs-router/src/doc_examples/outlet.rs:outlet_with_params}}
```

Or to get the full route, you can use the `use_route` hook.

```rust
{{#include ../docs-router/src/doc_examples/outlet.rs:outlet_route}}
```

[`Outlet`]: https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Outlet.html
[`use_route`]: https://docs.rs/dioxus-router/latest/dioxus_router/hooks/fn.use_route.html
