# Elements and Text

Most user interfaces are assembled by combining text and UI elements together in a useful and visually appealing tree. An example of some text and elements with RSX may look like:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:basic}}
```
```inject-dioxus
DemoFrame {
    building_uis_with_rsx::Basic {}
}
```

## Text Nodes

Any content surrounded by quotes is rendered as a text node in RSX:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:text}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::Text {}
}
```

Text nodes in Dioxus automatically implement the same rules as Rust's [`format!`](https://doc.rust-lang.org/std/macro.format.html) macro, including [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) and [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html) printing.

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:simple_formatted_text}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::SimpleFormattedText {}
}
```

Unlike Rust's format macro, `rsx!` lets us embed entire Rust expressions which can be quite handy when working with complex objects or calling functions inline.

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:formatted_text}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::FormattedText {}
}
```

## Elements

The most basic building block of HTML is an element. In RSX, an element is declared with a name and then curly braces. One of the most common elements is the `input` element. The input element creates an interactive input box:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:input}}
```
```inject-dioxus
DemoFrame {
    building_uis_with_rsx::Input {}
}
```

Elements can take additional parameters called attributes that modify how the element is rendered. Attributes are added inline, similar to adding fields to a struct instantiation:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:input_placeholder}}
```
```inject-dioxus
DemoFrame {
    building_uis_with_rsx::InputPlaceholder {}
}
```

There are a huge number of HTML elements available, including, but not limited to:

- Text and Content: `p`, `h1`, `span`, `div`, `a`, `pre`, etc.
- Forms and Input: `form`, `input`, `textarea`, `select`, `button`, etc.
- Media and Content: `img`, `video`, `audio`, `source`, `canvas`, `svg`, `iframe`, etc.
- Tables: `table`, `thead`, `tbody`, `tfoot`, `tr`, `th`, `td`, etc.
- Semantic Elements: `details`, `summary`, `dialog`, `progress`, `meter`, `time`, etc.

Check the [HTML Element reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements) for the full list.


## The `Element` type

The `rsx!` macro returns an object with the type of `Element`. These objects can be assigned to variables, cheaply cloned, and stored in state.

```rust
let header: Element = rsx! {
    div {
        h1 { "Dioxus!" }
    }
}
```

We can even create functions that return an `Element`:

```rust
fn create_description(content: &str) -> Element {
    rsx! {
        span { class: "description", "{content}" }
    }
}
```

Under the hood, the `Element` type is actually an alias for `Result<VNode>`. In Rust, a [Result](https://doc.rust-lang.org/std/result/) is an enumerated value that can either be an `Ok(value)` or an `Err(error)`. This means we can match on an Element, or even throw errors while rendering it:

```rust
fn create_description(content: &str) -> Element {
    if content.is_empty() {
        return Err("Missing description".into());
    }

    rsx! {
        span { class: "description", "{content}" }
    }
}
```

Dioxus defines its own error based on the [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) error which then composes with other utilities like Error Boundaries and Server Functions.
