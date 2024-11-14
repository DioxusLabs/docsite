# Resource

[`use_resource`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_resource.html) lets you run an async closure, and provides you with its result.

For example, we can make an API request (using [reqwest](https://docs.rs/reqwest/latest/reqwest/index.html)) inside `use_resource`:

```rust
{{#include src/doc_examples/use_resource.rs:use_resource}}
```

The code inside `use_resource` will be submitted to the Dioxus scheduler once the component has rendered.

We can use `.read()` to get the result of the future. On the first run, since there's no data ready when the component loads, its value will be `None`. However, once the future is finished, the component will be re-rendered and the value will now be `Some(...)`, containing the return value of the closure.

We can then render that result:

```rust
{{#include src/doc_examples/use_resource.rs:render}}
```

```inject-dioxus
DemoFrame {
    use_resource::App {}
}
```

## Restarting the Future

The `Resource` handle provides a `restart` method. It can be used to execute the future again, producing a new value.

## Dependencies

Often, you will need to run the future again every time some value (e.g. a state) changes. Rather than calling `restart` manually, you can read a signal inside of the future. It will automatically re-run the future when any of the states you read inside the future change. Example:

```rust, no_run
{{#include src/doc_examples/use_resource.rs:dependency}}
```
