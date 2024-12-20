# Introduction

Welcome to the Dioxus documentation! Dioxus is a framework for building cross-platform apps with the Rust programming language. With one codebase, you can build apps that run on web, desktop, and mobile.

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
- [Core Concepts](essentials/index.md) provides detail on topics like managing state.
- [Guides](reference/index.md) provides references for things like assets, routing, testing, and more.

First, try walking through the [Tutorial](guide/index.md) to get familiar with Dioxus. Before embarking on a larger project, we strongly recommend reading the entire [Essential Concepts](essentials/index.md) and glancing through the [Guides Overview](guides/index.md).

> This guide assumes you already know some [Rust](https://www.rust-lang.org/)! If not, we recommend reading [*the book*](https://doc.rust-lang.org/book/ch01-00-getting-started.html) to learn Rust first.

## What is Dioxus?

Dioxus is a developer-friendly framework that empowers developers to ship cross-platform apps with one codebase.

![Multi-platform app architecture diagram](/assets/static/dioxus-architecture-diagram.png)

In many ways, Dioxus is similar to Flutter: we integrate our own build tools, foster an ecosystem, and provide a markup language for declaring UI. In key areas, Dioxus takes a different approach:

- Apps are declared with HTML and CSS instead of custom styling solution
- Reactivity is inspired by web frameworks like React and SolidJS
- Dioxus code runs natively with no virtual machine and enables direct FFI with system APIs

Our goal is to provide a "better Flutter": faster, slimmer, and web-native. You can think of Dioxus is a hybrid of [Flutter](http://flutter.dev) and [NextJS](http://nextjs.org): cross-platform apps with stellar fullstack support. Today, Dioxus apps can only be written in Rust, but we plan to support more languages in the future.

## Why Dioxus?

We started Dioxus because we believe the current standard of building apps is too complex. Developers need to learn and install dozens of different tools just to get their app into the world.

![App stack](/assets/static/dioxus-app-stack.png)

Our vision for Dioxus is a framework that is fast, flexible, and has a minimal learning curve. We want developers to confidently ship their app from idea to production as fast as possible. We believe that fewer tools and a simpler architecture makes it easier to develop apps. Apps that are easier to build also ship faster and are more likely to succeed.

## Syntax and Ecosystem

The Dioxus syntax is similar to React's JSX markup, borrowing React's component and hooks approach. All components are Rust functions that take `Properties`, define state with hooks, and return an `Element`. We only support markup in with the `rsx! {}` markup; this ensures your app is automatically optimized and has stellar devtools support like advanced hot-reloading.

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

Dioxus is designed to be easy to extend and fairly thin over system APIs. This means you can easily drop into system APIs when first-party APIs are lacking. When targeting the web, this might mean using [`web-sys`](http://crates.io/crates/web-sys/) and on Android using [`jni`](http://crates.io/crates/jni).

```rust
fn PromptModal() {
    #[cfg(web)]
    web_sys::call_web_function();

    #[cfg(android)]
    jni_sys::call_android_function();
}
```

The core Dioxus framework covers a number of utilities that are either challenging to design or integrate the `dx` tooling:

- [App Routing](router/index.md)
- [Backend integration via server functions](guides/fullstack/server_functions.md)
- [Including and optimizing](guides/assets.md) assets
- [State management](guides/managing_state.md) (signals-based reactivity)
- [SDK](http://github.com/dioxusLabs/sdk): 1st-party System integrations

## Stability

Dioxus has not reached a "1.0" release yet.

We are currently on version 0.6, which has stabilized a huge number of APIs are drastically improved the developer experience. In version 0.5 we overhauled the state management system and in 0.6 we overhauled tooling.

It's likely that the next few versions of Dioxus (0.7, 0.8) will bring breaking changes to your apps. Fortunately, these planned changes will only affect the syntax of specific APIs and not your apps at large. With every version update, we ship a rather comprehensive migration guide - eg [0.6](migration/index.md).

## Examples, projects, tutorials, and more

The Dioxus ecosystem is growing and so are the number of examples, projects, tutorials, books, and other learning resources.

We highly recommend a few first-party sources:

- The [official folder of small examples](https://github.com/DioxusLabs/dioxus/tree/main/examples)
- The [official repository of example projects](https://github.com/DioxusLabs/dioxus/tree/main/example-projects)
- The official [YouTube channel](https://www.youtube.com/@DioxusLabs)

## Who's funding Dioxus?

Dioxus is funded by a mix of corporate sponsorships, enterprise support contracts, [crowd-sourced funding](https://github.com/sponsors/DioxusLabs#sponsors), and [venture capital](http://ycombinator.com/companies/dioxus-labs). We strive to maintain a healthy mix of funding to balance the various competing visions of the future. We want to provide a "Flutter but better" for everyone - not controlled by Apple, Meta, or Google - and we need to make sure Dioxus has a sustainable long-term financial future.

Ultimately, we'd like Dioxus to be self-sustaining. This means that you'll eventually have the option to deploy your production apps with [Dioxus Deploy](/deploy). Revenue from *Dioxus Deploy* will in turn fund development on Dioxus itself.

We're committed to keeping Dioxus free and open source forever. You'll never need to pay us to build apps nor will we ever change the license of Dioxus.
