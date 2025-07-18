# Components

Components are used to define a reusable piece of your UI. Each component will be called repeatedly during the rendering process, and can be used to encapsulate presentation, state, and interaction. This guide will cover how to think about components, how to define them, and how to use them in your Dioxus application.

## Defining Components

You can create a component by creating a function with the `#[component]`. Your function must meet the following requirements:

- Either start with a capital letter (`MyComponent`) or contains an underscore (`my_component`). This restriction both helps with readability and is used to differentiate between elements and components in the rsx macro
- Take arguments that implement [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) and [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
- Return an [`Element`](https://docs.rs/dioxus/latest/dioxus/prelude/type.Element.html)

We call the arguments a component takes its **properties**. Properties are used to pass data into the component, similar to how you would pass arguments to a function.

For example, we can define a simple component that takes a `name` property and renders a greeting:

```rust, no_run
{{#include ../docs-router/src/doc_examples/components.rs:MyComponent}}
```
```inject-dioxus
DemoFrame {
	components::MyComponent { name: "world" }
}
```

> Properties can be modified to accept a wider variety of inputs than a normal function argument. We don't cover all of the details here, but you can find information in the [component macro](https://docs.rs/dioxus/latest/dioxus/prelude/attr.component.html) documentation.
>
> You can use the `#[props()]` attribute on each function argument to modify properties your component accepts:
>
> - [`#[props(default)]`](https://docs.rs/dioxus/latest/dioxus/prelude/attr.component.html#default-props) - Makes the field optional in the component and uses the default value if it is not set when creating the component.
> - [`#[props(!optional)]`](https://docs.rs/dioxus/latest/dioxus/prelude/attr.component.html#optional-props) - Makes a field with the type `Option<T>` required.
> - [`#[props(into)]`](https://docs.rs/dioxus/latest/dioxus/prelude/attr.component.html#converting-props) - Converts a field into the correct type by using the [`Into`] trait.
> - [`#[props(extends = GlobalAttributes)]`](https://docs.rs/dioxus/latest/dioxus/prelude/attr.component.html#extending-elements) - Extends the props with all the attributes from an element or the global element attributes.
>
> Props also act slightly differently when used with:
>
> - [`Option<T>`](https://docs.rs/dioxus/latest/dioxus/prelude/attr.component.html#optional-props) - The field is automatically optional with a default value of `None`.
> - [`ReadOnlySignal<T>`](https://docs.rs/dioxus/latest/dioxus/prelude/attr.component.html#reactive-props) - The props macro will automatically convert `T` into `ReadOnlySignal<T>` when it is passed as a prop.
> - [`String`](https://docs.rs/dioxus/latest/dioxus/prelude/attr.component.html#formatted-props) - The props macro will accept formatted strings for any prop field with the type `String`.
> - [`children`](https://docs.rs/dioxus/latest/dioxus/prelude/attr.component.html#children-props) - The props macro will accept child elements if you include the `children` prop.

## Using Components in RSX

Once you have defined a component, you can use it in your RSX markup just like any other element:

```rust, no_run
{{#include ../docs-router/src/doc_examples/components.rs:MyComponentCall}}
```
```inject-dioxus
DemoFrame {
	components::MyComponentCall {}
}
```

> Although components are defined as functions, they should not be called like
> regular functions. When you use a component in RSX, Dioxus will create
> a new instance of the component that controls its own state and lifecycle.

## Components as a function of state

Components are a pure function of your current application state in the form `fn(State) -> Element`. They read state from various sources like props, [hooks](../hooks/index.md), or [context](../state/index.md#passing-context) and return a view of the current UI as an `Element`.

We have already seen how components map the props state into UI, but state can also come from the component itself in the form of hooks. For example, we can use a signal to keep track of a count in our component. The component defines the mapping from the current state of the signal to the UI that should be rendered:

```rust, no_run
{{#include ../docs-router/src/doc_examples/components.rs:MyStatefulComponent}}
```

```inject-dioxus
DemoFrame {
    components::MyStatefulComponent {}
}
```

### Components are Pure Functions

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

## Components Rerender

Components will be rerun when the state they depend on changes. This can happen in two main scenarios:

- The component's properties change, as determined by `PartialEq`
- Internal [state](../state/index.md) the component depends on changes (e.g. `signal.write()`)

```rust, no_run
{{#include ../docs-router/src/doc_examples/components.rs:Button}}
```

```inject-dioxus
DemoFrame {
    components::ButtonDemo {}
}
```

After a component reruns, dioxus will compare the old and new `Element`s to look for changes. If will only update
the parts of the DOM that have changed, which makes rerenders very efficient. This comparison process is called "diffing".
Dioxus optimizes diffing by only comparing dynamic parts of the RSX, so static elements are not checked for changes (see
this [blog post](https://dioxuslabs.com/blog/templates-diffing) for details).

## Summary

- Components can be used to break down your application into smaller, reusable pieces.
- Components are defined as functions that take properties and return an `Element`.
- Components must be pure which means:
  - They should return the same output (UI) for the same input (application state).
  - They should not modify application state or have side effects while rendering.
- Components will rerun when the application state they rely on changes, but only rerender the parts of the DOM that have changed.
