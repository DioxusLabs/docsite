# dioxuslabs.com

This repository contains the source code for the https://dioxuslabs.com website.

This website is written with Dioxus, pre-generated with `dioxus_ssr`, and then
rehydrated with interactivity provided by `dioxus_web`.

## Development

The documentation can be edited using any text editor. Most commonly used
editors support syntax highlighting for the `markdown` format. To view your
changes you can install the [`dx`][dx] tool locally, assuming you already have a
working `Rust` setup:

<!-- todo: switch to the installer -->
<!-- # curl -fsSL https://raw.githubusercontent.com/DioxusLabs/dioxus/refs/heads/main/.github/install.sh | bash -->
```sh
cargo binstall dioxus-cli@0.7.0-alpha.3 --force
```

With [`dx`][dx] installed, you can use it to build and serve the documentation
on your local system:

```sh
dx serve --package dioxus_docs_site
```

This will start a local server that will be available on
[localhost:8080](localhost:8080) and will automatically build and re-build the
documentation when it changes.

We use TailwindCSS which is included automatically with dx 0.7.

## Contributing

- Check out the website [section on contributing]
- Report issues on our [issue tracker]
- Join the discord and ask questions!

<a href="https://github.com/dioxuslabs/docsite/graphs/contributors">
  <img
    src="https://contrib.rocks/image?repo=dioxuslabs/docsite&max=30&columns=10"
  />
</a>

[dx]: https://github.com/DioxusLabs/dioxus/tree/main/packages/cli
[section on contributing]: https://dioxuslabs.com/learn/0.6/contributing
[issue tracker]: https://github.com/dioxuslabs/docsite/issues
