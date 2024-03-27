# Introduction

Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust. This guide will help you get started with writing Dioxus apps for the Web, Desktop, Mobile, and more.

```rust
{{#include src/doc_examples/readme.rs}}
```

```inject-dioxus
DemoFrame {
    readme::App {}
}
```

Dioxus is heavily inspired by React. If you know React, getting started with Dioxus will be a breeze.

> This guide assumes you already know some [Rust](https://www.rust-lang.org/)! If not, we recommend reading [*the book*](https://doc.rust-lang.org/book/ch01-00-getting-started.html) to learn Rust first.

## Features

- Cross platform apps in three lines of code. (Web, Desktop, Server, Mobile, and more)
- Incredibly ergonomic and powerful state management that combines the best parts of react, solid and svelte.
- Comprehensive inline documentation – hover and guides for all HTML elements, listeners, and events.
- High performance applications [approaching the fastest web frameworks on the web](https://dioxuslabs.com/blog/templates-diffing) and native speeds on desktop.
- First-class async support.

### Multiplatform

Dioxus is a *portable* toolkit, meaning the Core implementation can run anywhere with no platform-dependent linking. Unlike many other Rust frontend toolkits, Dioxus is not intrinsically linked to WebSys. In fact, every element and event listener can be swapped out at compile time. By default, Dioxus ships with the `html` feature enabled, but this can be disabled depending on your target renderer.

Right now, we have several 1st-party renderers:
- WebSys/Sledgehammer (for WASM): Great support
- Tao/Tokio (for Desktop apps): Good support
- Tao/Tokio (for Mobile apps): Poor support
- Fullstack (for SSR and server functions): Good support
- TUI/Plasmo (for terminal-based apps): Experimental

## Stability

Dioxus has not reached a stable release yet.

Web: Since the web is a fairly mature platform, we expect there to be very little API churn for web-based features.

Desktop: APIs will likely be in flux as we figure out better patterns than our ElectronJS counterpart.

Fullstack: APIs will likely be in flux as we figure out the best API for server communication.
