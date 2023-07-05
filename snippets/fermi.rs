//! Use Fermi to easily manage global state with a simple Atom-based API

// Define an atom of state
static COUNT: Atom<i32> = |_| 0;

// Read it anywhere
fn Demo(cx: Scope) -> Element {
	let count = use_read(cx, || COUNT);

	render!("Count: {count}")
}

// Or write to it from anywhere
fn Increment(cx: Scope) -> Element {
	let mut count = use_atom(cx, COUNT);

	render!( button { onclick: move |_| count += 1, "Increment" } )
}
