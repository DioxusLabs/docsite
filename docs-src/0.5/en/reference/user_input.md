# User Input

Interfaces often need to provide a way to input data: e.g. text, numbers, checkboxes, etc. In Dioxus, there are two ways you can work with user input.

## Controlled Inputs

With controlled inputs, you are directly in charge of the state of the input. This gives you a lot of flexibility, and makes it easy to keep things in sync. For example, this is how you would create a controlled text input:

```rust, no_run
{{#include src/doc_examples/input_controlled.rs:component}}
```

```inject-dioxus
DemoFrame {
    input_controlled::App {}
}
```

Notice the flexibility – you can:

- Also display the same contents in another element, and they will be in sync
- Transform the input every time it is modified (e.g. to make sure it is upper case)
- Validate the input every time it changes
- Have custom logic happening when the input changes (e.g. network request for autocompletion)
- Programmatically change the value (e.g. a "randomize" button that fills the input with nonsense)

## Uncontrolled Inputs

As an alternative to controlled inputs, you can simply let the platform keep track of the input values. If we don't tell a HTML input what content it should have, it will be editable anyway (this is built into the browser). This approach can be more performant, but less flexible. For example, it's harder to keep the input in sync with another element.

Since you don't necessarily have the current value of the uncontrolled input in state, you can access it either by listening to `oninput` events (similarly to controlled components), or, if the input is part of a form, you can access the form data in the form events (e.g. `oninput` or `onsubmit`):

```rust, no_run
{{#include src/doc_examples/input_uncontrolled.rs:component}}
```
```inject-dioxus
DemoFrame {
    input_uncontrolled::App {}
}
```
```
Submitted! UiEvent { data: FormData { value: "", values: {"age": "very old", "date": "1966", "name": "Fred"} } }
```

## Handling files
You can insert a file picker by using an input element of type `file`. This element supports the `multiple` attribute, to let you pick more files at the same time. You can select a folder by adding the `directory` attribute: Dioxus will map this attribute to browser specific attributes, because there is no standardized way to allow a directory to be selected.

`type` is a Rust keyword, so when specifying the type of the input field, you have to write it as `r#type:"file"`.

Extracting the selected files is a bit different from what you may typically use in Javascript.

The `FormData` event contains a `files` field with data about the uploaded files. This field contains a `FileEngine` struct which lets you fetch the filenames selected by the user. This example saves the filenames of the selected files to a `Vec`:

```rust, no_run
{{#include src/doc_examples/input_fileengine.rs:component}}
```

If you're planning to read the file content, you need to do it asynchronously, to keep the rest of the UI interactive. This example event handler loads the content of the selected files in an async closure:

```rust, no_run
{{#include src/doc_examples/input_fileengine_async.rs:onchange_event}}
```

Lastly, this example shows you how to select a folder, by setting the `directory` attribute to `true`.

```rust, no_run
{{#include src/doc_examples/input_fileengine_folder.rs:rsx}}
```