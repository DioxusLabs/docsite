# Breaking Out of Dioxus

Dioxus makes it easy to build reactive user interfaces. However, there are some cases where you may need to break out of the reactive paradigm to interact with the DOM directly.

## Interacting with JavaScript with `eval` and `web-sys`

Dioxus exposes a limited number of [web apis](https://developer.mozilla.org/en-US/docs/Web/API) with a nicer interface. If you need access to more APIs, you can use the `eval` function to run JavaScript in the browser.


For example, you can use the eval function to read the domain of the current page:

```rust, no_run
{{#include ../docs-router/src/doc_examples/breaking_out.rs:eval}}
```
```inject-dioxus
DemoFrame {
    breaking_out::Eval {}
}
```

If you are only targeting web, you can also use the [`web-sys`](https://crates.io/crates/web-sys) crate for typed access to the web APIs. Here is what reading the domain looks like with web-sys:

```rust, no_run
{{#include ../docs-router/src/doc_examples/breaking_out.rs:web_sys}}
```
```inject-dioxus
DemoFrame {
    breaking_out::WebSys {}
}
```

## Synchronizing DOM updates with `use_effect`

If you do need to interact with the DOM directly, you should do so in a `use_effect` hook. This hook will run after the component is rendered and all of the Dioxus UI has been rendered. You can read or modify the DOM in this hook.


For example, you can use the `use_effect` hook to write to a canvas element after it is created:

```rust, no_run
{{#include ../docs-router/src/doc_examples/breaking_out.rs:use_effect}}
```
```inject-dioxus
DemoFrame {
    breaking_out::Canvas {}
}
```

## Getting access to elements with `onmounted`

If you need a handle to an element that is rendered by dioxus, you can use the `onmounted` event. This event will fire after the element is first mounted to the DOM. It returns a live reference to the element with some methods to interact with it.


You can use the onmounted event to do things like focus or scroll to an element after it is rendered:

```rust, no_run
{{#include ../docs-router/src/doc_examples/breaking_out.rs:onmounted}}
```
```inject-dioxus
DemoFrame {
    breaking_out::OnMounted {}
}
```

## Down casting web sys events

Dioxus provides platform agnostic wrappers over each event type. These wrappers are often nicer to interact with than the raw event types, but they can be more limited. If you are targeting web, you can downcast the event with the `as_web_event` method to get the underlying web-sys event:

```rust, no_run
{{#include ../docs-router/src/doc_examples/breaking_out.rs:downcast}}
```
```inject-dioxus
DemoFrame {
    breaking_out::Downcast {}
}
```

## Externally Managed Nodes

Dioxus uses diffs between templates to determine which elements have changed in the DOM and applies those changes to the existing DOM. A great post about this can be found [here](https://dioxuslabs.com/blog/templates-diffing/). In short, what these means for managing elements outside of Dioxus with Javascript is, one must be careful that the diffing process does not overwite the changes or if so, this is accounted for.

An `rsx` block creates a tree of static and dynamic elements. If the structure of this tree changes then everything underneath it is replaced as well.

```rust, no_run
{{#include ../docs-router/src/doc_examples/breaking_out.rs:template_diffing}}
```
```inject-dioxus
DemoFrame {
    breaking_out::TemplateDiffing {}
}
```