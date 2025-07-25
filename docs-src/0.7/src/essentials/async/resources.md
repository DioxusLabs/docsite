# Asynchronous State with `use_resource`

The [`use_resource`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_resource.html) can be used to derive asynchronous state. It takes an async closure to calculate the state and returns a tracked value with the current state of the future. Any time a dependency of the resource changes, the resource will rerun:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:use_resource}}
```

```inject-dioxus
DemoFrame {
    asynchronous::UseResource {}
}
```

The `use_resource` hook might look similar to the `use_memo` hook, but unlike `use_memo`, the resource's output is not memoized with `PartialEq`. That means any components/reactive hooks that read the output will rerun if the future reruns even if the value it returns is the same:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:use_resource_memo}}
```

```inject-dioxus
DemoFrame {
    asynchronous::UseResourceDemo {}
}
```

> Note: The future you pass to `use_resource` must be cancel safe. Cancel safe futures are futures that can be stopped at any await point without causing causing issues. For example, this task is not cancel safe:
>
> ```rust
> {{#include ../docs-router/src/doc_examples/asynchronous.rs:not_cancel_safe}}
> ```
>
>
> ```inject-dioxus
> DemoFrame {
>     asynchronous::NotCancelSafe {}
> }
> ```
>
> It can be fixed by making sure the global state is restored when the future is dropped:
> ```rust
> {{#include ../docs-router/src/doc_examples/asynchronous.rs:cancel_safe}}
> ```
>
> ```inject-dioxus
> DemoFrame {
>     asynchronous::CancelSafe {}
> }
> ```
>
> Async methods will often mention if they are cancel safe in their documentation.
