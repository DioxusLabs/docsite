# Contributing

Development happens in the [Dioxus GitHub repository](https://github.com/DioxusLabs/dioxus). If you've found a bug or have an idea for a feature, please submit an issue (but first check if someone hasn't [done it already](https://github.com/DioxusLabs/dioxus/issues)).

[GitHub discussions](https://github.com/DioxusLabs/dioxus/discussions) can be used as a place to ask for help or talk about features. You can also join [our Discord channel](https://discord.gg/XgGxMSkvUM) where some development discussion happens.

## Improving Docs

If you'd like to improve the docs, PRs are welcome! Both Rust docs ([source](https://github.com/DioxusLabs/dioxus/tree/master/packages)) and this guide ([source](https://github.com/DioxusLabs/dioxus/tree/master/docs/guide)) can be found in the GitHub repo.

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
cargo fmt --all
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

<!-- Playwright doesn't work on my PC despite using the exact same commands as in playwright.yml.
Perhaps someone can enlighten me as to why?
- Test with Playwright. This tests the UI itself, right in a browser. Here are all steps, including installation:

```sh
cd playwright-tests # Make sure you navigate to this directory first!
npm ci
npm install -D @playwright/test
npx playwright install --with-deps
npx playwright test
```
--?

- Test unsafe crates with [MIRI](https://github.com/rust-lang/miri). Currently, this is used for the two MIRI tests in `dioxus-core` and `dioxus-native-core`:

```sh
cargo miri test --package dioxus-core --test miri_stress
cargo miri test --package dioxus-native-core --test miri_native
```
