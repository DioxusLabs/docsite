use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    // Use eval returns a function that can spawn eval instances
    let create_eval = use_eval(cx);

    // You can create as many eval instances as you want
    let mut eval = create_eval(
        r#"
        // You can send messages from JavaScript to Rust with the dioxus.send function
        dioxus.send("Hi from JS!");
        // You can receive messages from Rust to JavaScript with the dioxus.recv function
        let msg = await dioxus.recv();
        console.log(msg);
        "#,
    )
    .unwrap();

    // You can send messages to JavaScript with the send method
    eval.send("Hi from Rust!".into()).unwrap();

    let future = use_future(cx, (), |_| {
        to_owned![eval];
        async move {
            // You can receive any message from JavaScript with the recv method
            eval.recv().await.unwrap()
        }
    });

    match future.value() {
        Some(v) => cx.render(rsx!(
            p { "{v}" }
        )),
        _ => cx.render(rsx!(
            p { "hello" }
        )),
    }
}
