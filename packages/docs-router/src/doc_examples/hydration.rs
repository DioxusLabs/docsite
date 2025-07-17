mod hydration_intro {
    use dioxus::prelude::*;
    async fn fetch_weather() -> Result<String, ServerFnError> {
        todo!()
    }
    // ANCHOR: hydration_intro
    fn Weather() -> Element {
        let mut weather = use_server_future(fetch_weather)?;

        rsx! {
            div {
                "{weather:?}"
            }
            button {
                onclick: move |_| weather.restart(),
                "Refetch"
            }
        }
    }
    // ANCHOR_END: hydration_intro
}

mod server_cached {
    use dioxus::prelude::*;

    fn app() -> Element {
        // ANCHOR: server_cached
        // ❌ The random number will be different on the client and the server
        let random: u8 = use_hook(|| rand::random());
        // ✅ The same random number will be serialized on the server and deserialized on the client
        let random: u8 = use_server_cached(|| rand::random());
        // ANCHOR_END: server_cached

        let mut count = use_signal(|| random);

        rsx! {
            button {
                onclick: move |_| count += 1,
                "{count}"
            }
            for i in 0..count() {
                div {
                    "{i}"
                }
            }
        }
    }
}

mod server_future {
    use dioxus::prelude::*;

    #[server]
    async fn random_server_function() -> Result<u8, ServerFnError> {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(rand::random())
    }

    fn app() -> Element {
        // ANCHOR: server_future
        // ❌ The server function result may be finished on the server, but pending on the client
        let random: u8 = use_resource(|| random_server_function()).suspend()?().unwrap_or_default();
        // ✅ Once the server function is resolved on the server, it will be sent to the client
        let random: u8 = use_server_future(|| random_server_function())?()
            .unwrap()
            .unwrap_or_default();
        // ANCHOR_END: server_future

        let mut count = use_signal(|| random);

        rsx! {
            button {
                onclick: move |_| count += 1,
                "{count}"
            }
            for i in 0..count() {
                div {
                    "{i}"
                }
            }
        }
    }
}

mod effects {
    use dioxus::prelude::*;

    fn app() -> Element {
        // ANCHOR: effects
        // ❌ Using a different value client side before hydration will cause hydration issues
        // because the server rendered the html with another value
        let mut storage = use_signal(|| {
            #[cfg(feature = "server")]
            return None;
            let window = web_sys::window().unwrap();
            let local_storage = window.local_storage().unwrap().unwrap();
            local_storage.set_item("count", "1").unwrap();
            local_storage.get_item("count").unwrap()
        });
        // ✅ Changing the value inside of an effect is fine because effects run after hydration
        let mut storage = use_signal(|| None);
        use_effect(move || {
            let window = web_sys::window().unwrap();
            let local_storage = window.local_storage().unwrap().unwrap();
            local_storage.set_item("count", "1").unwrap();
            storage.set(local_storage.get_item("count").unwrap());
        });
        // ANCHOR_END: effects

        rsx! {
            for item in storage() {
                div {
                    "The count is {item}"
                }
            }
        }
    }
}

mod server_hook_side_effects {
    use dioxus::prelude::*;

    async fn server_future() -> Result<String, ServerFnError> {
        todo!()
    }

    fn app() -> Element {
        // ANCHOR: server_hook_side_effects
        // ❌ The state of the signal cannot be serialized on the server
        let mut storage = use_signal(|| None);
        use_server_future(move || async move {
            storage.set(Some(server_future().await));
        })?;
        // ✅ The value returned from use_server_future will be serialized on the server and hydrated on the client
        let storage = use_server_future(|| async move { server_future().await })?;
        // ANCHOR_END: server_hook_side_effects

        panic!()
    }
}
