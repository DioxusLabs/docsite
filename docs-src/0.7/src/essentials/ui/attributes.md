
## Attributes

Attributes provide extra information about an element. You can specify attributes in dioxus inside an element's braces by typing the name of the attribute, a colon, and then the value (typically a formatted string). We can use an attribute to set the `type` of an input element. The default type is `text` which shows a text input box, but we can set it to `number` to only accept numbers:

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


### Special Attributes

Most attributes in rsx are passed onto the html directly, but there are a few exceptions.

#### The HTML Escape Hatch

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

#### Style attributes

In addition to the standard `style` attribute, each style can also be passed as a separate attribute. For example, we can set the `color` and `font-size` of an element using the `color` and `font_size` attributes:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:style_attributes}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::StyleAttributes {}
}
```

#### Class attribute

Most attributes can only be defined once per element, but the `class` attribute can be defined multiple times. Each class will be added to the element's class list. This can be convenient when adding many optional classes to an element in a styling system like tailwindcss:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:class_attribute}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::ClassAttribute {}
}
```

#### Custom Attributes

Dioxus has a pre-configured set of attributes that you can use that are validated at compile time. You can use a custom attribute name surrounded in quotes to add an attribute outside of the pre-defined set:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:custom_attributes}}
```

```inject-dioxus
DemoFrame {
	building_uis_with_rsx::CustomAttributes {}
}
```

## Event Listeners

Event listeners allow you to respond to user input. In rsx, event handlers always start with `on`. The syntax is the same as normal attributes, but event handlers only accept a closure that responds to the event. We can attach an event listener to the `oninput` event of the input element to listen for changes to the input:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:on_input}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::OnInput {}
}
```

More information about event listeners including how events bubble and how to prevent default behavior can be found in the [Event Handlers](../reactivity/event_handlers.md) section.
