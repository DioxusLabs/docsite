use dioxus::prelude::*;
use futures::StreamExt as _;
use model::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{js_sys, MessageEvent, WebSocket};

pub fn start_socket(
    socket_uri: String,
    built_uri: String,
    mut is_compiling: Signal<bool>,
    mut built_page_uri: Signal<Option<String>>,
    mut compiler_messages: Signal<Vec<String>>,
) -> Coroutine<SocketMessage> {
    use_coroutine(move |mut rx: UnboundedReceiver<SocketMessage>| async move {
        let ws = WebSocket::new(&socket_uri).unwrap();

        // Setup socket callback
        let cl = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                let text: String = text.into();

                // Parse the message and determine what it is.
                if let Ok(msg) = SocketMessage::try_from(text) {
                    match msg {
                        SocketMessage::CompileFinished(id) => {
                            is_compiling.set(false);
                            built_page_uri.set(Some(format!("{}{}", built_uri, id)));
                        }
                        SocketMessage::CompileMessage(msg) => {
                            compiler_messages.push(msg);
                        }
                        // TODO: Handle banned words on both client and server
                        // This would avoid unnescessary requests.
                        SocketMessage::BannedWord(word) => {
                            compiler_messages.push(format!("Error:"));
                            compiler_messages.push(format!("A banned word was used: {word}"));
                            compiler_messages.push("Please remove any instances of that word and run again.".to_string());
                            compiler_messages.push("Using that word inside of another word is not allowed either. e.g. `move` in `remove`".to_string());
                            is_compiling.set(false);
                            built_page_uri.set(None);
                        }
                        SocketMessage::CompileFinishedWithError => {
                            is_compiling.set(false);
                        }
                        SocketMessage::SystemError(s) => {
                            is_compiling.set(false);
                            built_page_uri.set(None);
                            compiler_messages.push(format!("Server Error: {s}"));
                        },
                        _ => {}
                    }
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        ws.set_onmessage(Some(cl.as_ref().unchecked_ref()));
        cl.forget();

        // Handle sending messages to socket.
        while let Some(msg) = rx.next().await {
            let parsed = msg.to_string();
            // TODO: Error handling
            ws.send_with_str(parsed.as_str()).unwrap();
        }
    })
}
