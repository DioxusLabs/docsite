# Component Lifecycle

## Initializing State with `use_hook`

`use_hook` lets you create new state for your component. The closure you pass to `use_hook` will be called once the first time the component is rendered. Every time the component is re-rendered, the value that was created the first run will be re-used.

```rust, no_run
{{#include src/doc_examples/component_lifecycle.rs:use_hook}}
```
```inject-dioxus
DemoFrame {
	component_lifecycle::UseHookDemo {}
}
```
