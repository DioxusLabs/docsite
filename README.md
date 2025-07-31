# dioxuslabs.com

This repository contains the source code for the https://dioxuslabs.com website.

This website is written with Dioxus, pre-generated with `dioxus_ssr`, and then
rehydrated with interactivity provided by `dioxus_web`.

## Development

The documentation can be edited using any text editor. Most commonly used
editors support syntax highlighting for the `markdown` format. To view your
changes you can install the [`dx`][dx] tool locally, assuming you already have a
working `Rust` setup:

<!-- todo: switch to the installer -->
<!-- # curl -fsSL https://raw.githubusercontent.com/DioxusLabs/dioxus/refs/heads/main/.github/install.sh | bash -->
```sh
cargo binstall dioxus-cli@0.7.0-alpha.3 --force
```

With [`dx`][dx] installed, you can use it to build and serve the documentation
on your local system:

```sh
dx serve --package dioxus_docs_site --hotpatch
```

This will start a local server that will be available on
[localhost:8080](localhost:8080) and will automatically build and re-build the
documentation when it changes.

We use TailwindCSS which is included automatically with dx 0.7.


## Dioxus 0.7 Overhaul Progress
We are overhauling the docs for Dioxus 0.7. Here is the current progress:

```
✅ - [Core Concepts](essentials/index.md)
❌   - [Setup]()
❌     - [Tools]
❌     - [Feature Flags]
❌     - [Folder Structure]
✅   - [Building User Interfaces](essentials/ui/index.md)
✅     - [Introducing RSX](essentials/ui/rsx.md)
✅     - [Elements and Text](essentials/ui/elements.md)
✅     - [Dynamic Attributes](essentials/ui/attributes.md)
✅     - [Components and Properties](essentials/ui/components.md)
✅     - [How Components Render](essentials/ui/render.md)
✅     - [Conditional Rendering](essentials/ui/conditional.md)
✅     - [Rendering Lists](essentials/ui/iteration.md)
✅     - [Assets](essentials/ui/assets.md)
✅     - [Styling](essentials/ui/styling.md)
✅     - [Hot-Reload](essentials/ui/hotreload.md)
✅     - [Escape Hatches](essentials/ui/escape.md)

❌   - [User Interaction](essentials/state/index.md)
✅     - [Intro to Reactivity]
✅     - [Storing State in Hooks](essentials/state/hooks.md)
✅     - Reactive Signals use_signal
✅     - User Input and Events
✅     - Futures (crash-course (lite), loop/spawn, onclick: move |_| async {})
✅     - Fetching? (use_resource, network requests)
✅     - Effects and Memos (use_memo + use_effect)
✅     - Hoisting State (lifting state and refactoring)
✅     - Global Context (context, global signals)
❌     - Stores and Collections
❌     - Working with External State (sync signals, external callbacks, layout effects, onmounted)

❌   - [Advanced Reactivity](essentials/reactivity/index.md)
❌     - [Reactive Context](essentials/reactivity/reactivity.md)
❌     - [Custom Hooks (schedule_update, use_hook)](essentials/state/custom_hooks.md)
❌     - [Component Lifecycle](essentials/reactivity/lifecycle.md)
❌     - [Side Effects](essentials/reactivity/effects.md)
❌     - [Error Handling (+boundaries)](essentials/state/error_handling.md)
❌     - [Maintaing Purity](essentials/reactivity/purity.md)
❌     - [Memoization](essentials/state/memoization.md)
❌     - [Optimization](essentials/state/optimization.md)
❌     - [Suspense](essentials/async/suspense.md)
❌     - [Futures in Depth (+threads/workers)](essentials/async/crash_course.md)

❌   - [Routing](essentials/router/index.md)
✅     - [Defining Routes](essentials/router/routes.md)
✅     - [Nested Routes](essentials/router/nested-routes.md)
✅     - [Layouts](essentials/router/layouts.md)
✅     - [Navigation](essentials/router/navigation/index.md)
✅     - [Programmatic Navigation](essentials/router/programmatic-navigation.md)
✅     - [History Providers](essentials/router/history-providers.md)
✅     - [History Buttons](essentials/router/history-buttons.md)
✅     - [Routing Update Callback](essentials/router/routing-update-callback.md)

❌   - [Fullstack](essentials/fullstack/index.md)
✅     - [Hydration](essentials/fullstack/hydration.md)
✅     - [Managing Dependencies](essentials/fullstack/managing_dependencies.md)
✅   	- [Server Functions](essentials/fullstack/server_functions.md)
✅   	- [Extractors](essentials/fullstack/extractors.md)
✅   	- [Middleware](essentials/fullstack/middleware.md)
✅   	- [Authentication](essentials/fullstack/authentication.md)
✅   	- [Routing](essentials/fullstack/routing.md)
✅     - [Streaming](essentials/fullstack/streaming.md)
✅     - [Static Site Generation](essentials/fullstack/static_site_generation.md)
✅     - [Axum Integration](essentials/fullstack/axum.md)

---

❌ - [Guides (digging deeper)](guides/index.md)
❌   - [Tools](guides/tools/index.md)
❌     - [Serve](guides/tools/serve.md)
❌     - [Bundle](guides/tools/bundle.md)
❌     - [Create a Project](guides/tools/creating.md)
❌     - [Configure Project](guides/tools/configure.md)
❌     - [Translate HTML](guides/tools/translate.md)
❌     - [VSCode Extension](guides/tools/vscode.md)
❌   - [APIs?](guides/platforms/index.md)
❌     - feature overview
❌     - window
❌     - document
❌     - history
❌     - desktop/webview
❌     - native
❌     - components
❌     - sdk
❌   - [Platform Support](guides/platforms/index.md)
❌     - [Web](guides/platforms/web.md)
❌     - [Desktop](guides/platforms/desktop.md)
❌     - [Mobile](guides/platforms/mobile.md)
❌     - [Android](guides/tools/android.md)
❌     - [iOS](guides/tools/ios.md)
❌   - [Publishing](guides/deploy/index.md)
❌     - [Web Apps](guides/deploy/web.md)
❌     - [SSG](guides/deploy/ssg.md)
❌     - [iOS Apps](guides/deploy/ios.md)
❌     - [macOS Apps](guides/deploy/macos.md)
❌     - [Linux Apps](guides/deploy/linux.md)
❌     - [Windows Apps](guides/deploy/windows.md)
❌     - [Android Apps](guides/deploy/android.md)
❌     - [Bundle Config](guides/deploy/config.md)
❌   - [Organizing your Project](guides/organization/index.md)
❌     - [Single Files](guides/organization/single.md)
❌     - [Workspaces](guides/organization/workspaces.md)
❌     - [Shared Code](guides/organization/shared.md)
❌   - [Testing and Debugging](guides/testing/index.md)
❌     - [Web](guides/testing/web.md)
❌     - [Desktop](guides/testing/desktop.md)
❌     - [Debugging](guides/testing/debugging.md)
❌     - [Continuous Integration](guides/testing/ci.md)
❌     - [Docker](guides/testing/docker.md)
❌     - [Optimizing](guides/tips/optimizing.md)
❌     - [Anti-patterns](guides/tips/antipatterns.md)
❌   - [Utilities](guides/utilities/index.md)
❌     - [Logging](guides/utilities/logging.md)
❌     - [Internationalization](guides/utilities/internationalization.md)
❌     - [Tailwind](guides/utilities/tailwind.md)
❌   - [In-Depth](guides/depth/index.md)
❌     - [Asset Pipeline](guides/depth/assets.md)
❌     - [Custom Renderer](guides/depth/custom_renderer.md)
✅   - [Migration](migration/index.md)
✅     - [To 0.7](migration/to_07.md)
✅     - [To 0.6](migration/to_06.md)
✅     - [To 0.5](migration/to_05/index.md)
✅       - [Hooks](migration/to_05/hooks.md)
✅         - [State](migration/to_05/state.md)
✅       - [Fermi](migration/to_05/fermi.md)
✅       - [Props](migration/to_05/props.md)
```


## Contributing

- Check out the website [section on contributing]
- Report issues on our [issue tracker]
- Join the discord and ask questions!

<a href="https://github.com/dioxuslabs/docsite/graphs/contributors">
  <img
    src="https://contrib.rocks/image?repo=dioxuslabs/docsite&max=30&columns=10"
  />
</a>

[dx]: https://github.com/DioxusLabs/dioxus/tree/main/packages/cli
[section on contributing]: https://dioxuslabs.com/learn/0.6/contributing
[issue tracker]: https://github.com/dioxuslabs/docsite/issues
