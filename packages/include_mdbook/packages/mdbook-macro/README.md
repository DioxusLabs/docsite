# mdbook-macro

Compiles mdbooks with proc-macros and exposes their internals with TYPE SAFETY.

Whaaaaaaaat???

Crazy.

```rust
mdbook_router! {"../example-book"}
```

Integrates with the dioxus-router to create a router from a mdbook.


## Todo:

- Incrementally recompile the mdbook as its contents change
- Integrate with the `hot-reload` crate to allow live-editable dioxus websites that include mdbooks.
