# dioxuslabs.com

This repository contains the source code for the https://dioxuslabs.com website. 

This website is written with Dioxus, pre-generated with `dioxus_ssr`, and then rehydrated with interactivity provided by `dioxus_web`.

## Development

Run the following command in the root of the project to start the tailwind css compiler:

```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

The documentation can be edited using any text editor. Most commonly used editors support syntax highlighting for the `markdown` format. To view your changes you can install the [dioxus-cli] tool locally, assuming you already have a working `Rust` setup;
```console
cargo install dioxus-cli
```

With [dioxus] installed, you can use it to build and serve the documentation on your local system;
```console
dx serve --example spa --features web
```

this will start a local server that will be available on [localhost](http://localhost:8080) and will automatically build and re-build the documentation when it changes.

## Contributing
- Check out the website [section on contributing](https://dioxuslabs.com/learn/0.4/contributing).
- Report issues on our [issue tracker](https://github.com/dioxuslabs/docsite/issues).
- Join the discord and ask questions!


<a href="https://github.com/dioxuslabs/docsite/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=dioxuslabs/docsite&max=30&columns=10" />
</a>
