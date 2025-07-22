# Escaping the Dioxus Runtime

- native widgets
- dangerous inner html
- web-components


## Dangerous Inner HTML


## Direct DOM Access

- get element by ID
- `eval`

## Web Components

Any element with a dash in the name is a web component. Web components are rendered directly in dioxus without type checking. We recommend wrapping web components in a type safe component to make them easier to use.

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:web_component}}
```
