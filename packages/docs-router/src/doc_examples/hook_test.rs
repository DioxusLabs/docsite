use dioxus::prelude::*;
use futures::{
    channel::mpsc::{Receiver, Sender},
    StreamExt,
};
use std::{cell::RefCell, fmt::Debug, future::Future, rc::Rc};

#[tokio::test]
async fn test_use_resource() {
    let mut dom = TestDom::new(|| {
        let mut signal = use_signal(|| 0);

        use_events(|mut events| async move {
            while let Some(event) = events.next().await {
                signal.set(event);
            }
        });

        let async_doubled = use_resource(move || async move {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            signal * 2
        });

        rsx! {
            if let Some(value) = async_doubled() {
                div { "{value}" }
            } else {
                div { "Loading..." }
            }
        }
    });

    // Initial render should show "Loading..."
    dom.assert_eq(rsx! { div { "Loading..." } });

    dom.run_async().await;

    // After the async resource resolves, it should show "0"
    dom.assert_eq(rsx! { div { "0" } });

    dom.send(1);

    // While it is resolving it should show the old value "0"
    dom.assert_eq(rsx! { div { "0" } });

    dom.run_async().await;

    // After the async resource resolves again, it should show "2"
    dom.assert_eq(rsx! { div { "2" } });
}

fn use_events<E: 'static, F: Future<Output = ()> + 'static>(
    with_events: impl FnOnce(Receiver<E>) -> F,
) {
    use_hook(move || {
        let context = consume_context::<EventContext<E>>();
        let (sender, receiver) = futures::channel::mpsc::channel(1);
        context.sender.borrow_mut().push(sender);
        spawn(with_events(receiver));
    })
}

struct EventContext<E> {
    sender: Rc<RefCell<Vec<Sender<E>>>>,
}

impl<E> Default for EventContext<E> {
    fn default() -> Self {
        Self {
            sender: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl<E> Clone for EventContext<E> {
    fn clone(&self) -> Self {
        Self {
            sender: Rc::clone(&self.sender),
        }
    }
}

struct TestDom<E = ()> {
    vdom: RefCell<VirtualDom>,
    context: EventContext<E>,
}

impl<E: 'static> Debug for TestDom<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.render())
    }
}

impl<E: 'static> TestDom<E> {
    fn new(component: impl FnMut() -> Element + 'static) -> Self {
        #[derive(Props)]
        struct MockAppComponent {
            run: Rc<RefCell<Box<dyn FnMut() -> Element>>>,
        }

        impl PartialEq for MockAppComponent {
            fn eq(&self, _: &Self) -> bool {
                true
            }
        }

        impl Clone for MockAppComponent {
            fn clone(&self) -> Self {
                Self {
                    run: self.run.clone(),
                }
            }
        }

        fn mock_app(props: MockAppComponent) -> Element {
            props.run.borrow_mut()()
        }

        let context = EventContext::default();
        let mut vdom = VirtualDom::new_with_props(
            mock_app,
            MockAppComponent {
                run: Rc::new(RefCell::new(Box::new(component))),
            },
        );
        vdom.provide_root_context(context.clone());
        vdom.rebuild_in_place();

        Self {
            vdom: RefCell::new(vdom),
            context,
        }
    }

    async fn run_async(&mut self) {
        self.vdom.borrow_mut().wait_for_work().await;
    }

    fn render(&self) -> String {
        self.vdom
            .borrow_mut()
            .render_immediate(&mut dioxus_core::NoOpMutations);
        dioxus::ssr::render(&self.vdom.borrow())
    }

    fn send(&self, event: E)
    where
        E: Clone,
    {
        let mut senders = self.context.sender.borrow_mut();
        senders.retain_mut(|sender| sender.try_send(event.clone()).is_ok());
    }

    fn assert_eq(&self, other: Element) {
        pretty_assertions::assert_eq!(
            self.render(),
            dioxus::ssr::render_element(other),
        );
    }
}
