# Dioxus Reference

This Reference contains more detailed explanations for all concepts covered in the [guide](../guide/index.md) and more.

## Rendering

- [RSX](rsx.md): Rsx is a HTML-like macro that allows you to declare UI
- [Components](components.md): Components are the building blocks of UI in Dioxus
- [Props](component_props.md): Props allow you pass information to Components
- [Event Listeners](event_handlers.md): Event listeners let you respond to user input
- [User Input](user_input.md): How to handle User input in Dioxus
- [Dynamic Rendering](dynamic_rendering.md): How to dynamically render data in Dioxus

## State

- [Hooks](hooks.md): Hooks allow you to create components state
- [Context](context.md): Context allows you to create state in a parent and consume it in children
- [Routing](router.md): The router helps you manage the URL state
- [Resource](use_resource.md): Use future allows you to create an async task and monitor it's state
- [UseCoroutine](use_coroutine.md): Use coroutine helps you manage external state
- [Spawn](spawn.md): Spawn creates an async task

## Platforms

- [Choosing a Web Renderer](choosing_a_web_renderer.md): Overview of the different web renderers
- [Desktop](desktop/index.md): Overview of desktop specific APIS
- [Web](web/index.md): Overview of web specific APIS
- [Fullstack](fullstack/index.md): Overview of Fullstack specific APIS
    - [Server Functions](fullstack/server_functions.md): Server functions make it easy to communicate between your server and client
    - [Extractors](fullstack/extractors.md): Extractors allow you to get extra information out of the headers of a request
    - [Middleware](fullstack/middleware.md): Middleware allows you to wrap a server function request or response
    - [Authentication](fullstack/authentication.md): An overview of how to handle authentication with server functions
    - [Routing](fullstack/routing.md): An overview of how to work with the router in the fullstack renderer
- [SSR](ssr.md): Overview of the SSR renderer
- [Liveview](liveview.md): Overview of liveview specific APIS
