# Fetching Data

Our *HotDog* app has some basic interactivity but does not yet fetch new dog images. In this chapter, we'll interact with async and fetching data from an API.

## Adding Dependencies

Dioxus does not provide any built-in utilities for fetching data. Crates like [dioxus-query](https://github.com/marc2332/dioxus-query) exist, but for this tutorial we'll showcase how to implement data-fetching from scratch.

First, we need to add two new dependencies to our app: [serde](https://crates.io/crates/serde) and [reqwest](https://crates.io/crates/reqwest).

Serde will let us derive a JSON Deserializer and Reqwest provides us an HTTP client to fetch with.

In a new terminal window, add these crates to your app with `cargo add`.

```bash
cargo add reqwest --features json
cargo add serde --features derive
```

## Defining a Response Type

We'll be using the amazing [dog.ceo/api](https://dog.ceo/dog-api/) to fetch images of dogs for *HotDog*. Fortunately, the API response type is quite simple.

Let's create a new Rust struct that matches the format of the API and derive `Deserialize` for it.

```rust
#[derive(serde::Deserialize)]
struct DogApi {
	message: String,
}
```

## Using `reqwest` and `async`

Dioxus has stellar support for asynchronous Rust. We can simply convert our `onclick` handler to be `async` and then set the `img_src` after the future has resolved.

![Dog Fetching](/assets/06_docs/fetch-dog.mp4)

The changes to our code are quite simple - just add the `reqwest::get` call and then set `img_src` to the result.

```rust
fn DogView() -> Element {
    let mut img_src = use_signal(|| "".to_string());

    let fetch_new = move |_| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
			.unwrap();

        img_src.set(response.message);
    };

    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            button { onclick: fetch_new, id: "save", "save!" }
        }
    }
}
```

Under the hood, when Dioxus recognizes that an `async` closure is passed to an EventHandler, it calls `dioxus::spawn` on the resulting `Future`. You can use this API directly to do async work *without* async closures.

```rust
rsx! {
    button {
        onclick: move |_| {
            dioxus::spawn(async move {
                // do some async work...
            });
        }
    }
}
```

## Managing Data Fetching with use_resource

Eventually, using bare `async` calls might lead to race conditions and weird state bugs. For example, if the user clicks the *fetch* button too quickly, then two requests will be made in parallel. If the request is updating data somewhere else, then it's likely that the wrong request finishes early and causes a race condition.

Dioxus provides some helpful utilities to manage these scenarios with a hook called `use_resource`. In Dioxus, *Resources* are pieces of state whose value is dependent on the completion of some asynchronous work. The `use_resource` hook provides a `Resource` object with helpful methods to start, stop, pause, and modify the asynchronous state.

Resources are very powerful: they integrate with Suspense, Streaming HTML, reactivity, and more.

Let's change our component to use a resource instead:

```rust
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            img { src: img_src.value().cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "save", "save!" }
        }
    }
}
```

The details of the `Resource` API are not terribly important right now, but you'll be using Resources frequently in larger apps, so it's a good idea to [read the docs](../reference/use_resource.md).
