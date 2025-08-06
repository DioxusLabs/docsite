# Intro to Reactivity

So far, we've covered how to declare static user interfaces with RSX. Now, it's time to make our UI interactive. When the user clicks a button, moves their cursor, or inputs text, we want to update our UI in response.

The act of reacting to a state changes is called *reactivity*. Reactivity sits at the very heart of Dioxus. Everything from data fetching to routing is centered around reacting to state changes.

Similar to libraries like ReactJS, Dioxus provides an built-in reactivity system, saving you from manually queuing re-renders of components. If you're coming from web development, this should feel familiar. If you're mostly experienced with immediate-mode GUIs, this might seem foreign.

Reactivity systems enable larger, more modular, complex GUIs than is typically achievable with other approaches. To build UIs that effectively leverage reactivity, we follow three main pillars:

- **Data Flows Down**: Our apps maintain a one-way flow of data from parents to children
- **Data is Tracked**: Modifications to data is observed by reactive scopes and side-effects
- **Data is Derived**: Our UI and state is a pure function of our data sources

## Pillar 1: Data Flows Down

The first fundamental pillar of reactivity: data flows down. As our apps grow in size, so do their complexity. In apps with dozens of screens and hundreds of components, it can become difficult to reason about relationships between parent and child elements. By enforcing a unidirectional flow of data through our app, we can be sure that a child component renders *purely* as a function of its inputs.

To make our apps interactive, we follow a similar pattern. Components provide two items: *values* and *functions* to modify those values. These items are passed *down* the tree as properties. Dioxus does not provide a way for child components to "reach up" and modify the state of a parent.

The functions we pass down tree are free to mutate state in the *current* component. However, a child component cannot *directly* modify its parent's state. This ensures all mutations to state are defined in the **same scope as the state itself**, making it easier to reason about our app's state at scale.

![Reactivity Tree](/assets/07/reactivity-tree.png)

## Pillar 2: Data is Tracked

The second fundamental pillar of reactivity: data is tracked. When you use various state primitives in Dioxus, the Dioxus runtime *tracks* changes to underlying value. Whenever you call `.set()` or `.write()` on a *Reactive Value*, that operation is *observed* and *effects* are run.

The core Reactive Value in Dioxus is the Signal. When Signals are used in *reactive contexts*, their associated *reads* and *writes* are tracked. Every component is a reactive context. Whenever a Signal's value is modified, a *side-effect* is queued that re-reruns any reactive contexts that read the signal's value.

```rust
#[component]
fn Filter() -> Element {
    let mut selection = use_signal(|| "none");

    rsx! {
        // reading the *selection* value has a side-effect of re-rendering this component
        div { "{selection}" }

        // Calling `.set()` runs side-effects
        button { onclick: move |_| selection.set("dogs"), "Set Filter" }
    }
}
```

Many hooks utilize reactive contexts to attach their own side-effects.

## Pillar 3: Data is Derived

The third pillar of reactivity: values are *derived*. After any state in our app changes, we want the UI to match what we declared in our RSX. In this case, the UI is *derived* from our app state. Similarly, in reactive programming, all data is either a source, or *derived* from a source.

When rendering components, we therefore prefer to perform any transformations of the data while rendering or in a memo. We *do not* modify the data while rendering.

```rust
// ✅ *num_names* is derived from "names"
let names = use_signal(|| vec!["Jane", "Jack", "Jill"]);
let num_names = names.read().len();

// ❌ we do not store num_names in a signal
let names = use_signal(|| vec!["Jane", "Jack", "Jill"]);
let mut num_names = use_signal(|| 0);
num_names.set(names.len());
```

Hopefully, the fact that data should be *derived* is apparent. If we were to modify component state while rendering, we would inadvertently queue re-render side-effects, potentially leading to infinite loops.


## The Big Picture

These three pillars are essential to "thinking reactively" - a mindset required to understand the future chapters. By combining just a few primitives, we have a sophisticated reactivity system that automatically handles changes in our app's state.
