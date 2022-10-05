//! Encapsulate state in components.
//!
//! Use structs or autodderive props with `#[inline_props]`

#[derive(Props, PartialEq)]
struct PropBased {
    name: String,
    age: String,
}

fn Stateful(cx: Scope<PropBased>) -> Element {
    cx.render(rsx!("Hello {cx.name}, you are {cx.age} years old!"))
}
