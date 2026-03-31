# dioxuslabs.com

Source code for [dioxuslabs.com](https://dioxuslabs.com) -- the Dioxus documentation, blog, and landing page.

The site is built with Dioxus itself, pre-rendered with `dioxus_ssr`, and rehydrated on the client with `dioxus_web`.

## Getting started

You'll need a working Rust toolchain. Install the Dioxus CLI:

```sh
curl -fsSL https://dioxus.dev/install.sh | bash
```

Then serve the site locally:

```sh
dx serve --package dioxus_docs_site --hotpatch
```

This starts a dev server at [localhost:8080](http://localhost:8080) with hot-reloading. TailwindCSS is included automatically with dx 0.7.

## Repo structure

```
docs-src/           Markdown source files for all documentation
  0.7/              Current version docs (guides, essentials, migration)
  blog/             Blog post markdown (release notes, etc.)
  0.3/ - 0.6/       Older version docs

packages/
  docsite/          Main Dioxus app (components, routing, assets, styles)
  docs-07/          Generated Rust code from 0.7 markdown
  docs-blog/        Generated Rust code from blog markdown
  search/           Search functionality
  playground/       Interactive playground
  include_mdbook/   Build script that converts markdown into Dioxus components
```

The markdown in `docs-src/` is converted into Rust/Dioxus components at build time by the `include_mdbook` crate. You don't need to touch the generated code in `packages/docs-*` -- just edit the markdown.

## Editing docs

The docs live in `docs-src/0.7/`. Edit the markdown files directly and the dev server will pick up changes.

Each docs version has a `SUMMARY.md` that defines the sidebar structure. If you're adding a new page, make sure to add it there.

## Writing blog posts

Blog posts live in `docs-src/blog/src/`. See existing posts (like `release-070.md`) for formatting conventions. The blog `SUMMARY.md` controls which posts appear and in what order.

## Contributing

- Report issues on the [issue tracker](https://github.com/dioxuslabs/docsite/issues)
- Check out the [contributing guide](https://dioxuslabs.com/learn/0.7/beyond/contributing)
- Join the [Discord](https://discord.gg/XgGxMSkvUM) and ask questions

<a href="https://github.com/dioxuslabs/docsite/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=dioxuslabs/docsite&max=30&columns=10" />
</a>
