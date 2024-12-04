Welcome back, get your snacks, Dioxus 0.4 just dropped.

If you’re new here: Dioxus (dye•ox•us) is a library for building React-like user interface in Rust. Dioxus supports a ton of targets: web, desktop, mobile, TUI, and more.

Dioxus 0.4 is adding support for the next frontier: the server backend.

You can now write your entire fullstack web app in one file.

![meme](/assets/static/04meme.png)

The gist of this release:

- Server functions
- Server-compatible suspense
- Overhauled (and beautiful!) interactive documentation
- Overhauled and supercharged new router
- First-party support for Android with new getting started guides
- Backend-agnostic hooks for common app uses cases
- Desktop Hot Reloading
- Tauri-bundle built into the Dioxus CLI
- Polish, bug fixes, stability, testing, and more!

## Weekly Office Hours


Before we dive right into the bulk of this release, we want to make sure everyone knows that Dioxus Labs now has weekly office hours, every Friday at 9am PST.

These are held on the community Discord - with an invite here:

[Join the Dioxus Labs Discord Server!](https://discord.gg/XgGxMSkvUM)

In the office hours you can get help with your app, learn how to contribute, get feedback on code, and [help shape the roadmap.](https://www.notion.so/Dioxus-Labs-Public-Roadmap-771939f47d13423abe2a2195b5617555?pvs=21) We hope to see you there!

## Server Functions


These days, every cool UI library has some sort of backend framework to do server stuff. This could be interacting with a database, uploading files, working with websockets, you name it. With Dioxus 0.4, we’re adding our first backend solution: Server Functions.

Server Functions are functions annotated with the `server` procedural macro that generates an RPC client and server for your app. With a single function declaration, you get both the server endpoint *and* the client required to interact with the endpoint.

For example, take this simple Server Function. We’re using the awesome [turbosql](https://github.com/trevyn/turbosql) crate by [trevyn](https://github.com/trevyn) to interact with a sqlite database to load a person’s username.

```rust
#[server]
async fn get_username() -> Result<String> {
	// Using turbosql to extract some data from the DB
	Ok(select!(String "SELECT name FROM person")?)
}
```

The `server` macro will generate a different helper function depending on the configuration flags of your project. If you build your project for the backend (`server`), then your endpoint will be automatically injected into any axum/salvo/warp router created with `dioxus_fullstack`. However, if you build your project with any other feature flag, like, `client`, then the macro generates the *call* to the server.

![Server / Client code splitting](/assets/static/split_codegen.png)

This approach makes it easy to incrementally add new backend functionality to your app. Of course, this has some disadvantages - the backend is rather tightly coupled to the frontend - but for most apps, Server Functions will be a huge productivity win.

Server Functions are agnostic to the backend framework you’re using, and support a number of generic operations common across axum, warp, and salvo. For instance, you can extract any header from the request, guarding on things like valid headers and cookies:

```rust
use axum::{TypedHeader, headers::UserAgent};
use dioxus_fullstack::extract;

#[server]
fn log_user_agent() -> ServerFnResult {
    let user_agent: TypedHeader<UserAgent> = extract().await?;
    Ok(())
}
```

You can attach middleware either at the server level or individually on server functions. The new fullstack utilities work seamlessly with [Tower](https://docs.rs/tower/latest/tower/index.html), so any server function can be annotated with a middleware.

```rust
// Add a timeout middleware to the server function that will return
// an error if the function takes longer than 1 second to execute

use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tokio::time::sleep;

#[server]
#[middleware(TimeoutLayer::new(Duration::from_secs(1)))]
pub async fn timeout() -> ServerFnResult {
		sleep(Duration::from_secs(2)).await;
    Ok(())
}
```

Combining all these concepts together, you can quickly add features like Authentication to your fullstack app. We’ve put together a [simple axum-auth example for you to get started](https://github.com/dioxuslabs/dioxus/blob/main/packages/fullstack/examples/axum-auth/src/main.rs).

Our goal with Server Functions is to lay the foundation for our final vision of Dioxus: a fullstack, crossplatform, fully typed, and lightning fast toolkit for building, deploying, monitoring, and scaling any app you can dream of. With one ecosystem, you can quickly build complete apps that run on desktop, mobile, web with a type-safe backend to boot.

## Suspense


One feature that has been missing in Dioxus since its release is the ability to wait for futures to complete before generating the final server-side-rendered HTML. Before, the expectation was that you’d load any data *ahead of time,* and pass in the data via Root Props. We quickly learned this caused issues with scalability: you might not want to fetch every piece of props depending on the route.

![Diagram of how SSR data used to be fetched](/assets/static/old_fetch.png)

To solve this, we’re adding future-compatible `Suspense` - now integrated with Dioxus Fullstack and Server Functions.  Suspense is freely available to components via the `cx.suspend()` method. Calling `suspend` will tell Dioxus that this particular component is blocking the completion of the final render due to a pending future. The most basic usage of Suspense is pausing rendering until a data fetch has been completed:

```rust
fn Username(cx: Scope) -> Element {
	let username = use_future(cx, (), |_| get_username());

	// Try to extract the current value of the future
	let Some(username) = username.value() else {

		// Early return and mark this component as suspended
		return cx.suspend();
	}

	render! { "Username: {username}")
}
```

Now, we can do datafetching *inside* components, greatly simplifying our project structure. With the new `use_server_future` hook, we can persist the result of the fetch between server render and client render, allowing hydration to work seamlessly.

```rust

fn Dashboard(cx: Scope) -> Element {
    // use_server_future will persist the result of this call during SSR
    let ip = use_server_future(cx, (), |_| get_server_ip())?;

    render!{ "The edge node is {ip}" }
}

// When used on the server, this is just a simple function call
#[server]
async fn get_server_ip() -> Result<String> {
    Ok(reqwest::get("https://httpbin.org/ip").await?.text().await?)
}
```

With inline datafetching, we can greatly simplify the amount of work required to build fullstack apps. In this diagram of a Dioxus app with suspense, we can see that suspended components are marked yellow. When their futures completed, Dioxus will continue rendering them, cascading into their children until all futures have been resolved.

![Diagram of how SSR data is fetched now](/assets/static/new_fetch.png)

Adopting suspense and fullstack should be easy. Now, when you go to render the app, you can simply call `wait_for_suspense()` before pre-rendering the markup:

```rust
let mut app = VirtualDom::new(app_fn);

// Start the futures
app.rebuild();

// Wait for suspended futures to complete
app.wait_for_suspense();

// Now render the app out
dioxus_ssr::prerender(&app);
```

Note that this is not 100% equivalent to React’s suspense as it’s not currently possible to render loading states or error boundaries. These features are currently experimental and will be stabilized during the 0.4 release cycle.

## Enum Router


Since the initial release of Dioxus, we’ve had a very simple App Router, inspired by the older versions of React Router. Most UI toolkits these days provide a Router object that interacts with the browser’s router, and to date, Dioxus’ router has been pretty simple.

In the beginning we opted for simplicity and flexibility. The old router let you create route trees with just components. This router was easy to add new routes to and easy to compose.

```rust
pub fn app(cx: Scope) -> Element {
    render! {
        Router {
            Nav {}
            Route { to: "/", Homepage {} }
            Route { to: "/awesome", Awesome {}}
            Route { to: "/learn", Learn {} }
            Route { to: "/tutorials/:id", Tutorial {} }
            Route { to: "/blog", BlogList {} }
            Route { to: "/blog/going-fulltime", GoingFulltime {} }
            Route { to: "/blog/release-030", Release03 {} }
            Route { to: "", Err404 {} }
        }
        Footer {}
    }
}
```

However, after thinking about the new features we wanted to add to Dioxus, we realized that this router design wouldn’t cut it in the future. We were lacking many crucial features like nested routes and type-safe URLs.

So, we designed a new router built around the Rust `enum`. Now, you assemble your app’s route by deriving the `Routable` trait for a given enum. Routes are simply enum variants! Even better, everything is 100% typesafe. No longer can you craft invalid URLs - saving your apps from a whole host of problems.

```rust
#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize, Debug)]
enum Route {
	#[route("/")]
	#[redirect("/platforms", || Route::Homepage {})]
	Homepage {},

	#[route("/awesome")]
	Awesome {},

	#[route("/learn")]
	Learn {},

	#[route("/tutorials/:id")]
	Tutorial { id: usize },

	#[route("/blog")]
	BlogList {},

	#[route("/blog/:post")]
	BlogPost { post: String },

	#[route("/:..segments")]
  Err404 { segments: Vec<String> },
}
```

To render the new router, pass in your app’s Route enum as the generic type in the Router, and Dioxus will render the right component, given that the enum variant.

```rust
fn app(cx: Scope) -> Element {
	render! { Router::<Route> {} }
}
```

The `#[derive(Routable)]` will automatically generate the `render` function for your Router. The Router will render the right route given that a similarly named component is in scope. You can override this yourself, or just stick with the default. The simplest app using the new router would look something like this:

```rust

// 1. Declare our app's routes
#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]
enum Route {
	#[route("/")]
	Homepage {},
}

// 2. Make sure we have a component in scope that matches the enum variant
fn Homepage(cx: Scope) -> Element {
	render! { "Welcome home!" }
}

// 3. Now render our app, using the Router and Route
fn app(cx: Scope) -> Element {
	render! { Router::<Route> {} }
}
```

Passing in attributes from the route itself is easy too. The `Routable` macro will accept any fields that implement `FromStr`, so you can easily add attributes, queries, and named parameters to your app.

```rust
// Declare our app's routes
#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]
enum Route {
	#[route("/blog/:post")]
	BlogPost { post: String },
}

#[component]
fn BlogPost(cx: Scope, post: String) {
	render!{ "Currently viewing: {post}" }
}

```

Likewise, you can catch 404s using the trailing segments syntax.

```rust
// Declare our app's routes
#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]
enum Route {
	#[route("/:..segments")]
  Err404 { segments: Vec<String> },
}
```

Another exciting feature is layouts. We’re borrowing this concept from popular frameworks like Remix and Next.JS. Layouts make it easy to wrap Route components together in a shared set of components. A common use case is wrapping your app in a Header, Footer, or Navbar. Without layouts, you’d have a lot of code duplication

```rust
fn Home(cx: Scope) -> Element {
	render! {
		Header {}
		Navbar {}
		div { "actual home content" }
		Footer {}
	}
}

fn Blog(cx: Scope) -> Element {
	render! {
		Header {}
		Navbar {}
		div { "actual blog content" }
		Footer {}
	}
}
```

Now, with layouts, you can declare your layout in the Route enum itself.

```rust
#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]
enum Route {
  #[layout(HeaderFooterNav)]
		#[route("/")]
		Home {},

		#[route("/blog/")]
		Blog {},
}

// Wrap the rendered content of the Router with a header, navbar, and footer
fn HeaderFooterNav(cx: Scope) -> Element {
	render! {
		Header {}
		Navbar {}
		Outlet::<Route> {}
		Footer {}
	}
}
```

The new router was an absolutely massive amount of work, spearheaded by @TeFiLeDo, improved by @ealmloff, and made possible thanks to community members like @stephenandary and @attilio-oliva.

![PR of enum router](/assets/static/enum_router.png)

## New and beautiful interactive docs


It’s about time we revamped our documentation. When we launched, the original docs were pretty good, at least for a 0.1 crate. Now, we have over 16 crates in the main repo with another half dozen scattered around the github organization. New users deserve a better introduction experience.

To start, we revamped our landing page. Short, sweet, to the point.

![Screenshot of new doc site landing page](/assets/static/landing_1.png)

At the top of the page, you’ll see a new search bar. You can search for anything in our docs with just a `ctrl+/` .  This new search functionality uses a [new search crate we designed and built](https://github.com/dioxusLabs/dioxus-search). `Dioxus-search` is fully crossplatform and ready to use in your next Dioxus app.

![Screenshot of new doc site search](/assets/static/landing_2.png)

Selecting any of the search results will take you straight to the docs page without bouncing out to an external mdbook.

![Screenshot of new doc site mdbook](/assets/static/landing_3.png)

We’ve added a bunch more content to the docs, including new pages like:

- Building a hackernews clone from scratch
- Setup guides for mobile
- Suggestions for useful crates
- How to bundle your app
- Working with server functions

The best part? The docs are interactive! Examples like the `hello-world` and even the `hackernews` clone come to life from within the docs page.

![Screenshot of new doc site interactivity](/assets/static/landing_4.png)

We also moved the `awesome` dioxus page from GitHub to the docsite, so you can explore the various crates that other developers have submitted as “awesome.”

![Screenshot of new doc site awesome page](/assets/static/landing_5.png)

The new docs leverage many of the amazing new features from the router, including:

-

## Android Support, iOS fixes, Getting Started Guide for Mobile


To date, Dioxus has provided first-party support for mobile via iOS, but our Android support has been rather spotty and untested. In this release, we finally added iOS and Android testing to our suite of continuous integration. To round off mobile support, we’ve added a [mobile-specific getting started guide](https://dioxuslabs.com/learn/0.4/getting_started/mobile) with a walkthrough on setting up platform-specific dependencies, handling basic cross-compilation, and booting up a mobile simulator. We’ve also fixed some bugs in upstream libraries like Tauri’s Tao which gives Dioxus its window-creation capabilities.

iOS Demo:

![Screenshot of xcode with dioxus app](/assets/static/ios_demo.png)

Android Demo:

![Screenshot of android studio with dioxus app](/assets/static/android_demo.png)

## Window-Close Behavior


Another great contribution from the community: Dioxus desktop now provides functionality for managing the “close” behavior of windows in your app. We support three modes now:

- Closing the last window kills the app
- Closing the last window doesn’t kill the app
- Closing the last window simply hides the window (so the window can be cmd-tabbed back)

![window_close.mov](https://i.imgur.com/m4wJ6gN.mp4)

## Bidirectional Eval


The use_eval hook allows you to run snippets of Javascript in your Dioxus application when needed. @doge has made some improvements that make this hook has significantly more powerful. The new version of the hook is compatible between the desktop, web, and Liveview renderers. It also allows you to send messages to and from Javascript asynchronously. This makes it possible to listen for Javascript events that Dioxus doesn’t officially support like the intersection observer API.

```rust
use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let eval = use_eval(cx);

    let future = use_future(cx, (), |_| {
        to_owned![eval];
        async move {
            // Eval some javascript
            let eval = eval(
                r#"dioxus.send("Hi from JS!");
                let msg = await dioxus.recv();
                console.log(msg);
                return "hello world";"#,
            )
            .unwrap();

            // Send messages into the running javascript
            eval.send("Hi from Rust!".into()).unwrap();

            // Receive messages from the javascript
            let res = eval.recv().await.unwrap();

            // Wait for it to complete
            println!("{:?}", eval.await);

            res
        }
    });

    render!{ "{future.value():?}" }
}
```

## New onmount event


This release also introduces a new onmounted event that provides access to elements after they are created in a cross platform way. The onmounted event makes it possible to:

- Scroll to an element
- Read the size of an element
- Focus an element
- Get the platform specific element

```rust
use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let header_element = use_state(cx, || None);

    cx.render(rsx!(
        div {
            h1 {
                onmounted: move |cx| {
                    header_element.set(Some(cx.inner().clone()));
                },
                "Scroll to top example"
            }

            for i in 0..40 {
                div { "Item {i}" }
            }

            button {
                onclick: move |_| {
                    if let Some(header) = header_element.as_ref() {
                        header.scroll_to(ScrollBehavior::Smooth);
                    }
                },
                "Scroll to top"
            }
        }
    ))
}
```

![Scroll demo](https://i.imgur.com/yp7GyIf.mp4)

## Renaming dioxus-cli to dx


We made a small tweak to the CLI this release to rename the CLI from `dioxus` to `dx`. This is a tiny change but has a few benefits:

- It’s faster to type
- It emphasizes the *developer experience* of Dioxus
- It keeps our tooling agnostic to other projects that might want to take advantage of features like hotreloading, autoformatting, wasm-pack, tailwind integration, plugins, and more.

To install the new CLI, use the same old instructions:

```rust
cargo install dioxus-cli --force
```

## Hot Reloading for Desktop


Yet another small tweak the CLI: you can now use `dx serve` on desktop apps with hot reloading enabled! Using the same hot reloading engine that powers web, `dx serve` can now modify your currently-running desktop without causing a full rebuild. In the event that we can’t hot reload a particular change, then `dx serve` will shutdown the app and restart it with the new changes applied.

![Hotreloading on desktop](https://i.imgur.com/ML93XtT.mp4)

## Dioxus-Bundle


So far, the CLI has supported useful commands like `dx fmt` , `dx build` , `dx serve` . Until date, we haven’t provided a command to build a final distributable image of your app. In 0.4, we’re incorporating cargo-bundle support into the Dioxus CLI. Now, from the Dioxus CLI, you can bundle your app using the same configuration options as the Tauri bundler, making it easy to migrate existing projects over. `dx bundle` supports bundling apps for macOS, iOS, Windows, and Linux (.deb, .rpm).

![A bundled app on macos](/assets/static/bundle.png)

This is a great place for new contributors to help flesh out the ecosystem!

## Dioxus-Check


The Dioxus CLI has a new helper command: `dx check`. Thanks to the work from community member @eventualbuddha, `dx check` will now identify and help mitigate issues like hooks being called inside conditionals and loops.

![The new check command for dx](/assets/static/dxcheck.png)

These lints make it easier for newcomers to Dioxus to get started, especially if they’re not super familiar with React.

## VSCode Extension Updates


As part of improving stability and fixing bugs, we’ve made some improvements to the VSCode Extension.

- We fixed a bug with activation events where the extension wouldn’t activate until one of the commands were manually fired
- We fixed a handful of autoformatting bugs around event handlers
- We’ve moved the code for the extension out of the CLI and into a small WebAssembly binary so you don’t need the CLI installed and version matched

![The Dioxus VSCode extension page](/assets/static/extension.png)

The extension is a great place for new contributors to dive into the Dioxus codebase!

## General Fixes


- Several SSR and Hydration bugs were fixed including escaping text, and
- We have improved testing around Dioxus. We have added end to end tests for each platform and fuzzing tests for core libraries.
- Fix the provide context docs
- Fix trait bounds on generics with component
- Fix hot reloading in a workspace
- Fix anchor link for block-level elements
- Fix Guide link in README
- Fix new Clippy lints
- Fix event bubbling within a single template
- Fix panic when waking future on shutdown
- Fix style attributes in SSR
- Fix more complex workspaces with hot reloading
- Fix non-bubbling listener hydration
- Hotfix wry segfaulting
- Fix dangerous_inner_html with SSR
- Fix Linux Wry dependencies
- Fix native core dependencies in a different direction than the pass direction
- Fix raw attribute values in SSR
- fix: Update logos and custom assets example
- Fix non-ascii string decoding
- fix rng in svg dice example
- Fix clippy in the generic component example
- fix: change crossterm poll timeout to 10ms from 1s
- Fix HTML to RSX conversions example
- Fix LiveView Tokio features
- Fix non-bubbling event propagation
- Fix selected and checked with boolean attributes
- Fix form events with select multiple
- Fix click and input events on desktop
- Fix the link to tui widgets in the guide
- Fix duplicate example and test names
- fix router book link
- Fix web events starting on a text node
- Fix links in Liveview
- Fix URL encoded desktop assets
- Fix ssr guide examples
- Fix hot reloading with namespaces
- Add initial_value attribute & fix static special attribute handling
- Fix liveview interpreter JS
- Fix autoformatting raw strings
- Fix panic when events trigger on placeholders
- fix: Remove dbg that was causing TUI rendering problems
- Fix restarting MacOs hot reloading after a forced exit
- Fix `cargo doc`
- Fix: Remove play button from guide example
- Fix: bump liveview salvo version to 0.44.1. (#1124)
- fix: Remove conflicting rustfmt config for cli
- fix(docs): Fix dioxus-hooks crate description
- Fix CLI testing issue
- Fix boolean attributes with raw boolean values
- fix: Properly drop UseFuture's values to avoid leaking memory
- Fix the onload event
- fix: Fix stop_propagation example
- Fix playwrite tests
- Fix playwright fullstack clippy
- fix: readme awesome link
- fix: Remove duplicated doc links and improved some descriptions
- Fix the issue of duplicate unique ID for atoms using `newtype`.
- fix(cli): improve error message for invalid config
- fix: Update use_ref docs
- Fix playwright (again) and move the playwright stuff into the playwright directory
- Fix: dont use bumpslab anymore, just box scopestates
- Fix race condition in use_future
- Fix use_future always spawning a new task and never updating
- Fix route prerendering
- fix: Use correct cfg for file_watcher feature in dioxus-hot-reload
- docs(clean): fix copy-paste docs from `build`
- fix: Update use_coroutine example
- fix(playwright): remove unnecessary `await`s
- Fix all broken examples
- Fix root component suspense
