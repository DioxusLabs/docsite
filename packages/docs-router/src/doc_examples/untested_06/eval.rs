use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let future = use_resource(move || async move {
        // You can create as many eval instances as you want
        let mut eval = document::eval(
            r#"
            // You can send messages from JavaScript to Rust with the dioxus.send function
            dioxus.send("Hi from JS!");
            // You can receive messages from Rust to JavaScript with the dioxus.recv function
            let msg = await dioxus.recv();
            console.log(msg);
            "#,
        );

        // You can send messages to JavaScript with the send method
        eval.send("Hi from Rust!").unwrap();

        // You can receive any message from JavaScript with the recv method
        eval.recv::<String>().await.unwrap()
    });

    match future.read_unchecked().as_ref() {
        Some(v) => rsx! {
            p { "{v}" }
        },
        _ => rsx! {
            p { "hello" }
        },
    }
}
