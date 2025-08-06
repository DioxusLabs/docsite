# Handling User Input

It's time to make our app interactive! In Dioxus, user input is handled by attaching event listeners to elements. When an event listener is triggered, Dioxus runs the provided closure. In the closure, you can write to signals, log messages, make network requests, or take any action that makes the UI feel *alive*.

```rust
fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    }
}
```
```inject-dioxus
DemoFrame {
    readme::App {}
}
```

## Event Handlers

Event handlers are callbacks used to respond to user actions. Event handlers can capture dozens of different interactions: button clicks, page scrolls, mouse movements, text input, and more.

Adding an event handler to an element is similar to adding an attribute with the `key: value` syntax. The handler name usually starts with `on` - and accepts a closure as the value. For example, to handle clicks on a button, we can add an `onclick` handler:

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_click.rs:rsx}}
```
```inject-dioxus
DemoFrame {
    event_click::AppDemo {}
}
```

There are many different event handlers available:

- **Mouse Events**: `onclick`, `onmouseover`, `onmousedown`, `onmousemove`, etc.
- **Keyboard Events**: `onkeydown`, `onkeyup`, `onkeypress`
- **Form Events**: `onsubmit`, `oninput`, `onchange`, etc.
- **Focus Events**: `onfocus`, `onblur`
- **Drag-and-Drop Events**: `ondrag`, `ondrop`, `ondragover`, etc.
- **UI Events**: `onscroll`, `onload`, `onscroll`, `onresize`

The full list of event listeners is available in the [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Events).


## The Event Object

Event handlers receive an [`Event`](https://docs.rs/dioxus-core/latest/dioxus_core/struct.Event.html) object containing information about the event. Different types of events contain different types of data. For example, mouse-related events contain [`MouseData`](https://docs.rs/dioxus/latest/dioxus/events/struct.MouseData.html), which provides details like the mouse position and which mouse buttons were pressed.

The event object is the first argument in the event handler callback:

```rust
rsx! {
    button {
        onclick: move |event| {   // <-- our `Event` object
            //
        }
    }
}
```

In the example above, this event data was logged to the terminal:

```
Clicked! Event: UiEvent { bubble_state: Cell { value: true }, data: MouseData { coordinates: Coordinates { screen: (242.0, 256.0), client: (26.0, 17.0), element: (16.0, 7.0), page: (26.0, 17.0) }, modifiers: (empty), held_buttons: EnumSet(), trigger_button: Some(Primary) } }
Clicked! Event: UiEvent { bubble_state: Cell { value: true }, data: MouseData { coordinates: Coordinates { screen: (242.0, 256.0), client: (26.0, 17.0), element: (16.0, 7.0), page: (26.0, 17.0) }, modifiers: (empty), held_buttons: EnumSet(), trigger_button: Some(Primary) } }
```

> To learn what the different event types for HTML provide, read the [events module docs](https://docs.rs/dioxus-html/latest/dioxus_html/events/index.html).

## Handling Common Events

The most common action you'll take in an event handler is modifying the application's state. This might involve updating a filter, toggling a switch, or presenting feedback to text input.

For components like toggles, we might use a boolean signal and a checkbox:

```rust
let mut upload_enabled = use_signal(|| true);
rsx! {
    input {
        type: "checkbox",

        // set the upload_enabled signal
        oninput: move |evt| upload_enabled.set(evt.checked()),
    }
}
```

For components like filters, we might use the HTML `select` element:

```rust
let mut option = use_signal(|| None);
rsx! {
    select {
        // set the signal to the select's `value`
        oninput: move |evt| option.set(Some(evt.value())),

        option { label: "Sedan", value: "sedan" }
        option { label: "Suv", value: "suv" }
        option { label: "Truck", value: "truck" }
    }
}
```

For text, we might use a the `input` element:

```rust
let mut first_name = use_signal(|| "".to_string());
rsx! {
    input {
        r#type: "text",
        placeholder: "First Name…",

        // Update the first_name signal on text input
        oninput: move |e| first_name.set(e.value()),
    }
}
```

For forms, we might use a HashMap to hold the key-value pairs:

```rust
let mut values = use_signal(HashMap::new);

rsx! {
    form {
        // We can capture `onsubmit` and `oninput` events
        onsubmit: move |evt| {
            evt.prevent_default();
            submitted_values.set(ev.values());
        },

        label { r#for: "username", "Username" }
        input { r#type: "text", name: "username" }
    }
}
```

Dioxus bridges the Rust-JavaScript boundary by adding ergonomic acessor methods on the `Event` object to make reading values from the DOM easier. These include:

- A `.value()` method for input events to read the input contents
- A `.values()` method for form events to read all the form values
- A `.checked()` method on checkbox input events to read the `.checked` state
- A `.files()` event to read any uploaded files
- A `.key()` event to convert keydown events into a Rust `Key` enum
- and many more methods!

> We provide a large number of examples in the [Dioxus GitHub repository](https://github.com/DioxusLabs/dioxus/tree/main/examples). Be sure to also read the docs on [handling events in HTML](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Events).



## Controlled vs Uncontrolled Inputs

Dioxus provides two ways of handling the state of input elements:

- **Uncontrolled mode**: the default where the input element handles its own state
- **Controlled mode**: an alternative mode where you control the input element's state manually

Controlled mode is useful if you plan to transform or programmatically modify the user's input.

### Uncontrolled Inputs

*Uncontrolled mode* is the default mode for input elements. In this mode, the input element itself manages its own state like the input value, cursor position, and focusing. We simply attach an event listener to the input and react to changes in the value:

```rust, no_run
{{#include ../docs-router/src/doc_examples/input_uncontrolled.rs:component}}
```
```inject-dioxus
DemoFrame {
    input_uncontrolled::App {}
}
```
```
Submitted! UiEvent { data: FormData { value: "", values: {"age": "very old", "date": "1966", "name": "Fred"} } }
```

In this mode, we have no control over the actual value of the input. The user may enter any value and our code updates state in response.

### Controlled Inputs

*Controlled mode* as an alternative mode for input elements where you directly control the state of the input. If the user types invalid text in the input, you can reject it or overwrite it.

To put an input element in controlled mode, we drive its `value` attribute directly:

```rust, no_run
{{#include ../docs-router/src/doc_examples/input_controlled.rs:component}}
```

```inject-dioxus
DemoFrame {
    input_controlled::App {}
}
```

Controlled inputs enable more control over the input element behavior. You can:

- Transform the input as it's modified (e.g. to make sure it is upper case)
- Validate the input, rejecting invalid inputs
- Programmatically change the value (e.g. a "randomize" button that fills the input with nonsense)

## Event Propagation

When the user interacts with our app, their interactions might trigger multiple event listeners at once. In the simplest case, a `div` may contain a `button` - both with their own `onclick` listeners:

![Multiple Listeners](/assets/07/multiple-listeners.png)

In what order will the listeners fire? Event handling comes in two phases:

- **Event Capturing**: Listeners are triggered as the event "descends" to the target.
- **Event Bubbling**: Listeners are triggered as the event "bubbles" to the root.

By default, Dioxus only captures the "bubbling" phase of the event, so the inner `button` will receive the `onclick` event before the `div`.

![Bubbling Diagram](/assets/07/event-capturing.png)

As the event bubbles to the root element (in this case, the document root), you have an opportunity to prevent any further listeners from being triggered. To stop the event from propagating upwards, you can call the `stop_propagation()` method on the event:

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_nested.rs:rsx}}
```

This ensures *only* the inner `button` will run its `onclick` handler - the `div` handler will not be triggered. This behavior can be useful when building advanced UI like drag-and-drop interactions and custom menus.

> For more information about event propagation see [the MDN docs on event bubbling](https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Building_blocks/Events#event_bubbling)

## Prevent Default

Some events have a default behavior. For keyboard events, this might be entering the typed character. For mouse events, this might be selecting some text. For forms, this might be submitting the form and navigating the page.

You can call the `prevent_default()` method on the event to stop this default behavior.

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_prevent_default.rs:prevent_default}}
```

```inject-dioxus
DemoFrame {
    event_prevent_default::AppDemo {}
}
```

Event handlers will still be called, but the "default" behavior of the interaction will be cancelled. The `prevent_default()` method is frequently used in interactions like:

- Capturing file drops
- Preventing form navigations
- Overriding the `a` link element behavior
- Disallowing certain text input
- Enabling drag-and-drop behavior for arbitrary elements

> For more information about default behaviors, see the [MDN docs on preventDefault()](https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault)

## Downcasting to Native Events

In some cases, the Dioxus `Event` type does not bridge enough of the Event's data. In these cases, we can directly access the platform-specific event type. Dioxus does not do this automatically since not all platforms share a consistent event interface with some platforms providing richer detail than others.

To downcast the event, we can use `event.downcast::<T>()` where `T` is the type we are trying to downcast to. You'll mostly use this extension when building web applications to downcast into the underlying `web_sys` event:

```rust
rsx! {
    button {
        onclick: move |evt| {
            let web_evt = evt.downcast::<web_sys::Event>().unwrap();
            let target = web_evt.target().unwrap();
            log!("target: {:?}", target);
        },
        "Click me!"
    }
}
```

## Asynchronous Handlers

Event Handlers can either be synchronous or asynchronous. Dioxus automatically calls `spawn()` on the Futures produced by asynchronous event handlers:

```rust
rsx! {
    button {
        onclick: move |evt| async move {
            let res = reqwest::get("https://dog.ceo/api/breeds/image/random/")
                .await
                .unwrap()
                .json::<DogApi>()
                .await;
            log!("res: {:?}", res);
        },
        "Fetch a dog!"
    }
}
```

Dioxus won't cancel previous Tasks spawned by the onclick handler, so multiple rapid button clicks will start multiple concurrent fetches. Be careful to not mutate state in asynchronous handlers without synchronization first.


## Handling files
You can insert a file picker by using an input element of type `file`. This element supports the `multiple` attribute, to let you pick more files at the same time. You can select a folder by adding the `directory` attribute: Dioxus will map this attribute to browser specific attributes, because there is no standardized way to allow a directory to be selected.

`type` is a Rust keyword, so when specifying the type of the input field, you have to write it as `r#type:"file"`.

Extracting the selected files is a bit different from what you may typically use in Javascript.

The `FormData` event contains a `files` field with data about the uploaded files. This field contains a `FileEngine` struct which lets you fetch the filenames selected by the user. This example saves the filenames of the selected files to a `Vec`:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_06/input_fileengine.rs:component}}
```

If you're planning to read the file content, you need to do it asynchronously, to keep the rest of the UI interactive. This example event handler loads the content of the selected files in an async closure:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_06/input_fileengine_async.rs:onchange_event}}
```

Lastly, this example shows you how to select a folder, by setting the `directory` attribute to `true`.

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_06/input_fileengine_folder.rs:rsx}}
```

## Handler Props

Sometimes, you might want to make a component that accepts an event handler. A simple example would be a `FancyButton` component, which accepts an `onclick` handler:

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_handler_prop.rs:component_with_handler}}
```

To add the handler as a property to our component, we simply add a field of `EventHandler` type with a name that starts with `on`.

To actually call the event handler, you call it with the `.call()` method:

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_handler_prop.rs:usage}}
```

> Note: just like any other attribute, you can name the handlers anything you want! Any closure you pass in will automatically be turned into an `EventHandler`.

## Custom Data

Event Handlers are generic over the closure's first argument, so you can pass in any data, e.g:

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_handler_prop.rs:custom_data}}
```

The `EventHandler<T>` type is a subset of the base `Callback<Args, Ret>` type that allows you to return a value from the closure.

## Returning a Value From an Event Handler

If you want to accept a closure that returns a value, you can use the `Callback` type. The callback type accepts two generic arguments, `I`, the input type, and `O`, the output type. Just like `EventHandler`, `Callback` is automatically converted in props and can be easily copied anywhere in your component:

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_handler_prop.rs:callback}}
```

## Callbacks Carry the Runtime

Many Dioxus runtime functions are "free functions" - you can freely call them without needing an explicit handle to the Dioxus runtime. This works by implicitly setting a thread-local called "current runtime", roughly equivalent to this pseudocode:

```rust
thread_local! {
    static CURRENT_RUNTIME: Cell<Option<Runtime>> = Cell::new(None);
}

fn render_users_component(app: &Application) {
    CURRENT_RUNTIME.set(app.runtime());
    app.run_component();
    CURRENT_RUNTIME.set(None);
}
```

Whenever *your* code runs from within Dioxus, the runtime will always be set. However, plain closures do not automatically set the current runtime. Trying to call closures outside the app that reference state from within the app might cause a panic.

Fortunately, the `EventHandler` and `Callback` types carry a handle to the Dioxus runtime, ensuring runtime methods succeed. When the callback is called, the closure sets the `CURRENT_RUNTIME` variable. This means you can pass the `EventHandler` and `Callback` types to APIs like file-system watchers and system IO where the Dioxus runtime is not usually active.
