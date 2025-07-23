# Routing

You can easily integrate your fullstack application with a client side router using the `launch_router` macro. The `launch_router` macro works the same as the `launch` macro except it accepts a Router instead of a Component:

```rust
{{#include ../docs-router/src/doc_examples/untested_04/server_router.rs}}
```

```inject-dioxus
SandBoxFrame {
	url: "https://codesandbox.io/p/sandbox/dioxus-fullstack-router-s75v5q?file=%2Fsrc%2Fmain.rs%3A7%2C1"
}
```
