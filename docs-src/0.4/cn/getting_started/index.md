# 入门指南

本节将帮助你设置Dioxus项目!

## 先决条件

### 编辑器

Dioxus与[Rust-Analyzer LSP 插件](https://rust-analyzer.github.io)很好的集成, 该插件将提供适当的语法高亮显示、代码导航、折叠等。

### Rust

前往[https://rust-lang.org](http://rust-lang.org)并安装Rust编辑器。

我们强烈建议完全阅读[Rust官方书](https://doc.rust-lang.org/book/ch01-00-getting-started.html) _完全_. 然而，我们希望Dioxus应用程序可以作为第一个伟大的Rust项目。使用Dioxus,您将了解: 

- 错误处理
- 结构, 函数, 枚举
- 闭包
- 宏

我们非常注意使Dioxus语法熟悉且易于理解，因此在开始构建复杂的Dioxus应用程序之前，您不需要对异步、生命周期或智能指针有深入的了解。

## 设置指南

Dioxus支持多个平台。在下面选择您的目标平台，以获取特定于平台的设置说明：

- [选择一个网络渲染器](choosing_a_web_renderer.md)
- [客户端](wasm.md): 通过WebAssembly在浏览器中运行
- [Liveview](liveview.md): 在服务器上运行，使用WebSockets在浏览器中渲染
- [全栈](fullstack.md): 在服务器上以HTML文本呈现，在客户端上将其“注水”(使其内容渲染)
- [桌面端](desktop.md): 在桌面上的网页视图中运行
- [移动端](mobile.md): 在移动网络视图中运行
- [终端界面](tui.md): 在终端中渲染基于文本的图形

> 有关您选择的任何平台的更多信息，请参阅[参考资料](../reference/index.md)中同名部分
