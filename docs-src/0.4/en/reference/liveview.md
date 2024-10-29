# Liveview

This guide will cover concepts specific to the Dioxus liveview renderer.

## Router Integration

Currently, the Dioxus router does not integrate with the browser history in the liveview renderer. If you are interested in contributing this feature to Dioxus this issue is tracked [here](https://github.com/DioxusLabs/dioxus/issues/1038).

## Managing Latency

Liveview makes it incredibly convenient to talk to your server from the client, but there are some downsides. Mainly in Dioxus Liveview every interaction goes through the server by default.


Because of this, with the liveview renderer you need to be very deliberate about managing latency. Events that would be fast enough on other renderers like [controlled inputs](../../reference/user_input.md), can be frustrating to use in the liveview renderer.


To get around this issue you can inject bits of javascript in your liveview application. If you use a raw attribute as a listener, you can inject some javascript that will be run when the event is triggered:

```rust
render! {
    div {
        input {
            "oninput": "console.log('input changed!')"
        }
    }
}
```
