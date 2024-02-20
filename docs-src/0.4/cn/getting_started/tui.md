# 终端用户界面

您可以使用Dioxus构建一个基于文本的界面，该界面将在终端中运行。

![Hello World screenshot](https://github.com/DioxusLabs/rink/raw/master/examples/example.png)

> 注：这本书是考虑到基于HTML的平台而写的。你也许可以跟随TUI，但你必须适应一点。

## 支持

TUI支持目前是相当实验性的。但是，如果你愿意冒险进入未知的领域，本指南将帮助您入门。

- 它使用flexbox进行布局
- 它只支持属性和元素的子集
- 常规小部件在tui渲染中不起作用，但tui渲染器有自己的小部件组件，以大写字母开头。查看[小组件示例](https://github.com/DioxusLabs/dioxus/blob/master/packages/dioxus-tui/examples/widgets.rs)
- 1px是一个字符行高。您的常规CSS px不会翻译
- 如果您的应用程序出现panic，您的终端就会崩溃。这最终会修复的

## 设置

首先制作一个新软件包，并将Dioxus和TUI渲染器添加为依赖项。

```shell
cargo new --bin demo
cd demo
cargo add dioxus
cargo add dioxus-tui
```

然后，使用基本模板编辑您的`main.rs`

```rust
{{#include src/doc_examples/hello_world_tui.rs}}
```

运行我们的应用程序：

```shell
cargo run
```

按"ctrl-c"关闭应用程序。要从"ctrl-c"切换到"q退出，您可以使用配置启动应用程序以禁用默认退出，并使用根TuiContext自行退出。

```rust
{{#include src/doc_examples/hello_world_tui_no_ctrl_c.rs}}
```

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

2. 在rsx中更改一些代码或渲染宏
3. 保存并观看样式变化，无需重新编译

### 局限性

1. 解释器只能使用最后一个完整重新编译中存在的表达式。如果您向rsx调用引入一个新的变量或表达式，则需要完整的重新编译才能捕获表达式。
2. 组件、迭代器和一些属性可以包含任意的Rust代码，并在更改时触发完全重新编译。
