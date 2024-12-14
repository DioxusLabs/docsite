# Adding a Backend

Dioxus is a *fullstack* framework, meaning it allows you to seamlessly build your frontend alongside your backend.

We provide a number of utilities like *Server Functions*, *Server Futures*, and *Server State* for you to integrate into your apps. In this chapter, we'll cover loading and saving state to our backend with *Server Functions*. For an in-depth guide on fullstack, check out the dedicated [Fullstack Guide](../guides/fullstack/index.md).

## Enabling Fullstack

Before we can start using server functions, we need to enable the "fullstack" feature on Dioxus in our Cargo.toml.

```toml
[dependencies]
dioxus = { version = "0.6.0", features = ["fullstack"] }
```

We also need to add the "server" feature to our app's features, also in the Cargo.toml

```toml
[features]
default = ["web"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
server = ["dioxus/server"] # <----- add this additional platform
```

If you selected *yes* to the "use fullstack?" prompt when creating your app, you will already have this set up!

> Unfortunately, `dx` doesn't know how to hot-reload this change, so we'll need to kill our currently running `dx serve` process and launch it again.

Give your app a moment to build again and make sure that the "fullstack" feature is enabled in `dx serve`.

![Fullstack Enabled](/assets/06_docs/serve_with_fullstack.png)

## Our First Server Function

In Dioxus, Server Functions are async functions annotated with the `#[server]` attribute. Server Functions allow you to communicate with your server backend without writing any explicit web-server code.

Now, `dx` is compiling two versions of your app twice parallel:
- The client build with `--feature web`
- The server build with `--feature server`

When server functions are called on the client, they make an HTTP request to the corresponding endpoint on the server. When server functions are called on the server, the body of the server function is ran.

Let's add a server function to our dog app to save our favorite dog photos:

```rust
fn DogView() -> Element {
    let mut img_src = use_resource(/**/);

    let save = move |_| async move {
        // Grab the current image
        let current = img_src.value().cloned().unwrap();

        // Start fetching a new image
        img_src.restart();

        // And call the `save_dog` endpoint
        save_dog(current).await;
    };

    let skip = move |_| img_src.restart();

    rsx! {
        // ...
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: save, id: "save", "save!" }
        }
    }
}

// And then on the server, write the dog to "dogs.txt"
#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;
    _ = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap()
        .write_fmt(format_args!("{image}\n"));
    Ok(())
}
```

Wow, our app is really coming together!

![Working Server Functions](/assets/06_docs/dog-save-serverfn.mp4)
