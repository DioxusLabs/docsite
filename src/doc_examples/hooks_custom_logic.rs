#![allow(unused)]

use dioxus::prelude::*;

fn main() {}

// ANCHOR: use_signal
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

struct Signal<T> {
    value: Rc<RefCell<T>>,
    update: Arc<dyn Fn()>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            update: self.update.clone(),
        }
    }
}

fn my_use_signal<T: 'static>(init: impl FnOnce() -> T) -> Signal<T> {
    use_hook(|| {
        // The update function will trigger a re-render in the component cx is attached to
        let update = schedule_update();
        // Create the initial state
        let value = Rc::new(RefCell::new(init()));

        Signal { value, update }
    })
}

impl<T: Clone> Signal<T> {
    fn get(&self) -> T {
        self.value.borrow().clone()
    }

    fn set(&self, value: T) {
        // Update the state
        *self.value.borrow_mut() = value;
        // Trigger a re-render on the component the state is from
        (self.update)();
    }
}
// ANCHOR_END: use_signal

// ANCHOR: use_context
pub fn use_context<T: 'static + Clone>() -> T {
    use_hook(|| consume_context())
}

pub fn use_context_provider<T: 'static + Clone>(f: impl FnOnce() -> T) -> T {
    use_hook(|| {
        let val = f();
        // Provide the context state to the component
        provide_context(val.clone());
        val
    })
}

// ANCHOR_END: use_context
