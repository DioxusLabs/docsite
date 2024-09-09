# Getting Started

This section will help you set up your Dioxus project!

## Prerequisites

### An Editor

Dioxus integrates very well with the [Rust-Analyzer LSP plugin](https://rust-analyzer.github.io) which will provide appropriate syntax highlighting, code navigation, folding, and more.

### Rust

Head over to [https://rust-lang.org](http://rust-lang.org) and install the Rust compiler.

We strongly recommend going through the [official Rust book](https://doc.rust-lang.org/book/ch01-00-getting-started.html) _completely_. However, we hope that a Dioxus app can serve as a great first Rust project. With Dioxus, you'll learn about:

- Error handling
- Structs, Functions, Enums
- Closures
- Macros

We've put a lot of care into making Dioxus syntax familiar and easy to understand, so you won't need deep knowledge of async, lifetimes, or smart pointers until you start building complex Dioxus apps.

### Platform-specific dependencies

Most platforms don't require any additional dependencies, but if you are targeting desktop, you can install the following dependencies:

```inject-dioxus
DesktopDependencies {}
```

### Dioxus CLI

Next, lets install the Dioxus CLI:

```
cargo install dioxus-cli
```

If you get an OpenSSL error on installation, ensure the dependencies listed [here](https://docs.rs/openssl/latest/openssl/#automatic) are installed.

## Create a new project

You can create a new Dioxus project by running the following command and following the prompts:

```sh
dx new
```

```inject-dioxus
video {
    "type": "video/mp4",
    "name": "dx new demo",
    autoplay: "true",
    controls: "false",
    r#loop: "true",
    width: "800px",
    muted: "true",
    source {
        src: asset!("/public/static/dioxus-new.mov"),
    }
}
```

First you will need to select a platform. Each platform has its own reference with more information on how to set up a project for that platform. Here are the platforms we recommend starting with:

- Web
    - [Client Side](../reference/web/index.md): runs in the browser through WebAssembly
    - [Fullstack](../reference/fullstack/index.md): renders to HTML text on the server and hydrates it on the client
> If you are not sure which web platform you want to use, check out the [choosing a web renderer](choosing_a_web_renderer.md) chapter.
- WebView
    - [Desktop](../reference/desktop/index.md): runs in a web view on desktop
    - [Mobile](../reference/mobile/index.md): runs in a web view on mobile. Mobile is currently not supported by the dioxus CLI. The [mobile reference](../reference/mobile/index.md) has more information about setting up a mobile project

Next, you can choose a styling library. For this project, we will use vanilla CSS.

Finally, you can choose to start the project with the router enabled. The router is covered in the [router guide](../router/index.md).

## Running the project

Once you have created your project, you can start it with the following command:

```sh
cd my_project
dx serve
```

For projects using the liveview template, run `dx serve --desktop`.

For Web targets the application will be served at [http://localhost:8080](http://localhost:8080)

## Conclusion

That's it! You now have a working Dioxus project. You can continue learning about dioxus by [making a hackernews clone in the guide](../guide/index.md), or learning about specific topics/platforms in the [reference](../reference/index.md). If you have any questions, feel free to ask in the [discord](https://discord.gg/XgGxMSkvUM) or [open a discussion](https://github.com/DioxusLabs/dioxus/discussions).
