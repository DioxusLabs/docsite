# Configure Project

This chapter will introduce `Dioxus.toml` and anatomy the config file.

## Structure

We use `toml` to define some info for `dioxus` project.

### Application

1. **_name_** - project name & title
2. **_default_platform_** - which platform target for this project.
   ```
   # current support: web, desktop
   # default: web
   default_platform = "web"
   ```
   change this to `desktop`, the `dx build & serve` will default build desktop app.
3. **_out_dir_** - which directory to put the output file; use `dx build & service`, the output will put at this directory, and the `assets` will be also copy to here.
   ```
   out_dir = "dist"
   ```
4. **_asset_dir_** - which direcotry to put your `static, assets` file, cli will automatic copy all file to `out_dir`, so you can put some resource file in there, like `CSS, JS, Image` file.
   ```
   asset_dir = "public"
   ```

### Application.Tools

You can combine different tools with `dioxus`.

1. **_binaryen_** - Use the `binaryen` tooling suite.
   ```
   # current support: wasm-opt
   # default: web
   binaryen = { wasm_opt = true }
   ```
   Use the `wasm_opt = true` key/pair value to activate optimization with wasm-opt.
   When building on `release` profile, Dioxus will run `wasm_opt` with `-Oz` option.
2. **_tailwindcss_** - Use the `tailwindcss` standalone binary to generate a Tailwind CSS bundle file.

   ```
   tailwindcss = { input = "main.css", config = "tailwind.config.js" }
   ```

   You can set two optional keys :

   - input: path of the input CSS file (default value is "public/tailwind.css")
   - config: path to the config file for Tailwind (default value is "src/tailwind.config.js")

   When building on `release` profile, Dioxus will run `tailwindcss` with the `--minify` option.

   Note : Dioxus will automatically include the generated tailwind file in the `index.html`

### Web.App

Web platform application config:

1. **_title_** - this value will display on the web page title. like `<title></title>` tag.
   ```
   # HTML title tag content
   title = "dioxus app | ⛺"
   ```

### Web.Watcher

Web platform `dev-server` watcher config:

1. **_reload_html_** - a boolean value; when watcher trigger, regenerate `index.html` file.
   ```
   # when watcher trigger, regenerate the `index.html`
   reload_html = true
   ```
2. **_watch_path_** - which files & directories will be watcher monitoring.
   ```
   watch_path = ["src", "public"]
   ```

### Web.Resource

Include some `CSS Javascript` resources into html file.

1. **_style_** - include some style(CSS) file into html.
   ```
   style = [
      # include from public_dir.
      "./assets/style.css",
      # or some asset from online cdn.
      "https://cdn.jsdelivr.net/npm/bootstrap/dist/css/bootstrap.css"
   ]
   ```
2. **_script_** - include some script(JS) file into html.
   ```
   style = [
       # include from public_dir.
       "./assets/index.js",
       # or some asset from online cdn.
       "https://cdn.jsdelivr.net/npm/bootstrap/dist/js/bootstrap.js"
   ]
   ```

### Web.Resource.Dev

Only include resources at `Dev` mode.

1. **_style_** - include some style(CSS) file into html.
   ```
   style = [
      # include from public_dir.
      "./assets/style.css",
      # or some asset from online cdn.
      "https://cdn.jsdelivr.net/npm/bootstrap/dist/css/bootstrap.css"
   ]
   ```
2. **_script_** - include some script(JS) file into html.
   ```
   style = [
       # include from public_dir.
       "./assets/index.js",
       # or some asset from online cdn.
       "https://cdn.jsdelivr.net/npm/bootstrap/dist/js/bootstrap.js"
   ]
   ```

### Web.Proxy

Proxy requests matching a path to a backend server.

1. **_backend_** - the URL to the backend server.
   ```
   backend = "http://localhost:8000/api/"
   ```
   This will cause any requests made to the dev server with prefix /api/ to be redirected to the backend server at http://localhost:8000. The path and query parameters will be passed on as-is (path rewriting is not currently supported).

### Bundle

Information to include in the bundled output.

1. **_identifier_** - a string value; the identifier of the bundle.
   ```
   identifier = "com.dioxus.app"
   ```
2. **_publisher_** - a string value; the publisher of the bundle.
   ```
   publisher = "dioxus"
   ```
3. **_icon_** - a list of strings; the icons for the bundle.

   ```
   icon = ["assets/icon.png"]
   ```

4. **_resources_** - a list of glob patterns; resources to include in the bundle.

   ```
   resources = ["assets/*"]
   ```

5. **_copyright_** - a string value; the copyright of the bundle.

   ```
   copyright = "Copyright (c) Jane Doe 2016. All rights reserved."
   ```

6. **_category_** - a string value; the category of the bundle. one of these values: `Business`, `Developer Tool`, `Education`, `Entertainment`, `Finance`, `Game`, `Action Game`, `Adventure Game`, `Arcade Game`, `Board Game`, `Card Game`, `Casino Game`, `Dice Game`, `Educational Game`, `Family Game`, `Kids Game`, `Music Game`, `Puzzle Game`, `Racing Game`, `Role-Playing Game`, `Simulation Game`, `Sports Game`, `Strategy Game`, `Trivia Game`, `Word Game`, `Graphics and Design`, `Healthcare and Fitness`, `Lifestyle`, `Medical`, `Music`, `News`, `Photography`, `Productivity`, `Reference`, `Social Networking`, `Sports`, `Travel`, `Utility`, `Video`, `Weather`,

   ```
   category = "Business"
   ```

7. **_short_description_** - a string value; the short description of the bundle.

   ```
   short_description = "A short description of the bundle."
   ```

8. **_long_description_** - a string value; the long description of the bundle.

   ```
   long_description = """
   A long description of the bundle.
   """
   ```

## Config example

```toml
[application]

# App (Project) Name
name = "{{project-name}}"

# Dioxus App Default Platform
# desktop, web, mobile, ssr
default_platform = "web"

# `build` & `serve` dist path
out_dir = "dist"

# resource (public) file folder
asset_dir = "public"

[web.app]

# HTML title tag content
title = "dioxus | ⛺"

[web.watcher]

# when watcher trigger, regenerate the `index.html`
reload_html = true

# which files or dirs will be watcher monitoring
watch_path = ["src", "public"]

# include `assets` in web platform
[web.resource]

# CSS style file
style = []

# Javascript code file
script = []

[web.resource.dev]

# serve: [dev-server] only

# CSS style file
style = []

# Javascript code file
script = []

[[web.proxy]]
backend = "http://localhost:8000/api/"

[bundle]

# Bundle identifier
identifier = "com.dioxus.app"

# Bundle publisher
publisher = "dioxus"

# Bundle icon. On windows, the icon file will default to `icons/icon.ico`
icon = ["assets/icon.png"]

# Bundle resources
resources = ["assets/*"]

# Bundle copyright
copyright = "Copyright (c) Jane Doe 2016. All rights reserved."

# Bundle category
category = "Business"

# Bundle short description
short_description = "A short description of the bundle."

# Bundle long description
long_description = """
A long description of the bundle.
"""
```
