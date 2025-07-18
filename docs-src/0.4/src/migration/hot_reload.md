# Hot reloading

Desktop hot reloading has changed in the `0.4` release to use the [Dioxus CLI](../CLI/index.md) for all platforms.

Previously, you may have included the `hot_reload_init!` macro in your main function. This is no longer needed.

old:
```rust
fn main() {
    hot_reload_init!();
    // ...
}
```

new:
```rust
fn main() {
    // ...
}
```

Now you can run your project with the dioxus CLI by passing the `--platform` flag:

```sh
dx serve --platform desktop --hot-reload
```
