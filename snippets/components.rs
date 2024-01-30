//! Encapsulate state in components.
//!
//! Use structs or autodderive props with `#[component]`

#[derive(Props, PartialEq)]
struct PropBased {
    name: String,
    age: String,
}

fn Stateful(<PropBased>) -> Element {
    rsx!("Hello {cx.name}, you are {cx.age} years old!")
}
