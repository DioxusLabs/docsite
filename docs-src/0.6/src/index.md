# Introduction

<!--
Dioxus
- what is it
- why does it exist?
- what can you do with it
- what does it look like
- how does it work
- is it ready and what's next?
- how do I learn it?
-->

Welcome to the Dioxus documentation! Dioxus is a framework for building crossplatform apps with the Rust programming language. With one codebase, you can build apps that run on web, desktop, and mobile.

Dioxus is designed to be familiar for developers who already know tools like React and Flutter.

```rust
{{#include src/doc_examples/readme.rs}}
```

```inject-dioxus
DemoFrame {
    readme::App {}
}
```

This guide is split into different sections:

- [Tutorial](guide/index.md) walks you through your first Dioxus app.
- [Essential Concepts](essentials/index.md) provides detail on topics like managing state and structuring your app.
- [Guides](reference/index.md) provides references for things like assets, routing, testing, and more.

First, try walking through the Tutorial to get a sense of the Dioxus. Before embarking on a larger project, we strongly recommend reading the Essential Concepts and glancing through the [Guides Overview]()

> This guide assumes you already know some [Rust](https://www.rust-lang.org/)! If not, we recommend reading [*the book*](https://doc.rust-lang.org/book/ch01-00-getting-started.html) to learn Rust first.

## What is Dioxus?

Dioxus is a batteries-included framework that empowers developers to ship crossplatform apps with one codebase.

![maybe a graphic of our fileystem comparison?]()

In many ways, Dioxus is similar to Flutter: we integrate our own build tools, foster an ecosystem, and provide a markup language for declaring UI. In key areas, Dioxus takes a different approach:

- Apps are declared with HTML and CSS instead of a bespoke styling solution
- Reactivity is inspired by web frameworks like React and SolidJS
- Dioxus code runs natively with no virtual machine, enabling system APIs via direct FFI

Our goal is to provide a "better Flutter": faster, slimmer, and web-native. You can think of Dioxus is a hybrid of [Flutter]() and [NextJS](): crossplatform apps with stellar fullstack support. Today, Dioxus apps can only be written in Rust, but we plan to support more languages in the future.


## Why Dioxus?

Why yet another app framework?

We started Dioxus because we believe the status quo of building apps is too complex. Developers need to learn and install dozens of different tools just to get their app into the world.

Our vision for Dioxus is a framework that is fast, flexible, and has a minimal learning curve. We want developers to confidently ship their app from idea to production faster than ever before.

<!-- Dioxus reduces this complexity by integrating powerful tooling with core libraries. You can install our CLI, run `dx new`, and have an app running in less than 10 seconds. With `dx serve` you can rapidly iterate on your app with live hotreloading. With `dx bundle` you can produce procution-ready app bundles to publish to the app store or your favorite hosting provider. -->


## Syntax and Ecosystem

The Dioxus syntax is simlar to React's JSX markup, borrowing React's component and hooks approach. All components are Rust functions that take `Properties`, define state with hooks, and return an `Element`. We only support markup in with the `rsx! {}` markup; this ensures your app is automatically optimized and has stellar devtools support.

```rust
#[component]
fn Component(name: String) -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "Hello, {name}" }
        p { "Count: {count}" }
    }
}
```

Dioxus is designed to be easy to extend and fairly thin over system APIs. This means you can easily drop into system APIs when first-party APIs are lacking. When targetting the web, this might mean using [`web-sys`]() and on android using [`jni-sys`]().

```rust
fn PromptModal() {
    #[cfg(web)]
    web_sys::callfunction();

    #[cfg(android)]
    jni_sys::callfunction();
}
```

The core Dioxus framework covers a number of utilities that are either challenging to design or integrate the `dx` tooling:

- [App Routing]()
- Backend integration via [server functions]()
- [Including]() and [optimizing]() assets
- [State management]() (signals-based reactivity)
- [SDK](): 1st-party system integrations (geolocation, internationalization, storage, etc)

## Framework Architecture

The Dioxus framework is composed of two parts: the `dx` tool and the `dioxus` Rust crate. The `dioxus` crate is modular and can be used without `dx`, but you'll find that `dx` provides a much better developer experience than other alternatives in the Rust ecosystem.

![graphic of the dx tool archiecture]()

The `dx` tool can build your app for a many number of platforms:
- Mobile: Android (`.apk`) and iOS (`.ipa`),
- Desktop: macOS (`.app`), Windows (`.msi`, `.exe`), Linux (`.rpm`, `.deb`, `.appimage`)
- Web: WebAssembly (`.wasm` and `/public`), servers, and static site generation

The `dx` tool provides a number of helpful utilities that the default Rust `cargo` experience lacks:

- Instant hotreloading of `rsx! {}` markup blocks
- Instant hotreloading of assets like images and stylesheets
- Automatic reloading of Rust code with interactive controls
- Autoformatting of some Rust macros like `rsx! {}`

The `dioxus` crate includes a number of optional features that integrate with `dx`:

- [`router`](): Integrate with the platform's routing technology
- [`fullstack`](): Automatically add function-based server RPC with the `#[server_fn]` attribute
- [`server`](): Render your app using [server-side-rendering]() with [client-side hydration]()

Several other features like `devtools` and `document` are available, though it is unlikely you'll disable any of these for most apps.

## Stability

Dioxus has not reached a stable release yet.

> We strive to be transparent with the development of Dioxus: our [full roadmap is publicly available in the next chapter]()!

We are currently on version 0.6, which has stabilized a huge number of APIs are drastically improved the developer experience across the board. In version 0.5 we overhauled the state management system and in 0.6 we overhauled build tooling.

![image of the roadmap based on versioning]()


It's likely that the next few versions of Dioxus (0.7, 0.8) will bring breaking changes to your apps. Fortunately, these planned changes will only affect the syntax of specific APIs and not your apps at large. With every version update, we ship a rather comprehensive migration guide ([0.5](), [0.5](), [0.6]()).

## Examples, projects, tutorials, and more

The Dioxus ecosystem is rapidly growing and so are the number of examples, projects, tutorials, books, and other learning resources.

We highly recommend a few first-party sources:

- The official folder of small examples
- The ofificial repository of example projects (clones of social media apps, e-commerce sites, and more)
- The official YouTube channel for tutorials made by the Dioxus authors

## Who's funding Dioxus?

Dioxus is funded by a mix of [corporate sponsorships](), enterprise support contracts, [crowdsourced funding](), and [venture capital](). We strive to maintain a healthy mix of funding to balance the various competing visions of the future.

Our core team operates full time and we have plenty of runway to keep Dioxus funded for the forseeable future.

Longer term, we'd like Dioxus to be self-sustaining. This means that you'll have the option to deploy your production apps with [Dioxus Deploy](). Revenue from *Dioxus Deploy* will in turn fund development on Dioxus itself.

We're committed to keeping Dioxus open source forever and you'll never need to pay us to build your apps.
