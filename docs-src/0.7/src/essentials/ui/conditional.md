# Conditional Rendering

## Dynamic Content

Our user interfaces have been quite static so far. However, most apps we build with Dioxus usually contain lots of dynamic content. Our UIs will react to changes in buttons, form inputs, sliders, or external data sources like the network. Dioxus apps generally store this dynamic state in Hooks or Context.

In this chapter, we're not going to dive too deep in how we store this state - future chapters cover state in depth.

## Expressions

Just like JSX, RSX allows you to easily compose `Element` objects together using plain Rust code. You can write Rust expressions directly within your RSX. As long as the expression evaluates to an `Element` or anything that implements `IntoDynNode`, you can simply wrap it in curly braces (`{}`):

```rust
let content = "world!";
rsx! {
    h1 {
        "Hello"
        {content}
    }
}
```

For example, we might need to create a string from some complex formatting functions:

```rust
rsx! {
    span {
        {
            format!(
                "The time is: {now}, your timezone is {zone}",
                now = current_time(),
                zone = current_timezone()
            ).to_ascii_uppercase()
        }
    }
}
```

Or, we might want to render some RSX dynamically and assign it to a variable:

```rust
let header = match current_timezone() {
    TimeZone::PST => rsx! { h1 { "Welcome home" } },
    _ => rsx! { h1 { "Bon voyage!" } },
}

rsx! {
    div { {header} }
}
```

Rust's expression system makes evaluation of RSX from `match` statements and `if` blocks quite nice. While in JavaScript you might use a ternary:

```jsx
let screen = authenticated ? renderApp() : renderLogin();

return <div>{screen}</div>;
```

In Dioxus, you'd simply use an if/else statement:

```rust
let screen = if authenticated { render_app() } else { render_login() };
rsx! {
    div { {screen} }
}
```

Rust's guards can be especially helpful in these scenarios, letting us select match arms with inline `if` statements.

```rust
let header = match current_timezone() {
    TimeZone::PST => rsx! { h1 { "Welcome home" } },
    _ if app.snoozed() => rsx! { h1 { "snoozed..." } },
    _ => rsx! { h1 { "Bon voyage!" } },
}

rsx! {
    div { {header} }
}
```

## The IntoDynNode Trait

Dioxus uses the [`IntoDynNode`](https://docs.rs/dioxus-core/latest/dioxus_core/trait.IntoDynNode.html) trait to determine if an expression can be used within RSX. The conversion will take a Rust expression and turn it into one of four `DynamicNode` variants:

- Component: Functions that take Properties and render an Element
- Text: The Rust `String` type
- Placeholder: An optimized `None` value
- List: A Vec of Elements

Many things implement this trait. For example, empty expressions are valid:
```rust
rsx! {
    div { { /* empty.. */} }
}
```

Other Element objects are valid:

```rust
let inner = rsx! { "inner" };
rsx! {
    div { {inner} }
}
```

All the string types (str, String, Arguments) are valid:
```rust
rsx! {
    div {
        // Strings
        {"abc"}

        // lazy formatting
        {format_args!("lazy fmt -> {}", arg())}
    }
}
```

The Rust `Option` type is valid provided the inner type implements `IntoDynNode`:

```rust
let inner = Some(rsx! { "inner" });
rsx! {
    div { {inner} }
}
```

And even iterators can become a VNode through the List variant:

```rust
let cards = (0..10).map(|i| rsx! {
    li { "Card: {i}" }
});

rsx! {
    ol { {cards} }
}
```

Iterators are very interesting since IntoDynNode is implemented for anything that is an iterator. For exampe, we could build a custom iterator that returns an `Element`.

```rust
let mut count = 0;
let cards = std::iter::from_fn(move || {
    count += 1;
    if count > 6 {
        return None;
    }

    Some(rsx! { "card {count}" })
})

rsx! {
    ol { {cards} }
}
```

## Inline `If` Statements

When rendering content derived from a boolean condition (eg, "active" or "inactive"), RSX provides some small "syntax sugar" that enables inline `if` statements in RSX.

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:if_statement}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::IfStatement {}
}
```

Note that the body of inline `if` statements *is* RSX, not Rust expressions. This syntax sugar helps keep RSX blocks tidy and idiomatic.
```rust
rsx! {
    div {
        if logged_in() {
            LoggedInScreen {}
        } else {
            LoggedOutScreen {}
        }
    }
}
```

Inline `if` statements deviate from Rust in one way: they still evaluate to an `Element` even without an `else` branch. If RSX doesn't find an `else` branch on your `if` statement, it automatically returns a placeholder element instead.a

```rust
rsx! {
    div {
        if logged_out() {
            span { "You should log in!" }
        }
    }
}
```

## Inline `for` loops

Just like inline `if` chains, RSX supports some slight syntax sugar for iterators. In JSX you might be used to calling `map` on an iterable object. In RSX, we can do something similar with `.map` and `.iter()`

```rust
rsx! {
    // mapping existing iterators
    {(0..10).map(|idx| rsx! { "item {idx}" })}

    // or calling .iter()
    {users.iter().map(|user| rsx!{ User { id: user.id } })}
}
```

Since iterators are so prevelant in UI code, Dioxus provides a small amount of syntax sugar to make using iterators a bit nicer. Instead of wrapping your iterator in an expression, you can instead move it to an inline `for` block:

```rust
rsx! {
    for idx in 0..10 {
        "Item {idx}"
    }

    for user in users.iter() {
        User { id: user.id }
    }
}
```

Just like inline `if` blocks, the bodies of `for` loops are RSX - not Rust expressions. If you need to create temporary variables or do some extra computation while iterating, you can use an inline expression:

```rust
rsx! {
    for user in users.iter() {
        {
            let id = user.id();
            rsx! {
                User { id }
            }
        }
    }
}
```

The transformation RSX applies is very straightforward with no special "magic" - it expands to the same iterator you would've written otherwise.

## More syntax sugar?

Syntax can be very subjective and syntax sugar like inline `if` and `for` blocks can open the door to inconsistent behavior. We don't plan to introduce any further syntax sugar to RSX. Our goal is to maintain similiarity to React's JSX while slightly massaging ergonomics for Rust developers.

The RSX syntax was carefully designed to work well with your normal development flow:

- Typing an RSX element involves typing the name, and then a single curly
- RSX does not require additional editor extensions for superb support
- RSX follows Rust tokenization for automatic highlighting and code-folding

Definitely spend some time with RSX to get a feel for it.
