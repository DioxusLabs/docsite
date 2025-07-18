## Loops

You can insert for loops directly in rsx. The body of the loop accepts any number of children that will be rendered with each iteration of the loop. The `ul` element in html renders an unordered list with any number of `li` (list item) elements. We can use those two elements to render a list of items in a loop:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:for_loop}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::ForLoop {}
}
```

Each item in your list should have unique value that is stable across rerenders called a key. Keys are used to identify how items move while diffing. Without keys, it is easy to accidentally lose or move state when you reorder items in a list. We can add keys to our list items by using the `key` attribute:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:keyed_for_loop}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::KeyedForLoop {}
}
```
