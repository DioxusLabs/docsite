### Design
1. User submits code
2. The code is added to a compilation queue
3. The server compiles the code one at a time with the same cached dependencies. (dioxus)
4. The server stores the compiled wasm and provides a temporary url to the page.
5. The server deletes the wasm after a duration from being built.


#### Usage
You must include a `/public/ace` folder in your project with the files found in this repo at `/playground/public/ace`

Additionally, add the following to your `[web.resource]` section of `Dioxus.toml`:
```toml
script = ["/ace/ace.js", "/ace/mode-rust.js", "/ace/theme-github.js", "/ace/theme-github_dark.js"]
```

`dx-debian` is the dx cli version 0.5.1 with [--raw-out](https://github.com/DogeDark/dioxus/tree/cli-raw-out) support.