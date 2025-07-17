# Describing the UI

Dioxus is a _declarative_ framework. This means that instead of telling Dioxus what to do (e.g. to "create an element" or "set the color to red") we simply _declare_ what we want the UI to look like using RSX.

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_06/hello_world_desktop.rs:component}}
```

Here, we use the `rsx!` macro to _declare_ that we want a `div` element, containing the text `"Hello, world!"`. Dioxus takes the RSX and constructs a user interface from it.

## Editing RSX with Hot-Reloading

When using `dx serve`, your app's RSX is automatically hot-reloaded whenever you edit and save the file. You can edit RSX structure, add new elements, and style your markup without a full rebuild.

Whenever you edit *Rust* code, then `dx` will automatically force a "full rebuild" of your app.

![Dog App Hotreloading](/assets/06_docs/dog_app_hotreload.mp4)

For an in-depth guide in what can and can't be hot-reloaded, check the [hot-reload guide](../reference/hotreload.md) in the reference.

## RSX is just HTML

Dioxus provides the `rsx! {}` macro for assembling `Element`s in your app. The `rsx! {}` macro primarily speaks HTML: the web, desktop, and mobile Dioxus first-party renderers all use HTML and CSS as the layout and styling technologies.

This means you can reuse your knowledge of the web and build your app using `div`, `span`, `img`, `style`, `button`, and more.

The RSX syntax is a "strict" form of Rust that uses Rust's `Struct` syntax for assembling elements:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_rsx.rs:rsx_is_html}}
```

Elements in RSX differ slightly from Rust struct syntax: they can also contain child structs placed immediately after the final attribute.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_rsx.rs:child_rsx}}
```

Additionally, all quoted strings in RSX imply `format!()` automatically, so you can define a variable outside your markup and use it in your strings without an explicit format call:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_rsx.rs:rsx_format}}
```

Any expression that can be rendered to a String can be included directly in RSX. RSX also accepts `Option<Element>` and iterators of Elements:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_rsx.rs:rsx_expression}}
```

Dioxus provides two items of syntax sugar for these common cases: `for` loops and `if` chains. These blocks return the contained RSX directly.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_rsx.rs:rsx_loop}}
```

For lists, Dioxus uses the `key` attribute to ensure it's comparing the right elements between renders. If you forget to add a `key` attribute to your list item, you might run into performance and state management issues. Usually you can find a unique key to differentiate your list items:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_rsx.rs:rsx_keyed}}
```

## Adding UI to our *HotDog* App

Let's add a basic UI to our app. We'll add a header, a body image for the dog photo, and some basic buttons.


```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_rsx.rs:adding_ui}}
```

Our app is coming together!

![Unstyled Dog App](/assets/06_docs/unstyled_dog_app.png)

Unfortunately, it's not very beautiful yet. Let's move on to [styling our app](assets.md).
