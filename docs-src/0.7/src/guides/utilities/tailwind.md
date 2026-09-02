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
{{#include ../docs-router/src/doc_examples/tailwind.rs}}
```

### Bonus Steps

<details>
    <summary>VSCode</summary>

1. Install the Tailwind CSS VSCode extension
2. Go to the settings for the extension and find the experimental regex support section. Edit the setting.json file to look like this:

```json
"tailwindCSS.experimental.classRegex": ["class: \"(.*)\""],
"tailwindCSS.includeLanguages": {
    "rust": "html"
},
```

</details>

<details>
    <summary>Neovim</summary>

1. Create new file **`~/.local/nvim/lua/custom/dioxus-tailwind.lua`**:

```lua
local paths_to_check = { '/', '/../', '/../../' }
local is_dx = false
local cwd = vim.fn.getcwd()

-- detect dioxus project
for _, value in pairs(paths_to_check) do
  if vim.uv.fs_stat(cwd .. value .. 'Dioxus.toml') then 
    is_dx = true
    break
  end
end

-- this configuration causes regular html to not work anymore
-- so we only apply it if we're actually in a dioxus project
if is_dx then
  vim.notify("dioxus project detected; altering tailwind LSP config...")
  vim.lsp.config("tailwindcss", {
    filetypes = {'rust'},
    settings = {
      tailwindCSS = {
        includeLanguages = { rust = "html", },
        experimental = {
          classRegex = {
            -- if this option gets removed, bob help you
            'class: "(.*)"',
          }
        }
      }
    }
  })
  vim.lsp.enable("tailwindcss")
end
```

2. Add to **`~/.local/nvim/init.lua`**

```lua
require('custom.dioxus-tailwind')
```

</details>

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
dx serve --desktop
```
