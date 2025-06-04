//! Easily integrate async Rust code into your components.
use dioxus::prelude::*;

fn Tasks() -> Element {
    let mut count = use_signal(|| 0);

    use_future(move || async move {
        loop {
            count += 1;
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });

    rsx! { pre { "Count: {count}" } }
}
