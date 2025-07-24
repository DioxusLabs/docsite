# 📘 Cheat Sheet

⭐ = Most commonly used hooks

---

## 🏠 Managing Component State

*"Which scope does your state or utility belong to?"*

### Basic State

* ⭐ [`use_hook`](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_hook.html) — Non-reactive, runs on the first use of the hook, which is typically the initial render
* ⭐ [`use_signal`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_signal.html) — Basic local state, triggers re-renders on write, does not subscribe to other signals
* [`use_signal_sync`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_signal_sync.html) — Thread-safe signal - Send + Sync

### Derived State

* ⭐ [`use_memo`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_memo.html) — Derived state (memoized), composes signals
* ⭐ [`use_reactive`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_reactive.html) — Creates a closure (async/sync) to track 'non-reactive' data
* [`use_set_compare`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_set_compare.html) — Efficient value change tracking
* [`use_set_compare_equal`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_set_compare_equal.html) — Like `use_set_compare` but uses a custom equality function

### Callbacks

* [`use_callback`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_callback.html) — Keeps a callback up to date for all references to the handle

---

## 🔄 Sharing Data Between Components

*"Shared (Context)"*

### Context

* ⭐ [`use_context_provider`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context_provider.html) — Provides data to any child
* ⭐ [`use_context`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context.html) — Consume provided data
* [`try_use_context`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.try_use_context.html) — Like `use_context` but returns None if missing
* [`use_root_context`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_root_context.html) — Like `use_context` except creates context at root if missing
* [`use_coroutine_handle`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_coroutine_handle.html) — Obtain handle to a context-provided coroutine

### Context Utilities

* [`use_clipboard`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/clipboard/fn.use_clipboard.html) — Access the clipboard (dioxus\_sdk)
* [`use_window_size`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/utils/window/fn.use_window_size.html) — Initial window size will be returned with this hook and updated continously as the window is resized (dioxus\_sdk)
* [`use_geolocation`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/geolocation/use_geolocation/fn.use_geolocation.html) — Provides the latest geocoordinates. Good for navigation-type apps (dioxus\_sdk)
* [`use_system_theme`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/theme/fn.use_system_theme.html) — Initial theme will be returned and updated if the system theme changes (dioxus\_sdk)

---

## ⚡ Async Operations & Side Effects

*"When does the code need to run?"*

### First Render (Non-Reactive)

#### Sync

* ⭐ [`use_hook`](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_hook.html) — *(already listed)*
* [`use_hook_with_cleanup`](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_hook_with_cleanup.html) — Use a hook with a cleanup function that runs when component is dropped
* ⭐ [`use_debounce`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/utils/timing/fn.use_debounce.html) — Calls a function only after provided duration has passed (dioxus\_sdk)
* [`use_interval`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/utils/timing/fn.use_interval.html) — Repeatedly calls a function every certain period (dioxus\_sdk)
* [`use_server_cached`](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_server_cached.html) — Runs a function on the server (or client if server is not enabled) and sends result to client. Caches the value on the client

#### Async

* [`use_channel`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/utils/channel/fn.use_channel.html), [`use_listen_channel`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/utils/channel/fn.use_listen_channel.html) — Create and listen to channels for communication between components (dioxus\_sdk)

### Before Every Subsequent Render

* [`use_before_render`](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_before_render.html) — Register a function to run before every subsequent render

### Render Phase (While Component Is Alive)

#### Sync

* [`use_callback`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_callback.html) — *(already listed)*
* [`use_signal`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_signal.html) — *(already listed)*
* [`use_signal_sync`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_signal_sync.html) — *(already listed)*
* [`use_memo`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_memo.html) — *(already listed)*
* [`use_set_compare`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_set_compare.html) — *(already listed)*
* [`use_set_compare_equal`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_set_compare_equal.html) — *(already listed)*

#### Sync & Async

* [`use_reactive`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_reactive.html) — *(already listed)*

#### Async

* ⭐ [`use_future`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_future.html) — Run an async task once
* [`use_coroutine`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_coroutine.html) — Stream-based concurrency through channels
* ⭐ [`use_resource`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_resource.html) — Async derived state
* [`use_server_future`](https://docs.rs/dioxus-fullstack/latest/dioxus_fullstack/prelude/fn.use_server_future.html) — Async derived state that runs on the server and caches on the client

### Post Renders

* ⭐ [`use_effect`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_effect.html) — Side effects after render, composes signals
* [`use_hook_did_run`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_hook_did_run.html) — Lifecycle check if this hook has been called on the latest render
* [`use_after_render`](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_after_render.html) — Push a function to be run after the next render

### Cleanup

* [`use_drop`](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_drop.html) — Callback invoked when component is dropped

---

## 💾 Persisting Data

*"Persistent (Storage)"*

### Local Storage

* ⭐ [`use_persistent`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/storage/fn.use_persistent.html) — Store data across application reloads as either local storage or a file storage (dioxus\_sdk)
* [`use_storage`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/storage/fn.use_storage.html) — Like `use_persistent` except the storage location is generic, which may be useful for custom implementations (dioxus\_sdk)
* [`use_singleton_persistent`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/storage/fn.use_singleton_persistent.html) — Persistent storage shared for calls from same line (dioxus\_sdk)
* [`use_storage_entry`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/storage/fn.use_storage_entry.html) — Creates a `StorageEntry` with latest value from storage or init if does not exist (dioxus\_sdk)

### Synced Storage

* [`use_synced_storage`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/storage/fn.use_synced_storage.html) — Store data that persists and syncs across all app sessions (dioxus\_sdk)
* [`use_synced_storage_entry`](https://docs.rs/dioxus-sdk/latest/dioxus_sdk/storage/fn.use_synced_storage_entry.html) — Creates a `StorageEntry` with updates subscription (dioxus\_sdk)

---

## 🌐 Global State

*"Global (Truly app-wide)"*

* [`GlobalSignal`](https://docs.rs/dioxus/latest/dioxus/prelude/type.GlobalSignal.html) — Global signal (sync)
* [`GlobalMemo`](https://docs.rs/dioxus/latest/dioxus/prelude/type.GlobalMemo.html) — Derived global state (sync)
* [`Global`](https://docs.rs/dioxus/latest/dioxus/prelude/struct.Global.html) — A lazy value that is created once per application (sync)

---

## 🧭 Navigation & Routing

*"Route-based"*

### Core Routing

* ⭐ [`Routable`](https://docs.rs/dioxus/latest/dioxus/prelude/trait.Routable.html) — The `dioxus-router` macro used for routing
* ⭐ [`use_route`](https://docs.rs/dioxus-router/latest/dioxus_router/hooks/fn.use_route.html) — Access information about the current routing location
* ⭐ [`use_navigator`](https://docs.rs/dioxus-router/latest/dioxus_router/hooks/fn.use_navigator.html) — Access navigator to change router history

### Nested Routes

* [`use_outlet_context`](https://docs.rs/dioxus-router/latest/dioxus_router/hooks/fn.use_outlet_context.html) — Access to the outlet context for the route nesting level
