#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::collections::HashMap;

fn main() {
    launch(App);
}

fn App() -> Element {
    let you_are_happy = true;
    let you_know_it = false;

    // ANCHOR: conditional
    // ❌ don't call hooks in conditionals!
    // We must ensure that the same hooks will be called every time
    // But `if` statements only run if the conditional is true!
    // So we might violate rule 2.
    if you_are_happy && you_know_it {
        let something = use_signal(|| "hands");
        println!("clap your {something}")
    }

    // ✅ instead, *always* call use_signal
    // You can put other stuff in the conditional though
    let something = use_signal(|| "hands");
    if you_are_happy && you_know_it {
        println!("clap your {something}")
    }
    // ANCHOR_END: conditional

    // ANCHOR: closure
    // ❌ don't call hooks inside closures!
    // We can't guarantee that the closure, if used, will be called in the same order every time
    let _a = || {
        let b = use_signal(|| 0);
        b()
    };

    // ✅ instead, move hook `b` outside
    let b = use_signal(|| 0);
    let _a = || b();
    // ANCHOR_END: closure

    let names: Vec<&str> = vec![];

    // ANCHOR: loop
    // `names` is a Vec<&str>

    // ❌ Do not use hooks in loops!
    // In this case, if the length of the Vec changes, we break rule 2
    for _name in &names {
        let is_selected = use_signal(|| false);
        println!("selected: {is_selected}");
    }

    // ✅ Instead, use a hashmap with use_signal
    let selection_map = use_signal(HashMap::<&str, bool>::new);

    for name in &names {
        let is_selected = selection_map.read()[name];
        println!("selected: {is_selected}");
    }
    // ANCHOR_END: loop

    None
}
