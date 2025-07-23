# The Rules of Hooks

Hooks are a powerful way to manage state in Dioxus, but there are some rules you need to follow to insure they work as expected. Dioxus uses the order you call hooks to differentiate between hooks. Because the order you call hooks matters, you must follow these rules:

1. Hooks may be only used in components or other hooks (we'll get to that later)
2. On every call to the component function
   1. The same hooks must be called
   2. In the same order
3. Hooks name's should start with `use_` so you don't accidentally confuse them with regular functions

These rules mean that there are certain things you can't do with hooks:

#### No Hooks in Conditionals
```rust
{{#include ../docs-router/src/doc_examples/hooks_bad.rs:conditional}}
```

#### No Hooks in Closures
```rust
{{#include ../docs-router/src/doc_examples/hooks_bad.rs:closure}}
```

#### No Hooks in Loops
```rust
{{#include ../docs-router/src/doc_examples/hooks_bad.rs:loop}}
```

