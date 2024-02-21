# Configure Project

This chapter will teach you how to configure the CLI with the `Dioxus.toml` file. There's an [example](#config-example) which has comments to describe individual keys. You can copy that or view this documentation for a more complete learning experience.

"🔒" indicates a mandatory item. Some headers are mandatory, but none of the keys inside them are. In that case, you only need to include the header, but no keys. It might look weird, but it's normal.

## Structure

Each header has its TOML form directly under it.

### Application 🔒

```toml
[application]
```

Application-wide configuration. Applies to both web and desktop.

* **name** 🔒 - Project name & title.
   ```toml
   name = "my_project"
   ```
* **default_platform** 🔒 - The platform this project targets
   ```toml
   # Currently supported platforms: web, desktop
   default_platform = "web"
   ```
* **out_dir** - The directory to place the build artifacts from `dx build` or `dx serve` into. This is also where the `assets` directory will be copied into.
    ```toml
    out_dir = "dist"
    ```
* **asset_dir** - The directory with your static assets. The CLI will automatically copy these assets into the **out_dir** after a build/serve.
   ```toml
   asset_dir = "public"
   ```
* **sub_package** - The sub package in the workspace to build by default.
   ```toml
   sub_package = "my-crate"
   ```

### Web.App 🔒

```toml
[web.app]
```

Web-specific configuration.

* **title** - The title of the web page.
   ```toml
   # HTML title tag content
   title = "project_name"
   ```
* **base_path** - The base path to build the application for serving at. This can be useful when serving your application in a subdirectory under a domain. For example, when building a site to be served on GitHub Pages.
   ```toml
   # The application will be served at domain.com/my_application/, so we need to modify the base_path to the path where the application will be served
   base_path = "my_application"
   ```

### Web.Watcher 🔒

```toml
[web.watcher]
```

Development server configuration.

* **reload_html** - If this is true, the cli will rebuild the index.html file every time the application is rebuilt
   ```toml
   reload_html = true
   ```
* **watch_path** - The files & directories to monitor for changes
   ```toml
   watch_path = ["src", "public"]
   ```

* **index_on_404** - If enabled, Dioxus will serve the root page when a route is not found.
   *This is needed when serving an application that uses the router*. However, when serving your app using something else than Dioxus (e.g. GitHub Pages), you will have to check how to configure it on that platform. In GitHub Pages, you can make a copy of `index.html` named `404.html` in the same directory.
   ```toml
   index_on_404 = true
   ```

### Web.Resource 🔒

```toml
[web.resource]
```

Static resource configuration.

* **style** - CSS files to include in your application.
   ```toml
   style = [
      # Include from public_dir.
      "./assets/style.css",
      # Or some asset from online cdn.
      "https://cdn.jsdelivr.net/npm/bootstrap/dist/css/bootstrap.css"
   ]
   ```

* **script** - JavaScript files to include in your application.
    ```toml
    script = [
        # Include from asset_dir.
        "./public/index.js",
        # Or from an online CDN.
        "https://cdn.jsdelivr.net/npm/bootstrap/dist/js/bootstrap.js"
    ]
   ```

### Web.Resource.Dev 🔒

```toml
[web.resource.dev]
```

This is the same as [`[web.resource]`](#webresource-), but it only works in development servers. For example, if you want to include a file in a `dx serve` server, but not a `dx serve --release` server, put it here.

### Web.Proxy

```toml
[[web.proxy]]
```

Configuration related to any proxies your application requires during development. Proxies will forward requests to a new service.

* **backend** - The URL to the server to proxy. The CLI will forward any requests under the backend relative route to the backend instead of returning 404
   ```toml
   backend = "http://localhost:8000/api/"
   ```
   This will cause any requests made to the dev server with prefix /api/ to be redirected to the backend server at http://localhost:8000. The path and query parameters will be passed on as-is (path rewriting is currently not supported).

## Config example

This includes all fields, mandatory or not.

```toml
[application]

# App name
name = "project_name"

# The Dioxus platform to default to
default_platform = "web"

# `build` & `serve` output path
out_dir = "dist"

# The static resource path
asset_dir = "public"

[web.app]

# HTML title tag content
title = "project_name"

[web.watcher]

# When watcher is triggered, regenerate the `index.html`
reload_html = true

# Which files or dirs will be monitored
watch_path = ["src", "public"]

# Include style or script assets
[web.resource]

# CSS style file
style = []

# Javascript code file
script = []

[web.resource.dev]

# Same as [web.resource], but for development servers

# CSS style file
style = []

# JavaScript files
script = []

[[web.proxy]]
backend = "http://localhost:8000/api/"
```
