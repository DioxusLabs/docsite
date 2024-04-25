# 配置项目

本章将教你如何使用 `Dioxus.toml` 文件配置 CLI。这里有一个 [示例](#config-example)，其中有注释来描述每个键。你可以复制它，或者查看本文档以获取更完整的学习体验。

"🔒" 表示必填项。一些标题是必填的，但它们内部的键不是。在这种情况下，你只需要包含标题，但不需要包含键。这可能看起来有点奇怪，但是很正常。

## 结构

每个标题下面直接是其 TOML 表单。

### 应用程序 🔒

```toml
[application]
```

应用程序范围的配置。适用于 Web 和桌面应用。

* **name** 🔒 - 项目名称和标题。
  ```toml
  name = "my_project"
  ```
* **default_platform** 🔒 - 此项目的目标平台
  ```toml
    # 当前支持的平台：web，desktop
    default_platform = "web"
  ```
* **out_dir** - 将 `dx build` 或 `dx serve` 的构建结果输出的目录。这也是 `assets` 目录将被复制到的位置。
  ```toml
  out_dir = "dist"
  ```
* **asset_dir** - 包含静态资产的目录。CLI 将在构建/服务之后自动将这些资产复制到 **out_dir** 中。
  ```toml
  asset_dir = "public"
  ```
- **sub_package** - 默认情况下要构建的工作区中的子包。
  ```toml
  sub_package = "my-crate"
  ```

### Web.App 🔒

```toml
[web.app]
```

Web 特定的配置。

* **title** - 网页标题。
  ```toml
  # HTML 标题标签内容
  title = "project_name"
  ```
* **base_path** - 为应用程序构建的基本路径。当在域名的子目录下服务应用程序时，这可能很有用。例如，在构建要在 GitHub Pages 上提供服务的站点时。
  ```toml
  # 应用程序将在 domain.com/my_application/ 处提供服务，因此我们需要将 base_path 修改为应用程序将被服务的路径
  base_path = "my_application"
  ```

### Web.Watcher 🔒

```toml
[web.watcher]
```

开发服务器配置。

* **reload_html** - 如果为 true，则每次重新构建应用程序时 CLI 将重新构建 index.html 文件
  ```toml
  reload_html = true
  ```
* **watch_path** - 要监视更改的文件和目录
  ```toml
  watch_path = ["src", "public"]
  ```

* **index_on_404** - 如果启用，当找不到路由时 Dioxus 将提供根页面。
  *当服务使用路由器的应用程序时需要此选项*。但是，当使用非 Dioxus 的东西（例如 GitHub Pages）提供应用程序时，你需要检查如何在该平台上配置它。在 GitHub Pages 上，你可以将名为 `404.html` 的 `index.html` 的副本放在同一个目录中。
  ```toml
  index_on_404 = true
  ```

### Web.Resource 🔒

```toml
[web.resource]
```

静态资源配置。

* **style** - 要包含在应用程序中的 CSS 文件。
  ```toml
  style = [
  # 从 public_dir 包含。
  "./assets/style.css",
  # 或来自在线 CDN 的资产。
  "https://cdn.jsdelivr.net/npm/bootstrap/dist/css/bootstrap.css"
  ]
  ```

* **script** - 要包含在应用程序中的 JavaScript 文件。
  ```toml
  script = [
  # 从 asset_dir 包含。
  "./public/index.js",
  # 或来自在线 CDN。
  "https://cdn.jsdelivr.net/npm/bootstrap/dist/js/bootstrap.js"
  ]
  ```

### Web.Resource.Dev 🔒

```toml
[web.resource.dev]
```

这与 [`[web.resource]`](#webresource-) 相同，但仅在开发服务器中有效。例如，如果你想在 `dx serve` 服务器中包含一个文件，但不在 `dx serve --release` 服务器中包含，那么将它放在这里。

### Web.Proxy

```toml
[[web.proxy]]
```

与开发过程中应用程序需要的任何代理相关的配置。代理将请求转发到一个新的服务。

* **backend** - 要代理的服务器的 URL。CLI 将把任何位于后端相对路径下的请求转发到后端，而不是返回 404
  ```toml
  backend = "http://localhost:8000/api/"
  ```
  这将导致任何发送到具有前缀 /api/ 的开发服务器的请求被重定向到本地主机的后端服务器 http://localhost:8000。路径和查询参数将原样传递（目前不支持路径重写）。

## 配置示例

这包括所有字段，无论是否必需。

```toml
[application]

# 应用程序名称
name = "project_name"

# 默认的 Dioxus 平台
default_platform = "web"

# `build` & `serve` 输出路径
out_dir = "dist"

# 静态资源路径
asset_dir = "public"

[web.app]

# HTML 标题标签内容
title = "project_name"

[web.watcher]

# 当监视器触发时，重新生成 `index.html`
reload_html = true

# 将监视的文件或目录
watch_path = ["src", "public"]

# 包含样式或脚本资产
[web.resource]

# CSS 样式文件
style = []

# JavaScript 代码文件
script = []

[web.resource.dev]

# 与 [web.resource] 相同，但用于开发服务器

# CSS 样式文件
style = []

# JavaScript 文件
script = []

[[web.proxy]]
backend = "http://localhost:8000/api/"
```
