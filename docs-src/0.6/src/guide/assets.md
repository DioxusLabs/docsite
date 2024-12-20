# Styling and Assets

Unfortunately, our HotDog app isn't quite ready to show off - it's completely unstyled!

In this chapter we'll cover adding assets and styles to our app.

## Dioxus uses CSS for Styling

As mentioned earlier, Dioxus apps use HTML and CSS as the core markup and styling technology. Instead of re-inventing the wheel like Flutter and React-Native, we designed Dioxus to use HTML and CSS on every platform.

CSS is by-far the most popular styling system and is extremely capable. For example, here's a screenshot of [ebou](https://github.com/terhechte/Ebou), a very beautiful Mastodon client built with Dioxus.

![Ebou](/assets/06_docs/ebou-following.png)

HTML and CSS are very powerful - don't worry about being too limited!

## Adding the CSS File with asset!()

The bare-bones template already includes a base `main.css` in the `assets` folder.

```sh
├── Cargo.toml
├── assets
│   └── main.css
└── src
    └── main.rs
```

To include the CSS in our app, we can use the `asset!()` macro. This macro ensures the asset will be included in the final app bundle.

```rust
static CSS: Asset = asset!("/assets/main.css");
```

We also need to load the asset into our app using the `document::Stylesheet` component. This component is equivalent to the `<link>` HTML element but also ensures the CSS will be pre-loaded during server-side-rendering.

```rust
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        // rest of the app
    }
}
```

Unlike Rust's `include_str!()` macro, the `asset!()` macro does not actually include the *contents* of the asset in our final executable. Instead, it generates a unique path so that the asset can be loaded at runtime. This is ideal for web apps where assets are loaded in parallel through different HTTP requests.

> The `asset!()` macro generates a unique name that won't exactly match the input name. This helps prevents name collisions and improves caching on the web.

## Hot-Reloading

All assets in Dioxus participate in hot-reloading. Try editing your app's `main.css` and watch changes propagate in real time.

![CSS Hot-reloading](/assets/06_docs/dog-asset-hotreload.mp4)

## Including Images

In Dioxus, you can include images in two ways:

- Dynamically with a URL
- Statically with the `asset!()` macro.

When including assets with a URL, simply fill the `src` attribute of `img {}`. Note that when the app is offline, URL-based images won't download.

```rust
rsx! {
    // ...
    div {
        img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
    }
    // ...
}
```

For static images, you can use the same `asset!()` macro that we used to include the app's CSS.

```rust
static ICON: Asset = asset!("/assets/icon.png");

rsx! {
    img { src: ICON }
}
```

## Optimizations

By default, the `asset!()` macro will lightly optimize CSS, JavaScript, JSON, and images. The name of the asset will also be modified to include a content hash.

```rust
asset!("/assets/main.css").to_string() // would output main-j1238nask123.css
```

You can optimize assets even further, with an optional `Options` struct. For example, `dx` can automatically convert `.png` images to a more optimized `.avif` format:

```rust
// outputs image-j1238jd2.avif
asset!("/assets/image.png", ImageAssetOptions::new().with_avif())
```
For many apps, asset optimization is the most effective way of improving load times. As developers, we frequently overlook the size of images and accidentally make our sites load slower.

Check out the [assets guide](../guides/assets.md) for a more in-depth explanation of how the Dioxus asset system works.

## The Final CSS

We can use the asset hot-reload system of `dx` and our knowledge of CSS to create a beautiful app:

![Styled Dog App](/assets/06_docs/dog_app_styled.png)

The final CSS is here for reference:

```css
/* App-wide styling */
html, body {
    background-color: #0e0e0e;
    color: white;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    height: 100%;
    width: 100%;
    overflow: hidden;
    margin: 0;
}

#main {
    display: flex;
    flex-direction: column;
    height: 100%;
    justify-content: space-between;
}

#dogview {
    flex-grow: 1;
    width: 100%;
    min-height: 0px;
    padding: 10px
}

#dogview img {
    display: block;
    max-height: 90%;
    max-width: 90%;
    height: 1000px;
    object-fit: cover;
    border-radius: 5px;
    border: 1px solid rgb(233, 233, 233);
    box-shadow: 0px 5px 10px 10px rgb(216, 216, 216, 0.5);
    margin: auto;
}

#title {
    text-align: center;
    padding-top: 10px;
    border-bottom: 1px solid #a8a8a8;
    font-style: italic;
}

#title h1 {
    margin: 0.25em;
}

#buttons {
    display: flex;
    flex-direction: row;
    justify-content: center;
    gap: 20px;
    padding-top: 20px;
    padding-bottom: 20px;
}

#skip { background-color: gray }
#save { background-color: green; }

#skip, #save {
    padding: 5px 30px 5px 30px;
    border-radius: 3px;
    font-size: 2rem;
    font-weight: bold;
    color: rgb(230, 230, 230)
}
```



