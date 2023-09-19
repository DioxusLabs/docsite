# Publishing

After you have build your application, you will need to publish it somewhere. This reference will outline different methods of publishing your desktop or web application.

## WEB: Publishing with Github Pages
To build our app and publish it to Github:

Edit your `Dioxus.toml` to point your `out_dir` to the `docs` folder and the `base_path` to the name of your repo:

```toml
[application]
# ...
out_dir = "docs"
base_path = "your_repo"
```

Then build your app and publish it to Github:

- Make sure GitHub Pages is set up for your repo to publish any static files in the docs directory
- Build your app with:
```sh
dx build --release
```
- Add and commit with git
- Push to GitHub

## DESKTOP: Creating an Installer

Dioxus desktop app uses your operating system's WebView library, so it's portable to be distributed for other platforms.

In this section, we'll cover how to bundle your app for macOS, Windows, and Linux.

## Install `dioxus CLI`


The first thing we'll do is install the [dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli). This extension to cargo will make it very easy to package our app for the various platforms.

To install, simply run

`cargo install dioxus-cli --locked`

## Building

To bundle your application you can simply run `dx bundle --release` to produce a final app with all the optimizations and assets builtin.

Once you've ran `dx bundle --release`, your app should be accessible in

`dist/bundle/`.

For example, a macOS app would look like this:

![Published App](/static/images/publish.png)

Nice! And it's only 4.8 Mb – extremely lean!! Because Dioxus leverages your platform's native WebView, Dioxus apps are extremely memory efficient and won't waste your battery.

> Note: not all CSS works the same on all platforms. Make sure to view your app's CSS on each platform – or web browser (Firefox, Chrome, Safari) before publishing.

