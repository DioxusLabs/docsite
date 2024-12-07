# Extractors

Server functions are an ergonomic way to call a function on the server. Server function work by registering an endpoint on the server and using requests on the client. Most of the time, you shouldn't need to worry about how server functions operate, but there are some times when you need to get some value from the request other than the data passed in the server function.

For example, requests contain information about the user's browser (called the [user agent](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent)). We can use an extractor to retrieve that information.

You can use the `extract` method within a server function to extract something from the request. You can extract any type that implements `FromServerContext` (or when axum is enabled, you can use axum extractors directly):

```rust
{{#include src/doc_examples/server_function_extract.rs:server_function_extract}}
```
