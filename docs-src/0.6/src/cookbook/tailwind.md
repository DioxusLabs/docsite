# Tailwind

You can style your Dioxus application with whatever CSS framework you choose, or just write vanilla CSS.

One popular option for styling your Dioxus application is [Tailwind](https://tailwindcss.com/). Tailwind allows you to style your elements with CSS utility classes. This guide will show you how to setup Tailwind CSS with your Dioxus application.

## Setup

1. Install the Dioxus CLI:

```bash
cargo install dioxus-cli
```

2. Install NPM: [https://docs.npmjs.com/downloading-and-installing-node-js-and-npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
3. Install the Tailwind CSS CLI: [https://tailwindcss.com/docs/installation/tailwind-cli](https://tailwindcss.com/docs/installation/tailwind-cli)

4. Create a `input.css` file in the root of your project with the following content:

```css
@import "tailwindcss";
@source "./src/**/*.{rs,html,css}";
```

5. Create a link to the `tailwind.css` file using manganis somewhere in your rust code:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/tailwind.rs}}
```

### Bonus Steps

1. Install the Tailwind CSS VSCode extension
2. Go to the settings for the extension and find the experimental regex support section. Edit the setting.json file to look like this:

```json
"tailwindCSS.experimental.classRegex": ["class: \"(.*)\""],
"tailwindCSS.includeLanguages": {
    "rust": "html"
},
```

## Development

- Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

### Web

- Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve
```

- Open the browser to [http://localhost:8080](http://localhost:8080).

### Desktop

- Launch the Dioxus desktop app:

```bash
dx serve --platform desktop
```
