## Lifting State

One approach to share state between components is to "lift" it up to the nearest common ancestor. This means putting the `use_signal` hook in a parent component, and passing the needed values down as props.

Suppose we want to build a meme editor. We want to have an input to edit the meme caption, but also a preview of the meme with the caption. Logically, the meme and the input are 2 separate components, but they need access to the same state (the current caption).

> Of course, in this simple example, we could write everything in one component – but it is better to split everything out in smaller components to make the code more reusable, maintainable, and performant (this is even more important for larger, complex apps).

We start with a `Meme` component, responsible for rendering a meme with a given caption:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_06/meme_editor.rs:meme_component}}
```

> Note that the `Meme` component is unaware where the caption is coming from – it could be stored in `use_signal`, or a constant. This ensures that it is very reusable – the same component can be used for a meme gallery without any changes!

We also create a caption editor, completely decoupled from the meme. The caption editor must not store the caption itself – otherwise, how will we provide it to the `Meme` component? Instead, it should accept the current caption as a prop, as well as an event handler to delegate input events to:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_06/meme_editor.rs:caption_editor}}
```

Finally, a third component will render the other two as children. It will be responsible for keeping the state and passing down the relevant props.

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_06/meme_editor.rs:meme_editor}}
```

![Meme Editor Screenshot: An old plastic skeleton sitting on a park bench. Caption: "me waiting for a language feature"](/assets/static/meme_editor_screenshot.png)




## Moving Around State

As you create signals and derived state in your app, you will need to move around that state between components. Dioxus provides three different ways to pass around state:

### Passing props

You can pass your values through component [props](../ui/components.md). This should be your default when passing around state. It is the most explicit and local to your component. Use this until it gets annoying to pass around the value:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/moving_state_around.rs:PassingProps}}
```

```inject-dioxus
DemoFrame {
    moving_state_around::PassingProps {}
}
```


### Passing context

If you need a slightly more powerful way to pass around state, you can use the context API.

The context API lets you pass state from a parent component to all children. This is useful if you want to share state between many components. You can insert a unique type into the context with the [`use_context_provider`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context_provider.html) hook in the parent component. Then you can access the context in any child component with the [`use_context`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context.html) hook.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/moving_state_around.rs:PassingContext}}
```

```inject-dioxus
DemoFrame {
    moving_state_around::PassingContext {}
}
```

This is slightly less explicit than passing it as a prop, but it is still local to the component. This is really great if you want state that is global to part of your app. It lets you create multiple global-ish states while still making state different when you reuse components. If I create a new `ParentComponent`, it will have a new `MyState`.

### Using globals

Finally, if you have truly global state, you can put your state in a `Global<T>` static. This is useful if you want to share state with your whole app:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/moving_state_around.rs:UsingGlobals}}
```

```inject-dioxus
DemoFrame {
    moving_state_around::UsingGlobals {}
}
```

Global state can be very ergonomic if your state is truly global, but you shouldn't use it if you need state to be different for different instances of your component. If I create another `IncrementButton` it will use the same `COUNT`. Libraries should generally avoid this to make components more reusable.

> Note: Even though it is in a static, `COUNT` will be different for each app instance so you don't need to worry about state mangling when multiple instances of your app are running on the server
