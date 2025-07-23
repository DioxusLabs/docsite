use futures::FutureExt;
use std::{cell::RefCell, rc::Rc, sync::Arc, thread::Scope};

use dioxus::{dioxus_core::NoOpMutations, prelude::*};

#[test]
fn test() {
    test_hook(
        || use_signal(|| 0),
        |mut value, mut proxy| match proxy.generation {
            0 => {
                value.set(1);
            }
            1 => {
                assert_eq!(*value.read(), 1);
                value.set(2);
            }
            2 => {
                proxy.rerun();
            }
            3 => {}
            _ => todo!(),
        },
        |proxy| assert_eq!(proxy.generation, 4),
    );
}

fn test_hook<V: 'static>(
    initialize: impl FnMut() -> V + 'static,
    check: impl FnMut(V, MockProxy) + 'static,
    mut final_check: impl FnMut(MockProxy) + 'static,
) {
    #[derive(Props)]
    struct MockAppComponent<I: 'static, C: 'static> {
        hook: Rc<RefCell<I>>,
        check: Rc<RefCell<C>>,
    }

    impl<I, C> PartialEq for MockAppComponent<I, C> {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    impl<I, C> Clone for MockAppComponent<I, C> {
        fn clone(&self) -> Self {
            Self {
                hook: self.hook.clone(),
                check: self.check.clone(),
            }
        }
    }

    fn mock_app<I: FnMut() -> V, C: FnMut(V, MockProxy), V>(
        props: MockAppComponent<I, C>,
    ) -> Element {
        let value = props.hook.borrow_mut()();

        props.check.borrow_mut()(value, MockProxy::new());

        rsx! {
            div {}
        }
    }

    let mut vdom = VirtualDom::new_with_props(
        mock_app,
        MockAppComponent {
            hook: Rc::new(RefCell::new(initialize)),
            check: Rc::new(RefCell::new(check)),
        },
    );

    vdom.rebuild_in_place();

    while vdom.wait_for_work().now_or_never().is_some() {
        vdom.render_immediate(&mut NoOpMutations);
    }

    vdom.in_runtime(|| {
        ScopeId::ROOT.in_runtime(|| {
            final_check(MockProxy::new());
        })
    })
}

struct MockProxy {
    rerender: Arc<dyn Fn()>,
    pub generation: usize,
}

impl MockProxy {
    fn new() -> Self {
        let generation = dioxus::core::generation();
        let rerender = dioxus::core::schedule_update();

        Self {
            rerender,
            generation,
        }
    }

    pub fn rerun(&mut self) {
        (self.rerender)();
    }
}
