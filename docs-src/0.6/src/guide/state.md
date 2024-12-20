# Interactivity

Now that our *HotDog* app is scaffolded and styled, we can finally add some interactive elements.

## Encapsulating State

Before we get too far, let's split our app into two parts: the `NavBar` and the `DogView`. This will help us organize our app and keep the `DogView` state separated from `NavBar` state.

```rust
fn App() -> Element {
    rsx! {
        NavBar {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    rsx! {
        div { id: "dogview",
            img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
        }
        div { id: "buttons",
            button { id: "skip", "skip" }
            button { id: "save", "save!" }
        }
    }
}
```

## Event Handlers

In the `DogView` component, we want to attach an action to the click of the buttons. For example: skipping or saving the current dog photo. We can use an [EventListener](../reference/event_handlers.md) to listen for the `click` events.

Event handlers are similar to regular attributes, but their name usually starts with `on` - and they accept closures as values. The closure will be called whenever its corresponding event is triggered. The listener receives information about the event in the [Event](https://docs.rs/dioxus/latest/dioxus/prelude/struct.Event.html) object.

We'll add some closures inline and then pass them to the `onclick` attribute for both the *skip* and *save* buttons:

```rust
#[component]
fn DogView() -> Element {
    let skip = move |evt| {};
    let save = move |evt| {};

    rsx! {
        // ...
        div { id: "buttons",
            button { onclick: skip, id: "skip",  "skip" }
            button { onclick: save, id: "save",  "save!" }
        }
    }
}
```

> You can read more about Event Handlers in the [Event Handler reference](../reference/event_handlers.md)

## State with use_hook

So far, our components have no internal state. For our `DogView`, we want to change the currently displayed dog photo whenever the user clicks *skip* or *save*.

To store state in components, Dioxus provides the `use_hook` function. This makes it possible for bare Rust functions to store and load state without the use of an extra struct.

When called in a component, the `use_hook` function will return a `.clone()` of the originally stored value:

```rust
fn DogView() -> Element {
    let img_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");

    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        // ..
    }
}
```

Dioxus hooks are very similar to React's hooks and need to follow some [simple rules](#the-rules-of-hooks) to function properly.

## Signals and `use_signal`

While `use_hook` makes it possible to store any value that implements `Clone`, you'll frequently want a more capable form of state management. Built-in to Dioxus are *signals*.

`Signal` is a wrapper type around an ordinary Rust value that tracks reads and writes, bringing your app to life. You can wrap any Rust value in a signal. Signals can be created manually with `Signal::new()` but we strongly recommend using the `use_signal` hook instead.

> 📣 Manually creating Signals requires remembering to call `.dispose()` on the signal whereas `use_signal` cleans the Signal up for you automatically.

Whenever a signal's value changes, its containing "reactive scope" will be "marked dirty" and re-run. By default, Dioxus components are reactive scopes, and thus, will re-render whenever a signal value changes.

![Basic Interactivity](/assets/06_docs/hotdog-interactivity.mp4)

Signals are core to Dioxus and take time to master. We recommend reading the [state management](../essentials/state/index.md) guide in depth before diving into your first large app.

## Global State with Context

While hooks are good for state *local* to components, occasionally you'll want to manage state for your *entire* app.

Dioxus provides two mechanisms: `Context` and `GlobalSignal`.

The `Context` API makes it possible for parent components to share state with child components without explicitly declaring an additional property field. This is used by larger apps and libraries to share state across the app without modifying component signatures.

To "provide" context, simply call `use_context_provider()` with a struct that implements `Clone`. To read the context in a child, call `use_context()`.

```rust
// Create a new wrapper type
#[derive(Clone)]
struct TitleState(String);

fn App() -> Element {
    // Provide that type as a Context
    use_context_provider(|| TitleState("HotDog".to_string()))
    rsx! {
        Title {}
    }
}

fn Title() -> Element {
    // Consume that type as a Context
    let title = use_context::<TitleState>();
    rsx! {
        h1 { "{title.0}" }
    }
}
```

You can combine use_signal and `Context` to provide reactive state to your app:

```rust
#[derive(Clone, Copy)]
struct MusicPlayer {
    song: Signal<String>
}

fn use_music_player_provider() {
    let song = use_signal(|| "Drift Away".to_string());
    use_context_provider(|| MusicPlayer { song });
}
```

With `use_context` and `consume_context`, you can easily reach up to modify that state:

```rust
fn Player() -> Element {
    rsx! {
        button {
            onclick: move |_| consume_context::<MusicPlayer>().song.set("Vienna"),
            "Shuffle"
        }
    }
}
```

Any components that read the song signal will automatically re-render when the value changes.

## Global Signals

Occasionally you'll want a simple global value. This is where `GlobalSignal` helps. GlobalSignals are a combination of the Context system and Signals that require no additional structs or setup.

Simply declare a GlobalSignal somewhere in your app:

```rust
static SONG: GlobalSignal<String> = Signal::global(|| "Drift Away".to_string());
```

And then read and write to it from anywhere:

```rust
fn Player() -> Element {
    rsx! {
        h3 { "Now playing {SONG}" }
        button {
            onclick: move |_| *SONG.write() = "Vienna".to_string(),
            "Shuffle"
        }
    }
}
```

> 📣 GlobalSignals are only global to one app - not the entire program. On the server, every app gets its own GlobalSignal.

We won't need either GlobalSignal or Context for *HotDog*, but it's important to know that these are available to you.
