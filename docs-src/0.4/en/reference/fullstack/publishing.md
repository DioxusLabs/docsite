## Publishing with Github Pages

You can use Dioxus fullstack to pre-render your application and then hydrate it on the client. This works well with pages hosted statically on providers like Github Pages. In fact, the official Dioxus site uses this approach.

You can set up your application to statically generate all static pages:

```rust
{{#include src/doc_examples/fullstack_static.rs}}
```

Next, edit your `Dioxus.toml` to point your `out_dir` to the `docs` folder and the `base_path` to the name of your repo:

```toml
[application]
# ...
out_dir = "docs"

[web.app]
base_path = "your_repo"
```

Then build your app and publish it to Github:

- Make sure GitHub Pages is set up for your repo to publish any static files in the docs directory
- Build your app with:
```sh
dx build --release --features web
cargo run --features ssr
```
- Add and commit with git
- Push to GitHub
