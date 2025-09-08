Welcome back to another Dioxus release! Dioxus (dye • ox • us) is a framework for building cross-platform apps in Rust. We make it easy to ship full-stack web, desktop, and mobile apps with a single codebase.

Dioxus 0.7 delivers on a number of promises we made to improve Rust GUI, and more broadly, what we call “high level Rust.” Rust has excelled as a tool for building foundational software, but we hope with Dioxus 0.7, it’s one step closer to being suitable for rapid, high-level development.

In this release, we’re shipping some incredible features. The highlights of this release include:

- Subsecond: Hot-patching of Rust code at runtime
- Dioxus Native: WGPU-based HTML/CSS renderer for Dioxus
- WASM-Split: Code splitting and lazy loading for WebAssembly
- Stores: A new primitive for nested reactive state
- Dioxus Primitives: first-party radix-primitives implementation for Dioxus

Dioxus 0.7 also brings a number of other exciting new features:

- Automatic tailwind: zero-setup tailwind support built-in!
- LLMs.txt: first-party context file to supercharge AI coding models
- Blitz: our modular HTML/CSS renderer powering Dioxus Native, available for everyone!
- Fullstack WebSockets: websockets in a single line of code
- Integrated Debugger Support: open CodeLLDB or Neovim DAP with a single keystroke
- Fullstack error codes: Integration of status codes and custom errors in fullstack
- Configurable Mobile Builds: Customize your AndroidManifest and Info.plist

Plus, a number of quality-of-life upgrades:

- one-line installer ( curl [dioxus.dev/install.sh](http://dioxus.dev/install.sh) | sh )
- `dx self-update` and update notifications
- automatically open simulators
- Improved log coloring
- desktop and mobile toasts
- HTML streaming now waits for the router to render
- Axum 0.8 and Wry 52 upgrade
- Android and iOS device support (—device)
- More customization of iOS and Android projects
- Hash Router Support for dioxus-web
- Multi-package serve: `dx serve @client --package xyz @server --package xyz`
- Support for dyib bundling
- wasm32 support for fullstack
- Hashless assets
- And many, many bugs fixed!

## Note from the author

Dioxus 0.7 marks the second anniversary of me (Jonathan Kelley) going full time on Dioxus. How time flies! In the past two years we shipped so much:

- Template Hot-Reloading and Autoformatting
- Migration to Signals for state management
- First-party Android and iOS tooling
- Server Function integration
- Linker-based asset system
- and so much more!

The road here has been long and frankly, lots of work. When we started out, the Rust ecosystem had very few good solutions to the basic problems in application development. Even now, the Rust hot-patching and native renderers - while incredible achievements on their own - are just “par for the course” for application development.

With Dioxus 0.7, I feel like the Dioxus foundations are finally solid. We have excellent developer tools, lightning-fast hot-reload, a great asset system, a solid RPC solution, bundle splitting, automatic optimizations, autocomplete, auto-formatting, a capable state management solution, comprehensive docs, and funding for the foreseeable future.

What of the future? I finally feel like we’re on the “other side” of the once-impossible problems. With hot-patching and the native renderer behind us, we’re quite free to work on smaller projects. We could definitely use better marketing, more tutorial videos, better starter templates, and more libraries (native APIs in 0.8!). Thanks for all the support so far!

## Rust Hot-patching with Subsecond

The biggest feature of this release: Dioxus now supports hot-patching of Rust code at runtime! You can now edit your Rust code and see changes without losing your app’s state.

< demo of hotpatching >

We’ve been working on this feature for almost an *entire year,* so this is a very special release for us. The tool powering this hot-patching is called *Subsecond* and works across all major platforms: Web (WASM), Desktop (macOS, Linux, Windows), and even mobile (iOS, Android):

![android hotpatching demo](./assets/07/hotpatch-android.mp4)

![ios hotpatching demo](./assets/07/ios-binarypatch.mp4)

You can now iterate on your app’s frontend and backend *simultaneously* without skipping a beat.

![wasm hotpatching demo](./assets/07/hotpatch-wasm-complete.mp4)

Subsecond works in tandem with the Dioxus CLI to enable hot-patching for any Rust project. Simply run `dx serve` on your project and all `subsecond::call` sites will be hot-patched. For example, here’s Subsecond working with a Ratatui app:

![ratatui hotpatching demo](./assets/07/subsecond-tui.mp4)

The infrastructure to support Subsecond is quite complex. Currently, we plan to only ship the Subsecond engine within the Dioxus CLI itself with a long-term plan to spin the engine out into its own crate. For now, we still want the ecosystem to experience the magic of Subsecond, so we’ve made the CLI compatible with non-dioxus projects and removed “dioxus” branding when not serving a dioxus project.

![dx serve without dioxus branding](./assets/07/screenshot-6.avif)

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

![bevy hotpatching demo](./assets/07/bevy-hotpatch.mp4)

Hot-patching covers nearly *every* case in Dioxus. Many tasks that were previously massively burdensome are now a breeze:

- Adding a new `asset!()` call
- Editing strongly-typed interfaces on components like icon variants or links
- Dynamically adding children to a component
- Modifying backend server function code
- Modifying event handler logic - ie `onclick` or `onmouseover`
- Loading resources and async values
- Refactoring rsx into components

Hotpatching also handles quite large projects - for example, our docsite at 25k LoC:

< demo of editing the docsite landing page >

Under the hood, we implemented a form of incremental linking / binary patching tailored for running apps. This is not too distant from the idea laid out by Andrew Kelley for Zig. We have yet to release an in-depth technical writeup about how Subsecond works, but if you’re really interested, come join us at the Seattle RustConf and learn about it during our talk!

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
- Vello: Google’s GPU compute renderer

Blitz is an extremely capable renderer, often producing results indistinguishable from browsers like Chrome and Safari:

![blitz vs safari comparison](./assets/07/blitzvssafari.avif)

Not every CSS feature is supported yet, with some bugs like incorrect writing direction or the occasional layout quirk. Our support matrix is here: [https://blitz-website.fly.dev/support-matrix](https://blitz-website.fly.dev/support-matrix)

The samples that Blitz can create are quite incredible. Servo’s website:

![servo homepage rendered with blitz](./assets/07/image.avif)

Hackernews:

![hackernews homepage rendered with blitz](./assets/07/image-1.avif)

The BBC:

![bbc homepage rendered with blitz](./assets/07/screenshot-1.avif)

We even implemented basic `<form />` support, making it possible to search Wikipedia without a full browser:

![form submission blitz demo](./assets/07/screen-recording-3.mov)

Do note that Blitz is still very young and doesn’t always produce the best outputs, especially on pages that require JavaScript to function properly or use less-popular CSS features:

![blitz failing github render](./assets/07/screenshot-4.avif)

Blitz also provides a pluggable layer for interactivity, supporting actions like text inputs, pluggable widgets, form submissions, hover styling, and more. Here’s Dioxus-Motion working alongside our interactivity layer to provide high quality animations:

![blitz dioxus-motion demo](./assets/07/screen-recording-1.mov)

Bear in mind that Blitz is still considered a “work in progress.” We have not focused on performance

## Dioxus Primitives - a collection of Radix-UI equivalents

You asked, we listened. Dioxus now has a first-party component library based on the popular JavaScript library, Radix-Primitives. Our library implements 28 foundational components that you can mix, match, customize, and restyle to fit your project. Each component comes unstyled and is fully equipped with keyboard-shortucts, ARIA accessibility, and is designed to work seamlessly across web, desktop and mobile.

![component library calendar](./assets/07/screen-recording-2.mov)

In addition to the unstyled primitives, the [components page](https://www.notion.so/Dioxus-0-7-Release-Post-1c5f1847ef8e80579ddae7e4320de518?pvs=21) includes a shadcn-style version of each primitive with css you can copy into your project to build a component library for your project. You can combine these primitives to create larger building blocks like cards, dashboards and forms.

![component library homepage](./assets/07/screenshot-10.avif)

The community has already started construction on new component variants with an exciting project called Lumenblocks built by the [Leaf Computer](https://leaf.computer/) team.

![lumenblocks homepage](./assets/07/screenshot-13.avif)

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

![HTML tree of with photos div removed](./assets/07/untitled_(1).avif)

## Automatic Tailwind

The community has been asking for automatic Tailwind for a very long time. Finally in Dioxus 0.7,  `dx` detects if your project has a `tailwind.css` file at the root, and if it does, automatically starts a TailwindCSS watcher for you. You no longer need to manually start or download the Tailwind CLI - everything is handled for you seamlessly in the background:

![tailwind hotreloading demo](./assets/07/tailwind-inline.mp4)

We’ve updated our docs and examples to Tailwind V4, but we’ve also made sure the CLI can handle and autodetect both V3 and V4. Automatic Tailwind support is an amazing feature and we’re sorry for not having integrated it earlier!

## Improvements with AI - LLMs.txt and “vibe-coding”

If you’ve kept up with the news recently, it’s become obvious that AI and Large Language Models are taking over the world. The AI world moves quickly with new tools and improvements being released every week. While the reception of LLMs in the Rust community seems to be mixed, we don’t want Dioxus to be left behind!

In Dioxus 0.7, we’re shipping our first step in the AI world with a first-party `llms.txt` automatically generated from the Dioxus documentation! LLMs can easily stay up to date on new Dioxus features and best practices, hopefully reducing hallucinations when integrating with tools like Copilot and Cursor.

The latest version of the template also includes an optional set of prompts with context about the latest release of dioxus. The prompts provide condensed information about dioxus for tools that don’t have access to web search or llms.txt integration.

Combined with the Subsecond hot-patching work, users can now more effectively “vibe code” their apps without rebuilding. While we don’t recommend “vibe coding” high-stakes parts of your app, modern AI tools are quite useful for quickly whipping up prototypes and UI.

![rust vibe coding demo](./assets/07/vibe-code-2.mp4)

## WASM Bundle Splitting and Lazy Loading

![network tab of dioxuslabs homepage](./assets/07/bundle-split.mp4)

## Integrated Debugger

To date, debugging Rust apps with VSCode hasn’t been particularly easy. Each combination of launch targets, flags, and arguments required a new entry into your `vscode.json` or `nvim.dap` file. With Dioxus 0.7, we wanted to improve debugging, so we’re shipping a debugger integration! While running `dx serve`, simply press `d` and the current LLDB / DAP instance will attach to currently running app. The new debugger integration currently only works with VSCode-based editor setups, but we’d happily accept contributions to expand our support to Neovim, Zed, etc.

![vscode debugger with dioxus project](./assets/07/debugger-dx.mp4)

The integrated debugger is particularly interesting since it works across the web, desktop, and mobile. Setting up an Android debugger from VSCode is particularly challenging, and the new integration makes it much easier.

![vscode debugger with dioxus android project](./assets/07/debug-android-vscode.mp4)

When launching for the web, we actually open a new Chrome instance with a debugger attached. Provided you download the DWARF symbols extension, Rust symbols will show up properly demangled in the debugger tab instead of confusing function addresses.

![vscode debugger with dioxus web project](./assets/07/debugger-web.mp4)

## Fullstack WebSockets, improved streaming, and custom Error types

The new version of dioxus also includes several improvements to dioxus fullstack including improved streaming, custom error types and websocket support in server functions.

0.7 changes the behavior of streaming to partially resolve async data before the first chunk of html is sent to the browser. All [SuspenseBoundary](https://dioxuslabs.com/learn/0.7/essentials/advanced/suspense)s before the router is rendered will resolve and be sent in the initial http response. This makes it possible to do some data loading before you determine the status code or finish rendering the head elements:

```rust
#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    // ... other routes
    #[route("/blog/:id/")]
    Blog { id: i32 },
}

#[component]
fn Blog(id: i32) -> Element {
    // This will always resolve in the initial response because it is in the same suspense
    // boundary as the router
    let title = use_server_future(move || blog_title(id))?;
    rsx! {
        // This will be rendered on the server since it is in the initial chunk and be visible
        // to scrapers
        document::Title { {title} }
        // ... other blog content
    }
}
```

The new release of fullstack also bumps some major dependencies including axum and server functions. Thanks to a contribution from [ryo33](https://github.com/ryo33), the latest release of server functions includes support for fully custom error types. This lets you define strongly typed errors shared between your server and client:

```rust

#[derive(Serialize, Deserialize, Debug)]
enum MyCustomError {
    /// Failed to connect to database
    DatabaseConnectionError,
    /// Communication failed
    ServerFn(server_fn::error::ServerFnErrorErr),
}

impl FromServerFnError for MyCustomError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(err: server_fn::error::ServerFnErrorErr) -> Self {
        MyCustomError::ServerFn(err)
    }
}

#[server]
async fn post_server_data(data: String) -> Result<(), MyCustomError> {
    println!("Server received: {}", data);

    Ok(())
}
```

Errors now integrate with the status code system which means you get 500 or 404 errors from internal or routing errors automatically:

![404 network error](./assets/07/screenshot-11.avif)
![500 network error](./assets/07/screenshot-8.avif)

We worked with the server function team to rework the protocol system for server functions in 0.7 to enable websockets. Websockets enable a whole new class of apps that need realtime communication between the server and the client. To enable websockets, you can simply set the protocol to `Websocket` with generics for the serialization schema for the input and output message types:

```rust
// Accept and output a stream of json messages with a websocket
#[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]
async fn uppercase_ws(
    mut input: BoxedStream<String, ServerFnError>,
) -> ServerFnResult<BoxedStream<String, ServerFnError>> {
    // Create a channel with the output of the websocket
    let (mut tx, rx) = mpsc::channel(1);

    // Spawn a task that processes the input stream and sends any new messages to the output
    tokio::spawn(async move {
        while let Some(msg) = input.next().await {
            if tx
                .send(msg.map(|msg| msg.to_ascii_uppercase()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    // Return the output stream
    Ok(rx.into())
}
```

## Various Quality of Life Upgrades

We’ve shipped a number of quality-of-life upgrades that don’t necessarily warrant their own section in the release notes.

Now, when you launch a mobile app, `dx` will automatically open the iOS and Android simulator:

![ios simluator opening after dx build is done](./assets/07/auto-launch.mp4)

Desktop and mobile now have the same development-mode toasts:

![rebuild toast on dioxus ios app](./assets/07/mobile-toast.mp4)

The log coloring of the CLI and help menus have been upgraded to match cargo and reflect error/warn/debug/info levels:

![red compile error in dx console](./assets/07/screenshot-5.avif)

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

![bevy project using subsecond](./assets/07/bevy-scad_(online-video-cutter.com).mp4)

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

![dx self-update command](./assets/07/screenshot-9.avif)

When you try to use the dioxus-cli with an incompatible dioxus version, you’ll receive a warning and some instructions on how to update.

![dx serve error mismatching dx and dioxus versions](./assets/07/screenshot-12.avif)

## Customize AndroidManifest.xml and Info.plist

You can now customize the Info.plist and AndroidManifest.xml files that Dioxus generates for your Android, iOS, and macOS projects. This makes it possible to add entitlements, update permissions, set splash screens, customize icons, and fully tweak your apps for deployment.

![mobile app with open filepicker button](./assets/07/file-picker.mov)

### ADB Reverse Proxy for Device Hot-Reload

Thanks to community contributions, `dx serve --platform android` now supports Android devices! You can edit markup, modify assets, and even hot-patch on a real Android device without needing to boot a simulator. This works by leveraging `adb reverse`, and should help speed up Android developers looking to test their apps on real devices.

### iPad Support

A small update - Dioxus now properly supports iPad devices! When you `dx serve --platform ios` with an iPad simulator open, your Dioxus app will properly scale and adapt to the iPadOS environment.

![dioxus app on the ipad](./assets/07/screenshot-2.avif)

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

![0.7 learn intro to reactivity](./assets/07/screenshot-3.avif)

The new docsite also includes search results for rust items from [docs.rs](http://docs.rs) for more specific apis:

![use_set_ search results with two results from crates.io](./assets/07/screenshot-7.avif)

### New Office and Growing the Team

Dioxus has moved into a new office in San Francisco! If you’re interested in contributing to the future of app development and live in San Francisco, please reach out.

![sf office desktop setup](./assets/07/14aac481-e530-4e73-8903-377c39cbf248_1_105_c.avif)

1. Camera & Microphone File System Geolocation Push Notifications Biometric Authentication In-App Purchases. App Icon Badge. Battery & Power. Tray Icon & System Tray. Deep Linking & URL Schemes. File Open Dialogs. Auto-Update & Self-Patching. Background Processes. LocalStorage & Secure Storage. Drag & Drop. Device Motion & Sensors. Vibration. Clipboard. Titlebar & Menu & Multi-Window & Fullscreen Support. Paypal & Stripe. OAuth2. Gyroscope
