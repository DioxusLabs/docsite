# 介绍

Dioxus 是一个便携、高性能且符合人体工程学的框架，用于在 Rust 中构建跨平台用户界面。本指南将帮助您开始编写 Dioxus 应用程序，用于 Web、桌面、移动设备等多种平台。

\```rust
{{#include ../docs-router/src/doc_examples/readme.rs}}
\```

\```inject-dioxus
DemoFrame {
    readme::App {}
}
\```

Dioxus 受到 React 的很大启发。如果您了解 React，则开始使用 Dioxus 将会很轻松。

> 本指南假定您已经了解一些 [Rust](https://www.rust-lang.org/)！如果没有，我们建议先阅读 [*这本书*](https://doc.rust-lang.org/book/ch01-00-getting-started.html) 来学习 Rust。

## 特性

- 三行代码即可创建跨平台应用程序。（Web、桌面、服务器、移动设备等）
- 极具人体工程学且功能强大的状态管理，结合了 React、Solid 和 Svelte 的优点。
- 全面的内联文档 - 对所有 HTML 元素、监听器和事件进行悬停和指南。
- 高性能应用程序，[接近最快的 Web 框架](https://dioxuslabs.com/blog/templates-diffing)，在桌面上具有原生速度。
- 一流的异步支持。

### 多平台

Dioxus 是一个 *便携* 工具包，意味着核心实现可以在任何地方运行，无需平台相关的链接。与许多其他 Rust 前端工具包不同，Dioxus 并不与 WebSys 紧密关联。事实上，每个元素和事件监听器都可以在编译时进行替换。默认情况下，Dioxus 配备了启用了 `html` 功能的特性，但根据您的目标渲染器，这可以禁用。

目前，我们有几个一方渲染器：
- WebSys/Sledgehammer（用于 WASM）：很好的支持
- Tao/Tokio（用于桌面应用程序）：良好的支持
- Tao/Tokio（用于移动应用程序）：支持较差
- Fullstack（用于 SSR 和服务器功能）：良好的支持
- TUI/Plasmo（用于基于终端的应用程序）：实验性的

## 稳定性

Dioxus 尚未达到稳定版本。

Web：由于 Web 是一个相当成熟的平台，我们预计基于 Web 的功能 API 变动会很少。

桌面：API 可能会变动，因为我们正在摸索比我们的 ElectronJS 对应项更好的模式。

Fullstack：API 可能会变动，因为我们正在摸索与服务器通信的最佳 API。
