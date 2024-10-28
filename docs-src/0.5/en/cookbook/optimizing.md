# Optimizing

*Note: This is written primarily for the web, but the main optimizations will work on other platforms too.*

You might have noticed that Dioxus binaries are pretty big.
The WASM binary of a [TodoMVC app](https://github.com/tigerros/dioxus-todo-app) weighs in at 2.36mb!
Don't worry; we can get it down to a much more manageable 234kb.
This will get obviously lower over time.
With nightly features, you can even reduce the binary size of a hello world app to less than 100kb!

We will also discuss ways to optimize your app for increased speed.

However, certain optimizations will sacrifice speed for decreased binary size or the other way around.
That's what you need to figure out yourself. Does your app perform performance-intensive tasks, such as graphical processing or tons of DOM manipulations?
You could go for increased speed. In most cases, though, decreased binary size is the better choice, especially because Dioxus WASM binaries are quite large.

To test binary sizes, we will use [this](https://github.com/tigerros/dioxus-todo-app) repository as a sample app.
The `no-optimizations` package will serve as the base, which weighs 2.36mb as of right now.

Additional resources:
- [WASM book - Shrinking `.wasm` code size](https://rustwasm.github.io/docs/book/reference/code-size.html)
- [min-sized-rust](https://github.com/johnthagen/min-sized-rust)

## Building in release mode

This is the best way to optimize. In fact, the 2.36mb figure at the start of the guide is with release mode.
In debug mode, it's actually a whopping 32mb! It also increases the speed of your app.

Thankfully, no matter what tool you're using to build your app, it will probably have a `--release` flag to do this.

Using the [Dioxus CLI](https://dioxuslabs.com/learn/0.5/CLI) or [Trunk](https://trunkrs.dev/):
- Dioxus CLI: `dx build --release`
- Trunk: `trunk build --release`

## UPX

If you're not targeting web, you can use the [UPX](https://github.com/upx/upx) CLI tool to compress your executables.

Setup:

- Download a [release](https://github.com/upx/upx/releases) and extract the directory inside to a sensible location.
- Add the executable located in the directory to your path variable.

You can run `upx --help` to get the CLI options, but you should also view `upx-doc.html` for more detailed information.
It's included in the extracted directory.

An example command might be: `upx --best -o target/release/compressed.exe target/release/your-executable.exe`.

## Build configuration

*Note: Settings defined in `.cargo/config.toml` will override settings in `Cargo.toml`.*

Other than the `--release` flag, this is the easiest way to optimize your projects, and also the most effective way,
at least in terms of reducing binary size.

### Stable

This configuration is 100% stable and decreases the binary size from 2.36mb to 310kb.
Add this to your `.cargo/config.toml`:

```toml
[profile.release]
opt-level = "z"
debug = false
lto = true
codegen-units = 1
panic = "abort"
strip = true
incremental = false
```

Links to the documentation of each value:
- [`opt-level`](https://doc.rust-lang.org/rustc/codegen-options/index.html#opt-level)
- [`debug`](https://doc.rust-lang.org/rustc/codegen-options/index.html#debuginfo)
- [`lto`](https://doc.rust-lang.org/rustc/codegen-options/index.html#lto)
- [`codegen-units`](https://doc.rust-lang.org/rustc/codegen-options/index.html#codegen-units)
- [`panic`](https://doc.rust-lang.org/rustc/codegen-options/index.html#panic)
- [`strip`](https://doc.rust-lang.org/rustc/codegen-options/index.html#strip)
- [`incremental`](https://doc.rust-lang.org/rustc/codegen-options/index.html#incremental)

### Unstable

This configuration contains some unstable features, but it should work just fine.
It decreases the binary size from 310kb to 234kb.
Add this to your `.cargo/config.toml`:

```toml
[unstable]
build-std = ["std", "panic_abort", "core", "alloc"]
build-std-features = ["panic_immediate_abort"]

[build]
rustflags = [
    "-Clto",
    "-Zvirtual-function-elimination",
    "-Zlocation-detail=none"
]

# Same as in the Stable section
[profile.release]
opt-level = "z"
debug = false
lto = true
codegen-units = 1
panic = "abort"
strip = true
incremental = false
```

*Note: The omitted space in each flag (e.g., `-C<no space here>lto`) is intentional. It is not a typo.*

The values in `[profile.release]` are documented in the [Stable](#stable) section. Links to the documentation of each value:
- [`[build.rustflags]`](https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags)
- [`-C lto`](https://doc.rust-lang.org/rustc/codegen-options/index.html#lto)
- [`-Z virtual-function-elimination`](https://doc.rust-lang.org/stable/unstable-book/compiler-flags/virtual-function-elimination.html)
- [`-Z location-detail`](https://doc.rust-lang.org/stable/unstable-book/compiler-flags/location-detail.html)

## wasm-opt

*Note: In the future, `wasm-opt` will be supported natively through the [Dioxus CLI](https://crates.io/crates/dioxus-cli).*

`wasm-opt` is a tool from the [binaryen](https://github.com/WebAssembly/binaryen) library that optimizes your WASM files.
To use it, install a [binaryen release](https://github.com/WebAssembly/binaryen/releases) and run this command from the package directory:

```
wasm-opt dist/assets/dioxus/APP_NAME_bg.wasm -o dist/assets/dioxus/APP_NAME_bg.wasm -Oz
```

The `-Oz` flag specifies that `wasm-opt` should optimize for size. For speed, use `-O4`.

## Improving Dioxus code

Let's talk about how you can improve your Dioxus code to be more performant.

It's important to minimize the number of dynamic parts in your `rsx`, like conditional rendering.
When Dioxus is rendering your component, it will skip parts that are the same as the last render.
That means that if you keep dynamic rendering to a minimum, your app will speed up, and quite a bit if it's not just hello world.
To see an example of this, check out [Dynamic Rendering](../reference/dynamic_rendering.md).

Also check out [Anti-patterns](antipatterns.md) for patterns that you should avoid.
Obviously, not all of them are just about performance, but some of them are.

## Optimizing the size of assets

Assets can be a significant part of your app's size. Dioxus includes alpha support for first party [assets](../reference/assets.md). Any assets you include with the `mg!` macro will be optimized for production in release builds.
