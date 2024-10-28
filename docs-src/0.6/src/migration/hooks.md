# Hooks

Dioxus now uses signals as the backing for its state management. Signals are a smarter, more flexible version of the `use_ref` hook. Signals now back many hooks in dioxus to provide a more consistent and flexible API.

### State Hooks

State hooks are now backed by signals. `use_state`, `use_ref`, and `use_shared_state` have been replaced with the `use_signal` hook. The `use_signal` hook is a more flexible and powerful version of the `use_ref` hook with smarter scopes that only subscribe to a signal if that signal is read within the scope. You can read more about the `use_signal` hook in the [State Migration](state.md) guide.

### Async Hooks

The `use_future` hook has been replaced with the `use_resource` hook. `use_resource` automatically subscribes to any signals that are read within the closure instead of using a tuple of dependencies.

Dioxus 0.4:

```rust
fn MyComponent(cx: Scope) -> Element {
	let state = use_state(cx, || 0);
	let my_resource = use_future(cx, (**state,), |(state,)| async move {
		// start a request that depends on the state
		println!("{state}");
	});
	render! {
		"{state}"
	}
}
```

Dioxus 0.5:

```rust
{{#include src/doc_examples/migration_hooks.rs:use_resource}}
```

### Dependencies

Some hooks including `use_effect` and `use_resource` now take a single closure with automatic subscriptions instead of a tuple of dependencies. You can read more about the `use_resource` hook in the [Hook Migration](hooks.md) guide.

Dioxus 0.4:

```rust
fn HasDependencies(cx: Scope) -> Element {
	let state = use_state(cx, || 0);
	let my_resource = use_resource(cx, (**state,), |(state,)| async move {
		println!("{state}");
	});
	let state_plus_one = use_memo(cx, (**state,), |(state,)| {
		state() + 1
	});
	render! {
		"{state_plus_one}"
	}
}
```

Dioxus 0.5:

```rust
{{#include src/doc_examples/migration_hooks.rs:dependencies}}
```
