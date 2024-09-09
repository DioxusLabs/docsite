# TUI

You can build a text-based interface that will run in the terminal using Dioxus.

![Hello World screenshot](https://github.com/DioxusLabs/rink/raw/master/examples/example.png)

> Note: this book was written with HTML-based platforms in mind. You might be able to follow along with TUI, but you'll have to adapt a bit.

## Support

Development of dioxus-tui is currently paused while we focus on the backbone of dioxus' native render [Blitz](https://github.com/DioxusLabs/blitz). In the future, dioxus-tui may be updated to use the same backend as Blitz. If you are interested in contributing to dioxus-tui, pull requests are welcome.

Dioxus TUI is currently quite experimental. But, if you're willing to venture into the realm of the unknown, you can use the `dioxus-tui` crate to build interactive apps with a limited subset of HTML.

- It uses flexbox for the layout
- It only supports a subset of the attributes and elements
- Regular widgets will not work in the tui render, but the tui renderer has its own widget components that start with a capital letter. See the [widgets example](https://github.com/DioxusLabs/blitz/blob/master/packages/dioxus-tui/examples/widgets.rs)
- 1px is one character line height. Your regular CSS px does not translate
- If your app panics, your terminal is wrecked. This will be fixed eventually
