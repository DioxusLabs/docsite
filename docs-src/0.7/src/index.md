# Introduction

Dioxus is a framework for building cross-platform apps with the Rust programming language. With one codebase, you can build apps that run on web, desktop, and mobile platforms.

```rust
{{#include ../docs-router/src/doc_examples/readme.rs}}
```

```inject-dioxus
DemoFrame {
    readme::App {}
}
```

This guide is split into different sections:

- [Tutorial](tutorial/index.md) walks you through your first Dioxus app.
- [Core Concepts](essentials/index.md) provides detail on topics like managing state.
- [Guides](guides/index.md) provides references for things like assets, routing, testing, and more.

First, try walking through the [Tutorial](tutorial/index.md) to get familiar with Dioxus. Before embarking on a larger project, we strongly recommend reading the entire [Essential Concepts](essentials/index.md) and glancing through the [Guides Overview](guides/index.md). This guide assumes you already know some [Rust](https://www.rust-lang.org/)! If not, we recommend reading [*the book*](https://doc.rust-lang.org/book/ch01-00-getting-started.html) to learn Rust first.

## What is Dioxus?

Dioxus is a developer-friendly framework that empowers developers to ship cross-platform apps with one codebase. You write your apps in Rust, style them with HTML/CSS, enhance them with native APIs, and distribute them as platform-native bundles.

![Multi-platform app architecture diagram](/assets/static/dioxus-architecture-diagram.png)

In many ways, Dioxus is similar to Flutter: we integrate our own build tools, foster an ecosystem, and provide a GUI framework. In key areas, Dioxus takes a different approach:

- Apps are declared with HTML and CSS instead of custom styling solution
- Reactivity is inspired by web frameworks like React and SolidJS
- Dioxus code runs natively with no virtual machine, enabling zero-overhead calls to system APIs

Our goal is to provide a "better Flutter": faster, slimmer, and web-native. You can think of Dioxus as a hybrid of [Flutter](http://flutter.dev), [React Native](http://reactnative.dev), and [NextJS](http://nextjs.org): cross-platform apps with stellar fullstack support. Today, Dioxus apps can only be written in Rust, but we plan to support more languages in the future.

## Why Dioxus?

We built Dioxus because we believe the current standard of building apps is too complex. Developers need to learn and install dozens of different tools just to get their app into the world. We need a simpler and more powerful framework to bring apps from ideas to reality.

![App stack](/assets/static/dioxus-app-stack.png)

Our vision for Dioxus is a framework that is fast, flexible, and has a minimal learning curve. We want developers to confidently launch their app from idea to production as fast as possible. We believe that fewer tools and a simpler architecture makes it easier to develop apps. Apps that are easier to build ship faster and are more likely to succeed.

## Stellar Developer Experience

With Dioxus, we try to maintain a very high bar for developer experience. We believe that building apps should be fun and straightforward. We've worked to push forward the Rust itself, developing technologies like [Subsecond](https://crates.io/crates/subsecond) Rust hot-reloading, [WASM bundle-splitting](https://crates.io/crates/wasm-splitter), [linker-based asset bundling](https://crates.io/crates/manganis), and a modular [WGPU-based HTML/CSS renderer](https://crates.io/crates/blitz).

![dog_app.mp4](/assets/06assets/dogapphr2.mp4)


## Syntax and Ecosystem

The Dioxus syntax is similar to React's JSX markup, borrowing React's component and hooks approach. All components are Rust functions that take `Properties`, define state with hooks, and return an `Element`. We only support markup declared with the `rsx! {}` macro; this ensures your app is automatically optimized and supports our powerful devtools.

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

Dioxus is designed to be easy to extend and fairly thin over system APIs. This means you can easily drop into system APIs when first-party APIs are lacking. When targeting the web, this might mean using [`web-sys`](http://crates.io/crates/web-sys/), on Android using [`jni`](http://crates.io/crates/jni), or on iOS using [`objc2`](https://crates.io/crates/objc2).

```rust
fn PromptModal() {
    #[cfg(web)]
    web_sys::call_web_function();

    #[cfg(android)]
    jni_sys::call_android_function();

    #[cfg(ios)]
    obcj2::call_ios_function();
}
```

The core Dioxus framework covers a number of utilities that are either challenging to design or need support our devtools:

- [App Routing](essentials/router/index.md)
- [Backend integration via server functions](essentials/fullstack/index.md)
- [Including and optimizing](essentials/ui/assets.md) assets
- [State management](essentials/state/index.md) (signals-based reactivity)
- [SDK](http://github.com/dioxusLabs/sdk): 1st-party System integrations

## Who's funding Dioxus?

Dioxus is funded by a mix of corporate sponsorships, enterprise support contracts, [crowd-sourced funding](https://github.com/sponsors/DioxusLabs#sponsors), and [venture capital](http://ycombinator.com/companies/dioxus-labs). We strive to maintain a healthy mix of funding to balance the various competing visions of the future. We want to provide a "Flutter but better" for everyone - not controlled by Apple, Meta, or Google - and we need to make sure Dioxus has a sustainable long-term financial future.

Ultimately, we'd like Dioxus to be self-sustaining. This means that you'll eventually have the option to deploy your production apps with [Dioxus Deploy](https://dioxuslabs.com/deploy). Revenue from *Dioxus Deploy* will in turn fund development on Dioxus itself.

We're committed to keeping Dioxus free and open source forever. You'll never need to pay us to build apps nor will we ever change the license of Dioxus.
