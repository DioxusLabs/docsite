#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use std::collections::HashMap;

pub fn App() -> Element {
    // ANCHOR: use_coroutine
    // import futures::StreamExt to use the next() method
    use futures::StreamExt;
    let mut response_state = use_signal(|| None);
    let tx = use_coroutine(move |mut rx| async move {
        // Define your state before the loop
        let mut state = reqwest::Client::new();
        let mut cache: HashMap<String, String> = HashMap::new();
        loop {
            // Loop and wait for the next message
            if let Some(request) = rx.next().await {
                // Resolve the message
                let response = if let Some(response) = cache.get(&request) {
                    response.clone()
                } else {
                    let response = state
                        .get(&request)
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    cache.insert(request, response.clone());
                    response
                };
                response_state.set(Some(response));
            } else {
                break;
            }
        }
    });
    // Send a message to the coroutine
    tx.send("https://example.com".to_string());
    // Get the current state of the coroutine
    let response = response_state.read();
    // ANCHOR_END: use_coroutine

    todo!()
}
