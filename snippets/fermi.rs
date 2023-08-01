//! Use Fermi to easily manage global state with a simple Atom-based API

// Define an atom of state
static COUNT: fermi::Atom<i32> = |_| 0;

// Read it anywhere
fn Read(cx: Scope) -> Element {
    let count = fermi::use_read(cx, &|| COUNT);

    render!("Count: {count}")
}

// Or write to it from anywhere
fn Increment(cx: Scope) -> Element {
    let mut count = fermi::use_atom_state(cx, &COUNT);

    render!( button { onclick: move |_| count += 1 , "Increment" } )
}

fn App(cx: Scope) -> Element {
    //Initialize the atom root - this is what keeps track of your atoms
    fermi::use_init_atom_root(cx);

    render!(
        Read {},
        Increment {}
    )
}