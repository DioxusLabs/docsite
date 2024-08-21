# Tailwind

You can style your Dioxus application with whatever CSS framework you choose, or just write vanilla CSS.


One popular option for styling your Dioxus application is [Tailwind](https://tailwindcss.com/). Tailwind allows you to style your elements with CSS utility classes. This guide will show you how to setup tailwind CSS with your Dioxus application.

## Setup

1. Install the Dioxus CLI:

```bash
cargo install dioxus-cli
```
For 0.6.0-alpha.2
```bush
cargo install --git https://github.com/dioxuslabs/dioxus dioxus-cli --locked
dx --version
dioxus 0.6.0-alpha.2 (3c699aa)

[dependencies]
dioxus = { git = "https://github.com/DioxusLabs/dioxus", features = ["web", "router"] }
dioxus-logger = "0.5.1"
```

2. Install npm: [https://docs.npmjs.com/downloading-and-installing-node-js-and-npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
3. Install the tailwind css cli: [https://tailwindcss.com/docs/installation](https://tailwindcss.com/docs/installation)
4. Initialize the tailwind css project via cli for web platform:

```bash
// You can change platform, name and router
dx new -> web -> Project Name: project-name -> Tailwind -> true
```

This should create all Tailwind CSS files in the root of the project.

5. Start the Tailwind CSS compiler and the Dioxus dev server in different terminals:
```bush
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
dx serve --hot-reload true
```

## Hot reload with dioxus 0.6.0-alpha.2 (3c699aa)

6. Inside rsx, you need to add the following support to main.rs
`script { src: "https://cdn.tailwindcss.com" }` and `href: asset!("./assets/tailwind.css")`
[Use the Play CDN to try Tailwind right in the browser without any build step.](https://tailwindcss.com/docs/installation/play-cdn)

```rust
#[component]
fn App() -> Element {
    rsx! {
        // for Play CDN to try Tailwind
        script { src: "https://cdn.tailwindcss.com" }
        // for manganis
        head::Link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }
        
        // code
    }
}
```

If you need local stylesheet for rendering with custom styles inside input.css.
Insert your custom styles inside input.css:
```css
@tailwind base;
@tailwind components;
@tailwind utilities;


@layer components {
  p {
    @apply p-10 bg-yellow-600;
  }
  .red {
    @apply bg-red-600;
  }
  .yellow {
    @apply bg-yellow-600;
  }
  .blue {
    @apply bg-blue-600;
  }
}
```
Second you need to insert custom classes to the page:

```rust
rsx! {
        // for Play CDN to try Tailwind        
        script { src: "https://cdn.tailwindcss.com" }
        // for manganis
        head::Link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }
        img { src: "header.svg", id: "header" }
        div { id: "links",
            div { class: "p-5 bg-red-200", "Hello" }
            p { "I" }
            div { class: "red p-2", "love" }
            div { class: "yellow p-6", "Dioxus" }
            div { class: "blue text-white text-center py-2", "team" }
        }
    }
```

And than rebuild the app.
button `r` on terminal or `dx serve --hot-reload true`

When you are done editing, you should comment out the use of Tailwind CDN
```rust
// for Play CDN to try Tailwind
// script { src: "https://cdn.tailwindcss.com" }
```



### Bonus Steps

1. Install the tailwind css vs code extension
2. Go to the settings for the extension and find the experimental regex support section. Edit the setting.json file to look like this:

```json
"tailwindCSS.experimental.classRegex": ["class: \"(.*)\""],
"tailwindCSS.includeLanguages": {
    "rust": "html"
},
```


