# Project Structure

There are many packages in the Dioxus organization. This document will help you understand the purpose of each package and how they fit together:

![Dioxus Dependency Graph](/assets/static/workspace-graph.png)

## Entry Points

- [dioxus](https://github.com/DioxusLabs/dioxus/tree/main/packages/dioxus): The main crate for Dioxus applications. The dioxus crate has different feature flags to enable a specific [renderer](#renderers) with the launch API and expose different features like the router and [fullstack](#fullstack). The [CLI](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli) uses the renderer feature flag that is enabled to determine what rust target to compile for.
- [dioxus-lib](https://github.com/DioxusLabs/dioxus/tree/main/packages/dioxus-lib): Dioxus lib is a re-export of the dioxus crate without any renderer features. This crate is recommended for libraries because it is impossible to pull in a renderer feature accidentally which would enable that renderer for any downstream crates.

## Renderers

Renderers are the entry point for Dioxus applications. They handle rendering the application, polling async tasks, and handling events. Each renderer depends on `dioxus-core` for the core virtual dom and implements both the history trait from `dioxus-history` and the event conversion trait from `dioxus-html`. Dioxus has four renderers in the main repository:

- [desktop](https://github.com/DioxusLabs/dioxus/tree/main/packages/desktop): A Render that Runs Dioxus applications natively, but renders them with the system webview
- [mobile](https://github.com/DioxusLabs/dioxus/tree/main/packages/mobile): A Render that Runs Dioxus applications natively, but renders them with the system webview. This is currently a think wrapper on top of the desktop renderer since both renderers use the webview
- [web](https://github.com/DioxusLabs/dioxus/tree/main/packages/web): Renders Dioxus applications in the browser by compiling to WASM and manipulating the DOM. The web renderer has a hydration feature to take over rendering from the server if [fullstack](#fullstack) is enabled
- [liveview](https://github.com/DioxusLabs/dioxus/tree/main/packages/liveview): A Render that Runs on the server, and renders using a websocket proxy in the browser. The liveview renderer is currently supported, but development has been deprioritized in favor of fullstack and it may be removed in the future

> The [TUI](https://github.com/DioxusLabs/blitz/tree/legacy/packages/dioxus-tui) renderer has been deprecated but may be revisited in the future once the new version of Blitz is more stable

## Experimental Native Rendering

In addition to the renderers listed above, Dioxus also has an experimental native renderer called Blitz that uses WebGPU to render HTML+CSS for dioxus applications:

- [taffy](https://github.com/DioxusLabs/taffy): Layout engine powering Blitz-Core, Plasmo, and Bevy UI
- [blitz](https://github.com/DioxusLabs/blitz): An experimental native renderer for Dioxus applications using WGPU

## Fullstack

Fullstack can be layered on top of any renderer to add support for server functions and server-side rendering.

- [ssr](https://github.com/DioxusLabs/dioxus/tree/main/packages/ssr): dioxus-ssr handles rendering a dioxus virtual dom to a string for testing or on the server. SSR is used in the fullstack renderer to handle server side rendering and static generation.
- [isrg](https://github.com/DioxusLabs/dioxus/tree/main/packages/isrg): dioxus-isrg handles incremental static site generation for dioxus fullstack applications. It helps fullstack cache server side rendered routes in memory and on the file system.
- [fullstack](https://github.com/DioxusLabs/dioxus/tree/main/packages/fullstack): dioxus-fullstack package handles the integration between a [axum](https://github.com/tokio-rs/axum) server and a dioxus renderer. If the frontend renderer is targeting the web, the fullstack renderer will prepare html with embedded data so the client can take over rendering after the initial load (hydration)
- [server-macro](https://github.com/DioxusLabs/dioxus/tree/main/packages/server-macro): The server-macro crate defines the `server` macro used to define server functions in Dioxus applications. It integrates with the [server_fn](https://crates.io/crates/server_fn) to automatically register the server functions on the server and call them on the client.

## Core utilities

The core utilities contain the implementation of the virtual dom, and other macros used in all dioxus renderers. The core of dioxus doesn't assume it is running in a web context, so these utilities can be used by third party renderers like [Freya](https://github.com/marc2332/freya).

- [core](https://github.com/DioxusLabs/dioxus/tree/main/packages/core): The core virtual dom implementation every Dioxus application uses. The main entry point for core is the `VirtualDom`. The virtual dom diffing methods accept a cross platform `WriteMutations` trait that is called any time the renderer need to change what is rendered. The vdom also has methods for running futures, and inserting events. You can read more about the architecture of the core [in this blog post](https://dioxuslabs.com/blog/templates-diffing/)
- [core-types](https://github.com/DioxusLabs/dioxus/tree/main/packages/core-types): The core types crate contains some of the core functions used in both in dioxus core and the hot reloading engine.
- [core-macro](https://github.com/DioxusLabs/dioxus/tree/main/packages/core-macro): The core macro crate implement the `derive(Props)` and `#[component]` macros to derive builds for components. It also re-exports the rsx macro
- [rsx](https://github.com/DioxusLabs/dioxus/tree/main/packages/rsx): Implements parsing and expansion for the RSX macro. The parser is also used for hot reloading, and autoformatting in the CLI

## Web utilities

Every first party dioxus renderer targets html and css. With the exception of the blitz, all renderers run inside the browser context. Dioxus has a few utilities in the workspace with shared traits and javascript bindings to help interact with the browser:

- [interpreter](https://github.com/DioxusLabs/dioxus/tree/main/packages/interpreter): The interpreter implements the `WriteMutations` trait from dioxus core to modify the DOM with the diffs the virtual dom generates. The interpreter is used by the desktop, web and liveview renderers. It uses a combination of [`wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen) and [`sledgehammer-bindgen`](https://github.com/ealmloff/sledgehammer_bindgen) to interact with the browser
- [html](https://github.com/DioxusLabs/dioxus/tree/main/packages/html): defines html specific elements, events, and attributes. The elements and attributes are used in the rsx macro and hot reloading engine to map the rust identifiers to the html names. The events defined in the html crate are traits defined for each platform.
- [html-internal-macro](https://github.com/DioxusLabs/dioxus/tree/main/packages/html-internal-macro): The html-internal-macro crate is used by the html crate to define the html elements and attributes.
- [lazy-js-bundle](https://github.com/DioxusLabs/dioxus/tree/main/packages/lazy-js-bundle): A library to bundle typescript files at build time with bun only if the contents change. Only compiling the typescript when the files change and committing the build output lets us not require a ts compiler to be installed when dioxus is added as a library.
- [history](https://github.com/DioxusLabs/dioxus/tree/main/packages/history): The dioxus-history crate defines the history trait backing each renderer must provide for use with the router. For web renderers, this should call the javascript history api. Native renderers maintain their own history stack in memory.
- [document](https://github.com/DioxusLabs/dioxus/tree/main/packages/document): The dioxus-document crate defines the document trait backing each renderer must provide for use with `eval` and the `document::*` components. `eval` runs javascript code from rust, and the `document::*` components create html elements in the head.

## State Management

- [generational-box](https://github.com/DioxusLabs/dioxus/tree/main/packages/generational-box): Generational Box is the core of all `Copy` state management in Dioxus. It allocates an arena of dynamically borrow checked values used throughout the dioxus ecosystem. The `GenerationalBox` type backs `Signal`, `Memo`, and `Resource` in dioxus signals. It is also used in `dioxus-core` to make the `Closure` and `EventHandler` types `Copy`.
- [signals](https://github.com/DioxusLabs/dioxus/tree/main/packages/signals): Signals are the main user facing state management crate for Dioxus. Signals track when they are read and written to and automatically re-run any `ReactiveContext`s that depends on the signal.
- [hooks](https://github.com/DioxusLabs/dioxus/tree/main/packages/hooks): Hooks are a collection of common hooks for Dioxus applications. Most hooks are a thin wrapper over the new methods in the `signals` crate to only create the object once when the component is created.

## Logging

- [logger](https://github.com/DioxusLabs/dioxus/tree/main/packages/logger): The logger crate provides a simple logging interface for Dioxus applications that works across native and wasm targets. It is automatically called in the launch function if the logging feature is enabled.

## Routing

- [router](https://github.com/DioxusLabs/dioxus/tree/main/packages/router): The router crate handles routing in Dioxus applications. It uses the history provider the renderer provides to get and modify the url. The route parsing logic is derived with the `derive(Routable)` macro defined in the dioxus-router-macro crate.
- [router-macro](https://github.com/DioxusLabs/dioxus/tree/main/packages/router-macro): The router-macro crate defines the `derive(Routable)` macro used to the route enum from a url and display it as a url.

## Assets

- [manganis](https://github.com/DioxusLabs/dioxus/tree/main/packages/manganis/manganis): Manganis is dioxus' asset system. It uses a macro to inject assets from rust code into the linker. Every asset gets a unique hash for cache busting. The CLI pulls the asset out of the linker and bundled them into the final application.
- [manganis-macro](https://github.com/DioxusLabs/dioxus/tree/main/packages/manganis/manganis-macro): Manganis-macro defines the `asset!()` macro used to include assets in Dioxus applications.
- [manganis-core](https://github.com/DioxusLabs/dioxus/tree/main/packages/manganis/manganis-core): Manganis-core contains the builders for all options passed into the `asset!()` macro and the link sections the asset macro and CLI use to bundle assets.
- [const-serialize](https://github.com/DioxusLabs/dioxus/tree/main/packages/const-serialize): Const Serialize defines a trait to serialize rust types to a cross platform format at compile time. This is used to serialize the options for assets at compile time in manganis.
- [const-serialize-macro](https://github.com/DioxusLabs/dioxus/tree/main/packages/const-serialize-macro): Const Serialize Macro defines a derive macro for types that can be serialized at compile time with the `const-serialize` crate.
- [cli-opt](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli-opt): The cli-opt optimizes the assets that manganis produces.

## Formatting

- [autofmt](https://github.com/DioxusLabs/dioxus/tree/main/packages/autofmt): The autofmt crate finds and formats all rsx macros in a rust project. It uses the `dioxus-rsx` crate to parse rsx.

## Linting

- [check](https://github.com/DioxusLabs/dioxus/tree/main/packages/check): The dioxus-check crate analyzes dioxus code to check for common errors like calling hooks in conditionals or loops.

## Translation

- [rsx-rosetta](https://github.com/DioxusLabs/dioxus/tree/main/packages/rsx-rosetta): The rsx-rosetta crate translates html to rsx. It uses the element definitions from `dioxus-html` to translate html elements and attributes to their rust names and the `rsx` crate to generate the rsx macro.

## Hot Reloading

- [rsx-hotreload](https://github.com/DioxusLabs/dioxus/tree/main/packages/rsx-hotreload): The rsx-hotreload crate handles diffing rsx macros between builds and creating the hot reload templates for the CLI.
- [devtools](https://github.com/DioxusLabs/dioxus/tree/main/packages/devtools): The devtools crate contains the frontend for hot reloading each renderer needs to integrate with. It receives hot reload messages from a websocket connection with the CLI
- [devtools-types](https://github.com/DioxusLabs/dioxus/tree/main/packages/devtools-types): The devtools-types crate contains the types used to communicate between the devtools frontend and the backend in the CLI.

## CLI

- [cli](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli): The cli crate contains the dioxus CLI. It integrates check, autofmt, cli-opt, and rsx-hotreload to build and serve Dioxus applications.
- [cli-config](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli-config): The cli-config crate has shared types that are provided at runtime from the CLI to crates the CLI are built with. It is used by `dioxus-desktop` to set the title from the `Dioxus.toml` file and by `dioxus-fullstack` to set the port the CLI proxies the server from.
- [dx-wire-format](https://github.com/DioxusLabs/dioxus/tree/main/packages/dx-wire-format): The dx-wire-format crate has the unstable types the CLI emits in json mode. This is used by the dioxus playground.

## Extension

- [extension](https://github.com/DioxusLabs/dioxus/tree/main/packages/extension): The extension folder contains the source code for the dioxus VSCode extension. It uses many of the same crates as the CLI, but packaged into a wasm+JS bundle for VSCode.

## Testing

- [playwright-tests](https://github.com/DioxusLabs/dioxus/tree/main/packages/playwright-tests): The playwright-tests folder contains end to end tests for dioxus-web, dioxus-liveview and fullstack. These crates are not published on crates.io
