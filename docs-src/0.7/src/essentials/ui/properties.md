Properties and Children

## Children

You can add children to an element after all attributes and event listeners. Elements can accept text, components or other elements as children. We can add a `div` element around our input to center it:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:input_children}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::InputChildren {}
}
```
