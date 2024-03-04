//! Encapsulate state in components.
//!
//! Use structs or autodderive props with `#[component]`

#[derive(Props, PartialEq)]
struct PropBased {
    name: String,
    age: String,
}

fn Stateful(props: PropBased) -> Element {
    rsx!("Hello {props.name}, you are {props.age} years old!")
}
