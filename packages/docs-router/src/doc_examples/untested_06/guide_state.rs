mod split_app {
    use dioxus::prelude::*;
    static CSS: Asset = asset!("/assets/main.css");

    // ANCHOR: split_app
    #[component]
    fn App() -> Element {
        rsx! {
            document::Stylesheet { href: CSS }
            Title {}
            DogView {}
        }
    }

    #[component]
    fn Title() -> Element {
        rsx! {
            div { id: "title",
                h1 { "HotDog! 🌭" }
            }
        }
    }

    #[component]
    fn DogView() -> Element {
        rsx! {
            div { id: "dogview",
                img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
            }
            div { id: "buttons",
                button { id: "skip", "skip" }
                button { id: "save", "save!" }
            }
        }
    }
    // ANCHOR_END: split_app
}

mod event_handler {
    use dioxus::prelude::*;

    // ANCHOR: event_handler
    #[component]
    fn DogView() -> Element {
        let skip = move |evt| {};
        let save = move |evt| {};

        rsx! {
            // ...
            div { id: "buttons",
                button { onclick: skip, id: "skip",  "skip" }
                button { onclick: save, id: "save",  "save!" }
            }
        }
    }
    // ANCHOR_END: event_handler
}

mod use_hook {
    use dioxus::prelude::*;

    // ANCHOR: use_hook
    #[component]
    fn DogView() -> Element {
        let img_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");

        // ..

        rsx! {
            div { id: "dogview",
                img { src: "{img_src}" }
            }
            // ..
        }
    }
    // ANCHOR_END: use_hook
}

mod context {
    use dioxus::prelude::*;

    // ANCHOR: context
    // Create a new wrapper type
    #[derive(Clone)]
    struct TitleState(String);

    fn App() -> Element {
        // Provide that type as a Context
        use_context_provider(|| TitleState("HotDog".to_string()));
        rsx! {
            Title {}
        }
    }

    fn Title() -> Element {
        // Consume that type as a Context
        let title = use_context::<TitleState>();
        rsx! {
            h1 { "{title.0}" }
        }
    }
    // ANCHOR_END: context
}

mod signal_context {
    use dioxus::prelude::*;

    // ANCHOR: signal_context
    #[derive(Clone, Copy)]
    struct MusicPlayer {
        song: Signal<String>,
    }

    fn use_music_player_provider() {
        let song = use_signal(|| "Drift Away".to_string());
        use_context_provider(|| MusicPlayer { song });
    }
    // ANCHOR_END: signal_context

    // ANCHOR: signal_context_usage
    #[component]
    fn Player() -> Element {
        rsx! {
            button {
                onclick: move |_| consume_context::<MusicPlayer>().song.set("Vienna".to_string()),
                "Shuffle"
            }
        }
    }
    // ANCHOR_END: signal_context_usage
}

mod global_signal {
    use dioxus::prelude::*;

    // ANCHOR: initialize_global_signal
    static SONG: GlobalSignal<String> = Signal::global(|| "Drift Away".to_string());
    // ANCHOR_END: initialize_global_signal

    // ANCHOR: use_global_signal
    #[component]
    fn Player() -> Element {
        rsx! {
            h3 { "Now playing {SONG}" }
            button {
                onclick: move |_| *SONG.write() = "Vienna".to_string(),
                "Shuffle"
            }
        }
    }
    // ANCHOR_END: use_global_signal
}
