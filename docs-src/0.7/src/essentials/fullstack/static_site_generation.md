# Static Site Generation

Static site generation (SSG) lets you pre-generate all static pages of your application at build time. Once you have the static HTML pages, you can deploy them to any static hosting provider like GitHub Pages.

SSG is extremely powerful since it lets you cache the rendering of your pages before deploying to production. This cuts down on bandwidth costs, lets you cache content on a CDN, and allows for deploying *without* a server. Many deploy providers let you deploy SSG sites for free!

## How Dioxus SSG works

Dioxus SSG works by running your app locally, querying the app for a sitemap, and then indexing your site using `curl` requests manually. If your site is configured to use SSG, then it will cache HTML for each page on the filesystem.

This approach to SSG is quite different than a traditional static-site-generator like Hugo, Jekyll, or Zola. Dioxus SSG is designed to let you write your entire site in Rust, load data however you want, and then deploy a hybrid SSG app that loads SPA content.

## You might not need SSG

Even if your app has a significant amount of static content, you might not actually need SSG. You should use SSG in a few cases:

- You have *lots* of static content that benefits from pre-rendering before deploy
- You don't need a backend for your site

Sites like docs and portfolios benefit from SSG while apps like photo editors will not. In many cases, you can simply set `Cache-Control` headers while rendering pages and let your CDN or reverse-proxy handle caching for you!

## Setting up the ServeConfig


SSG builds on top of the incremental rendering feature of Dioxus fullstack. We need to set up the `ServeConfig` to enable incremental rendering. The incremental config needs to render to the `public` directory where Dioxus places all other public files like the wasm binary and static assets. The `public` directory in the web folder will always be placed alongside the server binary.

```rust
{{#include ../docs-router/src/doc_examples/static_site_generation.rs:main}}
```

## Configuring static routes

Once you have incremental rendering enabled, you need to tell the CLI about the static routes in your app. The CLI looks for a server function at the endpoint `"static_routes"` that returns a list of all static urls. It will call this server function at build time and pre-render all of the routes in the list.

```rust
{{#include ../docs-router/src/doc_examples/static_site_generation.rs:static_routes}}
```

## Publishing static sites

Finally, you can bundle your site with `dx bundle --web --ssg`. Once the CLI finishes bundling, you should see a `public` folder in the dx folder of your project:

![Dioxus SSG](/assets/06_docs/ssg_folder.png)

The folder contains all of the static assets that you need to serve your site. You can copy the public folder into any static hosting provider like GitHub Pages.
