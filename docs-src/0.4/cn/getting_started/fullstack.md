# 全栈

> 本指南假设您阅读了[Web](wasm.md)入门指南并安装了[Dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli)

# 入门指南

## 设置

在本指南中，我们将展示如何将Dioxus与[Axum](https://docs.rs/axum/latest/axum/)，但`dioxus-fullstack`还与[Warp](https://docs.rs/warp/latest/warp/)和[Salvo](https://docs.rs/salvo/latest/salvo/)Web框架集成。

确保您安装了Rust和Cargo，然后创建一个新项目：

```shell
cargo new --bin demo
cd demo
```

添加`dioxus`和`dioxus-fullstack`作为依赖项:

```shell
cargo add dioxus
cargo add dioxus-fullstack
```

接下来，为服务器(`ssr`)和客户端(`web`)设置功能:

```toml
[features]
default = []
ssr = ["dioxus-fullstack/axum"]
web = ["dioxus-fullstack/web"]
```

您的依赖项应该大致看起来像这样:

```toml
[dependencies]
dioxus = { version = "*" }
dioxus-fullstack = { version = "*" }

[features]
default = []
ssr = ["dioxus-fullstack/axum"]
web = ["dioxus-fullstack/web"]
```

现在，设置您的Axum应用程序以服务Dioxus应用程序。

```rust
{{#include src/doc_examples/server_basic.rs}}
```

现在，运行您的应用程序：

```
dx build --features web --release
cargo run --features ssr --release
```

最后，在浏览器中打开`http://localhost:8080`。您应该会看到一个带有计数器的服务器端渲染页面。

```inject-dioxus
SandBoxFrame {
	url: "https://codesandbox.io/p/sandbox/dioxus-fullstack-2nwsrz?file=%2Fsrc%2Fmain.rs%3A5%2C1"
}
```

## 热重载

1. 热重载通过解释rsx调用和流式传输编辑，允许在rsx调用中更快地迭代时间。
2. 在更改程序的样式/布局时，它很有用，但对更改程序的逻辑没有帮助。

### 用法

1. 运行:

```bash
dx build --features web
dx serve --features ssr --hot-reload --platform desktop
```

2. 在rsx中更改一些代码或渲染宏
3. 保存并观看样式变化，无需重新编译

### 局限性

1. 解释器只能使用最后一个完整重新编译中存在的表达式。如果您向rsx调用引入一个新的变量或表达式，则需要完整的重新编译才能捕获表达式。
2. 组件、迭代器和一些属性可以包含任意的Rust代码，并在更改时触发完全重新编译。
