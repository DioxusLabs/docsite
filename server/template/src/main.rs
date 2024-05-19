use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut counter = use_signal(|| 0);
    
    let counter_text = format!("Count: {}", counter());
    
    rsx! {
        div {
            style: "color: blue;",
            "This is super cool!"
        }
        
        Text { 
            text: counter_text,
        }
        
        button {
            onclick: move |_| *counter.write() += 1,
            "+1"
        }
        
        button {
            onclick: move |_| *counter.write() -= 1,
            "-1"
        }
    }
}

#[component]
fn Text(text: String) -> Element {
    rsx! {
        p { "{text}" }
    }
}