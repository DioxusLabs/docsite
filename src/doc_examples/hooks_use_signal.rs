#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App() -> Element {
    let mut list = use_signal(Vec::new);

    rsx!(
        p { "Current list: {list:?}" }
        button {
            onclick: move |event| {
                let list_len = list.len();
                // You can get a &mut T from a Signal<T> by calling .write() on it
                list.write().push(list_len);
                // Or Signal<Vec<T>> contains some helper methods to like .push() and .pop() that you can call directly
                list.push(list_len);
            },
            "Add two elements!"
        }
    )
}
// ANCHOR_END: component
