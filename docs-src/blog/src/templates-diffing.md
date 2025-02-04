# Making Dioxus (almost) as fast as SolidJS

[Dioxus](https://github.com/dioxuslabs/dioxus) is a UI library for Rust that makes it easy to target almost any platform with the same React-like codebase. You can build apps for WASM, desktop, mobile, TUI, static-sites, SSR, LiveView, and more.

---

In preparation for the next big release of Dioxus, one of the lead contributors, ealmloff, added a long-awaited feature: **subtree memoization**.

Subtree memoization reduces the overall work that Dioxus needs to do to get your desired UI state to the screen by several orders of magnitude. In fact, it’s so fast, that it pushes Dioxus to the leading edge of performance for web frameworks, on par with the likes of SolidJS, even beating out signal-based Rust libraries like Sycamore 0.8 and Leptos 0.0.3.

![Untitled](/assets/blog/diffing/jsframework-diffing.png)

There’s obviously still some room to grow as WASM-based UI libraries face unique challenges compared to their JavaScript counterparts. Ultimately, we hope that this update demonstrates what’s really possible with the current React paradigm.

And, of course, we need to mention that benchmarks never give a truly clear insight into how performant a library is, especially as an app scales. It’s definitely reasonable to believe that as an app grows in size, any other library might come out ahead. You shouldn’t make a decision on the framework for your next project just because it’s slightly more or less performant than any other library based on entirely arbitrary benchmarks.

![Untitled](https://imgs.xkcd.com/comics/optimization.png)

Anyways…

## Dioxus shares React’s DNA

As eloquently put by the creator of Svelte, the [“Virtual DOM is pure overhead”](https://svelte.dev/blog/virtual-dom-is-pure-overhead). So, why does Dioxus continue to share the React DNA if it’s ultimately just frivolous work?

Well, we still love React, despite its warts, footguns, and idiosyncrasies.

- React is just JavaScript, no magic compilation needed.
- Components are just tiny event loops with mostly predictable re-renders.
- React’s paradigm maps extremely well into Rust.

The final point is arguably the most important: React’s functional model maps well into Rust’s lifetime system. Any value provided to the component through `use_hook` is bounded by the lifetime of the `Scope` itself. `Scope` can be shared into any handler - like `onclick` in the following example. Since `value` shares a lifetime with `Scope`, it can be modified directly within element callbacks.

```rust
fn app(cx: Scope) -> Element {
		let value: &mut u32 = cx.use_hook(|| 0);

    cx.render(rsx!(
        button { onclick: move |_| value += 1, "Increment" }
    ))
}
```

This clean mapping of React’s paradigms into Rust makes it possible for Dioxus to achieve excellent developer experience.

- Components are just regular functions.
- The foundational `use_hook` provides a direct mutable reference to a value.
- Values created with the Scope’s lifetime can be passed directly into children, unlike nearly all non-signal-based libraries.

```rust
let doc = use_document_builder(cx);

rsx! {
	Doc { document: doc }
}

#[component]
fn Doc<'a>(cx: Scope<'a>, document: &'a SomeBigDocument) -> Element {
	// document is passed from a parent by reference!
	// no smart pointers required!
}
```

All in all, we’ve learned to love lifetimes rather than fear them. But for all the good of React, we’re still stuck with the bad.

## Overcoming the warts of React

One of the biggest issues React has is the need to recreate entire chunks of the virtual DOM between renders. If you’re not aware, in React, your JSX markup is converted directly to `React.createElement` calls.

```jsx
// This markup
<div class="abc"> Hello world </div>

// becomes these calls
React.createElement(div, { class: "abc" }, [React.createText("hello world")]);
```

This means for every new element in your tree, the transpiled JS is allocating several objects, arrays, and complex structures between *every* render. There’s no wonder why React isn’t on the top of the performance charts! In Rust, it’s generally not best practice to generate so many heap-allocated objects.

Heck, there was even a very popular reddit post talking about this problem.

[“Worried about “modern” Rust GUI libraries”](https://www.reddit.com/r/rust/comments/yd9ngs/worried_about_modern_rust_gui_libraries/)

In Dioxus, we noticed this early on and decided to see if we could reuse all the heap allocations instead of just tossing them out. Inspired by the work on Dodrio, Dioxus is implemented using a bump allocator and double-buffering, just like many high-performance GPU APIs.

When you create a div, or a piece of text, or anything in Dioxus, it simply gets allocated inside a bump arena that gets reset when diffed. No cleanup, no extra heap allocations, just steady-state reuse of pre-allocated memory.

![Making%20Dioxus%20(almost)%20as%20fast%20as%20SolidJS%20baea0d5b4e614351ac8e3d4fc4240d04/Screen_Shot_2021-08-17_at_2.24.39_AM.png](/assets/blog/diffing/bump-alloc.png)

This is fast. Really fast. And when coupled with automatic component memoization, it’s really easy to write Dioxus apps that are memory efficient and faster than React.

Great, case-closed, that’s it, right?

Well, no. Dioxus still wasn’t nearly as fast as Svelte, Sycamore, SolidJS, or even InfernoJS. We’ve optimized a bunch of tiny things, like string caching, batched DOM manipulations, faster PartialEq, diffing, and pretty much everything you could think of.

Except, we’re creating new objects, still in the heap, and doing a lot of diffing work. In the words of the creator of Svelte,

> But you know what would be even faster? *Not doing that.*


## Making Dioxus faster by doing less work

To really make Dioxus faster, we need to make it do less work - or, at the very least, less work at runtime. SolidJS does this by thrusting you into this world of signals. We love signals! They might even come to Dioxus at some point (aka Preact signals). But, in the world where we still care about providing `&mut T` from `use_hook` , we need to find a new solution that doesn’t require rewriting your React apps to use signals.

Well, we’re working in Rust, we’ve got const, macros, custom PartialEq… let’s see if we can move some of this work to compile time.

To build a Dioxus app, you pretty much have to use the `rsx!` proc macro. We unfortunately don’t support a builder API or alternatives. There’s a lot of good reasons to do this: performance, forward compatibility, tooling, ergonomics, etc.

A block of `rsx!` might look like this:

```rust
rsx! {
	div {
		h1 {"Glorious Counter"}
		p { "Count: {val}" }
		button { onclick: move |_| val += 1, "Increment" }
		button { onclick: move |_| val -= 1, "Decrement" }
	}
}
```

If you look closely, you’ll notice that the entire tree is declared within the macro. There aren’t  elements being created at compile time, except for the dynamic text within the paragraph element. In React, you’d have to create every element from scratch, one-by-one, every time. But in Dioxus, we can do better.

The new technique Dioxus uses is to split each `rsx!` call into a static `Template` and a list of dynamic nodes. For the above `rsx!` call, this might look like

```rust
static THIS_TEMPLATE: Template = Template { /* */ };

VNode {
	template: THIS_TEMPLATE,
	dynamic_nodes: [
		Text(format_args!("Count: {val}")
	]
}
```

Now, on every render, we only create the single dynamic node. When we go to diff the VNode, we only need to diff that one text node too. So now, instead of 11 comparisons (9 elements and 2 attributes) we have one comparison. Diffing this template takes 90% less time than before! This is a huge win! Our app can be 10x bigger for the same diffing cost. And the results speak for themselves. Combined with the integration of [Sledgehammer](https://crates.io/crates/sledgehammer), Dioxus is pushing the limits of what the React model can reasonably achieve.

![Untitled](/assets/blog/diffing/jsframework-diffing.png)

The React team also agrees that React can be better. That’s why they’ve started working on an experimental compiler for React.

[https://reactjs.org/blog/2022/06/15/react-labs-what-we-have-been-working-on-june-2022.html](https://reactjs.org/blog/2022/06/15/react-labs-what-we-have-been-working-on-june-2022.html)

The plan here is to cache these elements and only update them when variables inside the *component* change. However, React-Forget still doesn’t fix the underlying issue of node creation, memory usage, or anything of the other things compile-time memoization achieves.

## Taking it a step further

Templates make diffing the tree faster, but they can also make building the UI faster too. Both SolidJS and LitHTML take advantage of this hack to achieve fantastic performance.

With support from the renderer, Dioxus can actually serialize these parsed RSX templates and let the renderer do all the caching.

Before, if we wanted to assemble a tree of nodes from an iterator, we’d have to do a ton of tedious work, creating each list item part by part.

```rust
// This tree
ul {
	(0..len).map(|id| rsx!{
		li {
			h3 { "user" }
			div { "hello {id}" }
		}
	})
}

// item one...
Edit::CreateElement("li")
Edit::CreateElement("h3")
Edit::CreateText("user")
Edit::AppendChildren(1)
Edit::CreateElement("div")
Edit::CreateText("hello 0", 2)
Edit::AppendChildren(1)
Edit::AppendChildren(2)

// item two...
Edit::CreateElement("li")
Edit::CreateElement("h3")
Edit::CreateText("user")
Edit::AppendChildren(1)
Edit::CreateElement("div")
Edit::CreateText("hello 0", 2)
Edit::AppendChildren(1)
Edit::AppendChildren(2)

// and so on until we attach all the li to the ul
Edit::AppendChildren(len)
```

With templates, we can serialize the tree and pass it to the renderer:

```rust
static TEMPLATE_HTML = "<li><h3>user</h3><div>hello _id_</div></li>";

Edit::SaveTemplate("demo.rs:5:1", TEMPLATE_HTML);
```

Now, whenever we create the list elements, it’s as simple as cloning some nodes that already exist and precisely modifying just a small part

```rust
Edit::LoadTemplate("demo.rs:5:1");
Edit::HydateText(0, "hello 0");
```

For the tiny case we’re showing here, the benefit might seem limited. However, for real-world apps with lots of elements, custom styles, and all sorts of extra metadata, this caching system is immensely powerful and extremely performant.

## What does this enable?

Now that we’re working with the mindset of templates, we can start to build new functionality previously locked behind the old model.

### Hot Reloading

One amazing feature added to Dioxus using the new template model is hot reloading. You can now modify your running Dioxus app without recompiling, provided you add, remove, or modify elements inside of `rsx!` . This mechanism works for *any* renderer too, since each renderer has to implement the same protocol to manage edits.

![174206798-1b73e42a-0b36-4bce-83c4-aa7d875ec800.mp4](https://i.imgur.com/PSJdqKO.mp4)

Not only can templates be cached inside of a renderer, they can be modified after-the-fact. The renderer is smart enough to track down the instance of every template node on the page and apply the same patches.

### Performant LiveView

Another addition to Dioxus 0.3 is the new LiveView renderer. Much like its counterpart Phoenix LiveView, Dioxus LiveView enables entirely server-rendered apps and components while shipping minimal JS to the client. In the Liveview model, minimizing latency and bandwidth is crucial to keeping apps snappy, especially for lower-end clients.

![ElixirLivewview.jpg](/assets/blog/diffing/elixir.jpeg)

Now, instead of sending hundreds or thousands of edits to the client to render things like lists and complex markup, Dioxus packages all the templates the client will use inside of the HTML shipped to the client. A sample HTML document that might be sent from the server to the client may look like this:

```html
<head>
	<template id="demo.rs:123:456">
		<li>
			<h3>user</h3>
			<div>hello _id_</div>
		</li>
	</template>
</head>
<body>
	<div id="main">
		<!-- pre-rendered page -->
	</div>
</body>
```

Notice how the templates are collected during SSR and inserted into the header. The only edits sent over the network from the server to the client are commands to create/remove template nodes and to precisely modify just the nodes that changed. Fast, simple, and scalable!

## Faster Server-Side-Rendering (SSR)

The other technique that SolidJS uses to achieve faster SSR performance is combining pre-rendered portions of templates together through string concatenation. Since the template is known at compile time, we can break it up into smaller chunks and just stitch them together during rendering. No need to build and traverse huge element trees!

```rust
// Cached template segments:

PreRendered("<div class=\"asdasdasd\" class=\"asdasdasd\"".into(),),
Attr(0,),
PreRendered(">Hello world 1 -->".into(),),
Node(0,),
PreRendered(
    "<-- Hello world 2<div>nest 1</div><div></div><div>nest 2</div>".into(),
),
Node(1,),
Node(2,),
PreRendered("</div>".into(),)
```

## Disclaimer

Even with all the innovations here, it’s still very important to remember that Dioxus still takes after React. No matter how many tweaks, optimizations, and improvements we make to Dioxus, you can still shoot yourself in the foot with the classic React footguns.

These include

- Unkeyed lists
- Poor use of memoization and comparisons
- Misuse of use_effect
- “God components” that do everything

and a whole host of other issues that you might not find in frameworks like Solid and Sycamore.

That being said, since Dioxus relies on a VirtualDom, it can be used as the primary state system for any renderer. And we have a ton of options for renderers:

- Desktop (webview)
- Mobile (webview)
- Web
- TUI
- Skia
- LiveView
- Blitz (WGPU)
- SSR + Hydration
- Static site generation
- VR/AR (coming soon!)

Note that all this work is being done for Dioxus 0.3 and hasn’t yet been released as a major version. We’re still dogfooding these changes to make sure no new bugs have slipped through. If you want these changes released sooner rather than later, feel free to build something with `main` and let us know!

- Discord
- Github
- Reddit Post
