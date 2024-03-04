// ANCHOR: intro
use dioxus::prelude::*;

static NAME: GlobalSignal<String> = Signal::global(|| "world".to_string());
// Global signals work for copy and clone types in the same way
static NAMES: GlobalSignal<Vec<String>> = Signal::global(|| vec!["world".to_string()]);

fn app() -> Element {
    // No need to use use_init_atom_root, use_set, or use_atom_ref. Just use the global signal directly
    rsx! {
        button { onclick: move |_| *NAME.write() = "reset name".to_string(), "reset name" }
        "{NAMES:?}"
    }
}
// ANCHOR_END: intro

// ANCHOR: memos
static COUNT: GlobalSignal<u32> = Signal::global(|| 0);
static MEMO: GlobalMemo<u32> = Signal::global_memo(|| COUNT() + 1);

fn GlobalMemo() -> Element {
    rsx! {
        button { onclick: move |_| *COUNT.write() += 1, "increment" }
        // Global memos can be used like signals
        "{MEMO}"
    }
}
// ANCHOR_END: memos
