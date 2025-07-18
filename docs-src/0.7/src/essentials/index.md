# Core Topics

Dioxus is a Rust framework for building cross-platform apps with a single codebase. Every app leverages a user interface (UI) to display content and allow the user to take action. UI is built from small units like buttons, text, and images and then organized into components that encapsulate functionality. Dioxus apps are built by combining components into a larger interactive tree. Using conditional rendering, you write busienss logic that simply controls the component tree while Dioxus itself ensures your widget tree renders properly across web, desktop, and mobile.

## A high-level overview

Dioxus builds on entire generation of UI research. Building interactive, beautiful, and efficient user interfaces can actually be quite challenging. We designed Dioxus to address the complexity of frontend development with a simple mental model:

- User Interfaces are constructed *declaratively* using `rsx!` to describe the layout and styling of widgets
- Elements can handle user interaction with event handlers
- The state of application is stored in functions called "hooks"


## Declaring UI


## Thinking Reactively


## Managing State


## Working with Asynchronicity

<!-- This section will guide you through key concepts in Dioxus: -->

<!-- - [Building User Interfaces](ui/index.md) will teach you how to define html inside your Dioxus app with rsx. -->

<!-- - [Event Handlers](ui/e) will teach you how to handle events in Dioxus, including how to prevent default behavior and stop event propagation.

- [Hooks](hooks/index.md) will teach you about the concept of hooks and their limitations.

- [Components](components/index.md) will teach you about the concept of components and their role in building UIs.

- [Component Lifecycle](lifecycle/index.md) teaches you about the lifecycle of components along with the hooks you need to run code when the component is first created, mounted, and removed.

- [Managing State](state/index.md) guides you through how state works in Dioxus. It will teach you how to create state with `use_signal`, derive state with `use_memo`, and integrate state with asynchronous tasks with `use_resource`. Along the way, you will learn about you can use reactivity to declaratively describe your UI.

- [Breaking Out](breaking/index.md) will teach you how to break out of Dioxus' rendering model to run JavaScript or interact with the DOM directly with `web-sys`.

- [Async](async/index.md) will teach you how to integrate async tasks with Dioxus and how to handle loading states while waiting for async tasks to finish.

- [Error Handling](error_handling/index.md) will teach you how to throw and handle errors in Dioxus.


- [`Fullstack`](fullstack/index.md) Overview of Fullstack specific APIS
    - [`Server Functions`](fullstack/server_functions.md) Server functions make it easy to communicate between your server and client
    - [`Extractors`](fullstack/extractors.md) Extractors allow you to get extra information out of the headers of a request
    - [`Middleware`](fullstack/middleware.md) Middleware allows you to wrap a server function request or response
    - [`Authentication`](fullstack/authentication.md) An overview of how to handle authentication with server functions
    - [`Routing`](fullstack/routing.md) An overview of how to work with the router in the fullstack renderer -->
