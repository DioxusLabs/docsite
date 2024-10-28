# Publishing

After you have build your application, you will need to publish it somewhere. This reference will outline different methods of publishing your desktop or web application.

## Web: Publishing with GitHub Pages

Edit your `Dioxus.toml` to point your `out_dir` to the `docs` folder and the `base_path` to the name of your repo:

```toml
[application]
# ...
out_dir = "docs"

[web.app]
base_path = "your_repo"
```

Then build your app and publish it to Github:

- Make sure GitHub Pages is set up for your repo to publish any static files in the docs directory
- Build your app with:
```sh
dx build --release
```
- Make a copy of your `docs/index.html` file and rename the copy to `docs/404.html` so that your app will work with client-side routing
- Add and commit with git
- Push to GitHub

## Desktop: Creating an installer

Dioxus desktop app uses your operating system's WebView library, so it's portable to be distributed for other platforms.

In this section, we'll cover how to bundle your app for macOS, Windows, and Linux.

## Preparing your application for bundling

Depending on your platform, you may need to add some additional code to your `main.rs` file to make sure your app is ready for bundling. On Windows, you'll need to add the `#![windows_subsystem = "windows"]` attribute to your `main.rs` file to hide the terminal window that pops up when you run your app. **If you're developing on Windows, only use this when bundling.** It will disable the terminal, so you will not get logs of any kind. You can gate it behind a feature, like so:

```toml
# Cargo.toml
[features]
bundle = []
```

And then your `main.rs`:

```rust
#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]
```

## Adding assets to your application

If you want to bundle assets with your application, you can either use them with the `manganis` crate (covered more in the [assets](../reference/assets.md) page), or you can include them in your `Dioxus.toml` file:

```toml
[bundle]
# The list of files to include in the bundle. These can contain globs.
resources = ["main.css", "header.svg", "**/*.png"]
```

## Install `dioxus CLI`

The first thing we'll do is install the [dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli). This extension to cargo will make it very easy to package our app for the various platforms.

To install, simply run

`cargo install dioxus-cli`

## Building

To bundle your application you can simply run `dx bundle --release` (also add `--features bundle` if you're using that, see the [this](#preparing-your-application-for-bundling) for more) to produce a final app with all the optimizations and assets builtin.

Once you've ran the command, your app should be accessible in `dist/bundle/`.

For example, a macOS app would look like this:

![Published App](public/static/publish.png)

Nice! And it's only 4.8 Mb – extremely lean!! Because Dioxus leverages your platform's native WebView, Dioxus apps are extremely memory efficient and won't waste your battery.

> Note: not all CSS works the same on all platforms. Make sure to view your app's CSS on each platform – or web browser (Firefox, Chrome, Safari) before publishing.

