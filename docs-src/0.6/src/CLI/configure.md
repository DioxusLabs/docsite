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

### Web.https

```toml
[[web.https]]
```

Controls the https config for the CLI.

* **enabled** enables or disables https in the CLI
   ```toml
   enabled = true
   ```
* **mkcert** enables or disables generating certs with the mkcert CLI
   ```toml
   mkcert = true
   ```
* **key_path** sets the path to use for the https key
   ```toml
   key_path = "/path/to/key"
   ```
* **cert_path** sets the path to use for the https cert
   ```toml
   cert_path = "/path/to/cert"
   ```

### Web.pre_compress

If this setting is enabled, the CLI will pre-compress the built assets in release mode with brotli. This setting is enabled by default.

```toml
[web]
pre_compress = true
```

### Web.wasm_opt

Controls the wasm-opt config for the CLI.

* **level** sets the level of optimization to use for wasm-opt in release builds.
   - z: optimize aggressively for size
   - s: optimize for size
   - 1: optimize for speed
   - 2: optimize for more for speed
   - 3: optimize for even more for speed
   - 4: optimize aggressively for speed (default)
   ```toml
   level = "z"
   ```
* **debug** keep debug symbols in the wasm file even in release builds
   ```toml
   debug = true
   ```

### Bundle

```toml
[bundle]
```

Controls the bundling process for your application. Dioxus uses tauri-bundler under the hood. This section only includes a subset of the options available in tauri-bundler. More options can be found in the tauri-bundler [documentation](https://v1.tauri.app/v1/guides/building/#configuration-options).

* **identifier** - A unique identifier for your application (e.g., `com.dioxuslabs`).
   ```toml
   identifier = "com.dioxuslabs"
   ```
* **publisher** - The name of the entity publishing the application.
   ```toml
   publisher = "DioxusLabs"
   ```
* **icon** - Paths to icon files to be used in the bundle. Icon files must be square and 16, 24, 32, 64, or 256 pixels in size. PNG icons must have a 32 bit depth in the RGBA format. If you use a `.icns` file is must fit [this](https://github.com/tauri-apps/tauri/blob/d8db5042a28635259f646c329c3ec5ccf23eac9e/tooling/cli/src/helpers/icns.json) format. The icons must include a `.icns` icon for macOS, `.ico` for Windows and `.png` for Linux.
   ```toml
   icon = [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
   ]
   ```
* **resources** - Additional files to include in the bundle. Each asset is copied from the path and is accessible from the bundle at the same path. Any [assets](../guides/assets.md) are automatically bundled with the installer.
   ```toml
   resources = ["path/to/resource"]
   ```
* **copyright** - Copyright information for the application.
   ```toml
   copyright = "Copyright 2023 DioxusLabs"
   ```
* **category** - The category of the application. Must be one of `Business`, `DeveloperTool`, `Education`, `Entertainment`, `Finance`, `Game`, `ActionGame`, `AdventureGame`, `ArcadeGame`, `BoardGame`, `CardGame`, `CasinoGame`, `DiceGame`, `EducationalGame`, `FamilyGame`, `KidsGame`, `MusicGame`, `PuzzleGame`, `RacingGame`, `RolePlayingGame`, `SimulationGame`, `SportsGame`, `StrategyGame`, `TriviaGame`, `WordGame`, `GraphicsAndDesign`, `HealthcareAndFitness`, `Lifestyle`, `Medical`, `Music`, `News`, `Photography`, `Productivity`, `Reference`, `SocialNetworking`, `Sports`, `Travel`, `Utility`, `Video`, or `Weather`
   ```toml
   category = "Utility"
   ```
* **short_description** - A brief description of the application.
   ```toml
   short_description = "A utility application built with Dioxus"
   ```
* **long_description** - A detailed description of the application.
   ```toml
   long_description = "This application provides various utility functions..."
   ```
* **external_bin** - Paths to external sidecar binaries to include in the bundle. These bundles may be accessed at runtime with the name of the binary (not the absolute path). **the target triple will be automatically added to the binary name before it is added to the bundle.**
   ```toml
   external_bin = ["path/to/external_binary"] # On macos, the binary at path/to/external_binary-aarch64-apple-darwin will be included in the bundle. It can be accessed at runtime with the name external_binary
   ```

### Bundle.macos

```toml
[bundle.macos]
```

Configuration options for macOS bundles.

* **frameworks** - List of frameworks to include in the bundle.
   ```toml
   frameworks = ["CoreML"]
   ```
* **minimum_system_version** - Minimum macOS version required. (default: `10.13`)
   ```toml
   minimum_system_version = "10.13"
   ```
* **license** - Path to the license file.
   ```toml
   license = "LICENSE.txt"
   ```
* **exception_domain** - Domain for exception handling. The domain must be lowercase without a port or protocol.
   ```toml
   exception_domain = "mysite.com"
   ```
* **signing_identity** - macOS signing identity.
   ```toml
   signing_identity = "SIGNING IDENTITY KEYCHAIN ENTRY NAME"
   ```
* **provider_short_name** - Provider short name for the bundle.
   ```toml
   provider_short_name = "DioxusLabs"
   ```
* **entitlements** - Path to the entitlements file.
   ```toml
   entitlements = "entitlements.plist"
   ```
* **hardened_runtime** - Whether to enable the [hardened runtime](https://developer.apple.com/documentation/security/hardened-runtime) in the bundle.
   ```toml
   hardened_runtime = true
   ```

### Bundle.windows

```toml
[bundle.windows]
```

Configuration options for Windows bundles.

* **digest_algorithm** - Sets the file digest algorithm used for signing.
   ```toml
   digest_algorithm = "sha-256"
   ```
* **certificate_thumbprint** - SHA1 hash of the signing certificate.
   ```toml
   certificate_thumbprint = "A1B2C3D4E5F6..."
   ```
* **timestamp_url** - Sets the server to used for timestamping the signature.
   ```toml
   timestamp_url = "http://timestamp.digicert.com"
   ```
* **tsp** - Whether to use the time stamping protocol.
   ```toml
   tsp = true
   ```
* **icon_path** - Path to the icon for the system tray icon. (defaults to `./icons/icon.ico`)
   ```toml
   icon_path = "assets/icon.ico"
   ```
* **webview_install_mode** - Installation mode for WebView2.
   EmbedBootstrapper: embed the WebView2 bootstrapper into the installer
   ```toml
   [webview_install_mode.EmbedBootstrapper]
   silent = true
   ```
   DownloadBootstrapper: download the WebView2 bootstrapper in the installer at runtime
   ```toml
   [webview_install_mode.DownloadBootstrapper]
   silent = true
   ```
   OfflineInstaller: Embed the WebView2 installer into the main installer
   ```toml
   [webview_install_mode.OfflineInstaller]
   silent = true
   ```
   FixedRuntime: Use a fixed path to the WebView2 runtime
   ```toml
   [webview_install_mode.FixedRuntime]
   path = "path/to/runtime"
   ```
   Skip: Does not install WebView2 as part of the installer. This will cause the application to fail if webview was not already installed
   ```toml
   webview_install_mode = "Skip"
   ```

## Config example

This includes all fields, mandatory or not.

```toml
[application]

# App name
name = "project_name"

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

[bundle]
identifier = "com.dioxuslabs"
publisher = "DioxusLabs"
icon = "assets/icon.png"
```
