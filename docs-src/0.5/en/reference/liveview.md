# Liveview

Liveview allows apps to *run* on the server and *render* in the browser. It uses WebSockets to communicate between the server and the browser.

Examples:
- [Simple Example](https://github.com/DioxusLabs/dioxus/tree/main/packages/liveview/examples/axum.rs)

## Support

Dioxus liveview will be migrated to [dioxus-fullstack](./fullstack/index.md) in a future release. Once this migration occurs, you may need to update your code. We plan for this migration to be minimal.

Liveview is currently limited in capability when compared to the Web platform. Liveview apps run on the server in a native thread. This means that browser APIs are not available, so rendering WebGL, Canvas, etc is not as easy as the Web. However, native system APIs are accessible, so streaming, WebSockets, filesystem, etc are all viable APIs.

## Router Integration

Currently, the Dioxus router does not integrate with the browser history in the liveview renderer. If you are interested in contributing this feature to Dioxus this issue is tracked [here](https://github.com/DioxusLabs/dioxus/issues/1038).

## Managing Latency

Liveview makes it incredibly convenient to talk to your server from the client, but there are some downsides. Mainly in Dioxus Liveview every interaction goes through the server by default.


Because of this, with the liveview renderer you need to be very deliberate about managing latency. Events that would be fast enough on other renderers like [controlled inputs](../../reference/user_input.md), can be frustrating to use in the liveview renderer.


To get around this issue you can inject bits of javascript in your liveview application. If you use a raw attribute as a listener, you can inject some javascript that will be run when the event is triggered:

```rust
rsx! {
    div {
        input {
            "oninput": "console.log('input changed!')"
        }
    }
}
```
