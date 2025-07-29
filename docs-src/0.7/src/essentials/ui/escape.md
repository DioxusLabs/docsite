# Escaping RSX

While RSX is extremely capable, *some* changes to the UI require low-level access to system widgets or an imperative programming approach. Dioxus provides a number of "escape hatches," letting you use alternative UI techniques that don't fit into the declarative RSX paradigm. Also, Dioxus is relatively young and might not expose support for *every* feature the platform-specific renderer has.

## Custom Element Attributes

Under the hood, Dioxus declares every element attribute as a set of constants that describe the attribute's static properties. The definition is roughly equivalent to this pseudocode:

```rust
struct video;
impl video {
    const src: AttributeDefinition = AttributeDefinition {
        name: "src",
        namepsace: None,
        type: String
    };
    const autoplay: AttributeDefinition = AttributeDefinition {
        name: "controls",
        namepsace: None,
        type: Boolean
    };
}

//
rsx! {
    video { src: "...", autoplay: true }
}
```

In some cases, an attribute declaration might be missing, or you need to use a custom name. RSX enables this with quote-wrapped attribute names. Simply wrap the name of the custom attribute and pass in any expression that evalutes to `IntoAttributeValue`:

```rust
rsx! {
    div {
        "data-my-button-id": 123
    }
}
```

Custom attributes can be extremely handy when using data attributes and custom CSS selectors.

## Dangerous Inner HTML

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

## Web Components

While we generally suggest creating new components in Dioxus, some components might be distributed as a [*Web Component*](https://www.webcomponents.org). Web components provide a framework-agnostic way of building and distributing custom HTML elements.

Any element with a dash in the name is a web component. Web components are rendered directly in dioxus without type checking. Generally, you'll be importing a web component *into* Dioxus. We therefore recommend wrapping web components in a type safe component to make them easier to use.

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:web_component}}
```

## Direct DOM Access

As mentioned earlier, RSX is *declarative.* You compose your elements in an `rsx! {}` block and Dioxus does the rendering. Sometimes, you'll need lower-level access to DOM elements to build deeply interactive UI. This might involve rendering into a canvas element or performing a synchronous modification to an element's properties.

To get direct access to the underlying [HTML DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Introduction), Dioxus provides a few built-in mechanisms:

- Running JavaScript through `eval`
- Using [`web-sys`](https://docs.rs/web-sys/latest/web_sys/) to get element by ID
- Using the `onmounted` event handler
- Using the `onresize` and `onvisible` events

### Eval

Dioxus exposes an `eval` function that allows you to evaluate arbitary JavaScript in the platform renderer. On the web, this uses [`web-sys`](https://docs.rs/web-sys/latest/web_sys/), and for the webview renderer, this uses the webview's native `eval` method.

You can eval any valid JavaScript. Dioxus transforms the input source code into a `Function` declaration and then allows you to capture its result. To send custom data into JavaScript, we simply format the source code. To return data from JavaScript, we use `dioxus.send()` and the eval's `.recv()` method:

```rust
use dioxus::prelude::*;

fn app() -> Element {
    rsx! {
        button {
            onclick: move |_| async move {
                // You can send values from rust to the JavaScript code by using the `send` method on the object returned by `eval`.
                let mut eval = document::eval(
                    r#"for(let i = 0; i < 10; i++) {
                        // You can send values asynchronously with the `dioxus.send()` method.
                        dioxus.send(i);
                    }"#
                );

                // You can receive values from the JavaScript code with the `recv` method on the object returned by `eval`.
                for _ in 0..10 {
                    let value: i32 = eval.recv().await.unwrap();
                    println!("Received {}", value);
                }
            },
            "Log Count"
        }
    }
}
```

### Using `web-sys` and Event Downcasting

On the web, it's possible to use the [`web-sys`](https://docs.rs/web-sys/latest/web_sys/) crate to directly call JavaScript methods from Rust. This uses foreign-function-interfaces to bridge the gap between Rust and JavaScript. We don't necessarily suggest using web-sys in *all* cases since web-sys is currently not portable to the Dioxus Desktop and Mobile renderers.

With web-sys, we can call most JavaScript methods with a strongly-typed Rust interface:
```
rsx! {
    let alert_it = move |_| {
        let window = web_sys::window().unwrap();
        window.alert_with_message("Hello from Rust!");
    };
    button { onclick: alert_it, "Click to alert" }
}
```

If you *need* to use web-sys on desktop and mobile, you might want to use [Dioxus with Tauri](#using-dioxus-in-tauri) instead.

### Using `getElementById`

When using either eval or web_sys, it can be useful to get a direct handle to an HTML Dom Node. Generally, we suggest creating an ID for a given element, and then using `getElementById` to reference that node later.

```rust
// Set a unique ID for the div
let id = use_hook(|| Uuid::new_v4());

// Reference it in our eval logic
let set_div_contents = move |_| async move {
    dioxus::document::eval(format!(r#" document.getElementById("{{div-{id}}}").innerText = "one-two-three" "#)).await;
};

rsx! {
    // And then assign it on the element itself
    div { id: "div-{id}" }
    button { onclick:  set_div_contents, "Set Div Directly" }
}
```

## Child Windows and Overlays

Some apps require low-level access to the HTML [`canvas`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/canvas) element. For example, computer-aided-design (CAD) apps, video games, photo editors, video editors, or anything relying on lots of multimedia might want to render GUI textures manually.

When using the webview renderer, you can overlay your Dioxus HTML content *on top* of a native texture. This is an advanced escape hatch that unlocks the entire rendering pipeline - extremely powerful but also quite complex.

![wgpu-windows.mp4](/assets/06assets/wgpu-windows.mp4)

For more information, see the [accompanying example](https://github.com/DioxusLabs/dioxus/blob/main/examples/wgpu_child_window.rs).

## Using Dioxus in Tauri

If you *need* to use the [`web-sys`] crate, even on on desktop and mobile platforms, then [Tauri](https://tauri.app) might be useful for you. Tauri is a framework that lets you combine a custom frontend across an IPC boundary with Rust code running natively. This is somewhat similar to Dioxus, but instead of your UI code running natively, it insteads runs as WebAssembly *inside* the webview. This can be slower and harder to setup, but does enable direct DOM access across all platforms.

Dioxus is supported as a configuration option when creating a new Tauri app, so make sure to check out [their docs as well](https://tauri.app/start/).

## Native Widgets

The first-party Dioxus renderers currently only render HTML and CSS. This is either done via HTMLDomElement with web_sys or through an eval-equivalent for the webview renderers. Sometimes, you want to render a platform-native widgets. Internally, Dioxus uses this to open file dialogs, color pickers, and provide functionality where platform-consistency matters.

![File Upload Dialog](/assets/07/file-upload.png)

This is somewhat advanced and requires using FFI into Objective-C and Kotlin to use system libraries.

## Dioxus Native

For *complete* control over the DOM, we've built the *Dioxus Native* renderer. This renderer combines platform-native widgets along with a stripped-down HTML/CSS renderer to paint the Dioxus widget tree directly to the screen. Currently, Dioxus Native supports a variety of backends based on the [Vello](https://github.com/linebender/vello) crate.

Dioxus Native is nascent. We specifically built Dioxus-Native to unlock the ability to paint arbitrary GPU textures into canvas elements. Instead of using child windows and overlays, you can directly pump WGPU textures in HTML elements.

![WGPU Native](/assets/07/wgpu-native-overlay.mp4)

See the [dioxus-native crate](https://crates.io/crates/dioxus-native) for more information.
