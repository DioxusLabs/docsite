# UseEffect

[`use_effect`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_effect.html) lets you run a callback after the current render has finished which will be re-run when its dependencies change. Effects are a flexible way to handle side effects like manually changing the DOM.

If you are looking for a hook to handle async tasks with dependencies, you should use `use_resource` instead. Or if you would like to produce a new value from a callback with dependencies, you should use `use_memo` instead.

## Dependencies

You can make the callback re-run when some value changes. For example, you might want to fetch a user's data only when the user id changes. Effects will automatically subscribe to any signals that you read inside the effect. It will re-run it when any of those signals change.

## Example

```rust, no_run
{{#include src/doc_examples/use_effect.rs}}
```
