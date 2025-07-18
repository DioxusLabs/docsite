# Hooks

Any function that starts with `use_` is a hook. Hooks retain state between component reruns. This makes them more powerful than normal functions, but also introduces some limitations on how they can be called.


Hooks use the order they are called in to keep track of what state belongs to each hook. You must call hooks in the same order every time the component is run. To make sure the order is always the same, **you should only call hooks at the top level of a component or another hook**.


These rules mean that there are certain things you can't do with hooks:

## No Hooks in Conditionals
```rust
{{#include ../docs-router/src/doc_examples/hooks_bad.rs:conditional}}
```

## No Hooks in Closures
```rust
{{#include ../docs-router/src/doc_examples/hooks_bad.rs:closure}}
```

## No Hooks in Loops
```rust
{{#include ../docs-router/src/doc_examples/hooks_bad.rs:loop}}
```

