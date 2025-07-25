# Core Concepts

Dioxus is a Rust framework for building cross-platform apps with a single codebase. Every app leverages a user interface (UI) to display content and allow the user to take action. UI is built from small units like buttons, text, and images and then organized into components that encapsulate functionality. Dioxus apps are built by combining components into a larger interactive tree.

![Widget Tree](/assets/07/widget-tree.png)

With Dioxus, you simply declare what you want your app to look like, and the lightweight Dioxus runtime ensures your widgets render properly across web, desktop, and mobile.

## A high-level overview

Dioxus builds upon decades of UI research. Constructing interactive, beautiful, and efficient user interfaces can be challenging. We designed Dioxus to address the complexity of frontend development with a simple mental model:

1. User Interfaces are constructed *declaratively* to describe the layout and style of widgets
2. The state of the application is stored in small, composable, functions called "hooks"
3. Modifications to state are performed as a response to user interaction which result in a "re-render"

Manually creating and modifying UI elements can be tedious and error prone. The declarative UI approach Dioxus uses requires some training to master, but ultimately it results in efficient and easy-to-maintain frontend code. We recommend thoroughly walking through the Core Concepts to understand how the Dioxus UI and reactivity model works.

## Table of Contents

In the Core Concepts, we cover the four major pillars of UI development:

- [Building User Interfaces](./ui/index.md): Constructing UI by assembling text, elements, and components.
- [Adding Reactivity](./reactivity/index.md): Making our UI interactive, responsive, and efficient.
- [Managing State](./state/index.md): Structuring our apps to be modular, maintainable, and performant.
- [Working with Asynchronicity](./async/index.md): Handling long-running tasks, concurrency, and parallelism.

We also cover three major topics which aren't necessarily essential to UI development, but are still essential to building great apps:

- [App Routing](./router/index.md): Adding multiple screens and routes for the users to visit.
- [Fullstack](./fullstack/index.md): Integrating backend services, server-side-rendering, remote-procedure-calls, and more.
- [Breaking Out](./breaking/index.md): Tips on how to "escape" the Dioxus framework for advanced functionality.
