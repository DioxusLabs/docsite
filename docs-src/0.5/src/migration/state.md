# State Migration

The `use_state` and `use_ref` hooks have been replaced with the `use_signal` hook. The `use_signal` hook is a more flexible and powerful version of the `use_ref` hook with smarter scopes that only subscribe to a signal if that signal is read within the scope.

With `use_state`, if you had this code:
```rust
fn Parent(cx: Scope) -> Element {
	let state = use_state(cx, || 0);

	render! {
		Child {
			state: state.clone()
		}
	}
}

#[component]
fn Child(cx: Scope, state: UseState<i32>) -> Element {
	render! {
		"{state}"
	}
}
```

Parent would re-render every time the state changed even though only the child component was using the state. With the new `use_signal` hook, the parent would only re-render if the state was changed within the parent component:

```rust
{{#include src/doc_examples/migration_state.rs:use_signal}}
```
Only the child component will re-render when the state changes because only the child component is reading the state.

## Context Based State

The `use_shared_state_provider` and `use_shared_state` hooks have been replaced with using the `use_context_provider` and `use_context` hooks with a `Signal`:

```rust
{{#include src/doc_examples/migration_state.rs:context_signals}}
```

Signals are smart enough to handle subscribing to the right scopes without a special shared state hook.

## Opting Out of Subscriptions

Some state hooks including `use_shared_state` and `use_ref` hooks had a function called `write_silent` in `0.4`. This function allowed you to update the state without triggering a re-render any subscribers. This function has been removed in `0.5`.


Instead, you can use the `peek` function to read the current value of a signal without subscribing to it. This inverts the subscription model so that you can opt out of subscribing to a signal instead of opting all subscribers out of updates:

```rust
{{#include src/doc_examples/migration_state.rs:peek}}
```

`peek` gives you more fine-grained control over when you want to subscribe to a signal. This can be useful for performance optimizations and for updating state without re-rendering components.

## Global State

In `0.4`, the fermi crate provided a separate global state API called atoms. In `0.5`, the `Signal` type has been extended to provide a global state API. You can use the `Signal::global` function to create a global signal:

```rust
{{#include src/doc_examples/migration_state.rs:global_signals}}
```

You can read more about global signals in the [Fermi migration guide](fermi.md).
