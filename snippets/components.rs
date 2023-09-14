//! Encapsulate state in components.
//!
//! Use structs or let `#[component]` autoderive props for you from function arguments.

#[derive(Props, PartialEq)]
struct PropBased {
    name: String,
    age: String,
}

#[component]
fn Stateful(cx: Scope<PropBased>) -> Element {
    render!("Hello {cx.name}, you are {cx.age} years old!")
}
