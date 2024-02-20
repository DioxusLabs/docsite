# Desktop 概述

构建一个独立的原生桌面应用程序，在不同的操作系统上看起来和使用感觉都一样。

Dioxus桌面构建的应用程序使用系统级WebView来渲染页面。这使得应用程序的最终大小比其他WebView渲染器小得多(通常低于5MB)。

虽然桌面应用程序是在WebView中渲染的，你的Rust代码是会原生运行。这意味着浏览器API是 _不_ 可用的,因此渲染WebGL、Canvas等不像Web那么容易。然而，原生系统级别的API是 _可以_ 访问的，因此流、WebSocket、文件系统等都可以通过系统API轻松访问。

Dioxus桌面是在[Tauri](https://tauri.app/)上构建的。目前，菜单栏、事件处理等上的Dioxus抽象程度有限。在某些地方，你可能需要直接使用Tauri - 通过[Wry](http://github.com/tauri-apps/wry/)和[Tao](http://github.com/tauri-apps/tao)。

> 未来，我们计划转向具有WGPU集成的([Blitz](https://github.com/DioxusLabs/blitz))的基于自定义Web渲染器的DOM渲染器。

## 示例

- [File Explorer](https://github.com/DioxusLabs/example-projects/blob/master/file-explorer)
- [WiFi Scanner](https://github.com/DioxusLabs/example-projects/blob/master/wifi-scanner)

[![File Explorer screenshot](https://raw.githubusercontent.com/DioxusLabs/example-projects/master/file-explorer/image.png)](https://github.com/DioxusLabs/example-projects/tree/master/file-explorer)

这里是对于主要仓库的[查询](https://github.com/search?q=repo%3ADioxusLabs%2Fdioxus+path%3A%2F%5Eexamples%5C%2F%2F+%22use+dioxus_desktop%22&type=code)以查找使用`dioxus_desktop`的示例(可能不是100%的)。

# 入门指南

## 特定于平台的依赖项

Dioxus desktop通过WebView渲染。根据您的平台，您可能需要安装一些依赖项。

### Windows

Windows应用程序依赖于WebView2——一个应该安装在所有现代Windows发行版中的库。如果您安装了Edge，那么Dioxus将正常工作。如果您 _没有_ 安装WebView2，[那么您可以通过微软安装它](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)。MS提供了3个选项：

1. 一个小型的“evergreen” _引导程序_ ,从微软的CDN中获取安装程序。
2. 一个小型的从微软的CDN获取的WebView2的 _安装程序_。
3. 针对离线用户的最终二进制文件中WebView2的静态链接版本。

出于开发目的，请使用选项1。

### Linux

WebView Linux应用程序需要WebkitGtk。不同版本发行时，这可能已经成为`.rpm`或`.deb`中依赖树的一部分。因此，你的用户可能已经拥有了WebkitGtk。

```bash
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev
```

当使用Debian/bullseye时，`libappindicator3-dev`不再可用并且被`libayatana-appindicator3-dev`取代。

```bash
# on Debian/bullseye use:
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev
```

如果您遇到问题，请确保您安装了所有基础配置,正如[Tauri文档](https://beta.tauri.app/guides/prerequisites/)中所述。

### MacOS

目前 – macOS的所有内容都内置了！但是，如果您使用的是nightly Rust，由于我们的Tao依赖项中的一些权限问题（这些问题已解决但尚未发布），您可能会遇到问题。

## 创建一个项目

创建一个新的crate:

```shell
cargo new --bin demo
cd demo
```

添加Dioxus和桌面渲染器作为依赖(这将编辑你的`Cargo.toml`):

```shell
cargo add dioxus
cargo add dioxus-desktop
```

编辑你的`main.rs`:

```rust
{{#include src/doc_examples/hello_world_desktop.rs:all}}
```

## 热重载

1. 热重载通过解释rsx调用和流式传输编辑，允许在rsx调用中更快地迭代时间。
2. 在更改程序的样式/布局时，它很有用，但对更改程序的逻辑没有帮助。

### 设置

安装[dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli).

### 用法

1. 运行:

```bash
dx serve --hot-reload --platform desktop
```

2. 在`rsx`或`render`宏中更改一些代码。
3. 保存并观看样式变化，无需重新编译。

### 局限性

1. 解释器只能使用最后一个完整重新编译中存在的表达式。如果您向rsx调用引入一个新的变量或表达式，则需要完整的重新编译才能捕获表达式。
2. 组件、迭代器和一些属性可以包含任意的Rust代码，并在更改时触发完全重新编译。
