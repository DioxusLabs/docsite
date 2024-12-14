# Interactivity

Now that our *HotDog* app is scaffolded and styled, we can finally add some interactive elements.

## Encapsulating State

Before we get too far, let's split our app into two parts: the `Title` and the `DogView`. This will help us organize our app and keep the `DogView` state separated from `Title` state.

```rust
fn App() -> Element {
    rsx! {
        Title {}
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

In the `DogView` component, we'll want to attach some action to the click of the buttons. This would let the user skip or save the current dog photo. We can use an [event listener](../reference/event_handlers.md) to listen for the hover and focus events.

Event handlers are similar to regular attributes, but their name usually starts with `on` - and they accept closures as values. The closure will be called whenever its corresponding event triggered. The listener receives information about the event in the [Event](https://docs.rs/dioxus/latest/dioxus/prelude/struct.Event.html) object.

We'll add some closures inline and then pass them to the `onclick` attribute for both the *skip* and *save* buttons:

```rust
#[component]
fn DogView() -> Element {
    let skip = move |_| {};
    let save = move |_| {};

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

## State

So far, our components have no internal state. However, for our `DogView`, we want to change the currently displayed dog photo whenever the user clicks *skip* or *save*.

Dioxus makes it possible for bare Rust functions to store and load state - without the use of an extra struct - using the `use_hook` function.

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

While `use_hook` makes it possible to store any value that implements `Clone`, you'll frequently want a more advanced form of state management. Built-in to Dioxus are *signals*.

Signals are wrapper type around ordinary Rust values that track reads and writes to bring your app to life. You can wrap any Rust value in a signal. Signals can be created manually with `Signal::new()` but we strongly recommend using the `use_signal` hook instead.

> Manually creating Signals requires remembering to call `.dispose()` on the signal whereas `use_signal` cleans the Signal up for you automatically.

Whenever a signal's value changes, its containing "reactive scope" will be "marked dirty" and re-run. By default, Dioxus components are reactive scopes, and thus, will re-render whenever a signal value changes.

![Interactivity Basic](/assets/06_docs/hotdog-interactivity.mp4)

Signals are core to Dioxus and might take some time to master. We recommend reading the [state management](../essentials/state/index.md) guide in depth before diving into your first large app.

## Global State with Context

While hooks are good for state local to components, occasionally you'll want to manage state for your entire app.

Dioxus provides two mechanisms: `Context` and `GlobalSignal`.

The `Context` API makes it possible for parent components to share state with child components without explicitly declaring an additional property field. This is generally used by larger apps and libraries to share state across the app that doesn't modify component signatures.

To "provide" context, simply call `use_context_provider()` with a struct that implements `Clone`. To read that context in a child, call `use_context()`

```rust
#[derive(Clone)]
struct TitleState(String);

fn App() -> Element {
    use_context_provider(|| TitleState("HotDog".to_string()))
    rsx! {
        Title {}
    }
}

fn Title() -> Element {
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

And then with `use_context` and `consume_context`, you can easily reach up to modify that state:

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

Any components that read the *song* signal will automatically re-render.

## Global Signals

While Context is good for encapsulating complex interactions, you'll occasionally just want a single global value for your entire app. This is where `GlobalSignal` helps. GlobalSignals are a combination of the Context system and Signals that require no additional structs or setup.

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
