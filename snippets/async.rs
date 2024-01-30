//! Easily integrate async Rust code into your components.

fn Tasks() -> Element {
    let count = use_signal(|| 0);

    use_coroutine(|_| {
        to_owned![count];
        async move {
            loop {
                count += 1;
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    });

    rsx!( pre { "Count: {count}" } )
}
