# Your First Component

This chapter will teach you how to create a [Component](../reference/components.md) that displays a link to a post on hackernews.

## Setup

> Before you start the guide, make sure you have the dioxus CLI and any required dependencies for your platform as described in the [getting started](../getting_started/index.md) guide.

First, let's create a new project for our hacker news app. We can use the CLI to create a new project. You can select a platform of your choice or view the getting started guide for more information on each option. If you aren't sure what platform to try out, we recommend getting started with web or desktop:

```sh
dx new
```

The template contains some boilerplate to help you get started. For this guide, we will be rebuilding some of the code from scratch for learning purposes. You can clear the `src/main.rs` file. We will be adding new code in the next sections.

Next, let's setup our dependencies. We need to set up a few dependencies to work with the hacker news API: 

```sh
cargo add chrono --features serde
cargo add futures
cargo add reqwest --features json
cargo add serde --features derive
cargo add serde_json
cargo add async_recursion
```

## Describing the UI

Now, we can define how to display a post. Dioxus is a *declarative* framework. This means that instead of telling Dioxus what to do (e.g. to "create an element" or "set the color to red") we simply *declare* how we want the UI to look. 

To declare what you want your UI to look like, you will need to use the `rsx` macro. Let's create a ``main`` function and an ``App`` component to show information about our story:

```rust
{{#include src/doc_examples/hackernews_post.rs:story_v1}}
```

Now if you run your application you should see something like this:

```inject-dioxus
DemoFrame {
	hackernews_post::story_v1::App {}
}
```

> RSX mirrors HTML. Because of this you will need to know some html to use Dioxus.
> 
> Here are some resources to help get you started learning HTML:
> - [MDN HTML Guide](https://developer.mozilla.org/en-US/docs/Learn/HTML)
> - [W3 Schools HTML Tutorial](https://www.w3schools.com/html/default.asp)
> 
> In addition to HTML, Dioxus uses CSS to style applications. You can either use traditional CSS (what this guide uses) or use a tool like [tailwind CSS](https://tailwindcss.com/docs/installation):
> - [MDN Traditional CSS Guide](https://developer.mozilla.org/en-US/docs/Learn/HTML)
> - [W3 Schools Traditional CSS Tutorial](https://www.w3schools.com/css/default.asp)
> - [Tailwind tutorial](https://tailwindcss.com/docs/installation) (used with the [Tailwind setup example](https://github.com/DioxusLabs/dioxus/tree/main/examples/tailwind))
> 
> If you have existing html code, you can use the [translate](../CLI/translate.md) command to convert it to RSX. Or if you prefer to write html, you can use the [html! macro](https://github.com/DioxusLabs/dioxus-html-macro) to write html directly in your code.

## Dynamic Text

Let's expand our `App` component to include the story title, author, score, time posted, and number of comments. We can insert dynamic text in the render macro by inserting variables inside `{}`s (this works similarly to the formatting in the [println!](https://doc.rust-lang.org/std/macro.println.html) macro):

```rust
{{#include src/doc_examples/hackernews_post.rs:story_v2}}
```

```inject-dioxus
DemoFrame {
	hackernews_post::story_v2::App {}
}
```

## Creating Elements

Next, let's wrap our post description in a [`div`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div). You can create HTML elements in Dioxus by putting a `{` after the element name and a `}` after the last child of the element:

```rust
{{#include src/doc_examples/hackernews_post.rs:story_v3}}
```

```inject-dioxus
DemoFrame {
	hackernews_post::story_v3::App {}
}
```

> You can read more about elements in the [rsx reference](../reference/rsx.md).

## Setting Attributes

Next, let's add some padding around our post listing with an attribute.

Attributes (and [listeners](../reference/event_handlers.md)) modify the behavior or appearance of the element they are attached to. They are specified inside the `{}` brackets before any children, using the `name: value` syntax. You can format the text in the attribute as you would with a text node:

```rust
{{#include src/doc_examples/hackernews_post.rs:story_v4}}
```

```inject-dioxus
DemoFrame {
	hackernews_post::story_v4::App {}
}
```

> Note: All attributes defined in [`dioxus-html`](https://docs.rs/dioxus-html/latest/dioxus_html/) follow the snake_case naming convention. They transform their `snake_case` names to HTML's `camelCase` attributes.

> Note: Styles can be used directly outside of the `style:` attribute. In the above example, `padding: "0.5rem"` is turned into `style="padding: 0.5rem"`.

> You can read more about elements in the [attribute reference](../reference/rsx.md)

## Creating a Component

Just like you wouldn't want to write a complex program in a single, long, `main` function, you shouldn't build a complex UI in a single `App` function. Instead, you should break down the functionality of an app in logical parts called components.

A component is a Rust function, named in UpperCamelCase, that takes a props parameter and returns an `Element` describing the UI it wants to render. In fact, our `App` function is a component!

Let's pull our story description into a new component:

```rust
{{#include src/doc_examples/hackernews_post.rs:story_v5}}
```

We can render our component like we would an element by putting `{}`s after the component name. Let's modify our `App` component to render our new StoryListing component:

```rust
{{#include src/doc_examples/hackernews_post.rs:app_v5}}
```

```inject-dioxus
DemoFrame {
	hackernews_post::story_v5::App {}
}
```

> You can read more about elements in the [component reference](../reference/components.md)

## Creating Props

Just like you can pass arguments to a function or attributes to an element, you can pass props to a component that customize its behavior!

We can define arguments that components can take when they are rendered (called `Props`) by adding the `#[component]` macro before our function definition and adding extra function arguments.

Currently, our `StoryListing` component always renders the same story. We can modify it to accept a story to render as a prop.


We will also define what a post is and include information for how to transform our post to and from a different format using [serde](https://serde.rs). This will be used with the hackernews API in a later chapter:

```rust
{{#include src/doc_examples/hackernews_post.rs:story_v6}}
```

Make sure to also add [serde](https://serde.rs) as a dependency:

```bash
cargo add serde --features derive
cargo add serde_json
```

We will also use the [chrono](https://crates.io/crates/chrono) crate to provide utilities for handling time data from the hackernews API:
```bash
cargo add chrono --features serde
```


Now, let's modify the `App` component to pass the story to our `StoryListing` component like we would set an attribute on an element:

```rust
{{#include src/doc_examples/hackernews_post.rs:app_v6}}
```

```inject-dioxus
DemoFrame {
	hackernews_post::story_v6::App {}
}
```

> You can read more about Props in the [Props reference](../reference/component_props.md)

## Cleaning Up Our Interface

Finally, by combining elements and attributes, we can make our post listing much more appealing:

Full code up to this point:

```rust
{{#include src/doc_examples/hackernews_post.rs:story_final}}
```

```inject-dioxus
DemoFrame {
	hackernews_post::story_final::App {}
}
```
