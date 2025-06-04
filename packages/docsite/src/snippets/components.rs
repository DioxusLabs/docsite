//! Encapsulate state in components
use dioxus::prelude::*;

#[component]
fn Stateful(name: String, age: u64) -> Element {
    rsx! {"Hello {name}, you are {age} years old!"}
}

fn App() -> Element {
    rsx! { Stateful { name: "Alice", age: 30 } }
}
