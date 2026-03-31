Welcome back to another Dioxus release! Dioxus (dye • ox • us) is a framework for building cross-platform apps in Rust. We make it easy to ship full-stack web, desktop, and mobile apps with a single codebase.

Dioxus 0.7 delivers on a number of promises we made to improve Rust GUI, and more broadly, what we call “high level Rust.” Rust has excelled as a tool for building foundational software, but we hope with Dioxus 0.7, it’s one step closer to being suitable for rapid, high-level development.

In this release, we’re shipping some incredible features. The highlights of this release include:

- **[Subsecond](#rust-hot-patching-with-subsecond)**: Hot-patching of Rust code at runtime(!)
- **[Dioxus Native](#dioxus-native-and-blitz)**: WGPU-based HTML/CSS renderer for Dioxus without webview
- **[Fullstack](#dioxus-fullstack-overhaul)**: Rebuild of Server Functions around Axum, supporting websockets, error codes, and more.
- **[WASM-Split](#wasm-bundle-splitting-and-lazy-loading)**: Code splitting and lazy loading for WebAssembly
- **[Stores](#stores---a-new-primitive-for-nested-reactive-state)**: A new primitive for nested reactive state
- **[Dioxus Primitives](#dioxus-primitives---a-collection-of-radix-ui-equivalents)**: first-party radix-primitives implementation for Dioxus

Dioxus 0.7 also brings a number of other exciting new features:

- **[Automatic tailwind](#automatic-tailwind)**: zero-setup tailwind support built-in!
- **[LLMs.txt](#improvements-with-ai---llmstxt-and-vibe-coding)**: first-party context file to supercharge AI coding models
- **[Blitz](#dioxus-native-and-blitz)**: our modular HTML/CSS renderer powering Dioxus Native, available for everyone!
- **[Integrated Debugger Support](#integrated-debugger)**: open CodeLLDB with a single keystroke
- **[Configurable Mobile Builds](#customize-androidmanifestxml-and-infoplist)**: Fully customize bundle options from dioxus.toml

Plus, a number of quality-of-life upgrades:

- **[one-line installer](#improved-version-management-experience)** ( `curl https://dioxus.dev/install.sh | sh` )
- **[`dx self-update`](#improved-version-management-experience)** and update notifications
- **[automatically open simulators](#various-quality-of-life-upgrades)**
- **[Improved log coloring](#various-quality-of-life-upgrades)**
- **[desktop and mobile toasts](#various-quality-of-life-upgrades)**
- **[Android + iOS device support](#adb-reverse-proxy-for-device-hot-reload)**
- **[Fully customize iOS and Android projects](#customize-androidmanifestxml-and-infoplist)**
- **[Improved build and bundling](#improved-build-and-bundling)**: multi-package serve, automatic framework bundling, hashless assets, `/public` dir,wasm32 fullstack

## Rust Hot-patching with Subsecond

The biggest feature of this release: Dioxus now supports hot-patching of Rust code at runtime! You can now edit your Rust code and see changes without losing your app’s state.

We’ve been working on this feature for almost an *entire year,* so this is a very special release for us. The tool powering this hot-patching is called *Subsecond* and works across all major platforms: Web (WASM), Desktop (macOS, Linux, Windows), and even mobile (iOS, Android):

![android hotpatching demo](/assets/07/hotpatch-android.mp4)

![ios hotpatching demo](/assets/07/ios-binarypatch.mp4)

You can now iterate on your app’s frontend and backend *simultaneously* without skipping a beat.

![wasm hotpatching demo](/assets/07/hotpatch-wasm-complete.mp4)

Subsecond works in tandem with the Dioxus CLI to enable hot-patching for any Rust project. Simply run `dx serve` on your project and all `subsecond::call` sites will be hot-patched. For example, here’s Subsecond working with a Ratatui app:

![ratatui hotpatching demo](/assets/07/subsecond-tui.mp4)

The infrastructure to support Subsecond is quite complex. Currently, we plan to only ship the Subsecond engine within the Dioxus CLI itself with a long-term plan to spin the engine out into its own crate. For now, we still want the ecosystem to experience the magic of Subsecond, so we’ve made the CLI compatible with non-dioxus projects and removed “dioxus” branding when not serving a dioxus project.

![dx serve without dioxus branding](/assets/07/screenshot-6.avif)

Hot-patching Rust code is no simple feat. To achieve a segfault-free experience, we recommend framework authors to tie into Subsecond’s minimal runtime. For application developers, you can simply use `subsecond::call(some_fn)` at clean integration points to take advantage of hot-patching. If you use Dioxus, hot-patching comes directly integrated with components and server functions.

```rust
pub fn launch() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        subsecond::call(|| tick());
    }
}

fn tick() {
    println!("edit me to see the loop in action!!!!!!!!! ");
}
```

While in theory we could *implicitly* override calls to `tick` with function detouring, we instead chose *explicit* integration points. The first version of subsecond modified process memory externally, but we struggled with issues where the program would be stuck in a task with no way to “resurface”. For this example, the program would always be waiting for IO, making our edits not take effect:

```rust
fn main() {
    loop {
        let next_event = wait_for_io();
        do_thing();
    }
}
```

Instead, the explicit runtime integration provides a simple “synchronization point” where the framework can handle things like closing TCP connections, re-instancing state, dropping event listeners, etc. If you add or remove a field of a struct between hot-patches, Subsecond does not automatically migrate your state for you. Libraries like `bevy-reflect` make this easier - and we might integrate reflection at some point - but for now, frameworks should take care to either dispose or safely migrate structs that change.

We expect folks to use Subsecond outside of Dioxus, namely in web development, so we’ve provided a few starter-integrations for popular libraries:

- Axum
- Bevy
- Ratatui

Subsecond has already made its way into popular projects like Bevy and Iced. Right now, you can `git pull` the latest Bevy and Iced repositories and start hot-patching with zero setup:

![bevy hotpatching demo](/assets/07/bevy-hotpatch.mp4)

Hot-patching covers nearly *every* case in Dioxus. Many tasks that were previously massively burdensome are now a breeze:

- Adding a new `asset!()` call
- Editing strongly-typed interfaces on components like icon variants or links
- Dynamically adding children to a component
- Modifying backend server function code
- Modifying event handler logic - ie `onclick` or `onmouseover`
- Loading resources and async values
- Refactoring rsx into components

Under the hood, we implemented a form of incremental linking / binary patching tailored for running apps. This is not too distant from the idea laid out by Andrew Kelley for Zig.

## Dioxus Native and Blitz

We’re extremely excited to announce the first-ever version of *Dioxus Native*: our new renderer that paints Dioxus apps entirely on the GPU with WGPU.

Out of the box, it already supports a huge number of features

- Accessibility integration
- Event handling
- Asset fetching and loading.

Dioxus Native required a monumental amount of work, pushing forward

- HTML/CSS layout and rendering
- High quality text painting

We’re *extremely* excited to release Blitz: our modular HTML/CSS rendering engine.

Blitz combines a number of exciting projects to bring customizable HTML rendering engine to everyone. Blitz is a result of collaboration across many projects: Firefox, Google, Servo, and Bevy. We’re leveraging a number of powerful libraries:

- Taffy: our high-performance flexbox layout engine
- Stylo: Firefox and Servo’s shared CSS resolution engine
- Vello: Linebender’s GPU compute renderer

Blitz is an extremely capable renderer, often producing results indistinguishable from browsers like Chrome and Safari:

![blitz vs safari comparison](/assets/07/blitzvssafari.avif)

Not every CSS feature is supported yet, with some bugs like incorrect writing direction or the occasional layout quirk. Our support matrix is here: [https://blitz.is/status/css](https://blitz.is/status/css)

The samples that Blitz can create are quite incredible. Servo’s website:

![servo homepage rendered with blitz](/assets/07/image.avif)

Hackernews:

![hackernews homepage rendered with blitz](/assets/07/image-1.avif)

The BBC:

![bbc homepage rendered with blitz](/assets/07/screenshot-1.avif)

We even implemented basic `<form />` support, making it possible to search Wikipedia without a full browser:

![form submission blitz demo](/assets/07/screen-recording-3.mov)

Do note that Blitz is still very young and doesn’t always produce the best outputs, especially on pages that require JavaScript to function properly or use less-popular CSS features:

![blitz failing github render](/assets/07/screenshot-4.avif)

Blitz also provides a pluggable layer for interactivity, supporting actions like text inputs, pluggable widgets, form submissions, hover styling, and more. Here’s Dioxus-Motion working alongside our interactivity layer to provide high quality animations:

![blitz dioxus-motion demo](/assets/07/screen-recording-1.mov)

Bear in mind that Blitz is still considered a “work in progress.” We have not focused on performance

## Dioxus Fullstack Overhaul

We completely revamped Dioxus Fullstack, bringing in a new syntax and a whole host of new features.

To start, we've introduced a new `dioxus::serve` entrypoint for server apps that enables hot-patching of axum routers:

![fullstack hotpatching demo](/assets/07/fullstack-hotpatch.mp4)

The new syntax upgrades makes it easy to declare stable endpoints with a syntax inspired by the popular Rocket library.

```rust
/// you can now encode query and path parameters in the macro!
#[get("/api/{name}/?age")]
async fn get_message(name: String, age: i32) -> Result<String> {
    Ok(format!("Hello {}, you are {} years old!", name, age))
}
```

This revamp allows you to use any valid Axum handler as a Dioxus Server Function, greatly expanding what bodies are allowed. This introduces a number of useful utilities like:

- Server Sent Events with `ServerEvents<T>` type
- `Websocket` and `use_websocket` for long-lived bidirectional communication
- `Streaming<T>` for arbitrary data streams
- Typed `Form<T>` type and `MultipartFormData` for handling forms
- `FileStream` for streaming uploads and downloads

An example of the new APIs in example is this simple websocket handler:

```rust
#[get("/api/uppercase_ws?name&age")]
async fn uppercase_ws(
    name: String,
    age: i32,
    options: WebSocketOptions,
) -> Result<Websocket<ClientEvent, ServerEvent, CborEncoding>> {
    Ok(options.on_upgrade(move |mut socket| async move {
        // send back a greeting message
        _ = socket
            .send(ServerEvent::Uppercase(format!(
                "Fist message from server: Hello, {}! You are {} years old.",
                name, age
            )))
            .await;

        // Loop and echo back uppercase messages
        while let Ok(ClientEvent::TextInput(next)) = socket.recv().await {
            _ = socket.send(ServerEvent::Uppercase(next)).await;
        }
    }))
}
```

Paired with the `use_websocket` hook, you can easily send and receive messages directly from your frontend.

```rust
fn app() -> Element {
    // Track the messages we've received from the server.
    let mut messages = use_signal(std::vec::Vec::new);

    // The `use_websocket` wraps the `WebSocket` connection and provides a reactive handle to easily
    // send and receive messages and track the connection state.
    let mut socket = use_websocket(|| uppercase_ws("John Doe".into(), 30, WebSocketOptions::new()));

    // Calling `.recv()` automatically waits for the connection to be established and deserializes
    // messages as they arrive.
    use_future(move || async move {
        while let Ok(msg) = socket.recv().await {
            messages.push(msg);
        }
    });

    rsx! {
        h1 { "WebSocket Example" }
        p { "Type a message and see it echoed back in uppercase!" }
        p { "Connection status: {socket.status():?}" }
        input {
            placeholder: "Type a message",
            oninput: move |e| async move { _ = socket.send(ClientEvent::TextInput(e.value())).await; },
        }
        button { onclick: move |_| messages.clear(), "Clear messages" }
        for message in messages.read().iter().rev() {
            pre { "{message:?}" }
        }
    }
}
```

![websocket demo](/assets/07/websocket-demo.mp4)

## Dioxus Primitives - a collection of Radix-UI equivalents

You asked, we listened. Dioxus now has a first-party component library based on the popular JavaScript library, Radix-Primitives. Our library implements 28 foundational components that you can mix, match, customize, and restyle to fit your project. Each component comes unstyled and is fully equipped with keyboard-shortucts, ARIA accessibility, and is designed to work seamlessly across web, desktop and mobile.

![component library calendar](/assets/07/screen-recording-2.mov)

In addition to the unstyled primitives, the [components page](https://www.notion.so/Dioxus-0-7-Release-Post-1c5f1847ef8e80579ddae7e4320de518?pvs=21) includes a shadcn-style version of each primitive with css you can copy into your project to build a component library for your project. You can combine these primitives to create larger building blocks like cards, dashboards and forms.

![component library homepage](/assets/07/screenshot-10.avif)

The community has already started construction on new component variants with an exciting project called Lumenblocks built by the [Leaf Computer](https://leaf.computer/) team.

![lumenblocks homepage](/assets/07/screenshot-13.avif)

## Stores - a new primitive for nested reactive state

We introduced signals in 0.5 to enable fine grained reactive updates in dioxus. Signals are great for atomic piece of state in a component like a string or number, but they are difficult to use with external or nested state. Stores are a powerful new primitive for nested reactive state in 0.7.

With stores, you can derive a store trait on your data to let you zoom into specific parts of the state:

```rust
#[derive(Store)]
struct Dir {
    children: BTreeMap<String, Dir>,
}

// You can use the children method to get a reactive reference to just that field
let mut children: Store<Vec<Dir>, _> = directory.children();
```

Stores also include implementations for common data structures like BTreeMap that mark only the changed items as dirty for each operation:

```rust
#[component]
fn Directory(directory: Store<Dir>) -> Element {
    // Create a temporary to reference just the reactive child field
    let mut children = directory.children();
    rsx! {
        ul {
            // Iterate through each reactive value in the children
            for (i, dir) in children.iter().enumerate() {
                li {
                    key: "{dir.path()}",
                    div {
                        display: "flex",
                        flex_direction: "row",
                        "{dir.path()}",
                        button {
                            onclick: move |_| {
                                children.remove(i);
                            },
                            "x"
                        }
                    }
                    Directory { directory: dir }
                }
            }
        }
    }
}

```

When we remove a directory from the store, it will only rerun the parent component that iterated over the BTreeMap and the child that was removed.

![HTML tree of with photos div removed](/assets/07/untitled_(1).avif)

## Automatic Tailwind

The community has been asking for automatic Tailwind for a very long time. Finally in Dioxus 0.7,  `dx` detects if your project has a `tailwind.css` file at the root, and if it does, automatically starts a TailwindCSS watcher for you. You no longer need to manually start or download the Tailwind CLI - everything is handled for you seamlessly in the background:

![tailwind hotreloading demo](/assets/07/tailwind-inline.mp4)

We’ve updated our docs and examples to Tailwind V4, but we’ve also made sure the CLI can handle and autodetect both V3 and V4. Automatic Tailwind support is an amazing feature and we’re sorry for not having integrated it earlier!

## Improvements with AI - LLMs.txt and “vibe-coding”

If you’ve kept up with the news recently, it’s become obvious that AI and Large Language Models are taking over the world. The AI world moves quickly with new tools and improvements being released every week. While the reception of LLMs in the Rust community seems to be mixed, we don’t want Dioxus to be left behind!

In Dioxus 0.7, we’re shipping our first step in the AI world with a first-party `llms.txt` automatically generated from the Dioxus documentation! LLMs can easily stay up to date on new Dioxus features and best practices, hopefully reducing hallucinations when integrating with tools like Copilot and Cursor.

The latest version of the template also includes an optional set of prompts with context about the latest release of dioxus. The prompts provide condensed information about dioxus for tools that don’t have access to web search or llms.txt integration.

Combined with the Subsecond hot-patching work, users can now more effectively “vibe code” their apps without rebuilding. While we don’t recommend “vibe coding” high-stakes parts of your app, modern AI tools are quite useful for quickly whipping up prototypes and UI.

![rust vibe coding demo](/assets/07/vibe-code-2.mp4)

## WASM Bundle Splitting and Lazy Loading

Dioxus 0.7 introduces automatic code splitting and lazy loading for WebAssembly apps. Instead of shipping a single monolithic `.wasm` binary to the browser, `dx` now splits your app into smaller chunks based on your router. Each route's code is loaded on-demand as the user navigates, dramatically reducing initial load times for larger apps.

To enable bundle splitting, simply add `wasm_split` to your route:

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Home,
    #[wasm_split("/dashboard")]
    Dashboard,
    #[wasm_split("/settings")]
    Settings,
}
```

Routes marked with `#[wasm_split]` are split into separate `.wasm` files that are fetched only when the user navigates to that route. This works seamlessly with the Dioxus router -- users see a loading state while the chunk is fetched, then the route renders as normal.

![network tab of dioxuslabs homepage](/assets/07/bundle-split.mp4)

## Integrated Debugger

To date, debugging Rust apps with VSCode hasn’t been particularly easy. Each combination of launch targets, flags, and arguments required a new entry into your `vscode.json` With Dioxus 0.7, we wanted to improve debugging, so we’re shipping a debugger integration! While running `dx serve`, simply press `d` and the current LLDB instance will attach to currently running app. The new debugger integration currently only works with VSCode-based editor setups, but we’d happily accept contributions to expand our support to Neovim, Zed, etc.

![vscode debugger with dioxus project](/assets/07/debugger-dx.mp4)

The integrated debugger is particularly interesting since it works across the web, desktop, and mobile. Setting up an Android debugger from VSCode is particularly challenging, and the new integration makes it much easier.

![vscode debugger with dioxus android project](/assets/07/debug-android-vscode.mp4)

When launching for the web, we actually open a new Chrome instance with a debugger attached. Provided you download the DWARF symbols extension, Rust symbols will show up properly demangled in the debugger tab instead of confusing function addresses.

![vscode debugger with dioxus web project](/assets/07/debugger-web.mp4)


## Various Quality of Life Upgrades

We’ve shipped a number of quality-of-life upgrades that don’t necessarily warrant their own section in the release notes.

Now, when you launch a mobile app, `dx` will automatically open the iOS and Android simulator:

![ios simluator opening after dx build is done](/assets/07/auto-launch.mp4)

Desktop and mobile now have the same development-mode toasts:

![rebuild toast on dioxus ios app](/assets/07/mobile-toast.mp4)

The log coloring of the CLI and help menus have been upgraded to match cargo and reflect error/warn/debug/info levels:

![red compile error in dx console](/assets/07/screenshot-5.avif)

### DX Compatibility with *any* project

The dioxus CLI “dx” tooling is now usable with any Rust project, not just Dioxus projects! You can use `dx` alongside any Rust project, getting a number of awesome features for free:

- Rust hot-reloading with Subsecond
- Packaging and bundling for Web/Desktop/Mobile
- Extraction and optimization of assets included with the `asset!()` macro
- Interactive TUI with shortcuts to rebuild your app
- Tracing integration to toggle “verbose” and “tracing” log levels
- Simultaneous multi-package `dx serve @client @server` support
- Integrated debugger

Notably, Bevy has already integrated support for Subsecond and works well with the new dx:

![bevy project using subsecond](/assets/07/bevy-scad_(online-video-cutter.com).mp4)

We have big plans for `dx` and will improve it by adding support for more features:

- Remote build caching for instant fresh compiles
- Advanced caching for incremental builds in CI
- Dedicated Docker and GitHub images for cross-platform distribution
- Adapters to make your project usable with Bazel / Buck2
- Built-in deploy command for deploying to AWS/GCP/Azure/Cloudflare
- Integrated `#[test]` and `#[preview]` attributes that work across web, desktop, and mobile
- Inline VSCode Simulator support
- Detailed build timings for cargo and bundling
- CI integration with integrated dashboard

## Improved Version Management Experience

Dioxus has supported binary installation for quite a while - but we’ve always required users to install `cargo binstall` and then run `cargo binstall dioxus-cli`. Now, we’re dropping the `cargo binstall` requirement entirely, making it easy to install the CLI and then keep it updated.

To install the CLI:

```bash
curl -fsSL https://dioxus.dev/install.sh | bash
```

Whenever the Dioxus team pushes new updates, the CLI will automatically give you a one-time update notification. To update, you can use

```bash
dx self-update
```

![dx self-update command](/assets/07/screenshot-9.avif)

When you try to use the dioxus-cli with an incompatible dioxus version, you’ll receive a warning and some instructions on how to update.

![dx serve error mismatching dx and dioxus versions](/assets/07/screenshot-12.avif)

## Customize AndroidManifest.xml and Info.plist

Dioxus 0.7 ships with a unified interface in `dioxus.toml` to customize the entire mobile build process. You can set permissions, entitlements, and every field of `AndroidManifest.xml` and `Info.plist` directly from your config -- no need to manually edit platform-specific XML or plist files. This makes it easy to configure splash screens, customize icons, request camera or location permissions, set URL schemes, and fully tweak your apps for deployment, all from a single place.

### ADB Reverse Proxy for Device Hot-Reload

Thanks to community contributions, `dx serve --platform android` now supports Android devices! You can edit markup, modify assets, and even hot-patch on a real Android device without needing to boot a simulator. This works by leveraging `adb reverse`, and should help speed up Android developers looking to test their apps on real devices.

### iPad Support

A small update - Dioxus now properly supports iPad devices! When you `dx serve --platform ios` with an iPad simulator open, your Dioxus app will properly scale and adapt to the iPadOS environment.

![dioxus app on the ipad](/assets/07/screenshot-2.avif)

## Basic Telemetry

- Anonymized by default
- Using it to hunt down panics in tooling and in dioxus itself (during development)
- Want to provide more robust library and tooling - github issues only captures a snapshot
- Opt out

Over time, the CLI has grown from a simple server that watches for file changes and reruns cargo to a tool that helps you through every stage of your apps lifecycle with support for bundling, asset optimization, hot patching, hot reloading, and translation. As the complexity has grown, so has the surface area for bugs and UX issues. To make the CLI more robust, we have started collecting a minimal set of telemetry data in 0.7. This information will help to catch rare panics, performance issues and trends over time that don’t show up in github issues. All telemetry is anonymized with all personal information stripped. We collect:

- Commands invoked without args (eg. `dx serve --hot-patch --profile <stripped> --package <stripped>`)
- Timing information for each build state (eg. `asset optimization: 2s, linking: 1s, wasm-bindgen: 4s`)
- Panics and errors from the CLI with all paths stripped (eg. `unwrap() at <stripped>/cli/src/build.rs` )
- An unique identifier based on a hash of your system information (eg. `HARDWARE_ID=218853676744316928865703503826531902998`)
- Your target triple, dx version and if you are running in CI: (eg. `TRIPLE=arch64-apple-darwin CI=false DX=0.7.0-alpha.3` )

For reference, here is a snippet of the telemetry collected over the last week on my installation:

```rust
{"identity":{"device_triple":"aarch64-apple-darwin","is_ci":false,"cli_version":"0.7.0-alpha.3 (ba856ac)","session_id":218853676744316928865703503826531902998},"name":"cli_command","module":null,"message":"serve","stage":"start","time":"2025-07-24T14:34:07.684804Z","values":{"args":{"address":{"addr":null,"port":null},"always_on_top":null,"cross_origin_policy":false,"exit_on_error":false,"force_sequential":false,"hot_patch":false,"hot_reload":null,"interactive":null,"open":null,"platform_args":{"client":null,"server":null,"shared":{"args":false,"targets":{"build_arguments":{"all_features":false,"base_path":false,"bin":null,"bundle":null,"cargo_args":false,"debug_symbols":true,"device":false,"example":false,"features":false,"inject_loading_scripts":true,"no_default_features":false,"package":null,"platform":null,"profile":false,"release":false,"renderer":{"renderer":null},"rustc_args":false,"skip_assets":false,"target":null,"target_alias":"Unknown","wasm_split":false},"fullstack":null,"ssg":false}}},"watch":null,"wsl_file_poll_interval":null}}}
{"identity":{"device_triple":"aarch64-apple-darwin","is_ci":false,"cli_version":"0.7.0-alpha.3 (ba856ac)","session_id":218853676744316928865703503826531902998},"name":"build_stage","module":null,"message":"Build stage update","stage":"installing_tooling","time":"2025-07-24T14:34:08.358050Z","values":{}}
{"identity":{"device_triple":"aarch64-apple-darwin","is_ci":false,"cli_version":"0.7.0-alpha.3 (ba856ac)","session_id":218853676744316928865703503826531902998},"name":"build_stage","module":null,"message":"Build stage update","stage":"starting","time":"2025-07-24T14:34:08.950001Z","values":{}}
{"identity":{"device_triple":"aarch64-apple-darwin","is_ci":false,"cli_version":"0.7.0-alpha.3 (ba856ac)","session_id":218853676744316928865703503826531902998},"name":"build_stage","module":null,"message":"Build stage update","stage":"compiling","time":"2025-07-24T14:34:09.543745Z","values":{}}
{"identity":{"device_triple":"aarch64-apple-darwin","is_ci":false,"cli_version":"0.7.0-alpha.3 (ba856ac)","session_id":218853676744316928865703503826531902998},"name":"build_stage","module":null,"message":"Build stage update","stage":"extracting_assets","time":"2025-07-24T14:34:10.142290Z","values":{}}
{"identity":{"device_triple":"aarch64-apple-darwin","is_ci":false,"cli_version":"0.7.0-alpha.3 (ba856ac)","session_id":218853676744316928865703503826531902998},"name":"build_stage","module":null,"message":"Build stage update","stage":"bundling","time":"2025-07-24T14:34:10.319760Z","values":{}}
{"identity":{"device_triple":"aarch64-apple-darwin","is_ci":false,"cli_version":"0.7.0-alpha.3 (950b12e)","session_id":207130784956002540532192240548422216472},"name":"cli_command","module":null,"message":"serve","stage":"start","time":"2025-07-24T14:55:31.419714Z","values":{"args":{"address":{"addr":null,"port":null},"always_on_top":null,"cross_origin_policy":false,"exit_on_error":false,"force_sequential":false,"hot_patch":false,"hot_reload":null,"interactive":null,"open":null,"platform_args":{"client":null,"server":null,"shared":{"args":false,"targets":{"build_arguments":{"all_features":false,"base_path":false,"bin":null,"bundle":null,"cargo_args":false,"debug_symbols":true,"device":false,"example":false,"features":false,"inject_loading_scripts":true,"no_default_features":false,"package":null,"platform":null,"profile":false,"release":false,"renderer":{"renderer":null},"rustc_args":false,"skip_assets":false,"target":null,"target_alias":"Unknown","wasm_split":false},"fullstack":null,"ssg":false}}},"watch":null,"wsl_file_poll_interval":null}}}
```

The full logs are available here: [http://gist.github.com/ealmloff/815d859bb8c592a72769e958e685f7f2](http://gist.github.com/ealmloff/815d859bb8c592a72769e958e685f7f2)

You can opt-out of telemetry by compiling the CLI with the `disable-telemetry` feature, setting `TELEMETRY=false` in your environment variables or running `dx config set disable-telemetry true`

### Expanded Documentation

We reorganized and expanded the documentation for core concepts in 0.7. The docs now go into more details about important concepts like reactivity, the rendering model of dioxus, and async state in dioxus. The new docs also come with a new look for the docsite with a wider panel that fits more documentation in the screen:

![0.7 learn intro to reactivity](/assets/07/screenshot-3.avif)

The new docsite also includes search results for rust items from [docs.rs](http://docs.rs) for more specific apis:

![use_set_ search results with two results from crates.io](/assets/07/screenshot-7.avif)

## Improved Build and Bundling

Dioxus 0.7 includes a number of improvements to the build and bundling pipeline in `dx`.

**Multi-package serve:** You can now serve multiple packages simultaneously with `dx serve @client --package xyz @server --package xyz`, making it easy to work on complex workspaces with separate client and server crates.

**Automatic dylib / framework bundling:** The CLI now automatically intercepts `-framework` flags in the linker and bundles frameworks properly. No more manually copying `.dylib` or `.framework` files into your app bundle -- `dx` handles it for you.

**wasm32 support for fullstack:** Fullstack apps can now target `wasm32`, enabling deployment to platforms like Cloudflare Workers that run server-side WebAssembly.

**Hashless assets:** You can now use `asset!()` without writing an asset hash, making it easier to reference assets in your code without dealing with content-hashed filenames.

**`/public` directory:** Dioxus now supports a `/public` directory at the root of your project. Files placed here are merged into your final bundle as-is, making it easy to include things like `robots.txt`, `.well-known` files, favicons, and other static resources that need to live at a fixed path.

## Upgrading from 0.6 to 0.7

We've assembled a [migration guide](https://dioxuslabs.com/learn/0.7/migration/to_07) to help you upgrade your existing apps from Dioxus 0.6 to 0.7. For a full overview of the framework, check out the [updated documentation](https://dioxuslabs.com/learn/0.7/).

## Conclusion

That's it for Dioxus 0.7! This release represents over 350 pull requests merged and hundreds of issues closed. From hot-patching Rust code at runtime to a brand new GPU-powered native renderer, we've pushed the boundaries of what's possible for app development in Rust.

Looking ahead, we're planning to focus on:

- Native platform APIs (camera, geolocation, file system, push notifications, and more) in 0.8
- `dx deploy` for one-command deployment to cloud platforms
- Continued performance improvements for Blitz and Dioxus Native
- More tutorial videos, starter templates, and documentation

If you're interested in contributing, check out our [GitHub](https://github.com/DioxusLabs/dioxus), join our [Discord](https://discord.gg/XgGxMSkvUM), or follow us on [Twitter](https://twitter.com/dioxuslabs).

## Thanks to the community!

We want to extend a huge thank-you to everyone who helped test and improve this release. Dioxus 0.7 saw an incredible number of contributors fix bugs and add features. Special thanks to:

[@3lpsy](https://github.com/3lpsy) - [@Andrew15-5](https://github.com/Andrew15-5) - [@arvidfm](https://github.com/arvidfm) - [@atty303](https://github.com/atty303) - [@avij1109](https://github.com/avij1109) - [@bananabit-dev](https://github.com/bananabit-dev) - [@brianmay](https://github.com/brianmay) - [@bwskin](https://github.com/bwskin) - [@carlosdp](https://github.com/carlosdp) - [@caseykneale](https://github.com/caseykneale) - [@chrivers](https://github.com/chrivers) - [@clouds56](https://github.com/clouds56) - [@conectado](https://github.com/conectado) - [@Craig-Macomber](https://github.com/Craig-Macomber) - [@created-by-varun](https://github.com/created-by-varun) - [@CristianCapsuna](https://github.com/CristianCapsuna) - [@CryZe](https://github.com/CryZe) - [@d-corler](https://github.com/d-corler) - [@dabrahams](https://github.com/dabrahams) - [@damccull](https://github.com/damccull) - [@danielkov](https://github.com/danielkov) - [@DanielWarloch](https://github.com/DanielWarloch) - [@davidB](https://github.com/davidB) - [@debanjanbasu](https://github.com/debanjanbasu) - [@DogeDark](https://github.com/DogeDark) - [@dowski](https://github.com/dowski) - [@DrewRidley](https://github.com/DrewRidley) - [@dsgallups](https://github.com/dsgallups) - [@dtolnay](https://github.com/dtolnay) - [@ealmloff](https://github.com/ealmloff) - [@emmanuel-ferdman](https://github.com/emmanuel-ferdman) - [@FalkWoldmann](https://github.com/FalkWoldmann) - [@fasterthanlime](https://github.com/fasterthanlime) - [@flba-eb](https://github.com/flba-eb) - [@fontlos](https://github.com/fontlos) - [@gammahead](https://github.com/gammahead) - [@gbutler69](https://github.com/gbutler69) - [@hackartists](https://github.com/hackartists) - [@Hasnep](https://github.com/Hasnep) - [@hecrj](https://github.com/hecrj) - [@Himmelschmidt](https://github.com/Himmelschmidt) - [@Jasper-Bekkers](https://github.com/Jasper-Bekkers) - [@javierEd](https://github.com/javierEd) - [@Jayllyz](https://github.com/Jayllyz) - [@jerome-caucat](https://github.com/jerome-caucat) - [@jjvn84](https://github.com/jjvn84) - [@Kbz-8](https://github.com/Kbz-8) - [@Kijewski](https://github.com/Kijewski) - [@Klemen2](https://github.com/Klemen2) - [@kristoff3r](https://github.com/kristoff3r) - [@ktechhydle](https://github.com/ktechhydle) - [@kyle-nweeia](https://github.com/kyle-nweeia) - [@laundmo](https://github.com/laundmo) - [@leo030303](https://github.com/leo030303) - [@LeWimbes](https://github.com/LeWimbes) - [@LilahTovMoon](https://github.com/LilahTovMoon) - [@luckybelcik](https://github.com/luckybelcik) - [@Makosai](https://github.com/Makosai) - [@marc2332](https://github.com/marc2332) - [@marcobergamin-videam](https://github.com/marcobergamin-videam) - [@mcmah309](https://github.com/mcmah309) - [@MintSoup](https://github.com/MintSoup) - [@mockersf](https://github.com/mockersf) - [@mohe2015](https://github.com/mohe2015) - [@mzdk100](https://github.com/mzdk100) - [@Nathy-bajo](https://github.com/Nathy-bajo) - [@navyansh007](https://github.com/navyansh007) - [@nicoburns](https://github.com/nicoburns) - [@nilswloewen](https://github.com/nilswloewen) - [@OlivierLemoine](https://github.com/OlivierLemoine) - [@omar-mohamed-khallaf](https://github.com/omar-mohamed-khallaf) - [@otgerrogla](https://github.com/otgerrogla) - [@pandarrr](https://github.com/pandarrr) - [@PhilTaken](https://github.com/PhilTaken) - [@Plebshot](https://github.com/Plebshot) - [@priezz](https://github.com/priezz) - [@pythoneer](https://github.com/pythoneer) - [@rennanpo](https://github.com/rennanpo) - [@rhaskia](https://github.com/rhaskia) - [@RickWong](https://github.com/RickWong) - [@RobertasJ](https://github.com/RobertasJ) - [@ryo33](https://github.com/ryo33) - [@s3bba](https://github.com/s3bba) - [@sebdotv](https://github.com/sebdotv) - [@sehnryr](https://github.com/sehnryr) - [@SilentVoid13](https://github.com/SilentVoid13) - [@snatvb](https://github.com/snatvb) - [@srghma-old](https://github.com/srghma-old) - [@stevelr](https://github.com/stevelr) - [@StudioLE](https://github.com/StudioLE) - [@tekacs](https://github.com/tekacs) - [@tgrushka](https://github.com/tgrushka) - [@Threated](https://github.com/Threated) - [@thyseus](https://github.com/thyseus) - [@Tumypmyp](https://github.com/Tumypmyp) - [@wdcocq](https://github.com/wdcocq) - [@wdjcodes](https://github.com/wdjcodes) - [@wheregmis](https://github.com/wheregmis) - [@windows-fryer](https://github.com/windows-fryer) - [@wiseaidev](https://github.com/wiseaidev) - [@wosienko](https://github.com/wosienko) - [@Yuhanawa](https://github.com/Yuhanawa) - [@yydcnjjw](https://github.com/yydcnjjw) - [@zhiyanzhaijie](https://github.com/zhiyanzhaijie)
