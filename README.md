# `use_mdbook` - hooks and components for loading mdbooks with Dioxus

This crate provides the `use_mdbook` hook and `include_mdbook` macro that allows access to the contents of MdBooks at compile time.

This crate will integrate with a future Dioxus Assets system that allows image bundling outside the final output binary.

The point of this project is to power the Dioxus MdBook component ecosystem which enables any Dioxus app to easily include and render an MdBook.

Planned features for this crate:
- MdBook components (search, navbars, renderers)
- Hotreloading for mdbooks
- Devtool integration for live mdbook editing


## Todo:

- incremental processing with invalidation
- search manifest generation
- integration with dioxus bundle
- extract all logic to a generic asset system
- write mdbook as static str and not require json bouncing
- investigate compile time performance impacts

