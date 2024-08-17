# How to Upgrade to Dioxus 0.5

This guide will outline the API changes between the `0.4` and `0.5` releases. 

`0.5` has includes significant changes to hooks, props, and global state.

## Cheat Sheet

Here is a quick cheat sheet for the changes:

### Scope

Dioxus 0.4:
```rust
fn app(cx: Scope) -> Element {
    cx.use_hook(|| {
        /*...*/
    });
    cx.provide_context({
        /*...*/
    });
    cx.spawn(async move {
        /*...*/
    });
    cx.render(rsx! {
        /*...*/
    })
}
```
Dioxus 0.5:
```rust
{{#include src/doc_examples/migration.rs:scope}}
```

### Props

Dioxus 0.4:
```rust
#[component]
fn Comp(cx: Scope, name: String) -> Element {
    // You pass in an owned prop, but inside the component, it is borrowed (name is the type &String inside the function)
    let owned_name: String = name.clone();

    cx.render(rsx! {
        "Hello {owned_name}"
        BorrowedComp {
            "{name}"
        }
        ManualPropsComponent {
            name: name
        }
    })
}

#[component]
fn BorrowedComp<'a>(cx: Scope<'a>, name: &'a str) -> Element<'a> {
    cx.render(rsx! {
        "Hello {name}"
    })
}

#[derive(Props, PartialEq)]
struct ManualProps {
    name: String
}

fn ManualPropsComponent(cx: Scope<ManualProps>) -> Element {
    cx.render(rsx! {
        "Hello {cx.props.name}"
    })
}
```

Dioxus 0.5:
```rust
{{#include src/doc_examples/migration.rs:props}}
```

You can read more about the new props API in the [Props Migration](props.md) guide.

### Futures

Dioxus 0.4:
```rust
use_future((dependency1, dependency2,), move |(dependency1, dependency2,)| async move {
	/*use dependency1 and dependency2*/
});
```
Dioxus 0.5:
```rust
{{#include src/doc_examples/migration.rs:futures}}
```

Read more about the `use_resource` hook in the [Hook Migration](hooks.md) guide.

### State Hooks

Dioxus 0.4:
```rust
let copy_state = use_state(cx, || 0);
let clone_local_state = use_ref(cx, || String::from("Hello"));
use_shared_state_provider(cx, || String::from("Hello"));
let clone_shared_state = use_shared_state::<String>(cx);

let copy_state_value = **copy_state;
let clone_local_state_value = clone_local_state.read();
let clone_shared_state_value = clone_shared_state.read();

cx.render(rsx!{
	"{copy_state_value}"
	"{clone_shared_state_value}"
	"{clone_local_state_value}"
	button {
		onclick: move |_| {
			copy_state.set(1);
			*clone_local_state.write() = "World".to_string();
			*clone_shared_state.write() = "World".to_string();
		},
		"Set State"
	}
})
```

Dioxus 0.5:

```rust
{{#include src/doc_examples/migration.rs:state}}
```

Read more about the `use_signal` hook in the [State Migration](state.md) guide.

### Fermi

Dioxus 0.4:
```rust
use dioxus::prelude::*;
use fermi::*;

static NAME: Atom<String> = Atom(|_| "world".to_string());

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let name = use_read(cx, &NAME);

    cx.render(rsx! {
        div { "hello {name}!" }
        Child {}
        ChildWithRef {}
    })
}

fn Child(cx: Scope) -> Element {
    let set_name = use_set(cx, &NAME);

    cx.render(rsx! {
        button {
            onclick: move |_| set_name("dioxus".to_string()),
            "reset name"
        }
    })
}

static NAMES: AtomRef<Vec<String>> = AtomRef(|_| vec!["world".to_string()]);

fn ChildWithRef(cx: Scope) -> Element {
    let names = use_atom_ref(cx, &NAMES);

    cx.render(rsx! {
        div {
            ul {
                names.read().iter().map(|f| rsx!{
                    li { "hello: {f}" }
                })
            }
            button {
                onclick: move |_| {
                    let names = names.clone();
                    cx.spawn(async move {
                        names.write().push("asd".to_string());
                    })
                },
                "Add name"
            }
        }
    })
}
```

Dioxus 0.5:
```rust
{{#include src/doc_examples/migration.rs:fermi}}
```

You can read more about global signals in the [Fermi migration guide](fermi.md).
