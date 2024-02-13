# Hooks

Dioxus now uses signals as the backing for its state management. Signals are a smarter, more flexible version of the `use_ref` hook. Signals now back all hooks in dioxus to provide a more consistent and flexible API.

Compared to `0.4`, the following hooks have been removed:

- `use_future` -> `use_resource`
- `use_effect((dependency1,dependency2,), |(dependency1,dependency2,)| async move {/*side effect*/})` -> `use_effect(|| {/*side effect*/})`


### State Hooks

These hooks have been replaced with the `use_signal` hook:
- `use_shared_state_provider(|| your_data)` -> `use_context_provider(|| Signal::new(your_data))`
- `use_shared_state::<T>` -> `use_context::<Signal<T>>()`
- `use_state` -> `use_signal`
- `use_ref` -> `use_signal`

You can read more about the `use_signal` hook in the [State Migration](state.md) guide.
