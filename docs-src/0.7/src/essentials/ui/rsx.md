# Introducing RSX

Dioxus apps are comprised of small elements composed into components and then assembled into a tree. To construct our components, we use a combination of Rust code and a simplified dialect of Rust that we call "RSX".

RSX is designed to look and feel like a blend between HTML and SwiftUI:

```rust
rsx! {
    h1 { "Welcome to Dioxus!" }
    h3 { "Brought to you by {author}" }
    p { class: "main-content", {article.content} }
}
```

Currently, RSX is the primary syntax that developers use to build Dioxus apps. There are other options, but RSX has the best developer tooling like instant hot-reloading, code-folding, and syntax highlighting.

## The `rsx!` macro

If you're familiar with libraries like React or Vue, you will likely be familiar with the JSX markup language. JSX is an alternative form of JavaScript that lets developers blend JavaScript code and XML in one file. These libraries rely on compiler plugins to transform the syntax into pure JavaScript code which then renders HTML.

With Rust, we achieve a similar experience through the use of [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) (proc macros). Proc macros are tiny compiler plugins that transform Rust tokens into Rust code. You can recognize that a function is a proc macro thanks to the `!` modifier. For example, the `rsx!` macro transforms RSX syntax into Rust code:

```rust
// This macro...
rsx! {
    div { "hello {world}!" }
}

// Expands to this template:
static TEMPLATE: Template = Template {
    nodes: [
        ElementNode {
            tag: div,
            children: [
                TextNode {
                    contents: DynamicText(0)
                }
            ]
        }
    ]
}
TEMPLATE.render([
    format!("hello {world}")
])
```

RSX does a *lot* of heavy lifting for us, significantly cutting down on the verbosity of declaring UI. It also constructs our UIs in the *most* efficient representation, making rendering extremely fast.

## Dioxus renders HTML and CSS

We want to ensure Dioxus is easy to learn and extremely portable. Instead of inventing a new styling, layout, and markup system, we instead opted to simply rely on HTML and CSS everywhere. For the web, this is handy - websites are already built with HTML and CSS. On desktop and mobile, we ship a renderer that converts your HTML and CSS to native widgets automatically.

Our hybrid HTML approach combines the best parts of frameworks like Flutter and React Native. Teams get maximum code-reuse, get to work with a familiar markup language, and AI tools are immediately helpful. Instead of building bespoke native apps for each platform, teams can simply write their components once and render them everywhere.

Our rendering engine [Blitz](https://github.com/dioxuslabs/blitz) is open source and is often indistinguishable from browser-grade engines:

![Blitz vs Safari](/assets/07/blitz-vs-safari.png)

If you are not familiar with HTML, this guide will help you get started with the basics. For more detail, the [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML) are a great resource. Many tools provide visual designers or AI assistance for constructing UI from HTML elements.

## A Variety of Renderers

Dioxus is extremely modular. Because the RSX representation is generic, you can even swap out the elements, choosing to render UI that is *not* made of HTML and CSS. While the Dioxus team intends to only maintain HTML/CSS-based renderers, third-party renderers exist that unlock extended functionality.

For example, the [Freya](https://freyaui.dev) project renders the Dioxus tree using Google's Skia renderer. Skia has a CPU-only architecture, works across a wide range of devices, and enables deeper control over UI effects:

![Example Freya App](/assets/07/freya-todo.webm)

The Dioxus team maintains three first-party renderers:

- Dioxus-Web: A web-compatible engine that renders directly to HTML DOM Nodes
- Dioxus-Webview: A desktop and mobile engine that renders to the system webview
- Dioxus-Native: A desktop and mobile engine that renders to native elements

The Web and Webview renderers are the most mature engines while Dioxus-Native is still undergoing substantial improvements.


