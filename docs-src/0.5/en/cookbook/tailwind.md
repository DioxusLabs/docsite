# Tailwind

You can style your Dioxus application with whatever CSS framework you choose, or just write vanilla CSS.


One popular option for styling your Dioxus application is [Tailwind](https://tailwindcss.com/). Tailwind allows you to style your elements with CSS utility classes. This guide will show you how to setup tailwind CSS with your Dioxus application.

## Setup

1. Install the Dioxus CLI:

```bash
cargo install dioxus-cli
```

2. Install npm: [https://docs.npmjs.com/downloading-and-installing-node-js-and-npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
3. Install the tailwind css cli: [https://tailwindcss.com/docs/installation](https://tailwindcss.com/docs/installation)
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

6. Create a `input.css` file in the root of your project with the following content:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

7. Add [Manganis](https://github.com/DioxusLabs/manganis) to your project to handle asset collection.

```sh
cargo add manganis
```

8. Create a link to the `tailwind.css` file using manganis somewhere in your rust code:

```rust
{{#include src/doc_examples/tailwind.rs}}
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
dx serve
```

- Open the browser to [http://localhost:8080](http://localhost:8080).

### Desktop

- Launch the dioxus desktop app:

```bash
dx serve --platform desktop
```
