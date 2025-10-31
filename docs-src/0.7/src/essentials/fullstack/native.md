# Native Clients

So far, we've focused on using fullstack alongside a web application. However, not all apps you'll build with Dioxus will be loaded in a web browser. Dioxus supports mobile apps and desktop apps as well.

On these platforms, fullstack works a bit differently. You can still use server functions, but things like server-side-rendering no-longer apply.

## Developing a Native Fullstack App

Developing a native fullstack app works just the same as developing a fullstack web app. Make sure your `Cargo.toml` has the appropriate features:

```toml
[dependencies]
dioxus = { version = "0.7", features = ["fullstack"] }

[features]
server = ["dioxus/server"]
desktop = ["dioxus/desktop"]
```

And then, to serve the app, use `dx serve --<platform>`:

```sh
dx serve --desktop
```


## Server Functions for Native App

When you build a native app that relies on server functions, you can freely call any server function just as you would with a web app.

This simple hello world app works the same on web, desktop, and mobile:

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(|| {
        let mut message = use_action(get_message);

        rsx! {
            h1 { "Server says: "}
            pre { "{message:?}"}
            button { onclick: move |_| message.call("world".into(), 30), "Click me!" }
        }
    });
}

#[get("/api/{name}/?age")]
async fn get_message(name: String, age: i32) -> Result<String> {
    Ok(format!("Hello {}, you are {} years old!", name, age))
}
```

Your native code can still make requests to your backend. However, when if you deployed your app to production, you might notice that the native app *does not know where to make requests*.

In the web, requests are always made to the current host origin. For example, requests to this endpoint are made to `/api/dogs`:

```rust
#[get("/api/dogs")]
async fn get_message() -> Result<()> {
    // ..
}
```

Desktop and mobiles are not served from a specific URL, and thus do not know which host to make a request to.

To set a specific server URL, you must call `set_server_url` before making any requests:

```rust
fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::fullstack::set_server_url("https://hot-dog.fly.dev");

    dioxus::launch(app);
}
```

This ensures that requests are always properly joined with the correct host.

## Disabled Fullstack Features

When using fullstack with native apps, a number of features and optimizations are disabled. Native apps are usually meant to be used offline, so their rendering needs to happen entirely on the client. Architecturally, native apps are similar to single-page-applications (SPA) where the bundle loads and *then* HTTP requests are made to load content.

As such, a number of functions are disabled:

- There is no hydration context, and thus no hydration data to hydrate the page
- The app is never rendered on the server, skipping `[cfg(feature = "server")]` code
- There is no `FullstackContext` when rendering components
- HTML Streaming and SSG have no effect

Functionally, this won't change how you build your apps, but you should be aware that some code might never be executed.


## Prefer Known Endpoints

Dioxus supports two ways of annotating server functions:

- Explicitly with `#[get]`, `#[post]`, `#[delete]`, etc.
- Anonymously with `#[server]`

When you use the `#[server]` macro, the endpoint path is free to change as you update the endpoint's signature and body. If you re-redeploy your backend, URLs that worked previously are not guaranteed to exist in the future.


```rust
// ❌ this endpoint generates `/api/do_it12nldkj2378jnakls`
#[server]
async fn do_it() -> Result<()> {
    //
}
```

Because the endpoint receives a hash, its name is unique and might change as you update its code.

When using server functions with native clients, we strongly recommend using the `#[get]` / `#[post]` annotations since they guarantee a stable endpoint.

```rust
// ✅ This endpoint is stable
#[post("/api/do_it")]
async fn do_it() -> Result<()> {
    //
}
```

## Versioning

Because native clients are distributed as downloadable software, they might not always up to date as the latest version of your code. This can be particularly challenging when you want to update the signature of an endpoint.

```rust
// version 1
#[post("/api/do_it")]
async fn do_it() -> Result<()> { /* */ }

// version 2
// ❌ we are breaking old clients!
#[post("/api/do_it")]
async fn do_it(name: String) -> Result<()>  { /* */ }
```

We hope this problem is obvious to you - if you re-redploy your backend with breaking changes, previous clients will break!

To upgrade APIs, we recommend one of two options:

- Use `Option<T>` to add new fields
- Use `/api/v1`, `/api/v2/` versioning

For most API upgrades, you can simply add new fields by making new values optional with `Option<T>`:

```rust
// version 2
// ✅ Option<String> means `name` is not required
#[post("/api/do_it")]
async fn do_it(name: Option<String>) -> Result<()>  { /* */ }
```

For API changes that require much larger changes, we recommend using a versioning scheme to create different versions of the API accessible by the client:

```rust
// This is at /api/v1/
#[post("/api/v1/do_it")]
async fn do_it() -> Result<()> { /* */ }

// This is at api/v2
#[post("/api/v2/do_it")]
async fn do_it(name: String) -> Result<()> { /* */ }
```

Creating different API versions is common practice and helps prevent breaking old clients as you update your app.

## Deploying

You can deploy native fullstack apps just the same as you would deploy a regular web app. Server apps always generate a `server` binary and a `/public/` folder. Native apps will generate an app bundle (`.app`, `.ipa`, `.apk`, etc).

You can distribute the native app via an app store or by making the file downloadable to your users.

To distribute the server, simply upload it to a hosting provider of your choice. As long as you set the `server_url` in the native app, you should be able to access your backend from the native client.

