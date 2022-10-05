/// # Coroutines, async, and tasks: async.rs
///
/// Components may spawn a coroutine or task to perform asynchronous operations. These tasks may be started, stopped, or
/// reset by other logc in the component. Coroutines are extremely handy for asynchronous tasks like network requests,
/// websockets, and multi-threading.
fn Tasks(cx: Scope) -> Element {
    let count = use_state(&cx, || 0);

    use_future(cx, || {
        let mut count = count.to_owned();
        async move {
            loop {
                count += 1;
                timer::from_ms(500).await;
            }
        }
    });

    rsx!(cx, pre {"Count: {count}"})
}
