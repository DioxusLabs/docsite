use futures::FutureExt;
use std::{cell::RefCell, sync::Arc};

use dioxus::prelude::*;

#[test]
fn test() {
    test_hook(
        |cx| use_ref(cx, || 0).clone(),
        |value, mut proxy| match proxy.generation {
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
    initialize: impl FnMut(&ScopeState) -> V + 'static,
    check: impl FnMut(V, MockProxy) + 'static,
    mut final_check: impl FnMut(MockProxy) + 'static,
) {
    #[derive(Props)]
    struct MockAppComponent<
        I: FnMut(&ScopeState) -> V + 'static,
        C: FnMut(V, MockProxy) + 'static,
        V,
    > {
        hook: RefCell<I>,
        check: RefCell<C>,
    }

    impl<I: FnMut(&ScopeState) -> V, C: FnMut(V, MockProxy), V> PartialEq
        for MockAppComponent<I, C, V>
    {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    fn mock_app<I: FnMut(&ScopeState) -> V, C: FnMut(V, MockProxy), V>(
        cx: Scope<MockAppComponent<I, C, V>>,
    ) -> Element {
        let value = cx.props.hook.borrow_mut()(cx);

        cx.props.check.borrow_mut()(value, MockProxy::new(cx));

        render! {
            div {}
        }
    }

    let mut vdom = VirtualDom::new_with_props(
        mock_app,
        MockAppComponent {
            hook: RefCell::new(initialize),
            check: RefCell::new(check),
        },
    );

    let _ = vdom.rebuild();

    while vdom.wait_for_work().now_or_never().is_some() {
        let _ = vdom.render_immediate();
    }

    final_check(MockProxy::new(vdom.base_scope()));
}

struct MockProxy {
    rerender: Arc<dyn Fn()>,
    pub generation: usize,
}

impl MockProxy {
    fn new(scope: &ScopeState) -> Self {
        let generation = scope.generation();
        let rerender = scope.schedule_update();

        Self {
            rerender,
            generation,
        }
    }

    pub fn rerun(&mut self) {
        (self.rerender)();
    }
}
