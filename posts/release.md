# Introducing Dioxus

After many months of work, we're very excited to release the first version of Dioxus!

Dioxus is a new library for building interactive user interfaces with Rust. It is built around a Virtual DOM, making it portable for the web, desktop, server, mobile, and more.

Dioxus has the following design goals:

- **Familiar**: Offer a React-like mental model and API surface
- **Correct**: Avoid runtime bugs by moving rules and error handling into the type system
- **Performant**: Scale to the largest apps and the largest teams
- **Productive**: Comprehensive inline documentation, fast recompiles, and deeply integrated tooling
- **Extensible**: Reusable hooks and components that work on every platform

Dioxus is designed to be familiar for developers already comfortable with React paradigms. Our goal is to ensure a smooth transition from TypeScript to Rust without having to learn any major new concepts. In practice, Rust-Dioxus code looks and feels very similar to TypeScript-React code.

To give you a taste of what Dioxus is all about, here's a simple counter app:

```rust
use dioxus::prelude::*;

fn main() {
	dioxus::desktop::launch(CounterApp)
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

This simple counter is a fully-fledged desktop application, running at native speeds on a native thread. Dioxus automatically shuttles all events from the WebView runtime into the application code. In our app, we can interact natively with system APIs, run multi-threaded code, and do anything a regular native Rust application might do. Running `cargo build --release` will compile a portable binary that looks and feels the same on Windows, macOS, and Linux. In fact, our `CounterApp` component works exactly the same on desktop, mobile, and the web!

Dioxus supports everything React does and more, including but not limited to: <<<_almost_ everything? the roadmap explicitly calls out missing feature>>>

- Server-side-rendering, pre-rendering, and hydration
- Mobile, desktop, and web support
- Suspense, fibers, coroutines, and error handling
- Hooks, first-class state management, components
- Fragments, conditional rendering, and custom elements

As a demo, here's a Dioxus app running on all our current supported platforms:

<<<Would be good for this to link to the actual app running on the website>>>
![Untitled](static/Untitled.png)

This very site is built with Dioxus, and the source code is available [here](github.com/dioxuslabs/docsite).

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
- Shared state providers/consumers (i.e. React.Context)
- Function components
- Hooks
- Inline styles
- Custom elements
- Jank-free rendering (with fibers)
- Refs to elements

Some features also consciously diverge from React idioms:

- Automatic memoization (opt-out rather than opt-in)
- No use_effect hook (yes, this is a feature)
- Suspense is implemented as hooks, _not_ deeply ingrained within Dioxus Core
- Async code is _explicit_ with a preference for _coroutines_ instead

## Why should I use Rust and Dioxus for frontend?

We believe that Rust's ability to write high-level, statically typed, and efficient code should make it easier for frontend teams to take on even the most ambitious of projects. Rust projects can be refactored fearlessly: the powerful type system prevents an entire class of bugs at compile-time. No more `cannot read property of undefined` ever again! With Rust, all errors must be accounted for at compile time. You cannot ship an app that does not — in some way — handle its errors.

Dioxus' design encourage patterns that make your apps more robust. Typically, React apps struggle with async code, unnecessary re-renders, and global state management. To solve this, we've made a few opinionated choices:

- Explicit and centralized async code through coroutines
- Purposeful lack of use_effect <<<These first two are repetitive with the previous section>>>
- Radically simple state management solution
- Use of placeholder nodes for dynamic content

### What about TypeScript?

TypeScript is a fantastic addition to JavaScript, but it's still fundamentally JavaScript. TypeScript code runs slightly slower, has tons of configuration options, and not every package is properly typed — or typed at all!

In contrast, Dioxus is written in Rust, which is almost like "TypeScript on steroids".

By using Rust, we gain:

- Static types for every library
- Immutability by default
- A simple and intuitive module system
- Integrated documentation (go to source actually goes to source instead of the `.d.ts` file)
- Advanced pattern matching
- Clean, efficient, composable iterators
- Inline built-in unit/integration testing
- Best-in-class error handling
- Powerful and sane standard library
- Flexible macro system
- Access to the crates.io ecosystem

Dioxus itself leverages this platform to provide the following guarantees:

- Correct use of immutable data structures
- Guaranteed handling of errors and null-values in components
- Native performance on mobile
- Direct access to system IO

And much more. Dioxus makes Rust apps just as fast to write as React apps, but affords more robustness, giving your frontend team greater confidence in making big changes in shorter timespans.

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
  paragraph: string,
};

const Card: FunctionComponent<CardProps> = (props) => {
  return (
    <aside>
      <h2>{props.title}</h2>
      <p> {props.paragraph} </p>
    </aside>
  );
};
```

However, we recognize that these benefits are not for everyone, nor do they completely fix "everything that is wrong with frontend development." There are always going to be new patterns, frameworks, and languages that solve these problems better than Rust and Dioxus. We hope that Dioxus serves as a competent companion for developers looking to build reliable and efficient software that extends into the world of user interfaces.

## Show me more

Here, we'll dive into some features of Dioxus and why it's so fun to use. The API reference serves as a deeper and more comprehensive look at what Dioxus can do.

### Building a new project is simple!

To start a new project, all you need is Cargo, which comes with Rust. For a simple desktop app, all we'll need is the `dioxus` crate with the appropriate `desktop` feature. We start by initializing a new binary crate:

```shell
cargo init dioxus_example
cd dioxus_example
```

We then add a dependency on Dioxus to the `Cargo.toml` file, with the "desktop" feature enabled:

```rust
[dependencies]
dioxus = { version = "0.2", features = ["desktop"] }
```

Because it's so simple to get started, you probably won't need to reach for a prebuilt template. Nevertheless, we have pre-configured a few templates with a suggested project layout. <<<Would be good to have a link here, or remove this section>>>

For web development, you'll want to install the Dioxus CLI to run a local development server and run some basic Wasm optimization tools. This can be done with `cargo install dioxus-cli`. The `dioxus-cli` tool will handle building, bundling, development, and optimization for the web and mobile.

### Support for JSX-style templating

Dioxus ships with a templating macro called RSX, a spin on React's JSX. RSX is very similar to regular struct syntax for Rust so it integrates well with your IDE. RSX supports code-folding, block selection, bracket pair colorizing, autocompletion, symbol renaming — pretty much anything you would expect from writing regular struct-style code.

```rust
rsx! {
	div { "Hello world" }
	button {
		onclick: |_| log::info!("button pressed"),
		"Press me"
	}
}
```

If macros aren't your style, you can always drop down to the factory API:

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

The `rsx!` macro generates idiomatic Rust code that uses the factory API — no different than what you'd write by hand yourself. Feel free to try it out with [`cargo expand`](https://github.com/dtolnay/cargo-expand).

To make it easier to work with RSX, we've built a small VSCode extension with useful utilities. This extension provides a command that converts a selected block of HTML into RSX so you can easily reuse existing web templates.

### Dioxus is perfected for the IDE

<<<Would be good to have a small note on the IDE options, e.g. rust-analyzer or JetBrains Rust. There's still the confusion of the default Rust LSP for VS Code for newcomers>>>

All Dioxus code operates pleasantly with your IDE. If you really need to write HTML templates, you can use the `html!` macro, but if you're willing to depart from traditional syntax, the `rsx!` macro provides everything you need, and more.

For starters, all elements are documented through the Rustdoc system. A quick summary of the MDN docs is always under your finger tips:

<<<Would be good to have proper naming and alt text for all images in this doc>>>
![static/Screen_Shot_2021-07-06_at_9.42.08_PM.png](static/Screen_Shot_2021-07-06_at_9.42.08_PM.png)

Dioxus also wraps platform-specific events with a custom synthetic event system. This means events enjoy proper autocomplete and documentation, unlike [Yew](https://yew.rs/) which currently relies on [web-sys](https://crates.io/crates/web-sys) with incomplete IDE support:

![static/Screen_Shot_2021-07-06_at_10.24.03_PM.png](static/Screen_Shot_2021-07-06_at_10.24.03_PM.png)

Even element attributes and event handlers have top-notch documentation!

![static/Screen_Shot_2021-07-07_at_1.21.31_AM.png](static/Screen_Shot_2021-07-07_at_1.21.31_AM.png)

The `rsx!` macro also benefits from code folding, batch renaming, and block selection, making most basic code navigation and completion tasks a breeze.

![static/Screen_Shot_2021-07-06_at_10.16.46_PM.png](static/Screen_Shot_2021-07-06_at_10.16.46_PM.png)

Furthermore, the `rsx!` macro itself is documented, so if you ever forget how to use a certain feature, the documentation remains close at hand:

![static/Screen_Shot_2021-07-07_at_1.28.24_AM.png](static/Screen_Shot_2021-07-07_at_1.28.24_AM.png)

We spent a ton of time on this and we hope you enjoy it!

## Dioxus is extremely fast

We take the performance of Dioxus seriously. Instead of resolving to "good enough," Dioxus is designed to push the limits of what a declarative React-like framework can achieve. Dioxus is designed with multi-tenancy in mind: a single machine should be able to run thousands of simultaneous low-latency LiveView apps without skipping a beat. To accomplish this goal we've implemented a large number of optimizations:

- Specialized memory allocators
- Compile-time hashing and diffing hints
- Automatic component memoization
- Cooperative fiber-like scheduling
- DOM Patch Batching

Dioxus is built off the work done by [Dodrio](https://github.com/fitzgen/dodrio), a now-defunct research project by fitzgen exploring the <<<Missing something here>>>

### Works on mobile and desktop

We’ve mentioned before that Dioxus works practically anywhere that Rust does. When running natively as a desktop or mobile app, your Dioxus code will run on its own thread, not inside of a web runtime. This means you can access hardware, file system, and platform APIs directly without needing to go through a shim layer. In our examples, we feature a file explorer app and Bluetooth scanner app where platform access occurs inside an asynchronous multithreaded coroutine. This solves the problem faced by React Native and other cross-platform toolkits where JavaScript apps incur a massive performance penalty with substantial maintenance overhead associated with platform API shims.

A desktop app:

![](https://github.com/DioxusLabs/file-explorer/raw/master/image.png)

A mobile app:

![](https://github.com/DioxusLabs/ios_demo/raw/master/assets/screenshot.jpeg)

However, be warned that mobile is currently considered very experimental and there will likely be quirks. Dioxus is leveraging the work done by the [Tauri](https://github.com/tauri-apps/tauri) team to enable mobile support, and mobile support isn't technically complete in Tauri just yet.

iOS should be supported out of the box, but Android support will take custom some boilerplate that hasn't been completely figured out. If you're interested in contributing to Dioxus, improving mobile support would be extremely helpful.

## What's on the roadmap?

The world of Rust on the frontend is barely explored. Given the performance, ergonomics, and portability of Dioxus, we expect there to be a ton of different applications where having a React-like toolkit running natively can enable things previously considered impossible.

In the coming weeks, our plan is to finish the remaining outstanding features where Dioxus is lacking in comparison to React:

- Transition effects for Suspense
- Micro-optimizations and better cross-platform/browser bug mitigations
- Heuristics to guide the diffing algorithm
- Better support for subtree memoization (signals, etc.)
- More thorough documentation, fleshing out sore spots

We also need some help in important crates currently missing:

- First class cross-platform router <<<I believe this is in the works?>>>
- An extension to DioxusStudio that enables lazy bundling of static assets
- Animation library (see [React Spring](https://react-spring.io/), [Framer Motion](https://www.framer.com/motion/))
- A TUI renderer for Dioxus (see [Ink](https://github.com/vadimdemedes/ink))

And finally, some bigger, forward-thinking projects that are too big for a one-person team:

- Completely native renderer for the Dioxus Virtual DOM (see [Flutter](https://flutter.dev/))
- Better support for LiveView
- Code-splitting
- 3D renderer (see [react-three-fiber](https://github.com/pmndrs/react-three-fiber))

Stay tuned for our next article, which will go over some of the optimization techniques that went into making Dioxus blazing fast.

## Community

The future is bright for Rust frontends! If you'd like to get involved, we have a Discord server, a subreddit, and GitHub discussion pages. <<<Need links>>>

Check out the original `/r/rust` thread here.

Let us know what you build!
