# Conditional Rendering



## Expressions

Finally, you can use arbitrary Rust expressions inside of rsx by surrounding them with `{}`. Any expression you include must implement the [`IntoDynNode`](https://docs.rs/dioxus-core/latest/dioxus_core/trait.IntoDynNode.html) trait. You can use raw expressions to render nodes from an iterator, variable, or function:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:expression}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::Expression {}
}
```


## If Statements

You can also use if/else statements in rsx. Each branch of the if statement accepts child nodes that will be rendered if the condition is true. We can use the `if` statement to conditionally render a login screen:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:if_statement}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::IfStatement {}
}
```
