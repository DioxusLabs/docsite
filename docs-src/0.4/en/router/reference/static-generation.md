# Static Generation

## Getting the Sitemap

The [`Routable`] trait includes an associated [`SITE_MAP`] constant that contains the map of all of the routes in the enum.

By default, the sitemap is a tree of (static or dynamic) RouteTypes, but it can be flattened into a list of individual routes with the `.flatten()` method.

## Generating a Sitemap

To statically render pages, we need to flatten the route tree and generate a file for each route that contains only static segments:

```rust
{{#include src/doc_examples/untested_04/static_generation.rs}}
```

## Example

- [examples/static-hydrated](https://github.com/DioxusLabs/dioxus/tree/master/packages%2Ffullstack%2Fexamples%2Fstatic-hydrated)
