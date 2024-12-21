# 翻译现有的 HTML

Dioxus 使用一种称为 RSX 的自定义格式来表示 HTML，因为它更简洁，看起来更像 Rust 代码。然而，将现有的 HTML 转换为 RSX 可能有点麻烦。这就是为什么 Dioxus 提供了一个名为 `dx translate` 的工具，它可以自动将 HTML 转换为 RSX！

Dx translate 可以使将大块的 HTML 转换为 RSX 变得更加容易！让我们尝试翻译一些来自 Dioxus 主页的 HTML：

```sh
dx translate --raw  "<div class=\"relative w-full mx-4 sm:mx-auto text-gray-600\"><div class=\"text-[3em] md:text-[5em] font-semibold dark:text-white text-ghdarkmetal font-sans py-12 flex flex-col\"><span>Fullstack, crossplatform,</span><span>lightning fast, fully typed.</span></div><h3 class=\"text-[2em] dark:text-white font-extralight text-ghdarkmetal pt-4 max-w-screen-md mx-auto\">Dioxus is a Rust library for building apps that run on desktop, web, mobile, and more.</h3><div class=\"pt-12 text-white text-[1.2em] font-sans font-bold flex flex-row justify-center space-x-4\"><a href=\"/learn/0.5/getting_started\" dioxus-prevent-default=\"onclick\" class=\"bg-red-600 py-2 px-8 hover:-translate-y-2 transition-transform duration-300\" data-dioxus-id=\"216\">Quickstart</a><a href=\"/learn/0.5/reference\" dioxus-prevent-default=\"onclick\" class=\"bg-blue-500 py-2 px-8 hover:-translate-y-2 transition-transform duration-300\" data-dioxus-id=\"214\">Read the docs</a></div><div class=\"max-w-screen-2xl mx-auto pt-36\"><h1 class=\"text-md\">Trusted by top companies</h1><div class=\"pt-4 flex flex-row flex-wrap justify-center\"><div class=\"h-12 w-40 bg-black p-2 m-4 flex justify-center items-center\"><img src=\"static/futurewei_bw.png\"></div><div class=\"h-12 w-40 bg-black p-2 m-4 flex justify-center items-center\"><img src=\"static/airbuslogo.svg\"></div><div class=\"h-12 w-40 bg-black p-2 m-4 flex justify-center items-center\"><img src=\"static/ESA_logo.svg\"></div><div class=\"h-12 w-40 bg-black p-2 m-4 flex justify-center items-center\"><img src=\"static/yclogo.svg\"></div><div class=\"h-12 w-40 bg-black p-2 m-4 flex justify-center items-center\"><img src=\"static/satellite.webp\"></div></div></div></div>"
```

我们得到以下的 RSX，你可以轻松地复制并粘贴到你的代码中：

```rs
div { class: "relative w-full mx-4 sm:mx-auto text-gray-600",
div { class: "text-[3em] md:text-[5em] font-semibold dark:text-white text-ghdarkmetal font-sans py-12 flex flex-col",
span { "Fullstack, crossplatform," }
span { "lightning fast, fully typed." }
}
h3 { class: "text-[2em] dark:text-white font-extralight text-ghdarkmetal pt-4 max-w-screen-md mx-auto",
"Dioxus is a Rust library for building apps that run on desktop, web, mobile, and more."
}
div { class: "pt-12 text-white text-[1.2em] font-sans font-bold flex flex-row justify-center space-x-4",
a {
href: "/learn/0.5/getting_started",
data_dioxus_id: "216",
dioxus_prevent_default: "onclick",
class: "bg-red-600 py-2 px-8 hover:-translate-y-2 transition-transform duration-300",
"Quickstart"
}
a {
dioxus_prevent_default: "onclick",
href: "/learn/0.5/reference",
data_dioxus_id: "214",
class: "bg-blue-500 py-2 px-8 hover:-translate-y-2 transition-transform duration-300",
"Read the docs"
}
}
div { class: "max-w-screen-2xl mx-auto pt-36",
h1 { class: "text-md", "Trusted by top companies" }
div { class: "pt-4 flex flex-row flex-wrap justify-center",
div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
img { src: "static/futurewei_bw.png" }
}
div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
img { src: "static/airbuslogo.svg" }
}
div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
img { src: "static/ESA_logo.svg" }
}
div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
img { src: "static/yclogo.svg" }
}
div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
img { src: "static/satellite.webp" }
}
}
}
}
```

## 使用方法

`dx translate` 命令有几个标志，你可以使用它们来控制你的 HTML 输入和 RSX 输出。

你可以使用 `--file` 标志将 HTML 文件翻译为 RSX：

```sh
dx translate --file index.html
```

或者你可以使用 `--raw` 标志将一串 HTML 转换为 RSX：

```sh
dx translate --raw "<div>Hello world</div>"
```

这两个命令将输出以下 RSX：

```rs
div { "Hello world" }
```

`dx translate` 命令将 RSX 输出到 stdout。你可以使用 `--output` 标志将 RSX 写入文件。

```sh
dx translate --raw "<div>Hello world</div>" --output index.rs
```

你可以使用 `--component` 标志自动创建一个组件。

```sh
dx translate --raw "<div>Hello world</div>" --component
```

这将输出以下组件：

```rs
fn component() -> Element {
rsx! {
div { "Hello world" }
}
}
```

要了解更多 `dx translate` 支持的不同标志，请运行 `dx translate --help`。
