# Web

This guide will cover concepts specific to the Dioxus web renderer.

## Running Javascript

Dioxus provides some ergonomic wrappers over the browser API, but in some cases you may need to access parts of the browser API Dioxus does not expose. 


For these cases, Dioxus web exposes the use_eval hook

// TODO: write this once https://github.com/DioxusLabs/dioxus/pull/1080 is merged

If you are targeting web, but don't plan on targeting any other Dioxus renderer you can also use the generated wrappers in the [web-sys](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html) and [gloo](https://gloo-rs.web.app/) crates.
