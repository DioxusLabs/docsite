# Static Site Generation

Static site generation lets you pre-generate all static pages of your application at build time. Once you have the static html pages, you can deploy them to any static hosting provider like GitHub Pages.

## Setting up the ServeConfig

Static site generation builds on top of the incremental rendering feature of Dioxus fullstack. To use ssg, we need to set up the `ServeConfig` to enable incremental rendering and set the static directory to the directory Dioxus outputs all of the static assets to. Dioxus always bundles static assets to the `public` directory in the web folder alongside the server binary.

```rust
{{#include src/doc_examples/static_site_generation.rs:main}}
```

## Configuring static routes

Once you have incremental rendering enabled, you need to tell the CLI what routes are static in your application. You need to create a server function at the endpoint `"static_routes"` that returns a list of all static route urls in your application. The CLI will call this server function at build time and pre-render all of the routes in the list.

```rust
{{#include src/doc_examples/static_site_generation.rs:static_routes}}
```

## Publishing static sites

Finally, you can bundle your static site with `dx bundle --platform web --ssg`. Once the CLI finishes bundling, you should see a `public` folder in the dx folder of your project:

![Dioxus SSG](/assets/06_docs/ssg_folder.png)

The folder contains all of the static assets that you need to serve your site. You can copy the public folder into any static hosting provider like GitHub Pages.
