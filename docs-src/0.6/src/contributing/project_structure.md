# Project Structure

There are many packages in the Dioxus organization. This document will help you understand the purpose of each package and how they fit together.

## Renderers

- [Desktop](https://github.com/DioxusLabs/dioxus/tree/main/packages/desktop): A Render that Runs Dioxus applications natively, but renders them with the system webview
- [Mobile](https://github.com/DioxusLabs/dioxus/tree/main/packages/mobile): A Render that Runs Dioxus applications natively, but renders them with the system webview. This is currently a copy of the desktop render
- [Web](https://github.com/DioxusLabs/dioxus/tree/main/packages/Web): Renders Dioxus applications in the browser by compiling to WASM and manipulating the DOM
- [Liveview](https://github.com/DioxusLabs/dioxus/tree/main/packages/liveview): A Render that Runs on the server, and renders using a websocket proxy in the browser
- [Plasmo](https://github.com/DioxusLabs/blitz/tree/master/packages/plasmo): A Renderer that renders a HTML-like tree into a terminal
- [TUI](https://github.com/DioxusLabs/blitz/tree/master/packages/dioxus-tui): A Renderer that uses Plasmo to render a Dioxus application in a terminal
- [Blitz-Core](https://github.com/DioxusLabs/blitz/tree/master/packages/blitz-core): An experimental native renderer that renders a HTML-like tree using WGPU.
- [Blitz](https://github.com/DioxusLabs/blitz): An experimental native renderer that uses Blitz-Core to render a Dioxus application using WGPU.
- [SSR](https://github.com/DioxusLabs/dioxus/tree/main/packages/ssr): A Render that Runs Dioxus applications on the server, and renders them to HTML

## State Management/Hooks

- [Hooks](https://github.com/DioxusLabs/dioxus/tree/main/packages/hooks): A collection of common hooks for Dioxus applications
- [Signals](https://github.com/DioxusLabs/dioxus/tree/main/packages/signals): A experimental state management library for Dioxus applications. This currently contains a `Copy` version of Signal
- [SDK](https://github.com/DioxusLabs/sdk): A collection of platform agnostic hooks to interact with system interfaces (The clipboard, camera, etc.).
- [Fermi](https://github.com/DioxusLabs/dioxus/tree/main/packages/fermi): A global state management library for Dioxus applications.
- [Router](https://github.com/DioxusLabs/dioxus/tree/main/packages/router): A client-side router for Dioxus applications

## Core utilities

- [core](https://github.com/DioxusLabs/dioxus/tree/main/packages/core): The core virtual dom implementation every Dioxus application uses
  - You can read more about the architecture of the core [in this blog post](https://dioxuslabs.com/blog/templates-diffing/)
- [RSX](https://github.com/DioxusLabs/dioxus/tree/main/packages/RSX): The core parsing for RSX used for hot reloading, autoformatting, and the macro
- [core-macro](https://github.com/DioxusLabs/dioxus/tree/main/packages/core-macro): The rsx! macro used to write Dioxus applications. (This is a wrapper over the RSX crate)
- [HTML macro](https://github.com/DioxusLabs/dioxus-html-macro): A html-like alternative to the RSX macro

## Native Renderer Utilities

- [Taffy](https://github.com/DioxusLabs/taffy): Layout engine powering Blitz-Core, Plasmo, and Bevy UI
- [Blitz](https://github.com/DioxusLabs/blitz): An experimental native renderer for HTML+CSS

## Web renderer tooling

- [HTML](https://github.com/DioxusLabs/dioxus/tree/main/packages/html): defines html specific elements, events, and attributes
- [Interpreter](https://github.com/DioxusLabs/dioxus/tree/main/packages/interpreter): defines browser bindings used by the web and desktop renderers

## Developer tooling

- [hot-reload](https://github.com/DioxusLabs/dioxus/tree/main/packages/hot-reload): Macro that uses the RSX crate to hot reload static parts of any rsx! macro. This macro works with any non-web renderer with an [integration](https://crates.io/crates/dioxus-hot-reload)
- [autofmt](https://github.com/DioxusLabs/dioxus/tree/main/packages/autofmt): Formats RSX code
- [rsx-rosetta](https://github.com/DioxusLabs/dioxus/tree/main/packages/RSX-rosetta): Handles conversion between HTML and RSX
- [CLI](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli): A Command Line Interface and VSCode extension to assist with Dioxus usage
