# Hook Cheat Sheet

## Component Lifecycle Hooks

Useful hooks for reacting to the component lifecycle.

| Hook                                     | Derived | Async | Sync | Returns Value | Pre-render | Render | Post-render | Effects First Render | Effects Subsequent Renders | On Unmount |
|------------------------------------------|----------|-------|------|---------------|------------|--------|-------------|----------------------|------------------------------|------------|
| [use_hook]                               | ❌       | ❌    | ✅   | ✅            | ❌         | ✅     | ❌          | ✅                  | ❌                           | ❌         |
| [use_hook_with_cleanup]                  | ❌       | ❌    | ✅   | ✅            | ❌         | ✅     | ❌          | ✅                  | ❌                           | ✅         |
| [use_hook_did_run]                       | ❌       | ❌    | ✅   | ❌            | ❌         | ❌     | ✅          | ✅                  | ✅                           | ❌         |
| [use_signal]                             | ❌       | ❌    | ✅   | ✅            | ❌         | ✅     | ❌          | ✅                  | ✅                           | ❌         |
| [use_signal_sync]                        | ❌       | ❌    | ✅   | ✅            | ❌         | ✅     | ❌          | ✅                  | ✅                           | ❌         |
| [use_effect]                             | ✅       | ❌    | ✅   | ❌            | ❌         | ❌     | ✅          | ✅                  | ✅                           | ❌         |
| [use_memo]                               | ✅       | ❌    | ✅   | ✅            | ❌         | ✅     | ❌          | ✅                  | ✅                           | ❌         |
| [use_reactive]                           | ✅       | ✅    | ✅   | ✅            | ❌         | ✅     | ❌          | ✅                  | ✅                           | ❌         |
| [use_callback]                           | ❌       | ✅    | ✅   | ✅            | ❌         | ✅     | ❌          | ✅                  | ✅                           | ❌         |
| [use_future]                             | ❌       | ✅    | ❌   | ❌            | ❌         | ✅     | ❌          | ✅                  | ❌                           | ❌         |
| [use_coroutine] / [use_coroutine_handle] | ❌       | ✅    | ❌   | ✅            | ❌         | ✅     | ❌          | ✅                  | ❌                           | ❌         |
| [use_channel] / [use_listen_channel]     | ❌       | ✅    | ❌   | ✅            | ❌         | ✅     | ❌          | ✅                  | ❌                           | ❌         |
| [use_resource]                           | ✅       | ✅    | ❌   | ✅            | ❌         | ✅     | ❌          | ✅                  | ✅                           | ❌         |
| [use_before_render]                      | ❌       | ❌    | ✅   | ❌            | ✅         | ❌     | ❌          | ❌                  | ✅                           | ❌         |
| [use_after_render]                       | ❌       | ❌    | ✅   | ❌            | ❌         | ❌     | ✅          | ✅                  | ✅                           | ❌         |
| [use_drop]                               | ❌       | ❌    | ✅   | ❌            | ❌         | ❌     | ❌          | ❌                  | ❌                           | ✅         |
| [use_server_future]                      | ✅       | ✅    | ❌   | ✅            | ❌         | ✅     | ❌          | ✅                  | ✅                           | ❌         |
| [use_server_cached]                      | ❌       | ❌    | ✅   | ❌            | ❌         | ✅     | ❌          | ✅                  | ❌                           | ❌         |


### Context Hooks

Context hooks allow consuming and retrieving context in the component tree.

* [use_context_provider](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context_provider.html)
* [use_context](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context.html)
* [try_use_context](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.try_use_context.html)
* [use_root_context](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_root_context.html)

### Message Passing
Reactive message passing can be accomplished with both [use_coroutine]/[use_coroutine_handle] and [use_channel]/[use_listen_channel]. The difference is in ergonomics. For channels, the channel is passed around that one can send and listen on. For coroutines, only the send handle is passed around and the coroutine can be retrieved through the shared context.

### Hydration

[use_server_future] and [use_server_cached] are useful for hydration.

## Utility Hooks

Hooks into various utilities not necessarily related to reacting to the component lifecycle

### Collections

* [use_set_compare](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_set_compare.html)
* [use_set_compare_equal](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_set_compare_equal.html)

### Special Utilities

* [use_clipboard](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/clipboard/fn.use_clipboard.html)
* [use_window_size](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/utils/window/fn.use_window_size.html)
* [use_geolocation](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/geolocation/use_geolocation/fn.use_geolocation.html)
* [use_system_theme](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/theme/fn.use_system_theme.html)

### Execution Time and Rate Limiting

* [use_debounce](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/utils/timing/fn.use_debounce.html)
* [use_interval](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/utils/timing/fn.use_interval.html)

### Persisting Data

#### Local Storage

* [use_persistent](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/storage/fn.use_persistent.html)
* [use_storage](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/storage/fn.use_storage.html)
* [use_singleton_persistent](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/storage/fn.use_singleton_persistent.html)
* [use_storage_entry](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/storage/fn.use_storage_entry.html)

#### Synced Storage

* [use_synced_storage](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/storage/fn.use_synced_storage.html)
* [use_synced_storage_entry](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/storage/fn.use_synced_storage_entry.html)

### Global

* [GlobalSignal](https://docs.rs/dioxus/latest/dioxus/prelude/type.GlobalSignal.html)
* [GlobalMemo](https://docs.rs/dioxus/latest/dioxus/prelude/type.GlobalMemo.html)
* [Global](https://docs.rs/dioxus/latest/dioxus/prelude/struct.Global.html)

### Navigation & Routing

#### Core Routing

* [Routable](https://docs.rs/dioxus/latest/dioxus/prelude/trait.Routable.html)
* [use_route](https://docs.rs/dioxus-router/latest/dioxus_router/hooks/fn.use_route.html)
* [use_navigator](https://docs.rs/dioxus-router/latest/dioxus_router/hooks/fn.use_navigator.html)

#### Nested Routes

* [use_outlet_context](https://docs.rs/dioxus-router/latest/dioxus_router/hooks/fn.use_outlet_context.html)


[use_hook]: https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_hook.html
[use_hook_with_cleanup]: https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_hook_with_cleanup.html
[use_hook_did_run]: https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_hook_did_run.html
[use_signal]: https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_signal.html
[use_signal_sync]: https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_signal_sync.html
[use_effect]: https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_effect.html
[use_memo]: https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_memo.html
[use_reactive]: https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_reactive.html
[use_callback]: https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_callback.html
[use_future]: https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_future.html
[use_server_future]: https://docs.rs/dioxus-fullstack/latest/dioxus_fullstack/prelude/fn.use_server_future.html
[use_server_cached]: https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_server_cached.html
[use_coroutine]: https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_coroutine.html
[use_coroutine_handle]: https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_coroutine_handle.html
[use_channel]: https://docs.rs/dioxus-sdk/latest/dioxus_sdk/utils/channel/fn.use_channel.html
[use_listen_channel]: https://docs.rs/dioxus-sdk/latest/dioxus_sdk/utils/channel/fn.use_listen_channel.html
[use_resource]: https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_resource.html
[use_before_render]: https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_before_render.html
[use_after_render]: https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_after_render.html
[use_drop]: https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_drop.html