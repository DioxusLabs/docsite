# Building UIs with RSX

Dioxus renders to HTML, if you are not familiar with HTML, this guide will help you get started with the basics. For more detail, the [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML) are a great resource.

## Text Nodes

Any content surrounded by quotes is rendered as a text node in rsx:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:text}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::Text {}
}
```

You can include formatted segments inside of the text just like the `format!` macro:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:formatted_text}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::FormattedText {}
}
```

## Elements

The most basic building block of HTML is an element. In rsx, you can create elements with the name and then curly braces. One of the most common elements is the `input` element. The input element creates an interactive input box:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:input}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::Input {}
}
```

> Bonus: web components
> Any element with a dash in the name is a web component. Web components are rendered directly in dioxus without type checking. We recommend wrapping web components in a type safe component to make them easier to use.
>
> ```rust, no_run
> {{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:web_component}}
> ```

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

## Children

You can add children to an element after all attributes and event listeners. Elements can accept text, components or other elements as children. We can add a `div` element around our input to center it:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:input_children}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::InputChildren {}
}
```

## Loops

You can insert for loops directly in rsx. The body of the loop accepts any number of children that will be rendered with each iteration of the loop. The `ul` element in html renders an unordered list with any number of `li` (list item) elements. We can use those two elements to render a list of items in a loop:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:for_loop}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::ForLoop {}
}
```

Each item in your list should have unique value that is stable across rerenders called a key. Keys are used to identify how items move while diffing. Without keys, it is easy to accidentally lose or move state when you reorder items in a list. We can add keys to our list items by using the `key` attribute:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:keyed_for_loop}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::KeyedForLoop {}
}
```

## If Statements

You can also use if/else statements in rsx. Each branch of the if statement accepts child nodes that will be rendered if the condition is true. We can use the `if` statement to conditionally render a login screen:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:if_statement}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::IfStatement {}
}
```

## Expressions

Finally, you can use arbitrary Rust expressions inside of rsx by surrounding them with `{}`. Any expression you include must implement the [`IntoDynNode`](https://docs.rs/dioxus-core/latest/dioxus_core/trait.IntoDynNode.html) trait. You can use raw expressions to render nodes from an iterator, variable, or function:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:expression}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::Expression {}
}
```

## Why RSX and not HTML ?

If you've seen React's JSX or the `html!{}` Rust macro, you might be curious as to why Dioxus chose to use its own syntax instead of a syntax that looks more similar to HTML.

A few reasons:

- RSX gets token coloring and code-folding without additional tooling
- RSX is faster to type since curly braces are auto-closed
- Not all RSX is HTML - Dioxus can be used in non-HTML contexts
- HTML is not valid Rust - not all HTML can be used in html!{}
