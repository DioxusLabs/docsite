# dioxuslabs.com

This repository contains the source code for the https://dioxuslabs.com website.

This website is written with Dioxus, pre-generated with `dioxus_ssr`, and then
rehydrated with interactivity provided by `dioxus_web`.

## Development

Run the following command in the root of the project to start the Tailwind CSS
compiler:

```bash
npx tailwindcss -i ./tailwind.css -o ./public/tailwind.css --watch
```

The documentation can be edited using any text editor. Most commonly used
editors support syntax highlighting for the `markdown` format. To view your
changes you can install the [`dx`][dx] tool locally, assuming you already have a
working `Rust` setup:

```sh
cargo install dioxus-cli
```

With [`dx`][dx] installed, you can use it to build and serve the documentation
on your local system:

```sh
dx serve --example spa --features web
```

This will start a local server that will be available on
[localhost:8080](localhost:8080) and will automatically build and re-build the
documentation when it changes.

## Contributing

- Check out the website [section on contributing]
- Report issues on our [issue tracker]
- Join the discord and ask questions!

<a href="https://github.com/dioxuslabs/docsite/graphs/contributors">
  <img
    src="https://contrib.rocks/image?repo=dioxuslabs/docsite&max=30&columns=10"
  />
</a>

[dx]: https://github.com/DioxusLabs/dioxus/tree/master/packages/cli
[section on contributing]: https://dioxuslabs.com/learn/0.4/contributing
[issue tracker]: https://github.com/dioxuslabs/docsite/issues
