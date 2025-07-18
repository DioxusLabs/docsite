
## Text Nodes

Any content surrounded by quotes is rendered as a text node in rsx:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:text}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::Text {}
}
```

You can include formatted segments inside of the text just like the `format!` macro:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:formatted_text}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::FormattedText {}
}
```

## Elements

The most basic building block of HTML is an element. In rsx, you can create elements with the name and then curly braces. One of the most common elements is the `input` element. The input element creates an interactive input box:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:input}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::Input {}
}
```

> Bonus: web components
> Any element with a dash in the name is a web component. Web components are rendered directly in dioxus without type checking. We recommend wrapping web components in a type safe component to make them easier to use.
>
> ```rust, no_run
> {{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:web_component}}
> ```
