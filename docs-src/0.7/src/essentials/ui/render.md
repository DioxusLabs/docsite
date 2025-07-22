# How Components Render

We've covered extensively how components and their properties are defined, but we haven't yet covered how they actually work. Dioxus components are *not* normal Rust functions. While technically possible to call them like regular Rust functions, components rely on an *active* Dioxus runtime to function properly.

To use components properly, it's important to understand the fundamentals of how state flows, how elements are created, and how state is stored. We are going to outline how state works here, but state can be complex so we've given it its own [chapter](../state/index.md).


## Components Render

In Dioxus, the term "rendering" refers to the process that Dioxus uses to call your component functions and draw elements to the screen. When you call `dioxus::launch`, Dioxus sets up the app's runtime and then calls the provided initial component to create the initial `Element`. This element declares styles, layouts, children, and event listeners. Dioxus converts your elements into draw calls and converts your event listeners into native event handlers.

[Component Loop](/assets/07/render-calls.png)

Because Dioxus uses a "virtual" tree, the elements in your RSX tree are not actual handles to "real" nodes in the renderer. For example, the Dioxus `Element` type is not a full [HTMLElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement) object. When Dioxus receives your initial Element, it converts your virtual elements into real elements and draw calls using a platform-specific renderer.

## Components Rerender

Components will be rerun when the state they depend on changes. After the initial Element has been drawn with the platform-specific renderer, Dioxus listens for events on your elements. When an event is received, the corresponding event listeners are called, and your code has an opportunity to mutate state. Mutating state is the primary mechanism by which Dioxus knows to run your component functions again and look for changes in the tree.

[Component Loop](/assets/07/render-loop.png)

Dioxus considers state to have been changed in two secnarious:

- The component's properties change, as determined its `PartialEq` implementation
- Internal [state](../state/index.md) the component depends on changes (e.g. `signal.write()`) and an "update" has been scheduled

```rust, no_run
{{#include ../docs-router/src/doc_examples/components.rs:Button}}
```

```inject-dioxus
DemoFrame {
    components::ButtonDemo {}
}
```

After a component runs again, Dioxus will compare the old `Element` and the new `Element` to look for changes. The Dioxus runtime will identify the least amount of draw calls required to change the old UI to match your desired UI. This comparison process is called "diffing". Dioxus optimizes diffing by only comparing dynamic parts of the RSX, so static elements are not checked for changes (see this [blog post](https://dioxuslabs.com/blog/templates-diffing) for details). This entire loop - Render, Display, Listen, Mutate - is called "reconciliation" and Dioxus has one of the most performant implementations of any UI framework.

## Components as a function of state

Components are a pure function of your current application state in the form `fn(State) -> Element`. They read state from various sources like props, [hooks](../state/hooks.md), or [context](../state/context.md) and return a view of the current UI as an `Element`.

We have already seen how components map the props state into UI, but state can also come from the component itself in the form of hooks. For example, we can use a signal to keep track of a count in our component. The component defines the mapping from the current state of the signal to the UI that should be rendered:

```rust, no_run
{{#include ../docs-router/src/doc_examples/components.rs:MyStatefulComponent}}
```

```inject-dioxus
DemoFrame {
    components::MyStatefulComponent {}
}
```

When building Dioxus apps, it's important to understand that your UI code is a declaration of what you want your UI to be - it does not contain any logic on how to update the UI to get there. Dioxus itself is responsible for making the UI match your desired input.

## Components are Pure Functions

The body of a component must be pure. Pure functions always return the same output for the same input and do not have side effects. For example, this double function is pure:

```rust, no_run
{{#include ../docs-router/src/doc_examples/components.rs:double}}
```

If you call `double(2)`, it will always return `4`.

However, this function is not pure because it modifies external state:

```rust, no_run
{{#include ../docs-router/src/doc_examples/components.rs:increment_global_count}}
```

When you call `increment_global_count()` the first time, it will return `0`, but the next time you call it, it will return `1`. This function has side effects because it modifies the global state.

In addition to global variables, context and hooks are also external state in components. Components shouldn't modify state from context or hooks while rendering:

```rust, no_run
{{#include ../docs-router/src/doc_examples/components.rs:MyImpureComponent}}
```

Side effects that modify state should be placed in event handlers or [effects](../breaking/index.md#synchronizing-dom-updates-with-use_effect) which run after the component has rendered. This ensures that the component's output is stable and predictable.

```rust, no_run
{{#include ../docs-router/src/doc_examples/components.rs:MyPureComponent}}
```

If you find yourself writing components that *are not* pure, then you are likely misusing or misunderstanding the reactive paradigm. Mutations should be placed in event handlers as a response to user input or in long running async tasks as a response to background processing.

## Similar to React

If you're familiar with libraries like ReactJS, then this paradigm is familar to you. Dioxus borrows many ideas from React and your existing knowledge will be extremely helpful. If anything here is confusing to you, check out the [React docs](https://react.dev/learn) or do some extra research on React's reactivity system.
