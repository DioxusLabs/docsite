Here at Dioxus Labs, we have an unofficial rule: only one rewrite per year.

Our last rewrite brought some amazing features: templates, hot reloading, and insane performance. However, don’t be mistaken, rewrites are scary, time consuming, and a huge gamble. We started this new rewrite on January 1st of 2024, completed it by Feburary 1st, and then spent another month and a half writing tests, squashing bugs, and polishing documentation. Rewrites are absolutely not for the faint of heart.

If you’re new here, Dioxus (dye•ox•us) is a library for building GUIs in Rust. Originally, I built Dioxus as a rewrite of Yew with the intention of supporting proper server-side-rendering. Eventually, Dioxus got popular, we got some amazing sponsors, and I went full time. We’ve grown from a team of 1 (me) to a team of 4(!) - pulled entirely from the wonderful Dioxus community.

Now, Dioxus is something a little different. Real life, actual companies are shipping web apps, desktop apps, and mobile apps with Dioxus. What was once just a fun little side project powers a small fraction of apps out in the wild. We now have lofty goals of simplifying the entire app development ecosystem. Web, Desktop, Mobile, all end-to-end typesafe, blazing fast, living under one codebase. The dream!

With 0.5 we took a hard look at how Dioxus would need to change to achieve those goals. The request we got from the community was clear: make it simpler, make it robust, make it polished.

## What’s new?

This is probably the biggest release of Dioxus ever, with so many new features, bug fixes, and improvements that I can’t list them all. We churned over 100,000 lines of code (yes, 100,000+) with over 1,400 commits between 0.4.3 and 0.5.0. Here’s a quick overview:

*   Complete rewrite of `dioxus-core`, removing all unsafe code
*   Abandoning `use_state` and `use_ref` for a clone-free `Signal`\-based API
*   Removal of all lifetimes and the `cx: Scope` state
*   A single, unified `launch` function that starts your app for any platform
*   Asset hot reloading that supports Tailwind and Vanilla CSS
*   Rewrite of events, allowing access to the native `WebSys` event types
*   Extension of components with element properties (IE a Link now takes all of `<a/>` properties)
*   Integrated Error Boundaries and Server Futures with Suspense integration
*   5x faster desktop reconciliation and custom asset handlers for streaming bytes
*   Streaming server functions and Fullstack hot reloading
*   Tons of QoL improvements, bug fixes, and more!

💡 If you are updating from Dioxus 0.4, a [`migration guide`](https://dioxuslabs.com/learn/0.5/migration) is available

## Lifetime Problems

To make Dioxus simpler, we wanted to remove lifetimes entirely. Newcomers to rust are easily scared off by lifetime issues, and even experienced Rustaceans find wading through obtuse error messages exhausting.

In Dioxus 0.1-0.4, every value in a component lives for a `'bump` lifetime. This lifetime lets you easily use hooks, props and the scope within event listeners without cloning anything. It was the chief innovation that made Dioxus so much easier to use than Yew when it was released.

```rust
// Scope and Element have the lifetime 'bump
fn OldDioxusComponent(cx: Scope) -> Element {
  // hook has the lifetime 'bump
  let mut state = use_state(cx, || 0);
  cx.render(rsx! {
    button {
      // The closure has the lifetime 'bump which means you don't
      // need to clone hook before you move it into the closure
      onclick: move |_event| *state += 1,
    }
  })
}
```

This works great for hooks _most_ of the time. The lifetime lets you omit a bunch of manual clones every time you want to use a value inside an EventHandler (onclick, oninput, etc).

However, the lifetime doesn’t work for futures. Futures in Dioxus need to be `'static` which means you always need to clone values before you use them in the future. Since a future might need to run while the component is rendering, it can’t share the component’s lifetime.

```rust
// Scope and Element have the lifetime 'bump
fn OldDioxusComponent(cx: Scope) -> Element {
    // state has the lifetime 'bump
    let state = use_state(cx, || 0);

    cx.spawn({
        // Because state has the lifetime 'bump, we need to clone it to make it
        // 'static before we move it into the 'static future
        let state = state.clone();
        async move {
            println!("{state}");
        }
    });

    // ...
}
```

If you don’t clone the value, you will run into an error like this:

```shell
4  |   fn OldDioxusComponent(cx: Scope) -> Element {
   |                         --
   |                         |
   |                         `cx` is a reference that is only valid in the function body
   |                         has type `&'1 Scoped<'1>`
...
8  | /     cx.spawn(async move {
9  | |         println!("{state}");
10 | |     });
   | |      ^
   | |      |
   | |______`cx` escapes the function body here
   |        argument requires that `'1` must outlive `'static`
```

The error complains that `cx` must outlive `'static` without mentioning the hook at all which can be very confusing.

Dioxus 0.5 fixes this issue by first removing scopes and the `'bump` lifetime and then introducing a new `Copy` state management solution called signals. Here is what the component looks like in Dioxus 0.5:

```rust
// Element has no lifetime, and you don't need a Scope
fn NewComponent() -> Element {
  // state is 'static and Copy, even if the inner value you store is not Copy
  let mut state = use_signal(|| 0);

  // State is already 'static and Copy, so it is copied into the future automatically
  spawn(async move {
    println!("{state}");
  });

  rsx! {
    button {
      // The closure has the lifetime 'static, but state is copy so you don't need to clone into the closure
      onclick: move |_event| state += 1,
    }
  }
}
```

While this might seem like a rather innocuous change, it has an impressively huge impact on how easy it is to write new components. I’d say building a new Dioxus app is about 2-5x easier with this change alone.

## Goodbye scopes and lifetimes!

In the new version of Dioxus, scopes and the `'bump` lifetime have been removed! This makes declaring a component and using runtime functions within that component much easier:

You can now declare a component by just accepting your props directly instead of a scope parameter

```rust
#[component]
fn MyComponent(name: String) -> Element {
  rsx! { "Hello {name}!" }
}
```

And inside that component, you can use runtime functions directly

```rust
spawn(async move {
  tokio::time::sleep(Duration::from_millis(100)).await;
  // You can even use runtime functions inside futures and event handlers!
  let context: i32 = consume_context();
});
```

Now that lifetimes are gone, `Element`s are `'static` which means you can use them in hooks or even provide them through the context API. This makes some APIs like [virtual lists in Dioxus](https://github.com/matthunz/dioxus-lazy) significantly easier. We expect more interesting APIs to emerge from the community now that you don’t need to be a Rust wizard to implement things like virtualization and offscreen rendering.

## Removal of all Unsafe in Core

Removing the `'bump` lifetime along with the scope gave us a chance to remove a lot of unsafe from Dioxus. **dioxus-core 0.5 contains no unsafe code 🎉**

![No more unsafe in core](https://i.imgur.com/B0kf5Df.png)

There’s still a tiny bit of unsafe floating around various dependencies that we plan to remove throughout the 0.5 release cycle, but way less: all quite simple to cut or unfortunately necessary due to FFI.

## Signals!

Dioxus 0.5 introduces Signals as the core state primitive for components. Signals have two key advantages over the existing `use_state` and `use_ref` hooks: They are always `Copy` and they don’t require manual subscriptions.

### Copy state

`Signal<T>` is `Copy`, even if the inner `T` values is not. This is enabled by our new [generational-box](https://crates.io/crates/generational-box) crate (implemented with zero unsafe). Signals can even optionally be `Send+Sync` if you need to move them between threads, removing the need for a whole class of specialized state management solutions.

The combination of `Copy + Send + Sync` Signals, and static components makes it incredibly easy to move state to anywhere you need it:

```rust
fn Parent() -> Element {
  // We use a sync signal here so that we can use it in other threads,
  // but you could use a normal signal if you have !Send data
  let mut state = use_signal_sync(|| 0);

  spawn(async move {
    // Signals have a ton of helper methods that make them easy to work with.
    // You can call a signal like a function to get the current value
    let value: i32 = state();
  });

  // Because signals can be sync, we can copy them into threads easily
  std::thread::spawn(move || {
    loop {
      std::thread::sleep(Duration::from_millis(100));
      println!("{state}");
    }
  });

  rsx! {
    button {
      // You can easily move it into an event handler just like use_state
      onclick: move |_| state += 1
    }
  }
}
```

With `Copy` state, we’ve essentially bolted on a light form of garbage collection into Rust that uses component lifecycles as the triggers for dropping state. From a memory perspective, this is basically the same as 0.4, but with the added benefit of not needing to explicitly `Clone` anything.

### Smarter subscriptions

Signals are smarter about what components rerun when they are changed. A component will only rerun if you read the value of the signal in the component (not in an async task or event handler). In this example, only the child will re-render when the button is clicked because only the child component is reading the signal:

```rust
fn Parent() -> Element {
  let mut state = use_signal(|| 0);

  rsx! {
    button { onclick: move |_| state += 1, "increment" }
    Child { state }
  }
}

#[component]
fn Child(state: Signal<i32>) -> Element {
  rsx! { "{state}" }
}
```

Smarter subscriptions let us merge several different hooks into signals. For example, we were able to remove an entire crate dedicated to state management: Fermi. Fermi provided what was essentially a `use_state` API where statics were used as keys. This meant you could declare some global state, and then read it in your components:

```rust
static COUNT: Atom<i32> = Atom::new(|| 0);

fn Demo(cx: Scope) -> Element {
  let mut count = use_read_atom(cx, &COUNT);
  rsx! { "{count}" }
}
```

Since fermi didn’t support smart subscriptions, you had to explicitly declare use the right `use_read`/ `use_write` hooks to subscribe to the value. In Dioxus 0.5, we just use signals, eliminating the need for any sort of external state management solution altogether.

```rust
// You can use a lazily initialized signal called
// GlobalSignal in static instead of special Fermi atoms
static COUNT: GlobalSignal<i32> = Signal::global(|| 0);

// Using the GlobalSignal is just the same as any other signal!
// No need for use_read or use_write
fn Demo() -> Element {
   rsx! { "{COUNT}" }
}
```

Signals even work with the context API, so you can quickly share state between components in your app:

```rust
fn Parent() -> Element {
  // Create a new signal and provide it to the context API
  // without a special use_shared_state hook
  let mut state = use_context_provider(|| Signal::new(0));

  rsx! {
    button { onclick: move |_| state += 1, "Increment" }
    Child {}
  }
}

fn Child() -> Element {
  // Get the state from the context API
  let state = use_context::<Signal<i32>>();
  rsx! { "{state}" }
}
```

Smart subscriptions also apply to hooks. Hooks like `use_future` and `use_memo` will now automatically add signals you read inside the hook to the dependencies of the hook:

```rust
// You can use a lazily initialized signal called GlobalSignal in static instead of special Fermi atoms
static COUNT: GlobalSignal<i32> = Signal::global(|| 0);

fn App() -> Element {
  // Because we read COUNT inside the memo, it is automatically added to the memo's dependencies
  // If we change COUNT, then the memo knows it needs to rerun
  let memo = use_memo(move || COUNT() / 2);

  rsx! { "{memo}" }
}
```

## CSS Hot Reloading

As part of our asset system overhaul, we implemented hot reloading of CSS files in the asset directory. If a CSS file appears in your RSX, the `dx` CLI will watch that file and immediately stream its updates to the running app. This works for Web, Desktop, and Fullstack, with mobile support coming in a future mobile-centric update.

When combined with the Tailwind watcher, we now support hot reloading of Tailwind CSS! On top of that, we also support IDE hinting of Tailwind classes in VSCode with a [custom regex extension](https://github.com/tailwindlabs/tailwindcss/discussions/7073)

![CSS Hot reloading](https://imgur.com/CSjVVLL.mp4)

What’s even niftier is that you can stream these changes to several devices at once, unlocking simultaneous hot reloading across all devices that you target:

![CSS Hot reloading](https://i.imgur.com/cZ8qZCz.mp4)

## Event System Rewrite

Since its release, Dioxus has used a synthetic event system to create a cross platform event API. Synthetic events can be incredibly useful to make events work across platforms and even serialize them across the network, but they do have some drawbacks.

Dioxus 0.5 finally exposes the underlying event type for each platform along with a trait with a cross platform API. This has two advantages:

1.  You can get whatever information you need from the platform event type or pass that type to another library:

```rust
fn Button() -> Element {
  rsx! {
    button {
      onclick: move |event| {
        let web_sys_event: web_sys::MouseEvent = event.web_event();
        web_sys::console::log_1(&web_sys_event.related_target.into());
      }
    }
  }
}
```

3.  Dioxus can bundle split code for events apps don’t use. For a hello world example, this shrinks the gzipped size ~25%!

![Smaller bundles](https://i.imgur.com/6hZruyO.png)

Again, this seems like a small change on the surface, but opens up dozens of new use cases and possible libraries you can build with Dioxus.

💡 The [Dioxus optimization guide](https://dioxuslabs.com/learn/0.5/cookbook/optimizing#build-configuration) has tips to help you make the smallest possible bundle

## Cross platform launch

Dioxus 0.5 introduces a new cross platform API to launch your app. This makes it easy to target multiple platforms with the same application. Instead of pulling in a separate renderer package, you can now enable a feature on the Dioxus crate and call the launch function from the prelude:

```toml
[dependencies]
dioxus = "0.5"

[features]
default = []
desktop = ["dioxus/desktop"]
fullstack = ["dioxus/fullstack"]
server = ["dioxus/axum"]
web = ["dioxus/web"]
```

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(|| rsx!{ "hello world" })
}
```

With that single application, you can easily target:

```shell
# Desktop
dx serve --platform desktop

# SPA web
dx serve --platform web

# Or a fullstack application
dx serve --platform fullstack
```

The CLI is now smart enough to automatically pass in the appropriate build features depending on the platform you’re targeting.

## Asset System Beta

Currently assets in Dioxus (and web applications in general) can be difficult to get right. Links to your asset can easily get out of date, the link to your asset can be different between desktop and web applications, and you need to manually add assets you want to use into your bundled application. In addition to all of that, assets can be a huge performance bottleneck.

Lets take a look at the Dioxus Mobile guide in the docsite as an example:

![docsite_mobile_old.png](https://i.imgur.com/f7sGEdJ.png)

The 0.4 mobile guide takes 7 seconds to load and transfers 9 MB of resources. The page has 6 different large image files which slows down the page loading times significantly. We could switch to a more optimized image format like `avif` , but manually converting every screenshot is tedious and time consuming.

Lets take a look at the 0.5 mobile guide with the new asset system:

![docsite_mobile_new.png](https://i.imgur.com/GabzFJm.png)

The new mobile guide takes less than 1 second to load and requires only 1/3 of the resources with the exact same images!

Dioxus 0.5 introduces a new asset system called [manganis](https://github.com/DioxusLabs/manganis). Manganis integrates with the CLI to check, bundle and optimize assets in your application. The API is currently unstable so the asset system is currently published as a separate crate. In the new asset system, you can just wrap your assets in the `mg!` macro and they will automatically be picked up by the CLI. You can read more about the new asset system in the [manganis docs](https://docs.rs/crate/manganis/latest).

As we continue to iterate on the 0.5 release, we plan to add hot reloading to manganis assets, so you can interactively add new the features to your app like CSS, images, Tailwind classes, and more without forcing a complete reload.

## 5x Faster Desktop Rendering

Dioxus implements several optimizations to make diffing rendering fast. [Templates](https://dioxuslabs.com/blog/templates-diffing) let Dioxus skip diffing on any static parts of the rsx macro. However, diffing is only one side of the story. After you create a list of changes you need to make to the DOM, you need to apply them.

We developed [sledgehammer](https://github.com/ealmloff/sledgehammer_bindgen) for Dioxus Web to make applying those mutations as fast as possible. It makes manipulating the DOM from Rust almost as [fast as native JavaScript](https://krausest.github.io/js-framework-benchmark/2023/table_chrome_114.0.5735.90.html).

In Dioxus 0.5, we apply that same technique to apply changes across the network as fast as possible. Instead of using JSON to communicate changes to the Desktop and LiveView renderers, Dioxus 0.5 uses a binary protocol.

For render intensive workloads, the new renderer takes only 1/5 the time to apply the changes in the browser with 1/2 the latency. Here is one of the benchmarks we developed while working on the new binary protocol. In Dioxus 0.4, the renderer was constantly freezing. In Dioxus 0.5, it runs smoothly:

**Dioxus 0.4**
![Desktop performance 0.4](https://i.imgur.com/CX7DREF.mp4)

**Dioxus 0.5**
![Desktop performance 0.5](https://i.imgur.com/3l65D0G.mp4)

## Spreading props

One common pattern when creating components is providing some additional functionality to a specific element. When you wrap an element, it is often useful to provide some control over what attributes are set in the final element. Instead of manually copying over each attribute from the element, Dioxus 0.5 supports extending specific elements and spreading the attributes into an element:

```rust
#[derive(Props, PartialEq, Clone)]
struct Props {
    // You can extend a specific element or global attributes
    #[props(extends = img)]
    attributes: Vec<Attribute>,
}

fn ImgPlus(props: Props) -> Element {
    rsx! {
        // You can spread those attributes into any element
        img { ..props.attributes }
    }
}

fn app() -> Element {
    rsx! {
        ImgPlus {
            // You can use any attributes you would normally use on the img element
            width: "10px",
            height: "10px",
            src: "https://example.com/image.png",
        }
    }
}
```

## Shorthand attributes

Another huge quality-of-life feature we added was the ability to use shorthand struct initialization syntax to pass attributes into elements and components. We got tired of passing `class: class` everywhere and decided to finally implement this long awaited feature, at the expense of some code breakage. Now, it’s super simple to declare attributes from props:

```rust
#[component]
fn ImgPlus(class: String, id: String, src: String) -> Element {
    rsx! {
        img { class, id, src }
    }
}
```

This feature works for anything implementing `IntoAttribute`, meaning signals also benefit from shorthand initialization. While signals as attributes don’t yet skip diffing, we plan to add this as a performance optimization throughout the 0.5 release cycle.

## Multi-line attribute merging

Another amazing feature added this cycle was attribute merging. When working with libraries like Tailwind, you’ll occasionally want to make certain attributes conditional. Before, you had to format the attribute using an empty string. Now, you can simply add an extra attribute with a conditional, and the attribute will be merged using a space as a delimiter:

```rust
#[component]
fn Blog(enabled: bool) -> Element {
    rsx! {
        div {
            class: "bg-gray-200 border rounded shadow",
            class: if enabled { "text-white" }
        }
    }
}
```

This is particularly important when using libraries like Tailwind where attributes need to be parsed at compile time but also dynamic at runtime. This syntax integrates with the Tailwind compiler, removing the runtime overhead for libraries like tailwind-merge.

## Server function streaming

Dioxus 0.5 supports the latest version of [the server functions crate](https://crates.io/crates/server_fn) which supports streaming data. Server functions can now choose to stream data to or from the client. This makes it easier to do a whole class of tasks on the server.

Creating a streaming server function is as easy as defining the output type and returning a TextStream from the server function. Streaming server functions are great for updating the client during any long running task.

We built an AI text generation example here: [https://github.com/ealmloff/dioxus-streaming-llm](https://github.com/ealmloff/dioxus-streaming-llm) that uses Kalosm and local LLMS to serve what is essentially a clone of OpenAI’s ChatGPT endpoint on commodity hardware.

```rust
#[server(output = StreamingText)]
pub async fn mistral(text: String) -> Result<TextStream, ServerFnError> {
   let text_generation_stream = todo!();
   Ok(TextStream::new(text_generation_stream))
}
```

![Streaming server function AI app](https://i.imgur.com/JJaMT0Z.mp4)

Side note, the AI metaframework used here - Kalosm - is maintained by the Dioxus core team member ealmloff, and his AI GUI app Floneum is built with Dioxus!

## Fullstack CLI platform

The CLI now supports a `fullstack` platform with hot reloading and parallel builds for the client and sever. You can now serve your fullstack app with the `dx` command:

```shell
dx serve

# Or with an explicit platform
dx serve --platform fullstack
```

## LiveView router support

<https://github.com/DioxusLabs/dioxus/pull/1505>

[`@DonAlonzo`](https://github.com/DonAlonzo) added LiveView support for the router in Dioxus 0.5. The router will now work out of the box with your LiveView apps!

## Custom Asset Handlers

<https://github.com/DioxusLabs/dioxus/pull/1719>

[`@willcrichton`](https://github.com/willcrichton) added support for custom asset handlers to Dioxus Desktop. Custom asset handlers let you efficiently stream data from your rust code into the browser without going through JavaScript. This is great for high bandwidth communication like [video streaming](https://github.com/DioxusLabs/dioxus/pull/1727):

![Custom asset handlers](https://i.imgur.com/6bdUBdF.mp4)

Now, you can do things like work with gstreamer or webrtc and pipe data directly into the webview without needing to encode/decode frames by hand.

## Native File Handling

This is a bit smaller of a tweak, but now we properly support file drops for Desktop:

![Native file drop](https://i.imgur.com/vkkDDid.mp4)
Previously we just gave you the option to intercept filedrops but now it’s natively integrated into the event system

## Error handling

Error handling: You can use error boundaries and the throw trait to easily handle errors higher up in your app

Dioxus provides a much easier way to handle errors: throwing them. Throwing errors combines the best parts of an error state and early return: you can easily throw an error with `?`, but you keep information about the error so that you can handle it in a parent component.

You can call `throw` on any `Result` type that implements `Debug` to turn it into an error state and then use `?` to return early if you do hit an error. You can capture the error state with an `ErrorBoundary` component that will render the a different component if an error is thrown in any of its children.

```rust
fn Parent() -> Element {
  rsx! {
    ErrorBoundary {
        handle_error: |error| rsx! {
            "Oops, we encountered an error. Please report {error} to the developer of this application"
        },
        ThrowsError {}
    }
  }
}

fn ThrowsError() -> Element {
    let name: i32 = use_hook(|| "1.234").parse().throw()?;

    todo!()
}
```

You can even nest `ErrorBoundary` components to capture errors at different levels of your app.

```rust
fn App() -> Element {
  rsx! {
    ErrorBoundary {
        handle_error: |error| rsx! {
            "Hmm, something went wrong. Please report {error} to the developer"
        },
        Parent {}
    }
  }
}

fn Parent() -> Element {
  rsx! {
    ErrorBoundary {
        handle_error: |error| rsx! {
            "The child component encountered an error: {error}"
        },
      ThrowsError {}
    }
  }
}

fn ThrowsError() -> Element {
  let name: i32 = use_hook(|| "1.234").parse().throw()?;

  todo!()
}
```

This pattern is particularly helpful whenever your code generates a non-recoverable error. You can gracefully capture these "global" error states without panicking or handling state for each error yourself.

## Hot reloading by default and “develop” mode for Desktop

We shipped hot reloading in 0.3, added it to Desktop in 0.4, and now we’re finally enabling it by default in 0.5. By default, when you `dx serve` your app, hot reloading is enabled in development mode.

Additionally, we’ve drastically improved the developer experience of building desktop apps. When we can’t hot reload the app and have to do a full recompile, we now preserve the state of the open windows and resume that state. This means your app won’t block your entire screen on every edit and it will maintain its size and position, leading to a more magical experience. Once you’ve played with it, you can never go back - it’s that good.

![Hot reloading by default](https://i.imgur.com/qjHB4ho.mp4)

## Updates to the Dioxus template

![Dioxus template update](https://i.imgur.com/jpXNW5P.mp4)

With this update, our newest core team member Miles put serious work into overhauling documentation and our templates. We now have templates to create new Dioxus apps for Web, Desktop, Mobile, TUI, and Fullstack under one command.

We also updated the default app you get when using `dx new` to be closer to the traditional create-react-app. The template is now seeded with assets, CSS, and some basic deploy configuration. Plus, it includes links to useful resources like dioxus-std, the VSCode Extension, docs, tutorials, and more.

![New templates](https://i.imgur.com/DCrrDxD.png)

## Dioxus-Community and Dioxus-std

The Dioxus Community is something special: discord members marc and Doge have been hard at working updating important ecosystem crates for the 0.5 release. With this release, important crates like icons, charts, and the Dioxus-specific standard library are ready to use right out the gate. The `Dioxus Community` project is a new GitHub organization that keeps important crates up-to-date even when the original maintainers step down. If you build a library for Dioxus, we’ll be happy to help maintain it, keeping it at what is essentially “Tier 2” support.

![dioxus_community](https://i.imgur.com/yoLSrwj.png)

## Coming soon

At a certain point we had to stop adding new features to this release. There’s plenty of cool projects on the horizon:

*   Stabilizing and more deeply integrating the asset system
*   Bundle splitting the outputted `.wasm` directly - with lazy components
*   Islands and resumable interactivity (serializing signals!)
*   Server components and merging LiveView into Fullstack
*   Enhanced Devtools (potentially featuring some AI!) and testing framework
*   Complete Mobile overhaul
*   Fullstack overhaul with WebSocket, SSE, progressive forms, and more

## Sneak Peek: Dioxus-Blitz revival using Servo

We’re not going to say much about this now, but here’s a sneak peek at “Blitz 2.0”… we’re finally integrating servo into Blitz so you can render natively with WGPU using the same CSS engine that powers Firefox. To push this effort forward, we’ve brought the extremely talented Nico Burns (the wizard behind our layout library Taffy) on full time. More about this later, but here’s a little demo of [google.com](http://google.com) being rendered at 900 FPS entirely on the GPU:

![Google rendered with blitz](https://i.imgur.com/I1HRiBd.png)

Admittedly the current iteration is not quite there (google.com is in fact a little wonky) but we’re progressing rapidly here and are quickly approaching something quite usable. The repo is here if you want to take a look and get involved:

[https://github.com/jkelleyrtp/stylo-dioxus](https://github.com/jkelleyrtp/stylo-dioxus)

## How can you contribute?

Well, that’s it for the new features. We might’ve missed a few things (there’s so much new!). If you find Dioxus as exciting as we do, we’d love your help to completely transform app development. We’d love contributions including:

*   Translating docs into your native language
*   Attempting “Good First Issues”
*   Improving our documentation
*   Contributing to the CLI
*   Help answer questions from the discord community

That’s it! We’re super grateful for the community support and excited for the rest of 2024.

Build cool things! ✌️
