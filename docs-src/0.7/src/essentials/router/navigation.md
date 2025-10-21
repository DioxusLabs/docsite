# Links & Navigation

When we split our app into pages, we need to provide our users with a way to
navigate between them. On regular web pages, we'd use an anchor element for that,
like this:

```html
<a href="/other">Link to an other page</a>
```

However, we cannot do that when using the router for three reasons:

1. Anchor tags make the browser load a new page from the server. This takes a
   lot of time, and it is much faster to let the router handle the navigation
   client-side.
2. Navigation using anchor tags only works when the app is running inside a
   browser. This means we cannot use them inside apps using Dioxus Desktop.
3. Anchor tags cannot check if the target page exists. This means we cannot
   prevent accidentally linking to non-existent pages.

To solve these problems, the router provides us with a [`Link`] component we can
use like this:

```rust
{{#include ../docs-router/src/doc_examples/links.rs:nav}}
```

The `target` in the example above is similar to the `href` of a regular anchor
element. However, it tells the router more about what kind of navigation it
should perform. It accepts something that can be converted into a
[`NavigationTarget`]:

- The example uses a Internal route. This is the most common type of navigation.
  It tells the router to navigate to a page within our app by passing a variant of a [`Routable`] enum. This type of navigation can never fail if the link component is used inside a router component.
- [`External`] allows us to navigate to URLs outside of our app. This is useful
  for links to external websites. NavigationTarget::External accepts an URL to navigate to. This type of navigation can fail if the URL is invalid.

> The [`Link`] accepts several props that modify its behavior. See the API docs
> for more details.


# Programmatic Navigation

Sometimes we want our application to navigate to another page without having the
user click on a link. This is called programmatic navigation.

## Using a Navigator

We can get a navigator with the [`navigator`] function which returns a [`Navigator`].

We can use the [`Navigator`] to trigger four different kinds of navigation:

- `push` will navigate to the target. It works like a regular anchor tag.
- `replace` works like `push`, except that it replaces the current history entry
  instead of adding a new one. This means the prior page cannot be restored with the browser's back button.
- `Go back` works like the browser's back button.
- `Go forward` works like the browser's forward button.

```rust
{{#include ../docs-router/src/doc_examples/navigator.rs:nav}}
```

You might have noticed that, like [`Link`], the [`Navigator`]s `push` and
`replace` functions take a [`NavigationTarget`]. This means we can use either
`Internal`, or `External` targets.

## External Navigation Targets

Unlike a [`Link`], the [`Navigator`] cannot rely on the browser (or webview) to
handle navigation to external targets via a generated anchor element.

This means, that under certain conditions, navigation to external targets can
fail.


[`Link`]: https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Link.html
[`NavigationTarget`]: https://docs.rs/dioxus-router/latest/dioxus_router/navigation/enum.NavigationTarget.html
[`Navigator`]: https://docs.rs/dioxus-router/latest/dioxus_router/prelude/struct.Navigator.html


# History Providers

[`HistoryProvider`]s are used by the router to keep track of the navigation history
and update any external state (e.g. the browser's URL).

The router provides two [`HistoryProvider`]s, but you can also create your own.
The two default implementations are:

- The [`MemoryHistory`] is a custom implementation that works in memory.
- The [`LiveviewHistory`] is a custom implementation that works with the liveview renderer.
- The [`WebHistory`] integrates with the browser's URL.

By default, the router uses the [`MemoryHistory`]. It might be changed to use
[`WebHistory`] when the `web` feature is active, but that is not guaranteed.

You can override the default history:

```rust
{{#include ../docs-router/src/doc_examples/history_provider.rs:app}}
```


# History Buttons

Some platforms, like web browsers, provide users with an easy way to navigate
through an app's history. They have UI elements or integrate with the OS.

However, native platforms usually don't provide such amenities, which means that
apps wanting users to have access to them, need to implement them. For this
reason, the router comes with two components, which emulate a browser's back and
forward buttons:

- [`GoBackButton`]
- [`GoForwardButton`]

> If you want to navigate through the history programmatically, take a look at
> [`programmatic navigation`](./programmatic-navigation.md).

```rust
{{#include ../docs-router/src/doc_examples/history_buttons.rs:history_buttons}}
```

As you might know, browsers usually disable the back and forward buttons if
there is no history to navigate to. The router's history buttons try to do that
too, but depending on the [history provider] that might not be possible.

Importantly, neither `WebHistory` supports that feature.
This is due to limitations of the browser History API.

However, in both cases, the router will just ignore button presses, if there is
no history to navigate to.

Also, when using `WebHistory`, the history buttons might
navigate a user to a history entry outside your app.

[`GoBackButton`]: https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.GoBackButton.html
[`GoForwardButton`]: https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.GoForwardButton.html


# Routing Update Callback

In some cases, we might want to run custom code when the current route changes. For this reason, the [`RouterConfig`] exposes an `on_update` field.

## How does the callback behave?

The `on_update` is called whenever the current routing information changes. It is called after the router updated its internal state, but before dependent components and hooks are updated.

If the callback returns a [`NavigationTarget`], the router will replace the current location with the specified target. It will not call the `on_update` again.

If at any point the router encounters a navigation failure, it will go to the appropriate state without calling the `on_update`. It doesn't matter if the invalid target initiated the navigation, was found as a redirect target, or was returned by the `on_update` itself.

## Code Example

```rust
{{#include ../docs-router/src/doc_examples/routing_update.rs:router}}
```

[`NavigationTarget`]: https://docs.rs/dioxus-router/latest/dioxus_router/navigation/enum.NavigationTarget.html
[`RouterConfig`]: https://docs.rs/dioxus-router/latest/dioxus_router/prelude/struct.RouterConfig.html

# History Buttons

Some platforms, like web browsers, provide users with an easy way to navigate
through an app's history. They have UI elements or integrate with the OS.

However, native platforms usually don't provide such amenities, which means that
apps wanting users to have access to them, need to implement them. For this
reason, the router comes with two components, which emulate a browser's back and
forward buttons:

- [`GoBackButton`]
- [`GoForwardButton`]

> If you want to navigate through the history programmatically, take a look at
> [`programmatic navigation`](./navigation/programmatic-navigation.md).

```rust
{{#include ../docs-router/src/doc_examples/history_buttons.rs:history_buttons}}
```

As you might know, browsers usually disable the back and forward buttons if
there is no history to navigate to. The router's history buttons try to do that
too, but depending on the [history provider] that might not be possible.

Importantly, neither `WebHistory` supports that feature.
This is due to limitations of the browser History API.

However, in both cases, the router will just ignore button presses, if there is
no history to navigate to.

Also, when using `WebHistory`, the history buttons might
navigate a user to a history entry outside your app.

[`GoBackButton`]: https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.GoBackButton.html
[`GoForwardButton`]: https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.GoForwardButton.html
