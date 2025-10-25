# Streams and SSE

Dioxus Fullstack provides an easy way to send and receive streaming data from a server. This can be useful to
implement functionality like streaming LLM responses, file downloads, and server-sent-events (SSE).

Unlike websockets which allow two-way communication, streams are unidirectional. In browsers, it's usually impossible to have a streaming input *and* a streaming output, so you should stick to using streams for things like text/byte responses or file sending.

## Streaming Text

Dioxus Fullstack provides the `TextStream` type to easily send chunks of text between the client and the server. We can use this type as the input or output of a server function:

```rust
// The output is a `TextStream`
#[get("/api/test_stream?start")]
async fn text_stream(start: Option<i32>) -> Result<TextStream> {
    let mut count = start.unwrap_or(0);

    // We can create a new text stream with `spawn`
    Ok(TextStream::spawn(move |tx| async move {

        // Send a message with `unbounded_send`
        while tx.unbounded_send(format!("Hello, world! {}", count)).is_ok() {
            count += 1;

            // and then wait a bit
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }))
}
```

You can create a new stream with `TextStream::spawn` which gives you an `UnboundedSender` object, or from `TextStream::new()` which takes an existing type that implements the `Stream` trait

```rust
// the `rx` here implements `Stream` which can be used in `new()`
let (tx, rx) = futures::channel::mpsc::unbounded();

tokio::spawn(async move {
    let mut count = start.unwrap_or(0);
    loop {
        let message = format!("Hello, world! {}", count);
        if tx.unbounded_send(message).is_err() {
            break;
        }

        count += 1;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
});

Ok(Streaming::new(rx))
```

## Streaming Bytes

To send bytes


## The Generic `Streaming<T, E>` type


## File Streams
