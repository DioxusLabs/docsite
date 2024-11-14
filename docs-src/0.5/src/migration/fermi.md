# Fermi

In dioxus 0.5, fermi atoms have been replaced with global signals and included in the main dioxus library.


The new global signals can be used directly without hooks and include additional functionality like global memos.

Dioxus 0.4:
```rust
use dioxus::prelude::*;
use fermi::*;

static NAME: Atom<String> = Atom(|_| "world".to_string());
static NAMES: AtomRef<Vec<String>> = AtomRef(|_| vec!["world".to_string()]);

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let set_name = use_set(cx, &NAME);
	let names = use_atom_ref(cx, &NAMES);

    cx.render(rsx! {
        button {
			onclick: move |_| set_name("dioxus".to_string()),
			"reset name"
		}
		"{names.read():?}"
    })
}
```

Dioxus 0.5:
```rust
{{#include src/doc_examples/migration_fermi.rs:intro}}
```

## Memos

Dioxus 0.5 introduces global memos which can be used to store computed values globally.

```rust
{{#include src/doc_examples/migration_fermi.rs:memos}}
```
