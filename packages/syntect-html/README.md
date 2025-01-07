# mdbook-macro

Compiles mdbooks with proc-macros and exposes their internals with TYPE SAFETY.

Whaaaaaaaat???

Crazy.

```rust
static DOCS: MdBook = include_mdbook!("docs");

dbg!(DOCS.summary);
```

Will incrementally recompile the mdbook as its contents change.

Integrates with the `use_mdbook` crate to allow live-editable dioxus websites that include mdbooks.


## Todo:

- use static defs instead of serde on the boundary
