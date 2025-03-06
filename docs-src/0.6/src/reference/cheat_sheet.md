# State and Hooks Cheat Sheet

```inject-dioxus
MermaidBlock {
    chart: r#"flowchart TD
    A("**Start**<br />Which scope does your state or utility belong to?")

    A --> B["Local<br />(per component)"]
    A --> C["Shared<br />(Context)"]
    A --> D["Persistent<br />(Storage)"]
    A --> E["Global<br />(Truly app-wide)"]
    A --> F["Route-based"]

    B --> LH("When does the code need to run?")

    LH --> Init["First Render (Non-Reactive)"]
    LH --> BeforeRender["Before Every<br />Subsequent Render"]
    LH --> Render["Render Phase (While Component Is Alive)"]
    LH --> Post["Post Renders"]
    LH --> Clean["Cleanup"]

    %% First Render
    subgraph FR_Sync ["Sync"]
    L14["**use_hook**<br />Run on the first use of the hook, which is typically the initial render"]
    L16["**use_hook_with_cleanup**<br />Use a hook with a cleanup function that runs when component is dropped"]
    U1["**use_debounce**<br />Calls a function only after provided duration has passed (dioxus_sdk)"]
    U2["**use_interval**<br />Repeatedly calls a function every certain period (dioxus_sdk)"]
    L18["**use_server_cached**<br />Runs a function on the server (or client if server is not enabled) and sends result
    to client. Caches the value on the client"]
    end

    subgraph FR_Async ["Async"]
    U3["**use_channel/use_listen_channel**<br />Create and listen to channels for communication between components
    (dioxus_sdk)"]
    end

    Init --> FR_Sync
    Init --> FR_Async

    %% Before Every Render
    subgraph BR_Sync ["Sync"]
    L19["**use_before_render**<br />Register a function to run before every subsequent render"]
    end

    BeforeRender --> BR_Sync

    %% Render phase
    subgraph RP_Sync ["Sync"]
    L7["**use_callback**<br />Keeps a callback up to date for all references to the handle"]
    L1["**use_signal**<br />Basic local state, triggers re-renders on write, does not subscribe to other signals"]
    L1b["**use_signal_sync**<br />Thread-safe signal - Send + Sync"]
    L3["**use_memo**<br />Derived state (memoized), composes signals"]
    L10["**use_set_compare/use_set_compare_equal**<br />Efficient value change tracking"]
    end

    subgraph RP_Both ["Sync & Async"]
    L8["**use_reactive**<br />Creates a closure (async/sync) to track 'non-reactive' data"]
    end

    subgraph RP_Async ["Async"]
    L5["**use_future**<br />Run an async task once"]
    L6["**use_coroutine**<br />Stream-based concurrency through channels"]
    L4["**use_resource**<br />Async derived state"]
    L15["**use_server_future**<br />Async derived state that runs on the server and caches on the client"]
    end

    Render --> RP_Sync
    Render --> RP_Both
    Render --> RP_Async

    %% Post render
    subgraph PR_Sync ["Sync"]
    L2["**use_effect**<br />Side effects after render, composes signals"]
    L9["**use_hook_did_run**<br />Lifecycle check if this hook has been called on the latest render"]
    L17["**use_after_render**<br />Push a function to be run after the next render"]
    end

    Post --> PR_Sync

    %% Cleanup
    subgraph CL_Sync ["Sync"]
    L13["**use_drop**<br />Callback invoked when component is dropped"]
    end

    Clean --> CL_Sync

    %% Context
    subgraph CT_Sync ["Sync"]
    C1["**use_context_provider**<br />Provides data to any child"]
    C2["**use_context**<br />Consume provided data"]
    C3["**try_use_context**<br />Like use_context but returns None if missing"]
    C4["**use_root_context**<br />Like use_context except creates context at root if missing"]
    C5["**use_coroutine_handle**<br />Obtain handle to a context-provided coroutine"]

    subgraph CT_Utils ["Utils"]
    C6["**use_clipboard**<br />Access the clipboard (dioxus_sdk)"]
    C7["**use_window_size**<br />Initial window size will be returned with this hook and updated continously as the
    window is resized (dioxus_sdk)"]
    C8["**use_geolocation**<br />Provides the latest geocoordinates. Good for navigation-type apps (dioxus_sdk)"]
    C9["**use_system_theme**<br />Initial theme will be returned and updated if the system theme changes (dioxus_sdk)"]
    end
    end

    C --> CT_Sync

    %% Persistent
    subgraph PS_Sync ["Sync"]
    P1["**use_persistent**<br />Store data across application reloads as either local storage or a file storage
    (dioxus_sdk)"]
    P2["**use_storage**<br />Like use_persistent except the storage location is generic, which may be useful for custom
    implementations (dioxus_sdk)"]
    P3["**use_synced_storage**<br />Store data that persists and syncs across all app sessions (dioxus_sdk)"]
    P4["**use_singleton_persistent**<br />Persistent storage shared for calls from same line (dioxus_sdk)"]
    P5["**use_storage_entry**<br />Creates a StorageEntry with latest value from storage or init if does not exist
    (dioxus_sdk)"]
    P6["**use_synced_storage_entry**<br />Creates a StorageEntry with updates subscription (dioxus_sdk)"]
    end

    D --> PS_Sync

    %% Global
    subgraph GB_Sync ["Sync"]
    G1["**GlobalSignal**<br />Global signal (sync)"]
    G2["**GlobalMemo**<br />Derived global state (sync)"]
    G3["**Global<T>**<br />A lazy value that is created once per application (sync)"]
        end

        E --> GB_Sync

        %% Route
        subgraph RT_Sync ["Sync"]
        R3["**Routable**<br />The dioxus-router macro used for routing"]

        subgraph RT_Utils ["Utils"]
        R1["**use_route**<br />Access information about the current routing location"]
        R2["**use_navigator**<br />Access navigator to change router history"]
        R4["**use_outlet_context**<br />Access to the outlet context for the route nesting level"]
        end
        end

        F --> RT_Sync

        %% Legend
        subgraph Legend ["Legend"]
        L_HIGHLIGHT["Frequently Used"]
        L_NORMAL["Standard"]
        end

        %% Styling
        classDef frequentlyUsed fill:#ff9,stroke:#333,stroke-width:2px
        class L_HIGHLIGHT,L1,L2,L3,L4,L5,L8,L14,C1,C2,R1,R2,R3,P1 frequentlyUsed"#
}
```