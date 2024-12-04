If you’re new here: Dioxus (dye•ox•us) is a library for building React-like user interface in Rust. Dioxus supports a ton of targets: web, desktop, mobile, TUI, and more. On the web it renders via the DOM and on desktop and mobile you can choose between the WebView DOM, WGPU, or Skia.

Dioxus 0.3 is bringing a *lot* of fantastic new features:

- Massive performance improvements
- Hot reloading for web and desktop
- Autoformatting for RSX via `dioxus fmt`
- New LiveView renderer
- Input widgets for TUI
- Lua plugin system for CLI and overhaul of CLI
- Multi window desktop apps and direct access to Tao/Wry
- General improvements to RSX (if chains, for loops, boolean attributes, any values)
- Rusty event types with support for complex techniques like file uploading
- Skia renderer and WGPU renderer
- Chinese and Portuguese translations
- A new landing page

This release represents an absolutely massive jump forward for the Dioxus ecosystem. We hope to ship future features more quickly into stable now that many of the desired breaking changes have been incorporated into the core library.

## Templates and performance improvements

We’ve made huge changes underpinning the architecture of Dioxus. The significance of these changes is hard to describe in this simple release document, but we did write a blog post about it [here](https://dioxuslabs.com/blog/templates-diffing/). Now, Dioxus performance is on par with of SolidJS.

![Js-framework-benchmark of Dioxus showing good performance](https://i.imgur.com/9rbAXP9.png)

Additionally, we’ve reworked how desktop apps stream edits from the native thread into the webview, greatly improving performance.

## Hot Reloading

Dioxus can now update how your app looks without recompiling the underlying Rust code. For developers who choose to write their user interfaces with the RSX macro, the Dioxus development server will automatically update the appearance of a running app whenever the macro body is modified.

We’ve found hot reloading to significantly speed up development cycles, making it faster than ever to iterate your app.

![hotreload full](https://i.imgur.com/OzIURca.mp4)

Note that hot reloading works by interpreting the body of RSX macro calls. If the hot reloading engine detects a modification unrelated to RSX, then it will force a full refresh of the app.

## Autoformatting

Another widely requested feature - autoformatting - is now built into the Dioxus CLI and VSCode Extension. Using the same interpreter engine in hot reloading, your code can now be formatted automatically. This saves a ton of time and ensures consistency between code commits.

Autoformatting can be used via the VSCode Extension which will autoformat as you code.

![autofmt.mov](https://i.imgur.com/aPQEFNO.mp4)

Or directly for use in CI or non-vscode editors with the `dioxus fmt` command.

![dioxusfmt.mov](https://i.imgur.com/WrNZZdW.mp4)

Autoformatting respects some simple rustfmt features but is still in its early stages. If you find any quirks or disagree with the formatting style, feel free to file an issue.

## LiveView and LiveComponents

Dioxus 0.3 marks the first official release of dedicated tooling for LiveView. LiveView is a new web-app development paradigm that combines the simplicity of server-side rendering with the rich interactivity of the single-page-application.

![liveviewdemo.mov](https://i.imgur.com/Eiejo1h.mp4)

Because there’s no frontend build step or need for a dedicated backend, writing LiveView apps is easy. LiveView lets you freely mix database access into your frontend code, saving the hassle of a dedicated backend. LiveView is the fastest way to build a complete app in Rust.

```rust
async fn main() {
    let router = Router::new()
        .route("/", get(move || dioxus_liveview::body(addr))
        .route("/app", get(move |ws| dioxus_liveview::render(ws));

    axum::Server::bind("127.0.0.1".parse().unwrap())
        .serve(router.into_make_service())
        .await;
}

fn app(cx: Scope) -> Element {
		let posts = use_db_query(cx, RECENT_POSTS);

		render! {
				for post in posts {
						Post { key: "{post.id}", data: post }
				}
		}
}
```

## TUI Input Widgets

Up to this point, Dioxus rendered into the terminal using just static elements. Now, with the release of Dioxus 0.3, we’re shipping a collection of input widgets for common utilities like buttons, sliders, text inputs, checkboxes, and more. These same widgets provide a basis of functionality for the native renderers we mention below.

![tuiinputs.mp4](https://i.imgur.com/oXQC5o5.mp4)

## Multi-window Desktop Apps

The Dioxus VirtualDom and tao/wry event loop now share the same scheduler, allowing full control of the window and event loop from within your desktop and mobile app. This unlocks everything a typical tauri app might have access to, allowing Dioxus to share more code with the rest of the Tauri ecosystem.

One big advantage of this is the ability to open and close multiple windows from within your Dioxus app. With access to the event loop, you can even get a raw window handle, allowing alternative rendering engines like OpenGL or WGPU.

![multiwindow.mov](https://i.imgur.com/4Yg9FWd.mp4)

## Lowercase components

We’ve expanded what can be considered a component. Lowercase components are now accepted in the rsx macro provided that they either

- Use the path syntax (ie `module::component`)
- Container an underscore character

This is a similar restriction as found in other frameworks. Note that you still cannot define a one-word component without referencing it via path syntax. We’re hoping to resolve this soon, but it’s not a very easy problem to solve.

```rust
header {}              ❌
module::header {}      ❌
my_header {}           ✅
```

## For Loops, If Chains, and more flexible RSX

We’ve made the rsx macro a lot more flexible with some new features to simplify lists and if statements.

Before, if you wanted to render a list, you’d need to create an iterator and map it to rsx. Now, we apply an automatic transformation of any `for` loop into an iterator. This should make lists more readable!

```rust
for dog in doggos {
	div { key: "{dog.id}",  "Dog: {dog.name}" }
}
```

## Preliminary WGPU renderer

Dioxus 0.3 delivers on another commonly requested feature: native (non-web browser) rendering. This new update brings a very young, very unstable, but surprisingly capable WGPU renderer. This renderer is the culmination of many months of work: collaboration with the Bevy team to revive Taffy (flexbox), integration of the new Vello renderer, and research into highly efficient incremental screen patching.

The renderer is very raw but already capable of rendering HTML, CSS, and responding to user input. We’re actively working on adding accessibility support using the work done by EGUI as inspiration.

![wgpu](https://i.imgur.com/NVp4COt.mp4)

## Skia Renderer

While not exactly a Dioxus Labs project, we wanted to make sure to call out the new Freya editor for Dioxus which uses Skia instead of Vello. Freya takes a different approach from Dioxus-Native in that instead of adhering to HTML and CSS, it sets its own styling and layout strategy. This has a different learning curve - you  can’t take your CSS knowledge with you, but you get a styling system better designed for the job.

Freya is already an amazing piece of technology and has support for things like camera input and infinite canvas.

## Completing support for cross-platform events


A common complaint with Dioxus’ event system is its reliance on imperfect web standards. For Dioxus 0.3, we overhauled the public API for events to be more “Rusty.” Instead of shipping our own types like keyboard keys, we now provide an API comfortable for the everyday Rustacean. You can now do mouse position math with `euclid`, match on keys native to `keyboard-types`, and get helpful docs with cargo-doc. Dioxus also now provides better support for file upload and drag-and-drop operations by downcasting the native event type if it exists.

![dragdropworks.mov](https://i.imgur.com/DHBvvVy.mp4)

Note that the old JS-like API is still available (but deprecated) and will be phased out in a future release of Dioxus.

## Lua Plugin System for CLI

The CLI has been overhauled with a ton of new features and improved ergonomics. One major improvement to the CLI is the addition of a Lua-based plugin system. In the future we to expand the plugin system to any WASI-compatible modules but have simply opted for Lua support in the short term while we figure out the API.

## Translations

The community seems to really enjoy Dioxus! And they want their friends to know about Dioxus, too! But, our guides have not been available in every language that developers want. In this release, we’re adding two new languages to our guide:

- Chinese provided by [`@mrxiaozhuox`](https://github.com/mrxiaozhuox)
- Portuguese provided by [`@whoeverdidthis`](https://github.com/whoeverdidthis)

## A new landing page and better docs

If you haven’t already noticed, our homepage is cleaner, more direct, and a bit more eye-catching. Check it out if you haven’t!

As part of our translation and Rust-ification work, [`@renis`](https://github.com/renis) has overhauled our guide to be more familiar for Rust developers. This skips some of the boilerplate (IE install Rust) and gets straight into the action of building Dioxus apps.

## Community Projects

- Styled components
- Opinionated starter pack
- Icon pack
- Caesar cyhper
- LED Strip controller
- GTK Renderer
- Formalize
- Story diagrammer
- Table crate
- Dioxus Helmet
- Skia renderer
- Use fetch
- Bevy Integration
