Happy Holidays! As an early holiday present, we're happy to announce the release of Dioxus 0.6! 🎉

Dioxus (Dye-ox-us) is a framework for building fullstack web, desktop, and mobiles apps in Rust.

Dioxus 0.6 is the biggest release of Dioxus ever: over 350 pull requests, hundreds of issues closed, huge new features, and a complete overhaul the framework. With this release, we focused on fixing bugs, polishing existing features, and improving tooling.

Here's what's new in Dioxus 0.6:

- **[`dx serve` for mobile](#android-and-ios-support-for)**: Serve your app on Android and iOS simulators and devices.
- **[Interactive CLI](#interactive-command-line-tools)**: Rewrite of the Dioxus CLI with a new, interactive UX inspired by Astro.
- **[Inline Stack Traces](#inline-wasm-stacktraces-and)**: Capture WASM panics and logs directly into your terminal.
- **[Toasts and Loading Screens](#toasts-and-loading-screens)**: New toasts and loading screens for web apps in development.
- **[Server Functions for Native](#fullstack-desktop-and-mobile)**: Inline Server RPC for Desktop and Mobile apps.
- **[Revamped Autocomplete](#completely-revamped-autocomplete)**: Massively improved autocomplete using Rust-analyzer itself.
- **[Magical Hot-Reloading](#completely-revamped-hot-reloading)**: Hot-Reloading of formatted strings, properties, and nested `rsx!{}`.
- **[Mobile Hot-Reloading](#mobile-hot-reloading)**: Hot-Reloading of `rsx!{}` and assets for mobile devices and emulators.
- **[`asset!` Stabilization](#stabilizing-manganis)**: Stabilizing our linker-based asset system integrated for native apps.
- **[Streaming HTML](#suspense-and-html-streaming-for-the-web)**: Stream `Suspense` and `Error` Boundaries from the server to the client.
- **[SSG and ISG](#static-site-generation-and-isg)**: Support for Static Site Generation and Incremental Static Regeneration.
- **[Error Handling with `?`](#question-mark-error-handling)**: Use `?` to handle errors in event handlers, tasks, and components.
- **[Meta Elements](#document-elements)**: New `Head`, `Title`, `Meta`, and `Link` elements for setting document attributes.
- **[Synchronous `prevent_default`](#synchronous)**: Handle events synchronously across all platforms.
- **[`onresize` Event Handler](#tracking-size-with)**: Track an element's size without an IntersectionObserver.
- **[`onvisible` Event Handler](#tracking-visibility-with)**: Track an element's visiblility without an IntersectionObserver.
- **[WGPU Integration](#hybrid-wgpu-overlays)**: Render Dioxus as an overlay on top of WGPU surfaces and child windows.
- **[`dx bundle` for Web, iOS, and Android](#web-ios-and-android-bundle-support)**: Complete `dx bundle` support for every platform.
- **[`json` mode](#json-output-for-ci--cli)**: Emit CLI messages as JSON for use by 3rd party tools and CI/CD pipelines.
- **[New Templates](#new-starter-templates)**: Three new starter templates for cross-platform apps.
- **[Nightly Tutorial and Guides](#nightly-docs-tutorials-and-new-guides)**: New tutorials and guides for Dioxus 0.6 and beyond.
- **[Binary Patching Prototype](#preview-of-in-place-binary-patching)**: Prototype of our new pure Rust hot-reloading engine.

## About this Release

I’m happy to say that Dioxus is the most mature and complete it’s ever been, *finally* living up to the original mission.

Dioxus 0.6 is the culmination of nearly 6 months of work. While improving various pieces of the framework, we got carried away and basically shipped all the features we wanted for Dioxus 0.7. With this release, we set a goal to retain a very high bar for quality and polish: everything from CLI tools to APIs and ecosystem libraries have seen huge improvements.

With 0.6, we didn’t necessarily want to focus on shipping shiny new features. Instead, we wanted to continue much of the work started in Dioxus 0.5 and focus on cleaning up and improving existing features. The end result: a rebirth of Dioxus with hundreds of bug fixes, massively improved tooling, and the “ungating” of essential APIs. Everything from CLI tooling to hot-reloading and autocomplete saw huge jumps in quality.

Since this post is quite long, we made a quick video highlighting new features, bugs fixed, and a quick tour of everything you can do with Dioxus now:


<iframe style="width: 120%" height="500px" class="centered-overflow" src="https://www.youtube.com/embed/-RN4LB3-9AE" title="Dioxus 0.5 Overview preview" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>


## Android and iOS support for `dx serve`

Our goal with Dioxus has always been to support web, desktop, mobile, and more. And while *technically* Dioxus has supported mobile since its release, the Rust
tooling for mobile has been extremely unstable. Users constantly ran into issues with tools like [`cargo-mobile`](https://github.com/BrainiumLLC/cargo-mobile) and Tauri’s fork [cargo-mobile2](https://github.com/tauri-apps/cargo-mobile2). These tools, while useful, take a different architectural approach than what is a good fit for Dioxus.

We want to provide features like asset bundling, hot-reloading, and proper support for regular apps built with a traditional `main.rs` - none of which we can properly do with 3rd party tools.

With this release, we’ve decided to put the huge amount of effort into writing our own mobile support from scratch. Now, you can go from `dx new` to `dx serve --platform ios` in a matter of seconds.

![Dioxus Mobile Support](/assets/06assets/image.png)

The Android and iOS simulator targets support all the same features that desktop supports: hot-reloading, fast rebuilds, asset bundling, logging, etc. One notable accomplishment: you can build Rust mobile apps with a simple `main.rs`. Existing solutions like [xbuild](http://github.com/rust-mobile/xbuild) and [Tauri](http://github.com/tauri-apps/tauri) require you to fundamentally restructure your app. Your app also needs to be converted to a cdylib, meaning you can’t share a launch function between desktop and mobile.

The simplest Dioxus 0.6 Mobile app is tiny:

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(|| rsx! { "hello dioxus! 🧬" });
}
```

Especially, when compared to v0.5 which required you to migrate your app to a `cdylib` and manually set up the binding layer:

```rust
// no main allowed! - must expose a `start_app` function
#[no_mangle]
#[inline(never)]
#[cfg(any(target_os = "android", target_os = "ios"))]
pub extern "C" fn start_app() {
    #[cfg(target_os = "android")]
    {
        tao::android_binding!(
            com_dioxuslabs,
            app_name,
            WryActivity,
            wry::android_setup,
            _start_app
        );
        wry::android_binding!(com_dioxuslabs, app_name);
    }

    // need to manually catch panics!
    #[cfg(target_os = "ios")]
    stop_unwind(|| main())
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
            std::process::abort()
        }
    }
}
```

Our inline mobile support requires no extra configurations, no manual setup for Gradle, Java, Cocoapods, and no other specific mobile tooling! Provided you already have Android NDK installed or an iOS Simulator, you currently are less than 30 seconds away from a functional mobile app written entirely in Rust. In the time it takes for you to watch this gif, you could have your very own mobile app:

![full dx new, serve](/assets/06assets/dxnew.mp4)

While 1st-class support for mobile platforms is quite exciting, there are certainly many limitations: the Rust mobile ecosystem is practically nonexistent, we don’t have great ways of configuring the hundreds of XCode and AndroidStudio flags, and there isn’t a particularly great Rust/Java interop story. However, we’re very dedicated to making mobile app development as great as possible and will be rolling out improvements to mobile over the next year.

If you're interested in helping us build out mobile support, please join us on [Discord](https://discord.gg/XgGxMSkvUM). We have amibitious goals for Dioxus 0.7+ to integrate automatic deploys to app stores and a build a comprehensive SDK for mobile development in Rust.

## Interactive Command Line Tools

You might have noticed in the gifs above... Dioxus 0.6 is shipping with a completely overhauled CLI experience! We’ve completely rewritten the CLI to support a ton of new features and improve stability:

![image.png](/assets/06assets/image%201.png)

The new CLI sports live progress bars, animations, an interactive filter system, the ability to change log levels on the fly, and more.

![cli_animation](https://private-user-images.githubusercontent.com/10237910/391807201-00673048-9ba9-4df3-8878-93ef849e9fa0.mp4)

The CLI rewrite alone took more than half this release cycle. We went through several different design iterations and solved tons of bugs along the way. A few of the highlights:

- We dropped the `outdir` concept and instead use `target/dx` for all output.
- You can manually rebuild your app by pressing `r`
- You can toggle the log level of the CLI output on the fly and even inspect Cargo internal logs
- Log capture for WASM tracing and panics
- We output all internal logs of the CLI so you can debug any issues

You can install the new CLI using [cargo binstall](https://github.com/cargo-bins/cargo-binstall) with `cargo binstall dioxus-cli@0.6.0 --force`.

## Inline WASM stacktraces and `tracing` integration

Along with the rewrite of the CLI, we shipped a `tracing` integration for WASM apps that captures panics and logs and sends them `dx` in your terminal. When you build your app with debug symbols, stack traces directly integrate with your editor, allowing you to jump directly to the troublesome files from within your terminal.

![Inline Stack Traces](/assets/06assets/Screenshot_2024-11-14_at_8.52.18_PM.png)

Thanks to this integration, we now have much nicer logging around fullstack apps, showing status codes, fetched assets, and other helpful information during development. With the toggle-able verbosity modes, you can now inspect the internal logs of the CLI itself, making it easier to debug issues with tooling to understand what exactly `dx` is doing when it builds your app. Simply type `v` to turn on “verbose” mode and `t` to turn on “trace” mode for more helpful logs:

![Screenshot 2024-11-14 at 9.06.05 PM.png](/assets/06assets/Screenshot_2024-11-14_at_9.06.05_PM.png)

## Toasts and Loading Screens

As part of our CLI overhaul, we wanted to provide better feedback for developers when building web apps. Dioxus 0.6 will now show Popup Toasts and Loading Screens for web apps in development mode.

Now, when your app is building, Dioxus will rendering a loading screen with the current progress of the build:

![Screenshot 2024-11-14 at 9.41.38 PM.png](/assets/06assets/Screenshot_2024-11-14_at_9.41.38_PM.png)

Additionally, once the app is rebuilt, you’ll receive a toast indicating the status of the build:

![Screenshot 2024-11-14 at 9.42.33 PM.png](/assets/06assets/Screenshot_2024-11-14_at_9.42.33_PM.png)

## Fullstack Desktop and Mobile

Additionally, as improving tooling, we properly integrated server functions with native apps. Server functions finally work out-of-the-box when targeting desktop and mobile:

![native-serverfn12.mp4](/assets/06assets/native-serverfn12.mp4)

By default, in development, we set the server function endpoint to be localhost, so in production you need to make sure to point the functions to your deployed server:

```rust
fn main() {
    #[cfg(feature = "production")]
    server_fn::client::set_server_url("app.endpoint.com");

    dioxus::launch(app)
}
```

## Completely Revamped Autocomplete

Another huge overhaul in Dioxus 0.6: greatly improved autocomplete of `rsx! {}`.  Our old implementation of `rsx! {}` suffered from poor integration with tools like Rust-analyzer which provide language-server integration for your code. If the input to the macro wasn’t perfectly parsable, we failed to generate any tokens at all, meaning rust-analyzer couldn’t jump in to provide completions.

The work to fix this was immense. Macro parsing libraries like `syn` don’t provide great facilities for “partial parsing” Rust code which is necessary for implementing better errors and autocomplete. We had to rewrite the entire internals of `rsx! {}` to support partial parsing of `rsx! {}` , but finally, in 0.6, we’re able to provide stellar autocomplete. Not only can we autocomplete Rust code in attribute positions, but with a few tricks, we’re able to automatically insert the appropriate braces next to element names:

![Screenshot 2024-11-14 at 9.55.12 PM.png](/assets/06assets/Screenshot_2024-11-14_at_9.55.12_PM.png)

The autocomplete experience is much nicer now, with all attributes, elements, components, and inline Rust code benefiting from the overhauled experience.

![autocomplete-overhaul.mp4](/assets/06assets/autocomplete-overhaul.mp4)

## Completely Revamped Hot-Reloading

As part of our effort to improve the `rsx! {}` experience, we shipped massive improvements to the hot-reloading engine powering Dioxus. Our internal goal was to iterate on the Dioxus Docsite content with zero full rebuilds - we only wanted full rebuilds when modifying real Rust code.

This means we needed to add support for a number of new hot-reloading engine changes:

- Hotreload formatted strings
- Hotreload `for` and `if` blocks in RSX
- Hotreload children of components
- Hotreload properties of components
- Hotreload mobile platforms
- Hotreload as many Rust expressions as possible

The new hot-reloading engine almost feels like magic - you can quickly iterate on new designs without waiting for full Rust rebuilds:

![dogapphr2.mp4](/assets/06assets/dogapphr2.mp4)

## Hot-Reloading Formatted Strings

We can now hotreload any formatted string in your markup! For this component, we can hotreload both the `class` attribute on button as well as the text in the button itself.

```rust
#[component]
fn Counter(count: i32, class_ext: String) -> Element {
    rsx! {
        button { class: "btn-{class_ext}", "Count {count}" }
    }
}
```

Notice that these are *formatted strings.* Very frequently, when working on the docsite, we’d want to modify formatted tailwind classes, but these changes would cause a full rebuild. This drastically slowed down iteration time, making working on the docsite a rather unpleasant experience. Hot-Reloading of formatted strings works *everywhere* in rsx. This means you can get string hot-reloading in component props too.

## Hot-Reloading Rust Literals

As part of the hot-reloading overhauls, we also now support hot-reloading of any literals we can find inside your rsx. We built a very simple interpreter for Rust code! Any changes to literals are automatically propagated through the signal-based reactivity system shipped in 0.5. This means you can change the bounds on component props without causing a full rebuild.

```rust
fn LoopIt() -> Element {
    rsx! {
        // Change either prop without causing a full rebuild
        Link {
            to: "google.com",
            enabled: false
        }
    }
}
```

While limited in some ways, this can feel downright magical.

## Hot-Reloading nested rsx (`for`/ `if` / `component` )

With Dioxus 0.4 we shipped improvements that enabled a simpler syntax for `for` loops and `if` chains in rsx. However, we never properly implemented hot-reloading for the contents of these items, leading to frequent unnecessary rebuilds. With Dioxus 0.6, we finally had a chance to iron out hot-reloading in every possible nook and cranny. Now, more places properly support hot-reloading, like `for` loops and `if` chains:

```rust
fn LoopIt() -> Element {
    rsx! {
        for x in 0..10 {
            // modifying the body of this loop is hotrelodable!
            li { "{x}" }
        }
    }
}
```

We also now support hot-reloading of bodies of components:

```rust
fn LoopIt() -> Element {
    rsx! {
        Counter {
            div { "These div contents get hotreloaded too!" }
        }
    }
}
```

## Mobile Hot-Reloading

With Dioxus 0.6, we also wanted to fix the longstanding issue where mobile simulators didn’t properly get hot-reloading. Mobile can be tricky to work with - and will take a long time to get 100% right - but this is a solid step in making mobile targets better supported with Dioxus.

![bundled-ios-reload.mp4](/assets/06assets/bundled-ios-reload.mp4)

The changes here also unlocked hot-reloading of bundled assets used by the `asset!()` macro. If you're using Tailwind with Dioxus, you can now simply run your Tailwind watcher in the background and Dioxus will automatically hotreload your CSS files web, desktop, and mobile.

## Proper Workspace Hot-Reloading

We now properly support hot-reloading across multiple projects in a workspace. This solves the longstanding issue where you’re developing a component library in one crate and using it another crate. Our new hotreload engine intelligently walks your project’s dependencies across the filesystem and watches all the related Rust files.

## Stabilizing Manganis `asset!()` system

We introduced our new asset system, [Manganis](https://github.com/DioxusLabs/manganis), in an alpha state with the 0.5 release. Dioxus 0.6 stabilizes the asset system and fixes several bugs and performance issues. You can try out the new [linker based asset system](https://github.com/DioxusLabs/manganis/pull/30) by including an `asset!` anywhere in your code. It will automatically be optimized and bundled across all platforms:

```rust
rsx! {
    img { src: asset!("/assets/myimg.png") }
}
```

Manganis is a crucial step in supporting assets crossplatform, and specifically, through dependencies. Previously, if an upstream library wanted to export an asset like an image or a stylesheet, your app would need to manually add those assets in your `assets` folder. This gets complex and messy when libraries generate CSS: many classes are duplicated and might even conflict with each other. Now, all CSS collected by the `asset!()` macro is processed via our build pipeline, benefiting from minification and deduplication. Libraries can include their stylesheets and images and components and you can be guaranteed that those assets make it bundled into your app:

```rust
fn app() -> Element {
    rsx! {
        cool_icons::SomeCoolImage {}
    }
}

// in a distant library....
mod cool_icons {
    pub fn SomeCoolImage() -> Element {
        rsx! {
            img { src: asset!("/assets/some_cool_image.png") }
        }
    }
}
```

Even better, assets like images are automatically optimized to generate thumbnails and more optimized formats. This can cut huge amounts of data from your site - AVIF and Webp can reduce file sizes by up to 90%. A funny note - platforms like Vercel actually [provide paid products for image optimization](https://vercel.com/docs/image-optimization) while Manganis can do this for you, for free, at build time!

[ photo of delta between the two formats ](some-image.png)

Additionally, manganis automatically hashes the images and modifies the generated asset name, allowing for better integration with CDNs and browser caching.

![Screenshot 2024-11-14 at 10.22.48 PM.png](/assets/06assets/Screenshot_2024-11-14_at_10.22.48_PM.png)

Manganis can handle a wide variety of formats - applying optimizations to assets like CSS, JavaScript, images, videos, and more.

In Dioxus 0.5, we released Manganis in “alpha” status - and in 0.6 we’re stabilizing it. We’ve adjusted the API, so you’ll need to update any existing code that already uses it. Our new implementation is much more reliable, solving many of the bugs users were running into after the 0.5 release.

Our new system leverages *the linker* to extract asset locations from the compiled binary. This is a rather advanced technique and took a while to get right, but we believe it’s a more robust solution in the long term. If you’re interested in integrating Manganis into your libraries and apps (like say, Bevy!), we have a guide just for that.

## Suspense and HTML Streaming for the Web

Async is a core component of any UI framework. Dioxus provides hooks to handle async state. You can start a future and handle the loading and resolved states within the component:

```rust
fn Article() -> Element {
    // Use resource starts a future and returns the current state
    let article = use_resource(fetch_article);

    rsx! {
        // You can `match` the state of the future.
        match article() {
            Some(article) => rsx! { "{article}" },
            None =>  rsx! { p { "Loading..." } }
        }
    }
}
```

This works ok if you have a single future, but it quickly gets messy when combining many futures into one UI:

```rust
#[component]
fn Article() -> Element {
    // Use resource starts a future in the background and returns the current state
    let Some(title) = use_resource(fetch_title).cloned() else {
        return rsx! { "loading..." }
    };

    let Some(article) = use_resource(fetch_article).cloned() else {
        return rsx! { "loading..." }
    };

    let Some(category) = use_resource(move || article.title()).cloned() else {
        return rsx! { "loading..." }
    };

    rsx! {
        Title { "{title}" }
        Body { category, article }
    }
}
```

In addition to hooks, we need a way to display a different state when async is loading. Dioxus 0.6 introduces a new core primitive for async UI called suspense boundaries. A suspense boundary is a component that renders a placeholder when any child component is loading:

```rust
rsx! {
    SuspenseBoundary {
        fallback: |context: SuspenseContext| rsx! {
            // Render a loading placeholder if
            // any child component is "suspended"
            "Loading..."
        },
        Article {}
    }
}
```

In any child component, you can simply bubble up the pending state with `?` to pause rendering until the future is finished:

```rust
fn Article() -> Element {
    let title = use_resource(fetch_title).suspend()?;
    let article = use_resource(fetch_article).suspend()?;
    let category = use_resource(move || article.title()).suspend()?;

    // Since we bubbled up all the pending futures with `?` we can just
    // handle the happy path in the component
    rsx! {
        Title { "{title}" }
        Body { category, article }
    }
}
```

Along with suspense boundaries, dioxus fullstack also supports streaming each suspense boundary in from the server. Instead of waiting for the whole page to load, dioxus fullstack streams in each chunk with the resolved futures as they finish:

![streamingsuspense.mp4](/assets/06assets/streamingsuspense.mp4)

Many of these features are quite cutting-edge and are just now being rolled out in frameworks in the JavaScript ecosystem. Getting the details right for Dioxus was quite difficult. We wanted to support both the fullstack web as well as native desktop and mobile apps. These two platforms often have competing design considerations. Fortunately, suspense also works for desktop and mobile, allowing you to emulate web-like data fetching patterns for native apps.

## Static Site Generation and ISG

As part of our work on streaming, we also wanted to support another cutting-edge web feature: incremental static generation (ISG) and its cousin static site generation (SSG).

Static site generation is a technique used by many web frameworks like Jekyll, Hugo, or Zola, to emit static `.html` not reliant on JavaScript. Sites like blogs and portfolios typically use static site generation since platforms like GitHub Pages allow hosting static sites for free. In fact, this very docsite uses Dioxus SSG deployed to GitHub Pages! SSG helps improve SEO and speed up load times for your users.

In Dioxus 0.6, we now support static-site-generation out of the box for all fullstack projects. Simply add a server function to your app called `static_routes` that returns the list of routes that `dx` should generate:

```rust
#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .into_iter()
        .map(|route| route.to_string())
        .collect::<Vec<_>>())
}

```

Now, when you want to emit your static `.html`, add the `--ssg`  flag to `dx build`:

```rust
dx build --platform web --ssg
```

Static-site-generation is built on a new feature in Dioxus called incremental-site-generation (ISG). ISG is a technique similar to static-site-generation where the server generates pages on demand and caches them on the system filesystem. This allows the server to cache huge amounts of pages (for something like a school’s facebook directory or an ecommerce site with thousands of products) that get periodically invalidated. ISG is a somewhat advanced technique but is required to enable when using static-site-generation:

```rust
fn main() {
        dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder()
                // turn on incremental site generation with the .incremental() method
                .incremental(IncrementalRendererConfig::new())
                .build()
                .unwrap()
        })
        .launch(|| {
            rsx! {
                Router::<Route> {}
            }
        })
}
```

We will likely be changing these APIs in future releases, but we are eager to let users experiment with these new features to simplify the existing static site setup.

## Question Mark Error Handling

With this release, we’ve finally made the transition where `Element` is no longer an `Option<Node>` but rather a `Result<Node>`. This means we’re *finally* able to open up the use of typical rust error handling in components:

```rust
fn Slider() -> Element {
    let value: f64 = "1234".parse()?;

    rsx! {
        Handle { offset: value }
    }
}
```

The new `RenderError` acts like anyhow’s `Error` type that can take in any `dyn std::Error` type and propagate it upwards to the nearest error boundary.

```rust
fn Input() -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: |errors| rsx! {
                h3 { "Oops, we encountered an error.!" }
                pre { "Please report {errors:?} to the developers." }
            },
            Slider {}
        }
    }
}
```

What’s even better: the `?` syntax also works in event handlers, so you can quickly add things like server functions to your app without worrying about being bogged down with manual error handling:

```rust
fn Counter() -> Element {
    let mut data = use_signal(|| Data::default());

    rsx! {
        button {
            onclick: move |_| async move {
                // the `?` automatically throws this error upwards
                data.set(request_server_data().await?);
                Ok(())
            },
            "{data}"
        }
    }
}
```

This new syntax works with suspense and HTML-streaming to allow you to return errors while rendering that don’t bring down the entire page.

## Document Elements: `Title {}` , `Link {}` , `Stylesheet` , and `Meta {}`

To date, it’s been rather cumbersome to do what is seemingly simple JavaScript operations in Dioxus. Due to our crossplatform nature, we need to find solutions to what seems like simple problems in ways that work for web, desktop, and mobile with a single abstraction.

Finally with Dioxus 0.6, we’re providing special elements under the `document` namespace that make it possible to interact with the HTML `document` object without needing to write extra JavaScript.

Now, to set the `title` of your HTML document, simply use the `document::Title {}` component:

```rust
use dioxus::prelude::*;
use dioxus::document::Title;

fn main() {
    dioxus::launch(|| rsx! {
        Title { "WebAssembly rocks!" }
        h1 { "A site dedicated to webassembly" }
    })
}
```

And accordingly, the title of the page will update:

![Screenshot 2024-11-14 at 11.28.42 PM.png](/assets/06assets/Screenshot_2024-11-14_at_11.28.42_PM.png)

Similarly, with `Link` , `Stylesheet` , and `Style`, you can include elements that automatically get merged into the document’s `<head>` element. During server side rendering, these links get collected, deduplicated, and minified. With these built-in `document` components, you’re now guaranteed that your `<head>` element is properly set for pre-loading heavy assets like stylesheets and external JavaScript.

```rust
fn app() -> Element {
    rsx! {
        Title { "WebAssembly rocks!" }
        Stylesheet { href: asset!("/assets/main.css") }
        h1 { "A site dedicated to webassembly" }
    }
}
```

![Screenshot 2024-11-14 at 11.31.18 PM.png](/assets/06assets/Screenshot_2024-11-14_at_11.31.18_PM.png)

## Synchronous `prevent_default`

In addition to being able to access the native event type, Dioxus 0.6 also makes all event handling synchronous. Previously, all event handling in Dioxus had to occur outside the normal browser event handling flow to support platforms like `dioxus-desktop` which need to communicate over an interprocess communication (IPC) layer with the host webview. With this release, we’ve finally figured out how to enable synchronous communication for `dioxus-desktop` and can finally make event handling synchronous!

As such, we no longer need the special `dioxus_prevent_default` attribute and you can directly call `event.prevent_default()`.

```rust
fn Form() -> Element {
    rsx! {
        button {
            // we no longer need this!
            dioxus_prevent_default: "onclick",

            // instead we can just call .prevent_default()
            onclick: move |evt| {
                evt.prevent_default();
                todos.write().remove(&id);
            },
        }
    }
}
```

This now makes it possible to implement prevent_default conditionally which has previously been a major limitation with Dioxus. Components like `Link {}` now exhibit behavior exactly aligned with their native counterparts, solving long-standing issues with Dioxus apps.

## Tracking size with `onresize`

Thanks to the community, we now have two special handlers *not* found in the HTML spec: `onvisible` and `onresize`. These handlers are “special” dioxus handlers that automatically set up an `IntersectionObserver` which previously required JavaScript.

You can now implement particularly rich interactions with little hassle:

```rust
fn app() -> Element {
    let mut items = use_signal(|| 100);

    rsx! {
        h1 { "A site dedicated to webassembly" }

        // Adding a value will cause the `div` to be re-rendered with an extra div
        button { onclick: move |_| items += 1, "Add one" }

        div {
            // This will be called when the `div` is resized
            onresize: move |data| {
                tracing::info!("resized to {:#?}", data.get_border_box_size().unwrap());
            },

            for x in 0..items() {
                div { "{x}" }
            }
        }
    }
}
```

## Tracking visibility with `onvisible`

In addition to `onresize`, we now have a special handler *not* found in the HTML spec: `onvisible`.

```rust
fn app() -> Element {
    rsx! {
        div {
            onvisible: move |data| {
                println!("visible!");
            }
            "Hello world!"
        }
    }
}
```

This makes it possible to add rich animations to your app without needing to write custom JavaScript.

![gif_of_visible_working.mp4](/assets/06assets/onvisible.mp4)

## Hybrid WGPU Overlays

![wgpu-windows.mp4](/assets/06assets/wgpu-windows.mp4)

## Web, iOS, and Android bundle support



## JSON Output for CI / CLI

As part of our overhaul with the CLI, we’re also shipping a `json-output` mode. Now, when you pass `--json-output` to Dioxus commands, you will receive the logging in json format:

![Screenshot 2024-11-14 at 10.38.33 PM.png](/assets/06assets/Screenshot_2024-11-14_at_10.38.33_PM.png)

This is particularly important for users of `dx bundle` who want to automatically upload the their bundles to their hosting provider of choice. You can easily combine the output of `dx` with a tool like `jq` to extract important information like bundle outputs with a simple one-liner:

![Screenshot 2024-11-14 at 10.40.56 PM.png](/assets/06assets/Screenshot_2024-11-14_at_10.40.56_PM.png)

## New Starter Templates

Dioxus 0.6 ships with three new starter templates for cross-platform apps. Each template is a fully-featured, production-ready app that you can use as a starting point for your own Dioxus apps.

- Bare-Bones: A bare-bones starter template with no styling, assets, or structure.
- Jumpstart: A starter template with a basic structure, components, and a few pages.
- Workspace: A starter template with separate crates for web, desktop, and mobile.

These are baked directly into the `dx new` command - simply run `dx new` and follow the prompts to select the template you want.

## Nightly Docs, Tutorials, and New Guides

As usual with these large release, Dioxus 0.6 features a rather sizable overhaul to the documentation. We’ve completely overhauled the tutorial to be less heavy on code. The new tutorial focues on basics like including assets and deploying to production.

![Screenshot 2024-11-14 at 11.35.23 PM.png](/assets/06assets/Screenshot_2024-11-14_at_11.35.23_PM.png)

The docsite now includes all “modern” versions of Dioxus inline: 0.3, 0.4, 0.5, and 0.6 are all accessible under the same top-level website. Previously, we linked out to different mdbooks which eventually became a hassle. Now, you can simply switch between each version inline:

![Screenshot 2024-11-15 at 1.02.23 AM.png](/assets/06assets/version_switch_shadow.png)

The inline version switcher means we’ll now be able to publish documentation for alpha releases of Dioxus, hopefully making your life easier as we ship new features for the future. The new docs also feature small quality-of-life upgrades like breadcrumbs:

![Screenshot 2024-11-15 at 1.04.13 AM.png](/assets/06assets/breadcrumbs_shadow.png)

as well as new codeblocks with interactive examples:

![Screenshot 2024-11-15 at 1.05.03 AM.png](/assets/06assets/interacitve_widget_shadow.png)

## Preview of In-Place Binary Patching

While working on the new hot-reloading engine, we experimented with adding hot-reloading to Dioxus apps. The work here was inspired by Andrew Kelley’s “in-place-binary-patching” goal for Zig. Unfortunately, we didn’t have a chance to productionize the prototype for this release (way too many features already!) but we did put together a [small prototype](http://github.com/jkelleyrtp/ipbp):

![full_hr_dioxus_fast.mp4](/assets/06assets/full_hr_dioxus_fast.mp4)

We likely won’t have the time to ship true Rust hot-reloading in 0.7, but stay tuned for early next year!

## Smaller changes:

Not every change gets a particularly large section in the release notes, but we did land several new features and refactors.

- System tray support: we now have proper support for System Trays again, thanks to a wonderful community contribution.
- Custom event loops: you can provide your own event loop, making it possible to use Dioxus in contexts where you already have other windows.
- WGPU integration for dioxus-desktop: you can now overlay Dioxus with WGPU contexts.
- `dioxus-document`: we split out our `document` abstraction so any renderer can implement the `Document` trait to integrate with `Title {}`, `Script {}` , and `eval`
- `dioxus-history`: we also split out our `history` abstraction so other renderers can benefit from `Link` and `Router` without needing a dedicated feature flag on `dioxus-router`
- `eval` API was simplified to allow `.recv::<T>().await` on evals, making interoping with JavaScript easier.
- `dx fmt` now supports `#[rustfmt::skip]` attributes and respects `rustfmt.toml` settings

## Upgrading from 0.5 to 0.6

Generally there are few huge breaking changes in this release. However, we did change a few APIs that might break your existing apps but are easy to fix.

- `asset!()` syntax changes
- `eval()` API small changes
- migrating to `prevent_default()`
- migrating from VNode::None to `rsx! {}` for empty nodes

We’ve assembled a migration guide here to help.

## Conclusion

That’s it for all the new features! Due to the sheer size of this release, we might have missed several new features and bug fixes. The list of fixed bugs is also quite massive. Everything from bundling issues to spurious hotreloads and compatibility with rare platforms and editors has been addressed.

Dioxus 0.6 has been in alpha for quite a while, and we’re very thankful for all the testing the community has done to make this the most polished release yet. It’s quite difficult to run a large open source project such a vast scope. This release took *much* longer to get out than we wanted - basically consuming two release cycles instead of just one.

We focused hard this release to polish up as many rough edges as possible. Our continuous integration and deployment is in a much nicer place. We’re finally able to release nightly versions of documentation and the alpha release system has worked well for users eager to test out new features and bug fixes.

Unfortunately, this release contained many connected pieces which made it hard to release incrementally. Systems like assets integrate tightly with CLI tooling and crossplatform support: to get one configuration right you need to test them all. With 0.6 behind us, the future seems much more “incremental” which should let us release major versions with faster cadence.

We plan to release Dioxus 0.7 early next year once everyone has had a chance to play with Dioxus 0.6. Similar to 0.6, Dioxus 0.7 will focus on polishes and bug fixes - the Dioxus team wants to spend time building our own apps!

We have a few major items planned for beginning of 2025:

- Rust hot-reloading with binary patching
- Integrating wasm bundle splitting with the router
- `dx deploy` to a hosted deploy platform

We’re also hiring - if you want to come build Dioxus with me in San Francisco (or remote) please reach out!

## Thanks to the community!

I want to extend a huge thank-you to everyone who helped test and improve this release. We saw an incredible number of contributors fix bugs and add features. Special thanks to:

[ list of contributors ](some-image.png)
