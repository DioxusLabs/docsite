# Describing the UI

Dioxus is a _declarative_ framework. This means that instead of telling Dioxus what to do (e.g. to "create an element" or "set the color to red") we simply _declare_ what we want the UI to look like using RSX.

You have already seen a simple example of RSX syntax in the "hello world" application:

```rust, no_run
{{#include src/doc_examples/hello_world_desktop.rs:component}}
```

Here, we use the `rsx!` macro to _declare_ that we want a `div` element, containing the text `"Hello, world!"`. Dioxus takes the RSX and constructs a UI from it.

## RSX Features

RSX is very similar to HTML in that it describes elements with attributes and children. Here's an empty `button` element in RSX, as well as the resulting HTML:

```rust, no_run
{{#include src/doc_examples/rsx_overview.rs:button}}
```

```inject-dioxus
DemoFrame {
	rsx_overview::Button {}
}
```

### Attributes

Attributes (and [event handlers](event_handlers.md)) modify the behavior or appearance of the element they are attached to. They are specified inside the `{}` brackets, using the `name: value` syntax. You can provide the value as a literal in the RSX:

```rust, no_run
{{#include src/doc_examples/rsx_overview.rs:attributes}}
```

```inject-dioxus
DemoFrame {
	rsx_overview::Attributes {}
}
```

Some attributes, such as the ``type`` attribute for ``input`` elements won't work on their own in Rust. This is because ``type`` is a reserved Rust keyword. To get around this, Dioxus uses the ``r#`` specifier:
```rust, no_run
{{#include src/doc_examples/rsx_overview.rs:attributes_type}}
```

> Note: All attributes defined in `dioxus-html` follow the snake_case naming convention. They transform their `snake_case` names to HTML's `camelCase` attributes.

> Note: Styles can be used directly outside of the `style:` attribute. In the above example, `color: "red"` is turned into `style="color: red"`.

#### Conditional Attributes

You can also conditionally include attributes by using an if statement without an else branch. This is useful for adding an attribute only if a certain condition is met:

```rust, no_run
{{#include src/doc_examples/rsx_overview.rs:conditional_attributes}}
```

```inject-dioxus
DemoFrame {
	rsx_overview::ConditionalAttributes {}
}
```

#### Custom Attributes

Dioxus has a pre-configured set of attributes that you can use. RSX is validated at compile time to make sure you didn't specify an invalid attribute. If you want to override this behavior with a custom attribute name, specify the attribute in quotes:

```rust, no_run
{{#include src/doc_examples/rsx_overview.rs:custom_attributes}}
```

```inject-dioxus
DemoFrame {
	rsx_overview::CustomAttributes {}
}
```

### Special Attributes

While most attributes are simply passed on to the HTML, some have special behaviors.

#### The HTML Escape Hatch

If you're working with pre-rendered assets, output from templates, or output from a JS library, then you might want to pass HTML directly instead of going through Dioxus. In these instances, reach for `dangerous_inner_html`.

For example, shipping a markdown-to-Dioxus converter might significantly bloat your final application size. Instead, you'll want to pre-render your markdown to HTML and then include the HTML directly in your output. We use this approach for the [Dioxus homepage](https://dioxuslabs.com):

```rust, no_run
{{#include src/doc_examples/dangerous_inner_html.rs:dangerous_inner_html}}
```

```inject-dioxus
DemoFrame {
	dangerous_inner_html::App {}
}
```
> Note! This attribute is called "dangerous_inner_html" because it is **dangerous** to pass it data you don't trust. If you're not careful, you can easily expose [cross-site scripting (XSS)](https://en.wikipedia.org/wiki/Cross-site_scripting) attacks to your users.
>
> If you're handling untrusted input, make sure to sanitize your HTML before passing it into `dangerous_inner_html` – or just pass it to a Text Element to escape any HTML tags.

#### Boolean Attributes

Most attributes, when rendered, will be rendered exactly as the input you provided. However, some attributes are considered "boolean" attributes and just their presence determines whether they affect the output. For these attributes, a provided value of `"false"` will cause them to be removed from the target element.

So this RSX wouldn't actually render the `hidden` attribute:

```rust, no_run
{{#include src/doc_examples/boolean_attribute.rs:boolean_attribute}}
```

```inject-dioxus
DemoFrame {
	boolean_attribute::App {}
}
```

Not all attributes work like this however. _Only the following attributes_ have this behavior:

- `allowfullscreen`
- `allowpaymentrequest`
- `async`
- `autofocus`
- `autoplay`
- `checked`
- `controls`
- `default`
- `defer`
- `disabled`
- `formnovalidate`
- `hidden`
- `ismap`
- `itemscope`
- `loop`
- `multiple`
- `muted`
- `nomodule`
- `novalidate`
- `open`
- `playsinline`
- `readonly`
- `required`
- `reversed`
- `selected`
- `truespeed`

For any other attributes, a value of `"false"` will be sent directly to the DOM.

### Interpolation

Similarly to how you can [format](https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html) Rust strings, you can also interpolate in RSX text. Use `{variable}` to Display the value of a variable in a string, or `{variable:?}` to use the Debug representation:

```rust, no_run
{{#include src/doc_examples/rsx_overview.rs:formatting}}
```

```inject-dioxus
DemoFrame {
	rsx_overview::Formatting {}
}
```

### Children

To add children to an element, put them inside the `{}` brackets after all attributes and listeners in the element. They can be other elements, text, or [components](components.md). For example, you could have an `ol` (ordered list) element, containing 3 `li` (list item) elements, each of which contains some text:

```rust, no_run
{{#include src/doc_examples/rsx_overview.rs:children}}
```

```inject-dioxus
DemoFrame {
	rsx_overview::Children {}
}
```

### Fragments

You can render multiple elements at the top level of `rsx!` and they will be automatically grouped.

```rust, no_run
{{#include src/doc_examples/rsx_overview.rs:manyroots}}
```

```inject-dioxus
DemoFrame {
	rsx_overview::ManyRoots {}
}
```

### Expressions

You can include arbitrary Rust expressions as children within RSX by surrounding your expression with `{}`s. Any expression that implements [IntoDynNode](https://docs.rs/dioxus-core/0.3/dioxus_core/trait.IntoDynNode.html) can be used within rsx. This is useful for displaying data from an [iterator](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html#processing-a-series-of-items-with-iterators):

```rust, no_run
{{#include src/doc_examples/rsx_overview.rs:expression}}
```

```inject-dioxus
DemoFrame {
	rsx_overview::Expression {}
}
```

### Loops

In addition to iterators you can also use for loops directly within RSX:

```rust, no_run
{{#include src/doc_examples/rsx_overview.rs:loops}}
```

```inject-dioxus
DemoFrame {
	rsx_overview::Loops {}
}
```

### If statements

You can also use if statements without an else branch within RSX:

```rust, no_run
{{#include src/doc_examples/rsx_overview.rs:ifstatements}}
```

```inject-dioxus
DemoFrame {
	rsx_overview::IfStatements {}
}
```
