
# Attributes

While we could build a UI by assembling plain elements and text together, we typically want to customize their appearance and behavior. This is where we use Element Attributes. Attributes provide extra information to the renderer on how to display the UI.

You can specify attributes by adding the name of the attribute, a colon, and then the value to the body of an element. For example, we might want to style a div with a particular [class](https://www.w3schools.com/html/html_classes.asp) and [ID](https://www.w3schools.com/html/html_id.asp), which we add as attributes:

```rust
rsx! {
    div {
        class: "container",
        id: "root-container"
    }
}
```

We can use an attribute to set the `type` of an input element. The default type is `text` which shows a text input box, but we can set it to `number` to only accept numbers:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:input_type}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::InputType {}
}
```

Just like text nodes, attributes can include formatted segments. We can set the value of the input element to a signal to control it:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:input_value}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::InputValue {}
}
```

## Attribute Scope

Every element has two sets of attributes:

- [Global Attributes](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes): attributes which can be applied to *every* element
- [Specific Attributes](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes): attributes that only apply to one specific element

For example, all elements support the `id` and `class` attributes, but only `video` elements support the `autoplay` attribute. For a full list of attributes, visit the relevant links above.

## IDE Support

RSX provides autocomplete and inline docs support for elements and their attributes. To get autocomplete suggestions, simply start typing in your editor:

![Autocomplete](/assets/07/ide_autocomplete.png)

We've documented every element with documentation pulled from the [Mozilla Developer Docs](https://developer.mozilla.org/en-US/docs/Web). Simply hover over the element with your cursor for more information:

![Element Hover](/assets/07/ide_hover.png)

The same docs apply for attributes as well:

![Element Hover](/assets/07/ide_listener.png)

## Non-Text Attributes

Typically, you'll add attributes to your elements using formatted text. However, attributes can accept a wide range of value types, including:

- `Text`: Formatted text, the `String` type, or anything that implements `Display`.
- `Float`: Floating point numbers, typically on sliders and inputs.
- `Int`: Integer numbers for discrete gradations.
- `Bool`: A boolean value indicating true/false.
- `Listener`: A Rust callback that will be executed when the attribute is triggered.
- `Any`: A type-erased [`Rc<dyn Any>`](https://doc.rust-lang.org/std/any/index.html), typically used by 3rd-party renderers.
- `None`: The attribute will be removed from the element entirely.

Most commonly, you might use the `bool` attribute to set a boolean state:

```rust
rsx! {
    input {
        type: "checkbox",
        checked: true
    }
}
```

Or the `Some`/`None` variant for setting an HTML Boolean Attribute:
```rust
rsx! {
    div { itemscope: Some("scope") }
}
```

Note that Dioxus automatically converts `false` for some attributes to `None` in order to match the behavior of the [HTML Boolean Attribute](https://developer.mozilla.org/en-US/docs/Glossary/Boolean/HTML).

## Event Listeners

While some attributes influence how an element is rendered, other attributes influence its interactive behavior. These attributes, called Event Listeners, allow you to respond to user input.

In RSX, event handlers always start with `on`. The syntax is the same as normal attributes, but event handlers only accept a closure that responds to the event. We can attach an event listener to the `oninput` event of the input element to listen for changes to the input:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:on_input_simple}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::OnInputSimple {}
}
```

More information about event listeners including how events bubble and how to prevent default behavior can be found later in the [Event Handlers](../reactivity/event_handlers.md) Reactivity section.

There are a wide range of event listeners available - see the full [HTML list](https://developer.mozilla.org/en-US/docs/Web/Events) for more details.

## Spreading Attributes

Occasionally, the set of attributes you want to pass to an element might either be dynamic or defined elsewhere in your application. In these cases, you can spread attribute lists into elements with the `..` syntax. Typically, lists of attributes will be passed into a component via its Properties, which we cover in a later chapter.

```rust
let attributes = vec![
    Attribute {
        name: "id",
        namespace: "cool-button".into_value(),
        volatile: None,
        value: true,
    }
];

rsx! {
    button { ..attributes, "button" }
}
```

Attributes lists awill be merged in the order they appear, so later attributes in the list take precedence over earlier attributes. Attribute spreading becomes very useful when refactor your UI into reusable components and build component libraries.

## Special Attributes

Most attributes in RSX are rendered verbatim, but there are a few exceptions. In some cases, RSX deviates from traditional HTML to simplify development or work better with the ecosystem tools.

### Style attributes

In addition to the standard `style` attribute, each style can also be passed as a separate attribute. For example, we can set the `color` and `font-size` of an element using the `color` and `font_size` attributes:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:style_attributes}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::StyleAttributes {}
}
```

### Class attribute

Most attributes can only be defined once per element, but the `class` attribute can be defined multiple times. Each class will be added to the element's class list. This can be convenient when adding many optional classes to an element in a styling system like TailwindCSS:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:class_attribute}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::ClassAttribute {}
}
```

This feature is especially important when using TailwindCSS since the class compiler does not understand formatted Rust strings when collecting classes. By placing the dynamic class in a sibling attribute, the Tailwind compiler sees *both* class lists at compile time.

### Conditional Attributes

You can conditionally set an attribute by setting the attribute value to an unterminated if statement. If the if statement evaluates to true, the attribute will be set:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:input_disabled}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::InputDisabled {}
}
```

### Custom Attributes

Dioxus has a pre-configured set of attributes that you can use that are validated at compile time. You can use a custom attribute name surrounded in quotes to add an attribute outside of the pre-defined set:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:custom_attributes}}
```

```inject-dioxus
DemoFrame {
	building_uis_with_rsx::CustomAttributes {}
}
```

Note that this works even with event listeners. We occasionally use this to insert small snippets of JavaScript into our apps when writing the corresponding web-sys code might be verbose:

```rust
rsx! {
    button {
        "onclick": "navigator.clipboard.writeText(window.document.title);",
        "Copy to clipboard"
    }
}
```

```inject-dioxus
DemoFrame {
	building_uis_with_rsx::CustomAttributesEvents {}
}
```

### The HTML Escape Hatch

If you're working with pre-rendered assets, output from templates, or output from a JS library, then you might want to pass HTML directly instead of going through Dioxus. In these instances, reach for `dangerous_inner_html`.

`dangerous_inner_html` sets the text content of the element to the provided value. This will overwrite any other attributes or children of the element.

For example, shipping a markdown-to-Dioxus converter might significantly bloat your final application size. Instead, you'll want to pre-render your markdown to HTML and then include the HTML directly in your output. We use this approach for the [Dioxus homepage](https://dioxuslabs.com):

```rust, no_run
{{#include ../docs-router/src/doc_examples/dangerous_inner_html.rs:dangerous_inner_html}}
```

```inject-dioxus
DemoFrame {
	dangerous_inner_html::App {}
}
```

> Note! This attribute is called "dangerous_inner_html" because it is **dangerous** to pass it data you don't trust. If you're not careful, you can easily expose [cross-site scripting (XSS)](https://en.wikipedia.org/wiki/Cross-site_scripting) attacks to your users.
>
> If you're handling untrusted input, make sure to sanitize your HTML before passing it into `dangerous_inner_html` – or just pass it to a Text Element to escape any HTML tags.
