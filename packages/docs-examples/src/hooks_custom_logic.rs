#![allow(unused)]

use dioxus::core::ReactiveContext;
use dioxus::prelude::*;

fn main() {}

// ANCHOR: use_signal
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

struct Signal<T> {
    value: Rc<RefCell<T>>,
    subscribers: Arc<Mutex<HashSet<ReactiveContext>>>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            subscribers: self.subscribers.clone(),
        }
    }
}

fn my_use_signal<T: 'static>(init: impl FnOnce() -> T) -> Signal<T> {
    use_hook(|| {
        // A set of subscribers to notify about changes to this signals value
        let subscribers = Default::default();
        // Create the initial state
        let value = Rc::new(RefCell::new(init()));

        Signal { value, subscribers }
    })
}

impl<T: Clone> Signal<T> {
    fn get(&self) -> T {
        // Subscribe the context observing the signal (if any) to updates of its value.
        if let Some(reactive_context) = ReactiveContext::current() {
            reactive_context.subscribe(self.subscribers.clone());
        }

        self.value.borrow().clone()
    }

    fn set(&self, value: T) {
        // Update the state
        *self.value.borrow_mut() = value;
        // Trigger a re-render of the components that observed the signal's previous value
        let mut subscribers = std::mem::take(&mut *self.subscribers.lock().unwrap());
        subscribers.retain(|reactive_context| reactive_context.mark_dirty());
        // Extend the subscribers list instead of overwriting it in case a subscriber is added while reactive contexts are marked dirty
        self.subscribers.lock().unwrap().extend(subscribers);
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
