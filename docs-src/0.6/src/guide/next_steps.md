# Wrapping Up

Congrats on making it through our *HotDog* tutorial!

Hopefully this isn't the end of our journey, but rather, a bold new beginning.

## Up Next

We strongly recommend moving on to the [Essential Topics](../essentials/index.md) to become familiar with the important details of Dioxus.

The essentials section will guide you through key concepts in Dioxus:

- [Building UIs with RSX](../essentials/rsx/index.md) will teach you how to define html inside your Dioxus app with rsx.

- [Component Lifecycle](../essentials/lifecycle/index.md) teaches you about the lifecycle of components along with the hooks you need to run code when the component is first created, mounted, and removed.

- [Managing State](../essentials/state/index.md) guides you through how state works in Dioxus. It will teach you how to create state with `use_signal`, derive state with `use_memo`, and integrate state with asynchronous tasks with `use_resource`. Along the way, you will learn about you can use reactivity to declaratively describe your UI.

- [Breaking Out](../essentials/breaking/index.md) will teach you how to break out of Dioxus' rendering model to run JavaScript or interact with the DOM directly with `web-sys`.


## Ideas for New Features

Challenge yourself by adding new features to *HotDop*.

- Add animations to your app with CSS or [dioxus-motion](https://github.com/wheregmis/dioxus-motion).
- Style your app with a library like [TailwindCSS](http://tailwindcss.com).
- Add inputs to allow users to upload their own dog photos.
- Make it social! Add login, user accounts, and a feed.
- Remix HotDog for something entirely new.


## FAQ

You might be curious how particular features work in Dioxus. Let's try to address a few popular question:

### Is Dioxus Fast?

Dioxus is really fast. Dioxus is built around an *extremely* performant VirtualDom. While a VirtualDom might sound like ["pure overhead"](https://svelte.dev/blog/virtual-dom-is-pure-overhead), Dioxus leverages compile-time optimizations that make it faster than nearly all UI frameworks.

On the web, Dioxus [is on par with frameworks like Solid and Svelte](https://krausest.github.io/js-framework-benchmark/2023/table_chrome_120.0.6099.62.html) which leveragea new reactivity models or custom compilers.

### Is Rust too hard?

Rust is a notoriously difficult language to learn, but it's extremely powerful. Dioxus has been designed to use the "easy" parts of Rust. For example, Dioxus primarily uses single-threaded code and avoids complicated generic interfaces.

All-in-all, the productivity you gain by building once and deploying everywhere usually makes up for the steeper learning curve. Plus, as Dioxus matures, interfaces become easier to use and the ecosystem improves.

### Does Dioxus support "xyz"?

- TailwindCSS: Yes, but it requires an additional step, see the [docs for more info](../cookbook/tailwind.md).
- Native Widgets: Yes, but you might need to write this code yourself. The Rust ecosystem is young.
- Shadcn-ui: Not yet, a radix-ui port to Dioxus is underway but we're not done yet.
- Actix/Rocket: Yes, kind-of. In Dioxus, Server-Functions are built on axum, but you can use server-side-rendering with any framework.
- Static-site-generation: Yes! The docs are currently under construction, so stay tuned. 🏗️
- Animations: Yes, you can use CSS animations, [dioxus-motion](https://github.com/wheregmis/dioxus-motion), or integrate with the system animation APIs.
- Accessibility: Yes. Use semantic HTML and aria-tags for good screen-reader support.
- WGPU: Yes, kinda. You can overlay Dioxus on a WGPU scene or experiment with Dioxus-Native which renders with WGPU.
- AI: Yes, you can interface with providers like [OpenAI](https://crates.io/crates/openai-api-rs) or even [build your own provider](https://github.com/DioxusLabs/dioxus-ai).

If you have an FAQ not covered here, suggest an edit!

### Why RSX instead of HTML?

If you've seen React's JSX or the `html!{}` Rust macro, you might be curious as to why Dioxus chose to use its own syntax instead of a syntax that looks more similar to HTML.

A few reasons:

- RSX gets token coloring and code-folding without additional tooling
- RSX is faster to type since curly braces are auto-closed
- Not all RSX is HTML - Dioxus can be used in non-HTML contexts
- HTML is not valid Rust - not all HTML can be used in html!{}

We want the out-of-the-box experience for Dioxus to be great without setting up any custom tooling.

The Dioxus [VSCode Extension](http://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus) can convert HTML to RSX as well as the CLI with `dx translate`.

### Should I use Dioxus or Tauri or Leptos or Iced or...?

There's a budding ecosystem of Rust projects for building apps and websites! We have a good comparison in our [Readme.md](https://github.com/DioxusLabs/dioxus#dioxus-vs-other-frameworks)

Dioxus is an "all-in-one" solution for building apps with Rust. We try to be helpful every step of the way: getting started, iterating, testing, bundling, and deploying.

- **Tauri**: You can use Dioxus with Tauri, but we focus primarily on the standalone Dioxus experience. Tauri is a good choice if you want to use a JavaScript frontend or need full-featured access to the Webview DOM.
- **Leptos**: Dioxus and Leptos are both good choices for fullstack web development with differences in syntax and reactivity models. Dioxus is architected to support desktop and mobile "1st-class" whereas Leptos is primarily a web framework.
- **Iced:** Iced is GUI framework that renders using its own render engine, making it highly customizable. Iced is a good choice for apps that need access to lower-level rendering through WGPU. Dioxus will be releasing its own WGPU renderer in early 2025, so stay tuned!

Dioxus is quite a large project and has been a bit slower to mature than other projects with larger development teams or smaller scopes. With 0.6, Dioxus is more mature but still has some ground to cover.
