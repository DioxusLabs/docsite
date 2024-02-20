# Liveview

Liveview允许应用程序在服务器上*运行*并且在浏览器中*渲染*。它使用WebSockets在服务器和浏览器之间进行通信。

Examples:
- [Axum 示例](https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/axum.rs)
- [Salvo 示例](https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/salvo.rs)
- [Warp 示例](https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/warp.rs)


## 支持

与Web平台相比，Liveview目前功能有限。Liveview应用程序在服务器上以原生线程运行。这意味着浏览器API不可用，因此渲染WebGL、Canvas等不像Web那么容易。然而，本机系统API是可访问的，因此流媒体、WebSocket、文件系统等都是可行的API。


## 设置

在本指南中，我们将展示如何将Dioxus Liveview与[Axum](https://docs.rs/axum/latest/axum/)一起使用。

确保您安装了Rust和Cargo，然后创建一个新项目：

```shell
cargo new --bin demo
cd demo
```

添加Dioxus和带有Axum功能的liveview渲染器作为依赖项：

```shell
cargo add dioxus
cargo add dioxus-liveview --features axum
```

接下来，添加所有Axum依赖项。如果您使用不同的Web框架，这将有所不同

```
cargo add tokio --features full
cargo add axum
```

您的依赖应该大致看起来像这样：

```toml
[dependencies]
axum = "0.4.5"
dioxus = { version = "*" }
dioxus-liveview = { version = "*", features = ["axum"] }
tokio = { version = "1.15.0", features = ["full"] }
```

现在，设置您的Axum应用程序以在端点上响应。


```rust
{{#include src/doc_examples/hello_world_liveview.rs:glue}}
```


然后添加我们的应用程序组件：

```rust
{{#include src/doc_examples/hello_world_liveview.rs:app}}
```

就是这样！


## 热重载

1. 热重载通过解释rsx调用和流式传输编辑，允许在rsx调用中更快地迭代时间。
2. 在更改程序的样式/布局时，它很有用，但对更改程序的逻辑没有帮助。

### 设置

安装 [dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli).

### 用法

1. 运行:

```bash
dx serve --hot-reload --platform desktop
```

2. 在`rsx`或`渲染`宏中更改一些代码
3. 保存并观看样式变化，无需重新编译

### 局限性

1. 解释器只能使用最后一个完整重新编译中存在的表达式。如果您向rsx调用引入一个新的变量或表达式，则需要完整的重新编译才能捕获表达式。
2. 组件、迭代器和一些属性可以包含任意的Rust代码，并在更改时触发完全重新编译。
