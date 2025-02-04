# 创建项目

安装了 Dioxus CLI 之后，你可以使用它来创建自己的项目！

## 初始化项目

首先，运行 `dx new` 命令来创建一个新项目。

> 它会克隆这个 [模板](https://github.com/DioxusLabs/dioxus-template)，该模板用于创建 dioxus 应用程序。
>
> 你可以通过传递 `template` 参数从不同的模板创建项目：
> ```
> dx new --template gh:dioxuslabs/dioxus-template
> ```

接下来，使用 `cd project-name` 导航到你的新项目中，或者直接在 IDE 中打开它。

> 在运行项目之前，请确保已安装 WASM 目标。
> 你可以使用 rustup 安装 rust 的 WASM 目标：
> ```
> rustup target add wasm32-unknown-unknown
> ```

最后，使用 `dx serve` 来服务你的项目！CLI 将告诉你它正在哪个地址上提供服务，以及其他信息，如代码警告。