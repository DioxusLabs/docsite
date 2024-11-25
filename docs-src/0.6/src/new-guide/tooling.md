# Setting up Tooling

This section will help you install the necessary tooling to get started with Dioxus. You'll need an editor, Rust, and the Dioxus CLI.

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
