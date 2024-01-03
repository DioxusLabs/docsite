# Translating existing HTML

Dioxus uses a custom format called RSX to represent the HTML. RSX is more concise than HTML and looks more like normal Rust code. If you have existing HTML, you can translate it into RSX using the `dx translate` command.

You can use the `--file` flag to translate an HTML file to RSX:

```sh
dx translate --file index.html
```

Or you can use the `--raw` flag to translate a string of HTML to RSX:

```sh
dx translate --raw "<div>Hello world</div>"
```

Both of those commands will output the following RSX:

```rs
div { "Hello world" }
```

The `dx translate` command will output the RSX to stdout. You can use the `--output` flag to write the RSX to a file instead.

```sh
dx translate --raw "<div>Hello world</div>" --output index.rs
```

You can automatically create a component with the `--component` flag.

```sh
dx translate --raw "<div>Hello world</div>" --component
```

This will output the following component:

```rs
fn component(cx: Scope) -> Element {
   cx.render(rsx! {
      div { "Hello world" }
   })
}
```

To learn more about the different flags `dx translate` supports, run `dx translate --help`.
