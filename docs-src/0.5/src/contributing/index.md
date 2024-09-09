# Contributing

Development happens in the [Dioxus GitHub repository](https://github.com/DioxusLabs/dioxus). If you've found a bug or have an idea for a feature, please submit an issue (but first check if someone hasn't [done it already](https://github.com/DioxusLabs/dioxus/issues)).

[GitHub discussions](https://github.com/DioxusLabs/dioxus/discussions) can be used as a place to ask for help or talk about features. You can also join [our Discord channel](https://discord.gg/XgGxMSkvUM) where some development discussion happens.

## Improving Docs

If you'd like to improve the docs, PRs are welcome! The Rust docs ([source](https://github.com/DioxusLabs/dioxus/tree/main/packages)) and this guide ([source](https://github.com/DioxusLabs/docsite/tree/main/docs-src/0.5/en)) can be found in their respective GitHub repos.

## Working on the Ecosystem

Part of what makes React great is the rich ecosystem. We'd like the same for Dioxus! So if you have a library in mind that you'd like to write and many people would benefit from, it will be appreciated. You can [browse npm.js](https://www.npmjs.com/search?q=keywords:react-component) for inspiration. Once you are done, add your library to the [awesome dioxus](https://github.com/DioxusLabs/awesome-dioxus) list or share it in the `#I-made-a-thing` channel on [Discord](https://discord.gg/XgGxMSkvUM).

## Bugs & Features

If you've fixed [an open issue](https://github.com/DioxusLabs/dioxus/issues), feel free to submit a PR! You can also take a look at [the roadmap](./roadmap.md) and work on something in there. Consider [reaching out](https://discord.gg/XgGxMSkvUM) to the team first to make sure everyone's on the same page, and you don't do useless work!

All pull requests (including those made by a team member) must be approved by at least one other team member.
Larger, more nuanced decisions about design, architecture, breaking changes, trade-offs, etc. are made by team consensus.

## Before you contribute

You might be surprised that a lot of checks fail when making your first PR.
That's why you should first run these commands before contributing, and it will save you *lots* of time, because the
GitHub CI is much slower at executing all of these than your PC.

- Format code with [rustfmt](https://github.com/rust-lang/rustfmt):

```sh
cargo fmt -- src/**/**.rs
```

- You might need to install some packages on Linux (Ubuntu/deb) before the following commands will complete successfully (there is also a Nix flake in the repo root):

```sh
sudo apt install libgdk3.0-cil libatk1.0-dev libcairo2-dev libpango1.0-dev libgdk-pixbuf2.0-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev libwebkit2gtk-4.1-dev
```

- Check all code [cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html):

```sh
cargo check --workspace --examples --tests
```

- Check if [Clippy](https://doc.rust-lang.org/clippy/) generates any warnings. Please fix these!

```sh
cargo clippy --workspace --examples --tests -- -D warnings
```

- Test all code with [cargo-test](https://doc.rust-lang.org/cargo/commands/cargo-test.html):

```sh
cargo test --all --tests
```

- More tests, this time with [cargo-make](https://sagiegurari.github.io/cargo-make/). Here are all steps, including installation:

```sh
cargo install --force cargo-make
cargo make tests
```

- Test with Playwright. This tests the UI itself, right in a browser. Here are all steps, including installation:
  **Disclaimer: This might inexplicably fail on your machine without it being your fault.** Make that PR anyway!

```sh
cd playwright-tests
npm ci
npm install -D @playwright/test
npx playwright install --with-deps
npx playwright test
```

## How to test dioxus with local crate
If you are developing a feature, you should test it in your local setup before raising a PR. This process makes sure you are aware of your code functionality before being reviewed by peers.

- Fork the following github repo (DioxusLabs/dioxus):

`https://github.com/DioxusLabs/dioxus`

- Create a new or use an existing rust crate (ignore this step if you will use an existing rust crate):
This is where we will be testing the features of the forked

```sh
cargo new --bin demo
```

- Add the dioxus dependency to your rust crate (new/existing) in Cargo.toml:

```toml
dioxus = { path = "<path to forked dioxus project>/dioxus/packages/dioxus", features = ["web", "router"] }
```

This above example is for dioxus-web, with dioxus-router. To know about the dependencies for different renderer visit [here](https://dioxuslabs.com/learn/0.5/getting_started).

- Run and test your feature

```sh
dx serve
```

If this is your first time with dioxus, please read [the guide](https://dioxuslabs.com/learn/0.5/guide) to get familiar with dioxus.
