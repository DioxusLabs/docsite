# Moving Around State

As you create signals and derived state in your app, you will need to move around that state between components. Dioxus provides three different ways to pass around state:

## Passing props

You can pass your values through component [props](./component_props.md). This should be your default when passing around state. It is the most explicit and local to your component. Use this until it gets annoying to pass around the value:

```rust
{{#include src/doc_examples/moving_state_around.rs:PassingProps}}
```

```inject-dioxus
DemoFrame {
    moving_state_around::PassingProps {}
}
```


## Passing context

If you need a slightly more powerful way to pass around state, you can use the context API.

The context API lets you pass state from a parent component to all children. This is useful if you want to share state between many components. You can insert a unique type into the context with the [`use_context_provider`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context_provider.html) hook in the parent component. Then you can access the context in any child component with the [`use_context`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context.html) hook.

```rust
{{#include src/doc_examples/moving_state_around.rs:PassingContext}}
```

```inject-dioxus
DemoFrame {
    moving_state_around::PassingContext {}
}
```

This is slightly less explicit than passing it as a prop, but it is still local to the component. This is really great if you want state that is global to part of your app. It lets you create multiple global-ish states while still making state different when you reuse components. If I create a new `ParentComponent`, it will have a new `MyState`.

## Using globals

Finally, if you have truly global state, you can put your state in a `Global<T>` static. This is useful if you want to share state with your whole app:

```rust
{{#include src/doc_examples/moving_state_around.rs:UsingGlobals}}
```

```inject-dioxus
DemoFrame {
    moving_state_around::UsingGlobals {}
}
```

Global state can be very ergonomic if your state is truly global, but you shouldn't use it if you need state to be different for different instances of your component. If I create another `IncrementButton` it will use the same `COUNT`. Libraries should generally avoid this to make components more reusable.

> Note: Even though it is in a static, `COUNT` will be different for each app instance so you don't need to worry about state mangling when multiple instances of your app are running on the server
