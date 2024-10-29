# Tailwind

You can style your Dioxus application with whatever CSS framework you choose, or just write vanilla CSS.


One popular option for styling your Dioxus application is [Tailwind](https://tailwindcss.com/). Tailwind allows you to style your elements with CSS utility classes. This guide will show you how to setup tailwind CSS with your Dioxus application.

## Setup

1. Install the Dioxus CLI:

    ```bash
    cargo install --git https://github.com/DioxusLabs/cli
    ```

2. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
3. Install the tailwind css cli: https://tailwindcss.com/docs/installation
4. Initialize the tailwind css project:

    ```bash
    npx tailwindcss init
    ```

    This should create a `tailwind.config.js` file in the root of the project.

5. Edit the `tailwind.config.js` file to include rust files:

    ```js
    module.exports = {
        mode: "all",
        content: [
            // include all rust, html and css files in the src directory
            "./src/**/*.{rs,html,css}",
            // include all html files in the output (dist) directory
            "./dist/**/*.html",
        ],
        theme: {
            extend: {},
        },
        plugins: [],
    }
    ```

6. Create a `input.css` file with the following content:

    ```css
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
    ```

7. Create a `Dioxus.toml` file with the following content that links to the `tailwind.css` file:

    ```toml
    [application]

    # App (Project) Name
    name = "Tailwind CSS + Dioxus"

    # Dioxus App Default Platform
    # desktop, web, mobile, ssr
    default_platform = "web"

    # `build` & `serve` dist path
    out_dir = "dist"

    # resource (public) file folder
    asset_dir = "public"

    [web.app]

    # HTML title tag content
    title = "dioxus | ⛺"

    [web.watcher]

    # when watcher trigger, regenerate the `index.html`
    reload_html = true

    # which files or dirs will be watcher monitoring
    watch_path = ["src", "public"]

    # uncomment line below if using Router
    # index_on_404 = true

    # include `assets` in web platform
    [web.resource]

    # CSS style file
    style = ["/tailwind.css"]

    # Javascript code file
    script = []

    [web.resource.dev]

    # serve: [dev-server] only

    # CSS style file
    style = []

    # Javascript code file
    script = []
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

## Development

- Run the following command in the root of the project to start the tailwind css compiler:

    ```bash
    npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
    ```

### Web

- Run the following command in the root of the project to start the dioxus dev server:

    ```bash
    dx serve --hot-reload
    ```

- Open the browser to [http://localhost:8080](http://localhost:8080).

### Desktop

- Add a custom head pointing to the generated tailwind CSS file in your `main`. It looks like:

  ```rust
  dioxus_desktop::launch_cfg(
    App,
    dioxus_desktop::Config::new()
      .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string()))
  ```
- Launch the dioxus desktop app:

  ```bash
  cargo run
  ```
