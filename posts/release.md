# Introducing Dioxus

After many months of work, we're very excited to release the first version of Dioxus!

Dioxus is a new library for building interactive user interfaces with Rust. It is built around a VirtualDOM, making it portable for the web, desktop, server, mobile, and more.

Dioxus has the following design goals:

- **Familiar**: offer a React-like mental model and API surface
- **Correct**: Avoid runtime bugs by moving rules and error handling into the type system
- **Performant**: Scale to the largest of apps for the largest teams
- **Productive:** Comprehensive inline documentation, fast recompiles, and deeply integrated tooling
- **Extensible:** Reusable hooks and components that work on every platform

Dioxus is designed to be familiar for developers comfortable with React paradigms. Our goal is to ensure a smooth transition from TypeScript to Rust without having to learn any major new concepts. In practice, Rust-Dioxus code looks and feels very similar to TypeScript-React code.

To give you a taste of what Dioxus is all about, here's a simple counter app:

```rust
use dioxus::prelude::*;

fn main() {
	dioxus::desktop::launch(App)
}

fn App(cx: Scope, props: &()) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx! {
        h1 { "Count: {count}" }
        button { onclick: move |_| count += 1, "+" }
        button { onclick: move |_| count -= 1, "-" }
    })
}
```

This simple counter is a fully-fledged desktop app, running at native speeds on a native thread. Dioxus automatically shuttles all events from the WebView runtime into the application code. In our app, we can interact natively with system APIs, run multi-threaded code, and do anything a regular native Rust application might do. To publish our app, we simply need to run `cargo build` to compile a portable binary that looks and feels the same on Windows, Mac, and Linux. In fact, our `App` component works exactly the same on desktop, mobile, and the web too.

Dioxus supports everything React does, and more, including

- Server-side-rendering, pre-rendering, and hydration
- Mobile, desktop, and web support
- Suspense, fibers, coroutines, and error handling
- Hooks, first-class state management, components
- Fragments, conditional rendering, and custom elements
- and more!

As a demo, here's a Dioxus app running on all our current supported platforms:

![Untitled](static/Untitled.png)

This very site is built with dioxus, and the source code is available [here](github.com/dioxuslabs/docsite).

To get started with Dioxus, check out any of the "Getting Started" guides for your platform of choice, or check out the GitHub Repository for more details.

- Getting Started with Dioxus
- Getting Started with Web
- Getting Started with Desktop
- Getting Started with Mobile
- Getting Started with SSR

## Show me some examples of what can be built!

- File explorer desktop app
- Bluetooth scanner desktop app
- IoT management web app
- Chat mobile app
- Hackernews LiveView app

## What features does Dioxus support?

- Pre-rendering + hydration
- Server-side/static rendering
- Shared state providers/consumers (ie React.Context)
- Function components
- Hooks
- Inline styles
- Custom elements
- Jank-free rendering (with fibers)
- Refs to elements

We do support some differences from React as features:

- Automatic memoization (opt-out rather than opt-in)
- No use_effect hook (yes, this is a feature)
- Suspense is implemented as hooks, *not* deeply ingrained in Dioxus Core
- Async code is *explicit* with a preference to *coroutines* instead

## Why should I use Rust and Dioxus for frontend?

We believe that Rust's ability to write high-level, statically typed, and efficient code should make it easier for frontend teams to take on even the most ambitious of projects. Rust projects can be refactored fearlessly: the powerful type system prevents an entire class of bugs at compile time. No more `cannot read property of undefined` ever again! With Rust, all errors must be accounted for at compile time. You cannot ship an app that does not - in some way - handle its errors.

Dioxus' design encourage patterns that make your apps more robust. Typically, React apps struggle with async code, unnecessary re-renders, and global state management. To solve this, we've made a few opinionated choices:

- Explicit and centralized async code through coroutines.
- Purposeful lack of use_effect.
- Radically simple state management solution.
- Use of placeholder nodes for dynamic content.

### What about TypeScript?

TypeScript is a fantastic addition to JavaScript, but it's still fundamentally JavaScript. TypeScript code runs slightly slower, has tons of configuration options, and not every package is properly typed.

In contrast, Dioxus is written in Rust - which is almost like "TypeScript on steroids".

By using Rust, we gain:

- Static types for every library
- Immutability by default
- A simple and intuitive module system
- Integrated documentation (go to source actually... goes to source instead of the `.d.ts` file)
- Advanced pattern matching
- Clean, efficient, composable iterators
- Inline built-in unit/integration testing
- Best-in-class error handling
- Powerful and sane standard library
- Flexible macro system
- Access to crates.io

Specifically, Dioxus provides us many other assurances:

- Proper use of immutable datastructures
- Guaranteed error handling (so you can sleep easy at night not worrying about `cannot read property of undefined`)
- Native performance on mobile
- Direct access to system IO

And much more. Dioxus makes Rust apps just as fast to write as React apps, but affords more robustness, giving your frontend team greater confidence in making big changes in shorter time.

Semantically, TypeScript-React and Rust-Dioxus are very similar. In Dioxus, we would define a component as:
```rust
#[derive(Props, PartialEq)]
struct CardProps {
	title: String,
	paragraph: String
}

static Card: Component<CardProps> = |cx, props| {
	cx.render(rsx!(
		aside {
			h2 { "{props.title}" }
			p { "{props.paragraph}" }
		}
	))
};
```

In TypeScript, the corresponding component would be:
```jsx
type CardProps = {
  title: string,
  paragraph: string
}

const Card: FunctionComponent<CardProps> = (props) => {
	return (
		<aside>
			<h2>{ props.title }</h2>
			<p> { props.paragraph } </p>
		</aside>
	)
};
```

However, we do recognize that these benefits are not for everyone, nor do they completely fix "everything that is wrong with frontend development." There are always going to be new patterns, frameworks, and languages that solve these problems better than Rust and Dioxus. We hope that Dioxus serves as a competent companion for developers looking to build reliable and efficient software that extends into the world of user interfaces.

## Show me more:

Here, we'll dive into some features of Dioxus and why it's so fun to use. The API reference serves as a deeper and more comprehensive look at what Dioxus can do.


### Building a new Project is simple!

To start a new project, all you need is Cargo (comes with Rust). For a simple desktop app, all we'll need is the `dioxus` create with the appropriate `desktop` feature. In a new crate, we'll just add this to our `Cargo.toml`.

```rust
[dependencies]
dioxus = { version = "0.2", features = ["desktop"] }
```

Because it's so simple to get started, you probably won't need to reach for a prebuilt template, though we have pre-configured a few templates with suggested project layout.

For web development, you'll want to install the Dioxus CLI to run a local development server and run some basic WASM optimization tools. This can be done with a simple `cargo install dioxus-cli`. The `dioxus-cli` tool will handle building, bundling, development, and optimization for the web and mobile.


### Support for JSX-style templating

Dioxus ships with a templating macro called RSX (a spin of JSX). RSX is very similar to regular struct syntax for Rust so it integrates well with your IDE. RSX supports code-folding, block selection, bracket pair colorizing, autocompletion, symbol renaming - pretty much anything you would expect from writing just regular struct-style code.

```rust
rsx! {
	div { "Hello world" }
	button {
		onclick: |_| log::info!("button pressed"),
		"Press me"
	}
}
```

If macros aren't your style, then you can always just use the factory API directly:

```rust
LazyNodes::new(|f| {
	f.fragment([
		f.element(div, [f.text("hello world")], [], None, None)
		f.element(
			button,
			[f.text("Press Me")],
			[on::click(move |_| log::info!("button pressed"))],
			None,
			None
		)
	])
})
```

The `rsx!` macro generates idiomatic Rust code that uses the factory API - no different than what you'd write by hand, yourself. Feel free to try it out with `cargo expand` .

To make it easier to work with RSX, we've built a small VSCode extension with useful utilities. This extension provides a command that converts a selected block of HTML into RSX so you can use any web template instantly.

### Dioxus is perfected for the IDE.

All Dioxus code operates pleasantly with your IDE. If you really need to write HTML templates, you can use the `html!` macro, but if you're willing to depart from traditional syntax, the `rsx!` macro provides everything you need, and more.

For starters, all elements are documented through the Rustdoc system - a quick summary of the MDN docs is always under your finger tips:

![static/Screen_Shot_2021-07-06_at_9.42.08_PM.png](static/Screen_Shot_2021-07-06_at_9.42.08_PM.png)

Dioxus also wraps platform-specific events with a custom synthetic event system. This means events enjoy proper autocomplete and documentation, unlike Yew which currently relies WebSys (which is not IDE supported):

![static/Screen_Shot_2021-07-06_at_10.24.03_PM.png](static/Screen_Shot_2021-07-06_at_10.24.03_PM.png)

Even element attributes and event handlers have top-notch documentation!

![static/Screen_Shot_2021-07-07_at_1.21.31_AM.png](static/Screen_Shot_2021-07-07_at_1.21.31_AM.png)

The `rsx!` macro also enjoys code folding, batch renaming, block selection, making most basic code navigation and completion tasks a breeze.

![static/Screen_Shot_2021-07-06_at_10.16.46_PM.png](static/Screen_Shot_2021-07-06_at_10.16.46_PM.png)

Plus, the `rsx!` macro itself is documented, so if you ever forget how to use a certain feature, the documentation is literally right under your cursor:

![static/Screen_Shot_2021-07-07_at_1.28.24_AM.png](static/Screen_Shot_2021-07-07_at_1.28.24_AM.png)

We spent a ton of time on this - we hope you enjoy it!


## Dioxus is extremely fast


We take the performance of Dioxus seriously. Instead of resolving to "good enough," Dioxus is designed to push the limits of what a declarative React-like framework can achieve. Dioxus is designed with multi-tenancy in mind: a single machine should be able to run thousands of simultaneous low-latency LiveView apps without skipping a beat. To accomplish this goal we've implemented a large number of optimizations:

- Specialized memory allocators
- Compile-time hashing and diffing hints
- Automatic component memoization
- Cooperative fiber-like scheduling
- DOM Patch Batching

Dioxus is built off the work done by Dodrio, a now-defunct research project by fitzgen exploring the 

### Works on mobile and desktop


We’ve mentioned before that Dioxus works practically anywhere that Rust works. When running natively as a desktop or mobile app, your Dioxus code will run on its own thread: not inside of a web runtime. This means you can access hardware, file system, and platform APIs directly without needing to go through a shim layer. In our examples, we feature a file explorer app and Bluetooth scanner app where platform access occurs inside an asynchronous multithreaded coroutine. This solves the problem faced by React Native and other cross-platform toolkits where JavaScript apps occur a massive performance penalty with substantial maintenance overhead associated with platform API shims.

A desktop app:

![](https://github.com/DioxusLabs/file-explorer/raw/master/image.png)

A mobile app:

![](https://github.com/DioxusLabs/ios_demo/raw/master/assets/screenshot.jpeg)

However, be warned that mobile is considered very experimental and there will likely be quirks. Dioxus is leveraging work done by the Tauri team to enable mobile support, and mobile support isn't technically complete in Tauri - yet. 

iOS should be supported out of the box, but Android support will take custom some boilerplate that hasn't been figured out completely. If you're interested in contributing to Dioxus, improving mobile support would be extremely helpful.

## What's on the Roadmap?


The world of Rust on the frontend is barely explored. Given the performance, ergonomics, and portability of Dioxus, we expect there to be a ton of different applications where having a React-like toolkit running natively can enable things previously impossible.

In the coming weeks, the plan is to finish the final outstanding features where Dioxus is lacking in comparison to React:

- Transition Effects for suspense
- Micro-optimizations and better cross-platform/browser bug mitigations
- Techniques to guide the diffing algorithm
- Better support for subtree memoization (signals, etc)
- More thorough documentation, fleshing out sore spots

We also need some help in important crates currently missing:

- 1st class cross-platform router
- An extension to DioxusStudio that enables lazy bundling of static assets
- Animation library (like React Spring)
- A TUI renderer for Dioxus (like Ink.js)

And finally, some bigger, forward-thinking projects that are too big for one person:

- Completely native renderer for the Dioxus VirtualDOM (like Flutter)
- Better support for LiveView
- Code-splitting
- 3D renderer like React-three-fiber


## Community

The future is bright for Rust frontends! If you'd like to get involved, we have a

- Discord
- Subreddit
- Github Discussions page

Check out the original `r/rust` thread here.

Let us know what you build!
