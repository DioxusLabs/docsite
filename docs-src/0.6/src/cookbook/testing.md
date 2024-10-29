# Testing

When building application or libraries with Dioxus, you may want to include some tests to check the behavior of parts of your application. This guide will teach you how to test different parts of your Dioxus application.

## Component Testing

You can use a combination of [pretty-assertions](https://docs.rs/pretty_assertions/latest/pretty_assertions/) and [dioxus-ssr]() to check that two snippets of rsx are equal:

```rust
{{#include src/doc_examples/component_test.rs}}
```

## Hook Testing

When creating libraries around Dioxus, it can be helpful to make tests for your [custom hooks](./state/custom_hooks/index.md).


Dioxus does not currently have a full hook testing library, but you can build a bespoke testing framework by manually driving the virtual dom.

```rust
{{#include src/doc_examples/hook_test.rs}}
```

## End to End Testing

You can use [Playwright](https://playwright.dev/) to create end to end tests for your dioxus application.

In your `playwright.config.js`, you will need to run cargo run or dx serve instead of the default build command. Here is a snippet from the end to end web example:

```js
//...
webServer: [
    {
        cwd: path.join(process.cwd(), 'playwright-tests', 'web'),
        command: 'dx serve',
        port: 8080,
        timeout: 10 * 60 * 1000,
        reuseExistingServer: !process.env.CI,
        stdout: "pipe",
    },
],
```

- [Web example](https://github.com/DioxusLabs/dioxus/tree/main/playwright-tests/web)
- [Liveview example](https://github.com/DioxusLabs/dioxus/tree/main/playwright-tests/liveview)
- [Fullstack example](https://github.com/DioxusLabs/dioxus/tree/main/playwright-tests/fullstack)
