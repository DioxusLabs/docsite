#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use fermi::{use_atom_root, use_read, Atom, AtomRoot};
use futures::Stream;
use std::rc::Rc;
use tokio::time::Duration;

async fn connect_to_ws_server() -> impl Stream<Item = ()> {
    futures::stream::empty()
}

// ANCHOR: component
use futures_util::StreamExt;

fn app(cx: Scope) {
    let ws: &Coroutine<()> = use_coroutine(cx, |rx| async move {
        // Connect to some sort of service
        let mut conn = connect_to_ws_server().await;

        // Wait for data on the service
        while let Some(msg) = conn.next().await {
            // handle messages
        }
    });
}
// ANCHOR_END: component

fn to_owned(cx: Scope) {
    enum Status {
        Launching,
        Working,
    }
    enum SyncAction {}
    // ANCHOR: to_owned
    let sync_status = use_state(cx, || Status::Launching);
    let sync_task = use_coroutine(cx, |rx: UnboundedReceiver<SyncAction>| {
        let sync_status = sync_status.to_owned();
        async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                sync_status.set(Status::Working);
            }
        }
    });
    // ANCHOR_END: to_owned

    // ANCHOR: to_owned_continued
    let sync_status = use_state(cx, || Status::Launching);
    let load_status = use_state(cx, || Status::Launching);
    let sync_task = use_coroutine(cx, |rx: UnboundedReceiver<SyncAction>| {
        to_owned![sync_status, load_status];
        async move {
            // ...
        }
    });
    // ANCHOR_END: to_owned_continued
}

fn send(cx: Scope) -> Element {
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

    let profile = use_coroutine(cx, |mut rx: UnboundedReceiver<ProfileUpdate>| async move {
        let mut server = connect_to_server().await;

        while let Some(msg) = rx.next().await {
            match msg {
                ProfileUpdate::SetUsername(name) => server.update_username(name).await,
                ProfileUpdate::SetAge(age) => server.update_age(age).await,
            }
        }
    });

    cx.render(rsx! {
        button {
            onclick: move |_| profile.send(ProfileUpdate::SetUsername("Bob".to_string())),
            "Update username"
        }
    })
    // ANCHOR_END: send
}

fn services(cx: Scope) {
    enum ProfileCommand {}
    enum SyncCommand {}
    enum EditorCommand {}
    // ANCHOR: services
    let profile = use_coroutine(cx, profile_service);
    let editor = use_coroutine(cx, editor_service);
    let sync = use_coroutine(cx, sync_service);

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

fn fermi() {
    async fn sync_service(rx: UnboundedReceiver<()>, atoms: Rc<AtomRoot>) {
        todo!()
    }
    // ANCHOR: fermi
    static USERNAME: Atom<String> = Atom(|_| "default".to_string());

    fn app(cx: Scope) -> Element {
        let atoms = use_atom_root(cx);

        use_coroutine(cx, |rx| sync_service(rx, atoms.clone()));

        cx.render(rsx! {
            Banner {}
        })
    }

    fn Banner(cx: Scope) -> Element {
        let username = use_read(cx, &USERNAME);

        cx.render(rsx! {
            h1 { "Welcome back, {username}" }
        })
    }
    // ANCHOR_END: fermi
}

fn fermi_continued() {
    async fn set_name_on_server(name: &str) -> Result<(), ()> {
        Ok(())
    }
    // ANCHOR: fermi_continued
    use fermi::{Atom, AtomRoot, Readable, Writable};
    use futures_util::StreamExt;

    static USERNAME: &Atom<String> = &Atom(|_| "default".to_string());
    static ERRORS: &Atom<Vec<String>> = &Atom(|_| Vec::new());

    enum SyncAction {
        SetUsername(String),
    }

    async fn sync_service(mut rx: UnboundedReceiver<SyncAction>, atoms: Rc<AtomRoot>) {
        while let Some(msg) = rx.next().await {
            match msg {
                SyncAction::SetUsername(name) => {
                    if set_name_on_server(&name).await.is_ok() {
                        atoms.set(USERNAME.unique_id(), name);
                    } else {
                        atoms.set(
                            ERRORS.unique_id(),
                            vec!["Failed to set username".to_string()],
                        );
                    }
                }
            }
        }
    }
    // ANCHOR_END: fermi_continued
}

fn injection() {
    enum SyncAction {
        SetUsername,
    }
    // ANCHOR: injection
    fn Child(cx: Scope) -> Element {
        let sync_task = use_coroutine_handle::<SyncAction>(cx).unwrap();

        sync_task.send(SyncAction::SetUsername);

        todo!()
    }
    // ANCHOR_END: injection
}
