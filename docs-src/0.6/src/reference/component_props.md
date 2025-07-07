# Component Props

Just like you can pass arguments to a function or attributes to an element, you can pass props to a component that customize its behavior! The components we've seen so far didn't accept any props – so let's write some components that do.

## derive(Props)

Component props are a single struct annotated with `#[derive(PartialEq, Clone, Props)]`. For a component to accept props, the type of its argument must be `YourPropsStruct`.

Example:

```rust, no_run
{{#include src/doc_examples/component_owned_props.rs:Likes}}
```

You can then pass prop values to the component the same way you would pass attributes to an element:

```rust, no_run
{{#include src/doc_examples/component_owned_props.rs:App}}
```

```inject-dioxus
DemoFrame {
    component_owned_props::App {}
}
```

## Prop Options

The `#[derive(Props)]` macro has some features that let you customize the behavior of props.

### Optional Props

You can create optional fields by using the `Option<…>` type for a field:

```rust, no_run
{{#include src/doc_examples/component_props_options.rs:OptionalProps}}
```

Then, you can choose to either provide them or not:

```rust, no_run
{{#include src/doc_examples/component_props_options.rs:OptionalProps_usage}}
```

### Explicitly Required Option

If you want to explicitly require an `Option`, and not an optional prop, you can annotate it with `#[props(!optional)]`:

```rust, no_run
{{#include src/doc_examples/component_props_options.rs:ExplicitOption}}
```

Then, you have to explicitly pass either `Some("str")` or `None`:

```rust, no_run
{{#include src/doc_examples/component_props_options.rs:ExplicitOption_usage}}
```

### Default Props

You can use `#[props(default = 42)]` to make a field optional and specify its default value:

```rust, no_run
{{#include src/doc_examples/component_props_options.rs:DefaultComponent}}
```

Then, similarly to optional props, you don't have to provide it:

```rust, no_run
{{#include src/doc_examples/component_props_options.rs:DefaultComponent_usage}}
```

### Automatic Conversion with into

It is common for Rust functions to accept `impl Into<SomeType>` rather than just `SomeType` to support a wider range of parameters. If you want similar functionality with props, you can use `#[props(into)]`. For example, you could add it on a `String` prop – and `&str` will also be automatically accepted, as it can be converted into `String`:

```rust, no_run
{{#include src/doc_examples/component_props_options.rs:IntoComponent}}
```

Then, you can use it so:

```rust, no_run
{{#include src/doc_examples/component_props_options.rs:IntoComponent_usage}}
```

## The component macro

So far, every Component function we've seen had a corresponding ComponentProps struct to pass in props. This was quite verbose... Wouldn't it be nice to have props as simple function arguments? Then we wouldn't need to define a Props struct, and instead of typing `props.whatever`, we could just use `whatever` directly!

`component` allows you to do just that. Instead of typing the "full" version:

```rust, no_run
#[derive(Props, Clone, PartialEq)]
struct TitleCardProps {
    title: String,
}

fn TitleCard(props: TitleCardProps) -> Element {
    rsx!{
        h1 { "{props.title}" }
    }
}
```

...you can define a function that accepts props as arguments. Then, just annotate it with `#[component]`, and the macro will turn it into a regular Component for you:

```rust, no_run
#[component]
fn TitleCard(title: String) -> Element {
    rsx!{
        h1 { "{title}" }
    }
}
```

> While the new Component is shorter and easier to read, this macro should not be used by library authors since you have less control over Prop documentation.

## Component Children

In some cases, you may wish to create a component that acts as a container for some other content, without the component needing to know what that content is. To achieve this, create a prop of type `Element`:

```rust, no_run
{{#include src/doc_examples/component_element_props.rs:Clickable}}
```

Then, when rendering the component, you can pass in the output of `rsx!{...}`:

```rust, no_run
{{#include src/doc_examples/component_element_props.rs:Clickable_usage}}
```

> Warning: While it may compile, do not include the same `Element` more than once in the RSX. The resulting behavior is unspecified.

### The children field

Rather than passing the RSX through a regular prop, you may wish to accept children similarly to how elements can have children. The "magic" `children` prop lets you achieve this:

```rust, no_run
{{#include src/doc_examples/component_children.rs:Clickable}}
```

This makes using the component much simpler: simply put the RSX inside the `{}` brackets – and there is no need for a `render` call or another macro!

```rust, no_run
{{#include src/doc_examples/component_children.rs:Clickable_usage}}
```

```inject-dioxus
DemoFrame {
    component_children::App {}
}
```

## Reactive Props

In Dioxus, props are **the primary mechanism** for enabling communication between components. When passed down, they **trigger re-renders** upon changes, allowing your UI to reflect the latest application state. But what happens when props are used inside reactive systems such as `use_memo`, `use_future`, or `use_resource`? That's where **reactive props** become essential. Without making the prop reactive, those hooks **won't know** they need to update, leading to stale computations and inconsistent interfaces. Let's explore the principles of reactive props and how you can make them **predictable**, **efficient**, and **declarative**.

### Non-Reactive by Default

By default, primitive props like `f64`, `String`, etc., **aren't tracked** by hooks. They are copied in and won't trigger recomputation inside memoized or asynchronous logic.

```rust, no_run
{{#include src/doc_examples/component_reactive_props.rs:Temperature}}
```

This works once. But if `celsius` changes from `30.0` to `32.0`, the UI updates the Celsius label but **not** the Fahrenheit conversion. The memo still uses the **stale value** `30.0`.

### Reactive via `ReadOnlySignal<T>`

Dioxus offers a built-in, elegant solution: the [`ReadOnlySignal<T>`](https://docs.rs/dioxus/latest/dioxus/prelude/struct.ReadOnlySignal.html). It's a lightweight, `Copy`able, `Clone`able, zero-cost abstraction for reactive reads.

```rust, no_run
{{#include src/doc_examples/component_reactive_props.rs:ReactiveTemperature}}
```

The `use_memo` hook now **subscribes** to the signal. Whenever `celsius` changes, the memo is recalculated.

### `use_reactive!` for Precision Control

If you prefer to avoid signals, use [`use_reactive`](https://docs.rs/dioxus/latest/dioxus/prelude/macro.use_reactive.html) macro to **declare dependencies** manually inside hooks:

```rust, no_run
{{#include src/doc_examples/component_reactive_props.rs:UseReactive}}
```

This gives you **deterministic control** over which variables should cause recomputation, useful for deeply nested computations or external state systems.

### Gotchas and Tips

- **Signals are both `Copy`able and `Clone`**able: You can pass `ReadOnlySignal<T>` by value, reference, or clone it. Internally, it's a lightweight handle to a reactive state. This makes it easy to lift signals into other hooks without worrying about ownership issues.

- **You can combine signals with the `use_reactive!` macro**: This is useful when only part of your dependencies are reactive. For example:

  ```rust, no_run
  {{#include src/doc_examples/component_reactive_props.rs:UseReactiveGotcha}}
  ```

- **Avoid using non-reactive props in `use_future` or `use_resource`**: These hooks only re-run when their dependencies change. If those dependencies are non-reactive (like `String` or `f64`), the hook won't re-execute when they update. Example of incorrect usage:

  ```rust, no_run
  {{#include src/doc_examples/component_reactive_props.rs:UseFutureGotcha}}
  ```

  Correct approach using a reactive prop:

  ```rust, no_run
  {{#include src/doc_examples/component_reactive_props.rs:UseFutureCorrect}}
  ```

- **Memoized values from `use_memo` are reactive**: The result of `use_memo` is a value. If you need the result to drive other reactive state, store it in a memo:

  ```rust, no_run
  // ✅ This is a reactive memo that will update when `count` changes
  let doubled = use_memo(move || count() * 2);
  ```

- **`use_signal` vs. `use_memo`**: Use `use_signal` when the value will change due to internal component state or user interaction. Use `use_memo` when deriving a value from other reactive props or signals and you want automatic recomputation.

- **Signal updates are conditional on value changes**: If you call `.set()` on a signal with the same value it already holds, dependent effects and hooks will not re-run. If you need to force an update, consider mutating inner data if you're using something like `Signal<Vec<T>>`.

- **Cloning signals in every render can be inefficient**: Although cloning signals is cheap, avoid doing it repeatedly inside `rsx!` or render loops. Instead, just read the signal:

  ```rust, no_run
  rsx! {
      div { "Value: {count()}" }
  }
  ```

- **Closures can capture stale props if not handled properly**: If you capture a non-reactive prop in a closure inside `use_future` or `use_memo`, it won't reflect updates. Always make sure the closure references a signal or reactive value.

- **Debugging tip**: If a memo or effect isn't updating, log values or use visual indicators in the UI to confirm whether the input values are truly changing. In most cases, this is caused by a missing reactive dependency or an incorrectly captured closure.

Let's consider the following example

```rust, no_run
{{#include src/doc_examples/component_reactive_props.rs:GotchasApp}}
```

```inject-dioxus
DemoFrame {
    component_reactive_props::App {}
}
```

This pattern ensures that **networked async hooks** properly respond to prop changes using signals.

## Extending Elements

Often when building reusable components, you want them to **act like native HTML elements**, passing arbitrary attributes like `style`, `onmouseenter`, or `disabled`. Dioxus allows this through the `#[props(extends = ...)]` attribute. This lets you **extend the props** of your component with either:

- All **global HTML attributes**
- Attributes specific to a particular HTML tag (e.g. `button`, `input`, etc.)

Let's consider the following example:

```rust, no_run
{{#include src/doc_examples/component_extending_elements.rs:ExtendingPanel}}
```

Usage:

```rust, no_run
{{#include src/doc_examples/component_extending_elements.rs:ExtendingPanelUsage}}
```

```inject-dioxus
DemoFrame {
    component_extending_elements::App {}
}
```

No need to manually forward every attribute. Simply spread `..attrs`, and your component becomes fully HTML-compatible.

### Merging Multiple Sources

You can extend multiple attribute domains (e.g. global + tag-specific):

```rust, no_run
{{#include src/doc_examples/component_extending_elements.rs:ExtendingButton}}
```

```rust, no_run
{{#include src/doc_examples/component_extending_elements.rs:ExtendingButtonUsage}}
```

```inject-dioxus
DemoFrame {
    component_extending_elements::ActionApp {}
}
```

Dioxus resolves conflicts internally. However, avoid combining elements with **incompatible attribute semantics**.

### Caveats and Tips

- **Attribute collision resolution**: If multiple values are passed for the same attribute (e.g. `class` appears in both props and `..attrs`), the **last occurrence wins**. This is standard HTML behavior, but it's worth watching out for when merging attributes from different sources.

- **Cannot override named props**: The `extends` system only covers HTML attributes. You **cannot override custom or named props** declared explicitly in the component signature. These must still be handled and destructured manually.

  ```rust, no_run
  #[component]
  fn Card(
      title: String, // explicitly required named prop
      #[props(extends = GlobalAttributes)]
      attrs: Vec<Attribute>,
  ) -> Element {
      rsx! {
          div {
              ..attrs, // forwarded HTML attributes
              h2 { "{title}" }, // uses the explicit prop
              p { "This is a card." }
          }
      }
  }
  ```

- **Avoid extending incompatible element types**: You must **only extend attribute sets that match the tag you're rendering**. Extending `input` attributes on a `div` may compile, but attributes like `type` or `value` will be meaningless or even silently dropped in the browser.

- **Prop filtering is manual**: If you want to restrict which attributes are allowed or filter out certain ones (e.g., to enforce accessibility or prevent accidental overrides), you must do this manually inside your component logic.

- **Attribute order matters in practice**: While RSX generally follows a "last wins" policy, if you're conditionally overriding attributes (like `class`), you may want to control placement in the tree:

  ```rust, no_run
  rsx! {
      div {
          class: "default",
          ..attrs, // if attrs contains a 'class', it will override "default"
      }
  }
  ```

- **Spreading is shallow**: `..attrs` simply forwards attribute key/value pairs. If you need to merge structured data (e.g., two `style` maps), you must handle it manually.

- **Debugging tip**: When using `..attrs`, it can be hard to trace where an unexpected attribute came from. Consider logging or printing props during development to confirm which attributes were forwarded.
