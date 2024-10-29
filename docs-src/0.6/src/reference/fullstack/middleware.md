# Middleware

Extractors allow you to wrap your server function in some code that changes either the request or the response. Dioxus fullstack integrates with [Tower](https://docs.rs/tower/latest/tower/index.html) to allow you to wrap your server functions in middleware.

You can use the `#[middleware]` attribute to add a layer of middleware to your server function. Let's add a timeout middleware to a server function. This middleware will stop running the server function if it reaches a certain timeout:

```rust
{{#include src/doc_examples/server_function_middleware.rs:server_function_middleware}}
```
