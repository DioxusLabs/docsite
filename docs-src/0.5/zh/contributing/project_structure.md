# 项目结构

Dioxus 组织中有许多包。本文档将帮助您了解每个包的目的以及它们如何彼此关联。

## 渲染器

- [Desktop](https://github.com/DioxusLabs/dioxus/tree/main/packages/desktop): 一个渲染器，可以在本地运行 Dioxus 应用程序，但是使用系统 webview 进行渲染
- [Mobile](https://github.com/DioxusLabs/dioxus/tree/main/packages/mobile): 一个渲染器，可以在本地运行 Dioxus 应用程序，但是使用系统 webview 进行渲染。目前这是 desktop 渲染器的一个副本
- [Web](https://github.com/DioxusLabs/dioxus/tree/main/packages/Web): 将 Dioxus 应用程序编译为 WASM 并操作 DOM 在浏览器中渲染
- [Liveview](https://github.com/DioxusLabs/dioxus/tree/main/packages/liveview): 在服务器上运行，并使用浏览器中的 websocket 代理进行渲染的渲染器
- [Plasmo](https://github.com/DioxusLabs/blitz/tree/master/packages/plasmo): 将 HTML 样式树渲染到终端的渲染器
- [TUI](https://github.com/DioxusLabs/blitz/tree/master/packages/dioxus-tui): 使用 Plasmo 在终端中渲染 Dioxus 应用程序的渲染器
- [Blitz-Core](https://github.com/DioxusLabs/blitz/tree/master/packages/blitz-core): 使用 WGPU 渲染 HTML 样式树的实验性本地渲染器
- [Blitz](https://github.com/DioxusLabs/blitz): 使用 Blitz-Core 使用 WGPU 渲染 Dioxus 应用程序的实验性本地渲染器
- [SSR](https://github.com/DioxusLabs/dioxus/tree/main/packages/ssr): 在服务器上运行 Dioxus 应用程序，并将其呈现为 HTML 的渲染器

## 状态管理/Hooks

- [Hooks](https://github.com/DioxusLabs/dioxus/tree/main/packages/hooks): Dioxus 应用程序的常见 Hooks 集合
- [Signals](https://github.com/DioxusLabs/dioxus/tree/main/packages/signals): Dioxus 应用程序的实验性状态管理库。目前包含一个 `Copy` 版本的 Signal
- [SDK](https://github.com/DioxusLabs/sdk): 与系统接口交互的平台无关 Hooks 集合（剪贴板、相机等）。
- [Fermi](https://github.com/DioxusLabs/dioxus/tree/main/packages/fermi): Dioxus 应用程序的全局状态管理库。
- [Router](https://github.com/DioxusLabs/dioxus/tree/main/packages/router): Dioxus 应用程序的客户端路由器

## 核心工具

- [core](https://github.com/DioxusLabs/dioxus/tree/main/packages/core): 每个 Dioxus 应用程序都使用的核心虚拟 DOM 实现
  - 您可以在 [此博客文章](https://dioxuslabs.com/blog/templates-diffing/) 和 [指南中的自定义渲染器部分](../custom_renderer/index.md) 中了解更多关于 core 的架构信息
- [RSX](https://github.com/DioxusLabs/dioxus/tree/main/packages/RSX): 用于热重载、自动格式化和宏的 RSX 的核心解析
- [core-macro](https://github.com/DioxusLabs/dioxus/tree/main/packages/core-macro): 用于编写 Dioxus 应用程序的 rsx! 宏。（这是 RSX crate 的包装器）
- [HTML 宏](https://github.com/DioxusLabs/dioxus-html-macro): RSX 宏的 HTML 样式替代方案

## 本地渲染器工具

- [native-core](https://github.com/DioxusLabs/blitz/tree/main/packages/native-core): 增量计算状态（主要是样式）的树
  - 您可以在 [指南的自定义渲染器部分](../custom_renderer/index.html#native-core) 中了解有关 native-core 如何帮助您构建本地渲染器的更多信息
- [native-core-macro](https://github.com/DioxusLabs/blitz/tree/main/packages/native-core-macro): 本地 core 的辅助宏
- [Taffy](https://github.com/DioxusLabs/taffy): 驱动 Blitz-Core、Plasmo 和 Bevy UI 的布局引擎

## Web 渲染器工具

- [HTML](https://github.com/DioxusLabs/dioxus/tree/main/packages/html): 定义了特定于 HTML 的元素、事件和属性
- [Interpreter](https://github.com/DioxusLabs/dioxus/tree/main/packages/interpreter): 定义了 Web 和 Desktop 渲染器使用的浏览器绑定

## 开发者工具

- [hot-reload](https://github.com/DioxusLabs/dioxus/tree/main/packages/hot-reload): 使用 RSX crate 热重载任何 rsx! 宏的静态部分的宏。该宏与具有[集成](https://crates.io/crates/dioxus-hot-reload)的任何非 Web 渲染器一起使用
- [autofmt](https://github.com/DioxusLabs/dioxus/tree/main/packages/autofmt): 格式化 RSX 代码
- [rsx-rosetta](https://github.com/DioxusLabs/dioxus/tree/main/packages/RSX-rosetta): 处理 HTML 和 RSX 之间的转换
- [CLI](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli): 用于辅助 Dioxus 使用的命令行界面和 VSCode 扩展
