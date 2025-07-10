# History Providers

[`HistoryProvider`]s are used by the router to keep track of the navigation history
and update any external state (e.g. the browser's URL).

The router provides two [`HistoryProvider`]s, but you can also create your own.
The two default implementations are:

- The [`MemoryHistory`] is a custom implementation that works in memory.
- The [`LiveviewHistory`] is a custom implementation that works with the liveview renderer.
- The [`WebHistory`] integrates with the browser's URL.

By default, the router uses the [`MemoryHistory`]. It might be changed to use
[`WebHistory`] when the `web` feature is active, but that is not guaranteed.

You can override the default history:

```rust
{{#include src/doc_examples/history_provider.rs:app}}
```

