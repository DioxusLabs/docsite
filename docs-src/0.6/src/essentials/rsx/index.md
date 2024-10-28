# Building UIs with RSX

Dioxus renders to HTML, if you are not familiar with HTML, this guide will help you get started with the basics. For more detail, the [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML) are a great resource.


## Text Nodes

Any content surrounded by quotes is rendered as a text node in rsx:

```rust, no_run
{{#include src/doc_examples/building_uis_with_rsx.rs:text}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::Text {}
}
```

You can include formatted segments inside of the text just like the `format!` macro:

```rust, no_run
{{#include src/doc_examples/building_uis_with_rsx.rs:formatted_text}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::FormattedText {}
}
```

## Elements

The most basic building block of HTML is an element. In rsx, you can create elements with the name and then curly braces. One of the most common elements is the `input` element. The input element creates an interactive input box:

```rust, no_run
{{#include src/doc_examples/building_uis_with_rsx.rs:input}}
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
> {{#include src/doc_examples/building_uis_with_rsx.rs:web_component}}
> ```

## Attributes

Attributes provide extra information about an element. You can specify attributes in dioxus inside an element's braces by typing the name of the attribute, a colon, and then the value (typically a formatted string). We can use an attribute to set the `type` of an input element. The default type is `text` which shows a text input box, but we can set it to `number` to only accept numbers:

```rust, no_run
{{#include src/doc_examples/building_uis_with_rsx.rs:input_type}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::InputType {}
}
```

Just like text nodes, attributes can include formatted segments. We can set the value of the input element to a signal to control it:

```rust, no_run
{{#include src/doc_examples/building_uis_with_rsx.rs:input_value}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::InputValue {}
}
```

### Conditional Attributes

You can conditionally set an attribute by setting the attribute value to an unterminated if statement. If the if statement evaluates to true, the attribute will be set:

```rust, no_run
{{#include src/doc_examples/building_uis_with_rsx.rs:input_disabled}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::InputDisabled {}
}
```

## Event Listeners

Event listeners allow you to respond to user input. In rsx, event handlers always start with `on`. The syntax is the same as normal attributes, but event handlers only accept a closure that responds to the event. We can attach an event listener to the `oninput` event of the input element to listen for changes to the input:

```rust, no_run
{{#include src/doc_examples/building_uis_with_rsx.rs:on_input}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::OnInput {}
}
```

## Children

You can add children to an element after all attributes and event listeners. Elements can accept text, components or other elements as children. We can add a `div` element around our input to center it:

```rust, no_run
{{#include src/doc_examples/building_uis_with_rsx.rs:input_children}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::InputChildren {}
}
```

## Loops

You can insert for loops directly in rsx. The body of the loop accepts any number of children that will be rendered with each iteration of the loop. The `ul` element in html renders an unordered list with any number of `li` (list item) elements. We can use those two elements to render a list of items in a loop:

```rust, no_run
{{#include src/doc_examples/building_uis_with_rsx.rs:for_loop}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::ForLoop {}
}
```

Each item in your list should have unique value that is stable across rerenders called a key. Keys are used to identify how items move while diffing. Without keys, it is easy to accidentally lose or move state when you reorder items in a list. We can add keys to our list items by using the `key` attribute:

```rust, no_run
{{#include src/doc_examples/building_uis_with_rsx.rs:keyed_for_loop}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::KeyedForLoop {}
}
```

## If Statements

You can also use if/else statements in rsx. Each branch of the if statement accepts child nodes that will be rendered if the condition is true. We can use the `if` statement to conditionally render a login screen:

```rust, no_run
{{#include src/doc_examples/building_uis_with_rsx.rs:if_statement}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::IfStatement {}
}
```
