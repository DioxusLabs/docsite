# Describing the UI

Dioxus is a *declarative* framework. This means that instead of telling Dioxus what to do (e.g. to "create an element" or "set the color to red") we simply *declare* what we want the UI to look like using RSX.

You have already seen a simple example of RSX syntax in the "hello world" application:

```rust
{{#include src/doc_examples/hello_world_desktop.rs:component}}
```

Here, we use the `rsx!` macro to *declare* that we want a `div` element, containing the text `"Hello, world!"`. Dioxus takes the RSX and constructs a UI from it.

## RSX Features

RSX is very similar to HTML in that it describes elements with attributes and children. Here's an empty `div` element in RSX, as well as the resulting rendered page (To see the html you can right click on the rendered section and click inspect):

```rust
{{#include src/doc_examples/rsx_overview.rs:empty}}
```
```inject-dioxus
DemoFrame {
	Empty {}
}
```


### Attributes

Attributes (and [listeners](../interactivity/index.md)) modify the behavior or appearance of the element they are attached to. They are specified inside the `{}` brackets, using the `name: value` syntax. You can provide the value as a literal in the RSX:
```rust
{{#include src/doc_examples/rsx_overview.rs:attributes}}
```
```inject-dioxus
DemoFrame {
	Attributes {}
}
```

> Note: All attributes defined in `dioxus-html` follow the snake_case naming convention. They transform their `snake_case` names to HTML's `camelCase` attributes.

> Note: Styles can be used directly outside of the `style:` attribute. In the above example, `color: "red"` is turned into `style="color: red"`.

#### Custom Attributes

Dioxus has a pre-configured set of attributes that you can use. RSX is validated at compile time to make sure you didn't specify an invalid attribute. If you want to override this behavior with a custom attribute name, specify the attribute in quotes:

```rust
{{#include src/doc_examples/rsx_overview.rs:custom_attributes}}
```
```inject-dioxus
DemoFrame {
	CustomAttributes {}
}
```

### Interpolation

Similarly to how you can [format](https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html) Rust strings, you can also interpolate in RSX text. Use `{variable}` to Display the value of a variable in a string, or `{variable:?}` to use the Debug representation:

```rust
{{#include src/doc_examples/rsx_overview.rs:formatting}}
```
```inject-dioxus
DemoFrame {
	Formatting {}
}
```

### Children

To add children to an element, put them inside the `{}` brackets after all attributes and listeners in the element. They can be other elements, text, or [components](components.md). For example, you could have an `ol` (ordered list) element, containing 3 `li` (list item) elements, each of which contains some text:

```rust
{{#include src/doc_examples/rsx_overview.rs:children}}
```
```inject-dioxus
DemoFrame {
	Children {}
}
```

### Fragments

You can render multiple elements at the top level of `rsx!` and they will be automatically grouped.

```rust
{{#include src/doc_examples/rsx_overview.rs:manyroots}}
```

```inject-dioxus
DemoFrame {
	ManyRoots {}
}
```

### Expressions

You can include arbitrary Rust expressions as children within RSX that implements [IntoDynNode](https://docs.rs/dioxus-core/0.3/dioxus_core/trait.IntoDynNode.html). This is useful for displaying data from an [iterator](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html#processing-a-series-of-items-with-iterators):

```rust
{{#include src/doc_examples/rsx_overview.rs:expression}}
```
```inject-dioxus
DemoFrame {
	Expression {}
}
```

### Loops

In addition to iterators you can also use for loops directly within RSX:

```rust
{{#include src/doc_examples/rsx_overview.rs:loops}}
```
```inject-dioxus
DemoFrame {
	Loops {}
}
```

### If statements

You can also use if statements without an else branch within RSX:

```rust
{{#include src/doc_examples/rsx_overview.rs:ifstatements}}
```
```inject-dioxus
DemoFrame {
	IfStatements {}
}
```