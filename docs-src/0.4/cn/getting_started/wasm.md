# Web

使用Dioxus构建在浏览器中运行的单页应用程序。要在网络上运行，您的应用程序必须编译到WebAssembly，并依赖于`dioxus`和`dioxus-web` 库。

用于Web的Dioxus构建将大致相当于React构建的大小（70kb对比65kb），但它的加载速度将明显加快，因为 [WebAssembly可以在流式传输时进行编译](https://hacks.mozilla.org/2018/01/making-webassembly-even-faster-firefoxs-new-streaming-and-tiering-compiler/)。

示例:

- [TodoMVC](https://github.com/DioxusLabs/example-projects/tree/master/todomvc)
- [ECommerce](https://github.com/DioxusLabs/example-projects/tree/master/ecommerce-site)

[![TodoMVC example](https://github.com/DioxusLabs/example-projects/raw/master/todomvc/example.png)](https://github.com/DioxusLabs/example-projects/blob/master/todomvc)

> 注意: 由于Wasm的局限性， [不是每个rust库都能](https://rustwasm.github.io/docs/book/reference/which-crates-work-with-wasm.html)与你的网络程序配合使用, 因此你需要确保你的库在没有本机系统调用（计时器、IO等）的情况下工作。

## 支持

Web是Dioxus最受支持的目标平台。

- 由于您的应用程序将被编译为WASM，因此您可以通过[wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html)访问浏览器API。
- Dioxus提供水合作用，以恢复在服务器上呈现的应用程序。有关更多信息，请参阅[全栈](fullstack.md)入门指南。

## 工具

要为网络开发您的Dioxus应用程序，您需要一个工具来构建和服务您的资产。我们建议使用[dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli)，其中包括构建系统、Wasm优化、开发服务器，并支持热重新加载：

```shell
cargo install dioxus-cli
```

确保安装了`wasm32-unknown-unknown`的rust target：

```shell
rustup target add wasm32-unknown-unknown
```

## 创建一个项目

创建一个新的库:

```shell
cargo new --bin demo
cd demo
```

添加Dioxus和Web渲染器作为依赖项(这将编辑您的`Cargo.toml`):

```bash
cargo add dioxus
cargo add dioxus-web
```

编辑你的`main.rs`:

```rust
{{#include src/doc_examples/hello_world_web.rs}}
```

并为我们的应用程序提供服务：

```bash
dx serve
```

如果您打开浏览器并导航到`127.0.0.1`，您应该会看到一个像这样的应用程序：

```inject-dioxus
DemoFrame {
    hello_world::HelloWorldCounter {}
}
```


## 热重载

1. 热重新加载通过解释rsx调用和流式传输编辑，允许在rsx调用中更快地迭代时间。
2. 在更改程序的样式/布局时，它很有用，但对更改程序的逻辑没有帮助。

对于Web渲染器，您可以使用dioxus cli为启用热重新加载的应用程序提供服务。

### 设置

安装 [dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli).

### 用法

1. 运行:

```bash
dx serve --hot-reload
```

2. 在rsx中更改一些代码或渲染宏
3. 在浏览器中打开您的本地主机
4. 保存并观看样式变化，无需重新编译


### 局限性

1. 解释器只能使用最后一个完整重新编译中存在的表达式。如果您向rsx调用引入一个新的变量或表达式，则需要完整的重新编译才能捕获表达式。
2. 组件、迭代器和一些属性可以包含任意的Rust代码，并在更改时触发完全重新编译。
