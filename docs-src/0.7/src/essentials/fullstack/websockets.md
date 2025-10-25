# Websockets

Dioxus Fullstack provides built-in types for creating and managing websockets that work alongside server functions. Dioxus websockets are built on top of the underyling Axum websocket API, but with a few improvements tailored for building fullstack apps:

- Hybrid shared server/client types
- Reactive wrapper for use in UI code
- Typed inputs, outputs, and customizable encoding

Websockets are an extremely powerful communication protocol that allows bidirectional message passing to and from the server. Websockets are more efficient than HTTP requests for large amounts of messages, provide better real-time support, and allow for *ordered* data transmission. However, they are *stateful*, meaning that a websocket connection ties a client and server together for a given session. If you plan to use websockets in a "serverless" environment with time limits of request handling, then you need some way to "store" the websocket session across multiple requests.

## Websocket and WebsocketOptions

To create a new server function that returns a websocket, simply use `WebsocketOptions` as your body type and `Websocket` as your response type.

```rust
#[get("/api/uppercase_ws")]
async fn uppercase_ws(options: WebSocketOptions) -> Result<Websocket> {
    Ok(options.on_upgrade(move |mut socket| async move {
        // send back a greeting message
        _ = socket
            .send("Hello!".to_string())
            .await;

        // Loop and echo back uppercase messages
        while let Ok(msg) = socket.recv().await {
            _ = socket.send(msg.to_ascii_uppercase()).await;
        }
    }))
}
```

The `Websocket` type is generic over three parameters - the input type, output type, and encoding:

```rust
pub struct Websocket<In = String, Out = String, E = JsonEncoding> {
    // ...
}
```

The input and output types are the types used when you call `.send()` and `.recv()` on the `socket` object provided after `on_upgrade`. By strongly typing the websocket, we guarantee that your client and server always use the right message format across the client and server.

The `on_upgrade` method is a wrapper over the underlying Axum `on_upgrade` API that returns an axum response, indicating to the client that the websocket upgrade process is succesful. If the client agrees, then the server will run the `on_upgrade` callback, spawning the future. Note that this future is spawned on a tokio [LocalSet](https://docs.rs/tokio/latest/tokio/task/struct.LocalSet.html). This means the future does not need to be `Send`, which can drastically simplify your logic.

We can use our own message types for the input and output messages. Calls to `send` and `recv` will attempt to deserialize data websocket messages into the right type, returning an error if the deserialization fails.

```rust
// Events flowing *from* the client to the server
#[derive(Serialize, Deserialize, Debug)]
enum ClientEvent {
    TextInput(String),
}

// Events flowing *to* the client from the server
#[derive(Serialize, Deserialize, Debug)]
enum ServerEvent {
    Uppercase(String),
}

#[get("/api/uppercase_ws")]
async fn uppercase_ws(options: WebSocketOptions) -> Result<Websocket<ClientEvent, ServerEvent>> {
    // ...
}
```

We can also customize the encoding of the websocket with the third generic on `Websocket`. By default, messages are encoded using JSON with `JsonEncoding`, but you can opt for an alternative format like the binary Cbor format:

```rust
#[get("/api/uppercase_ws")]
async fn uppercase_ws(options: WebSocketOptions) -> Result<Websocket<ClientEvent, ServerEvent, CborEncoding>> {
    // ...
}
```

Generally, if you're working with Rust-only clients, then Cbor or MsgPack are better options, but 3rd-party clients might be better suited with the standard JSON encoding.

If you need to send extra details to the server before establishing the websocket connection, you can use path and query parameters as well as header extraction like usual.

```rust
#[get("/api/uppercase_ws?name&age")]
async fn uppercase_ws(
    name: String,
    age: i32,
    options: WebSocketOptions,
) -> Result<Websocket<ClientEvent, ServerEvent, CborEncoding>> {
    // ...
}
```

## Connecting to a Websocket

On the client, to connect to a websocket, we'll simply call the server function and await the result. You might do this inside a `use_future` hook to connect to the websocket endpoint when the component is mounted:

```rust
// Calling `.recv()` automatically waits for the connection to be established and deserializes
// messages as they arrive.
use_future(move || async move {
    // Connect to the websocket
    let socket = uppercase_ws(WebSocketOptions::new()).await;

    // Wait for the next message with `.recv()`
    while let Ok(msg) = socket.recv().await {
        messages.push(msg);
    }
});
```

The `Websocket` object has a number of utility methods you can use to assess the state of the connection, send messages, and receive messages. We expose a number of lower-level APIs like `send_raw` that let you send raw websocket frames in case the typed API is too strict.



## The use_websocket hook

You might notice in the `use_future` example above, the websocket is only accessible to its containining scope. In a practical app, you'll want to send messages into the websocket and react to any changes in connection status.

The `use_websocket` hook wraps the `websocket` object with signal-based reactivity. We can use `.status()` to read the websocket connection state, and `.send()` to send messages to the server.

To connect to the websocket, we might use `use_websocket`:

```rust
// The `use_websocket` wraps the `WebSocket` connection and provides a reactive handle to easily
// send and receive messages and track the connection state.
//
// We can customize the websocket connection with the `WebSocketOptions` struct, allowing us to
// set things like custom headers, protocols, reconnection strategies, etc.
let mut socket = use_websocket(|| uppercase_ws("John Doe".into(), 30, WebSocketOptions::new()));
```

To listen for messages, we can use `.recv()` in a future:

```rust
// Calling `.recv()` automatically waits for the connection to be established and deserializes
// messages as they arrive.
use_future(move || async move {
    while let Ok(msg) = socket.recv().await {
        messages.push(msg);
    }
});
```

And then to send messages, we can use `.send()` on the handle:

```rust
rsx! {
    input {
        placeholder: "Type a message",
        oninput: move |e| async move {
            _ = socket.send(ClientEvent::TextInput(e.value())).await;
        },
    }
}
```

If the connection fails, you can restart it by manually calling `.set()` on the handle with a new websocket object.
