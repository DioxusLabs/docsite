# Static Site Generation

Static site generation (SSG) lets you pre-generate all static pages of your application at build time. Once you have the static html pages, you can deploy them to any static hosting provider like GitHub Pages.

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

Finally, you can bundle your site with `dx bundle --platform web --ssg`. Once the CLI finishes bundling, you should see a `public` folder in the dx folder of your project:

![Dioxus SSG](/assets/06_docs/ssg_folder.png)

The folder contains all of the static assets that you need to serve your site. You can copy the public folder into any static hosting provider like GitHub Pages.
