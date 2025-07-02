#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use futures::Stream;
use std::rc::Rc;
use tokio::time::Duration;

async fn connect_to_ws_server() -> impl Stream<Item = ()> {
    futures::stream::empty()
}

// ANCHOR: component
use futures_util::StreamExt;

fn app() {
    let ws: Coroutine<()> = use_coroutine(|rx| async move {
        // Connect to some sort of service
        let mut conn = connect_to_ws_server().await;

        // Wait for data on the service
        while let Some(msg) = conn.next().await {
            // handle messages
        }
    });
}
// ANCHOR_END: component

fn to_owned() {
    enum Status {
        Launching,
        Working,
    }
    enum SyncAction {}
    // ANCHOR: to_owned
    let mut sync_status = use_signal(|| Status::Launching);
    let sync_task = use_coroutine(move |rx: UnboundedReceiver<SyncAction>| async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            sync_status.set(Status::Working);
        }
    });
    // ANCHOR_END: to_owned

    // ANCHOR: to_owned_continued
    let sync_status = use_signal(|| Status::Launching);
    let load_status = use_signal(|| Status::Launching);
    let sync_task = use_coroutine(|rx: UnboundedReceiver<SyncAction>| {
        async move {
            // ...
        }
    });
    // ANCHOR_END: to_owned_continued
}

fn send() -> Element {
    struct Server;
    impl Server {
        async fn update_username(&mut self, name: String) {}
        async fn update_age(&mut self, age: i32) {}
    }
    async fn connect_to_server() -> Server {
        Server
    }
    // ANCHOR: send
    use futures_util::StreamExt;

    enum ProfileUpdate {
        SetUsername(String),
        SetAge(i32),
    }

    let profile = use_coroutine(|mut rx: UnboundedReceiver<ProfileUpdate>| async move {
        let mut server = connect_to_server().await;

        while let Some(msg) = rx.next().await {
            match msg {
                ProfileUpdate::SetUsername(name) => server.update_username(name).await,
                ProfileUpdate::SetAge(age) => server.update_age(age).await,
            }
        }
    });

    rsx! {
        button { onclick: move |_| profile.send(ProfileUpdate::SetUsername("Bob".to_string())),
            "Update username"
        }
    }
    // ANCHOR_END: send
}

fn services() {
    enum ProfileCommand {}
    enum SyncCommand {}
    enum EditorCommand {}
    // ANCHOR: services
    let profile = use_coroutine(profile_service);
    let editor = use_coroutine(editor_service);
    let sync = use_coroutine(sync_service);

    async fn profile_service(rx: UnboundedReceiver<ProfileCommand>) {
        // do stuff
    }

    async fn sync_service(rx: UnboundedReceiver<SyncCommand>) {
        // do stuff
    }

    async fn editor_service(rx: UnboundedReceiver<EditorCommand>) {
        // do stuff
    }
    // ANCHOR_END: services
}

fn global() {
    async fn sync_service(rx: UnboundedReceiver<()>) {
        todo!()
    }
    // ANCHOR: global
    static USERNAME: GlobalSignal<String> = Signal::global(|| "default".to_string());

    fn app() -> Element {
        use_coroutine(sync_service);

        rsx! {
            Banner {}
        }
    }

    fn Banner() -> Element {
        rsx! {
            h1 { "Welcome back, {USERNAME}" }
        }
    }
    // ANCHOR_END: global
}

fn global_continued() {
    async fn set_name_on_server(name: &str) -> Result<(), ()> {
        Ok(())
    }
    // ANCHOR: global_continued
    use futures_util::StreamExt;

    static USERNAME: GlobalSignal<String> = Signal::global(|| "default".to_string());
    static ERRORS: GlobalSignal<Vec<String>> = Signal::global(|| Vec::new());

    enum SyncAction {
        SetUsername(String),
    }

    async fn sync_service(mut rx: UnboundedReceiver<SyncAction>) {
        while let Some(msg) = rx.next().await {
            match msg {
                SyncAction::SetUsername(name) => {
                    if set_name_on_server(&name).await.is_ok() {
                        *USERNAME.write() = name;
                    } else {
                        *ERRORS.write() = vec!["Failed to set username".to_string()];
                    }
                }
            }
        }
    }
    // ANCHOR_END: global_continued
}

fn injection() {
    enum SyncAction {
        SetUsername,
    }
    // ANCHOR: injection
    fn Child() -> Element {
        let sync_task = use_coroutine_handle::<SyncAction>();

        sync_task.send(SyncAction::SetUsername);

        todo!()
    }
    // ANCHOR_END: injection
}
