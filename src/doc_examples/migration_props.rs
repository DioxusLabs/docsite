use dioxus::prelude::*;

// ANCHOR: owned_props
// In dioxus 0.5, props are always owned. You pass in owned props and you get owned props in the body of the component
#[component]
fn Comp(name: String) -> Element {
    // Name is owned here already (name is the type String inside the function)
    let owned_name: String = name;

    rsx! {"Hello {owned_name}"}
}
// ANCHOR_END: owned_props

// ANCHOR: copy_props
// In dioxus 0.5, props are always owned. You pass in owned props and you get owned props in the body of the component
#[component]
fn CopyPropsComp(name: ReadOnlySignal<String>) -> Element {
    rsx! {
        button {
            // You can easily copy the value of a signal into a closure
            onclick: move |_| {
                println!("Hello {name}");
                async move {
                    println!("Hello {name}");
                }
            },
            "Click me"
        }
    }
}

fn CopyPropsCompParent() -> Element {
    rsx! { CopyPropsComp { name: "World" } }
}
// ANCHOR_END: copy_props

// ANCHOR: borrowed_props
fn Parent() -> Element {
    let state = use_signal(|| (1, "World".to_string()));

    rsx! { BorrowedComp { name: state.map(|s| &s.1) } }
}

#[component]
fn BorrowedComp(name: MappedSignal<String>) -> Element {
    rsx! {"Hello {name}"}
}
// ANCHOR_END: borrowed_props

// ANCHOR: manual_props
#[derive(Props, Clone, PartialEq)]
struct ManualProps {
    name: String,
}

// Functions accept the props directly instead of the component
fn ManualPropsComponent(props: ManualProps) -> Element {
    rsx! {"Hello {props.name}"}
}
// ANCHOR_END: manual_props
