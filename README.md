# dioxuslabs.com

This repository contains the source code for the https://dioxuslabs.com website. 

This website is written with Dioxus, pre-generated with `dioxus_ssr`, and then rehydrated with interactivity provided by `dioxus_web`.

## Development
The documentation can be edited using any text editor. Most commonly used editors support syntax highlighting for the `markdown` format. To view your changes you can install the [dioxus-cli] tool locally, assuming you already have a working `Rust` setup;
```console
> cargo install dioxus-cli
```

With [dioxus] installed you can use it to build and serve the documentation on your local system;
```console
> dixous develop --example spa
```
this will start a local server that will be available on [localhost](http://localhost:3000) and will automatically build and re-build the documentation when it changes.

## Contributing
Want to help out? Check out our [The "Contributing" document][contributing]

### Conduct

The Tide project adheres to the [Contributor Covenant Code of
Conduct](https://github.com/http-rs/tide/blob/main/.github/CODE_OF_CONDUCT.md).
This describes the minimum behavior expected from all contributors.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[Dioxus Book]: https://dioxuslabs.com/book
[mdbook]: https://rust-lang.github.io/mdBook/
[contributing]: https://github.com/http-rs/tide/blob/main/.github/CONTRIBUTING.md

