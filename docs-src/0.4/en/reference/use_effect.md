# UseEffect

[`use_effect`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_effect.html) lets you run a callback that returns a future, which will be re-run when its dependencies change. This is useful to synchronize with external events.

## Dependencies

You can make the callback re-run when some value changes. For example, you might want to fetch a user's data only when the user id changes. Effects will automatically subscribe to any signals that you read inside the effect. It will re-run it when any of those signals change.

## Example

```rust, no_run
{{#include src/doc_examples/use_effect.rs}}
```
