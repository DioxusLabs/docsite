## Lifting State

One approach to share state between components is to "lift" it up to the nearest common ancestor. This means putting the `use_signal` hook in a parent component, and passing the needed values down as props.

Suppose we want to build a meme editor. We want to have an input to edit the meme caption, but also a preview of the meme with the caption. Logically, the meme and the input are 2 separate components, but they need access to the same state (the current caption).

> Of course, in this simple example, we could write everything in one component – but it is better to split everything out in smaller components to make the code more reusable, maintainable, and performant (this is even more important for larger, complex apps).

We start with a `Meme` component, responsible for rendering a meme with a given caption:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_06/meme_editor.rs:meme_component}}
```

> Note that the `Meme` component is unaware where the caption is coming from – it could be stored in `use_signal`, or a constant. This ensures that it is very reusable – the same component can be used for a meme gallery without any changes!

We also create a caption editor, completely decoupled from the meme. The caption editor must not store the caption itself – otherwise, how will we provide it to the `Meme` component? Instead, it should accept the current caption as a prop, as well as an event handler to delegate input events to:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_06/meme_editor.rs:caption_editor}}
```

Finally, a third component will render the other two as children. It will be responsible for keeping the state and passing down the relevant props.

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_06/meme_editor.rs:meme_editor}}
```

![Meme Editor Screenshot: An old plastic skeleton sitting on a park bench. Caption: "me waiting for a language feature"](/assets/static/meme_editor_screenshot.png)

