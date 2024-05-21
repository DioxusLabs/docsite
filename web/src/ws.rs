use dioxus::prelude::*;
use futures::StreamExt as _;
use model::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{js_sys, MessageEvent, WebSocket};

use crate::{BUILT_URI, SOCKET_URI};

pub fn start_socket(mut is_compiling: Signal<bool>, mut built_src: Signal<Option<String>>) -> Coroutine<SocketMessage> {
    use_coroutine(move |mut rx: UnboundedReceiver<SocketMessage>| async move {
        let ws = WebSocket::new(SOCKET_URI).unwrap();

        // Setup socket callback
        let cl = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                let text: String = text.into();

                // Parse the message and determine what it is.
                if let Ok(msg) = SocketMessage::try_from(text) {
                    match msg {
                        SocketMessage::CompileFinished(id) => {
                            is_compiling.set(false);
                            built_src.set(Some(format!("{}{}", BUILT_URI, id)));
                        }
                        SocketMessage::SystemError(_) => todo!(),
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
