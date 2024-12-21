#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        p { "Count: {count}" }

        div {
            style: "display: flex;",
            button {
                onclick: move |_| count -= 1,
                "-"
                
            }
            button {
                onclick: move |_| count += 1,
                "+"
            }
        }
    }
}