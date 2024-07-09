Here is an overview of the state management system in dioxus. This doesn't cover all the internals, but it should serve as a pretty good reference:

# State

Signal is like a fancy version of RefCell<T> for UIs. Just like RefCell, it checks borrows at runtime. It has a bunch of helper methods to make it easier to use. Calling it like a function will clone the inner value. You can also call a few traits like AddAssign on it directly without writing to it manually.

```rust
// create a signal
let signal = use_signal(|| 0);

// update the signal
signal.write() += 1;

// read the signal
signal.read();
```

> Note: Signals are implemented with [generational-box](https://crates.io/crates/generational-box) which makes all values Copy even if the inner value is not Copy.
> This is incredibly convenient for UI development, but it does come with some tradeoffs. The lifetime of the signal is tied to the lifetime of the component it was created in. If you drop the component that created the signal, the signal will be dropped as well. You might run into this if you try to pass a signal from a child component to a parent component and drop the child component. To avoid this you can create your signal higher up in your component tree, use global signals, or create a signal in a specific scope (like the `ScopeId::ROOT`) with [`Signal::new_in_scope`](https://docs.rs/dioxus/latest/dioxus/prelude/struct.Signal.html#method.new_in_scope) 
> 
> TLDR **Don't pass signals up in the component tree**. It will cause issues:
> ```rust
> fn MyComponent() {
>     let child_signal = use_state(|| None);
>     
>     rsx! {
>         IncrementButton {
>             child_signal
>         }
>     }
> }
> 
> #[component]
> fn IncrementButton(mut child_signal: Signal<Option<Signal<i32>>>) {
>     let signal_owned_by_child = use_state(|| 0);
>     // Don't do this: it may cause issues if you drop the child component
>     child_signal.write() = Some(signal_owned_by_child);
> 
>     todo!()
> }
> ```

## Moving Around State

You will often need to move state around between your component. Dioxus provides three different ways to pass round state:

1. Just pass your values as [props](https://dioxuslabs.com/learn/0.5/reference/component_props):
```rust
fn MyComponent() {
    let count = use_state(|| 0);
    
    rsx! {
        IncrementButton {
            count
        }
    }
}

#[component]
fn IncrementButton(mut count: Signal<i32>) {
    rsx! {
        button {
            onclick: move |_| count += 1,
            "Increment"
        }
    }
}
```

This is the most common way to pass state around. It is the most explicit and local to your component. Use this when it isn't overly annoying to pass around a value.

2. Use [use_context](https://dioxuslabs.com/learn/0.5/reference/context) to pass state from a parent component to all children:
```rust
struct MyState {
    count: Signal<i32>
}

fn ParentComponent() {
    // Use context provider provides an unique type to all children of this component
    use_context_provider(|| MyState { count: Signal::new(0) });
    
    rsx! {
        // IncrementButton will have access to the count without explicitly passing it through props
        IncrementButton {}
    }
}

#[component]
fn IncrementButton() {
    // Use context gets the value from a parent component
    let count = use_context::<MyState>().count;
    
    rsx! {
        button {
            onclick: move |_| count += 1,
            "Increment"
        }
    }
}
```

This is slightly less explicit than passing it as a prop, but it is still local to the component. This is really great if you want state that is global to part of your app. It lets you create multiple globalish states while still making state different when you reuse components. If I create a new [`ParentComponent`], it will have a new [`MyState`].

3. Globals let you share state with your whole app with rust statics:
```rust
// Count will be created the first time you access it with the closure you pass to Signal::global
static COUNT: GlobalSignal<i32> = Signal::global(|| 0);

fn ParentComponent() {
    rsx! {
        IncrementButton {}
    }
}

#[component]
fn IncrementButton() {
    rsx! {
        button {
            // You don't need to pass anything around or get anything out of the context because COUNT is global
            onclick: move |_| *COUNT.write() += 1,
            "Increment"
        }
    }
}
```

Global state can be very ergonomic if your state is truly global, but you shouldn't use it if you need state to be different for different instances of your component. If I create another `IncrementButton` it will use the same `COUNT`. Libraries should generally avoid this to make components more reusable.

> Note: Even though it is in a static, `COUNT` will be different for each app instance (this is generally only reliant on the server).

# Derived State

Creating state is only part of the story. A huge part of state management is deriving state from other state. In dioxus, the main way you derive state is through memos. Memos are functions that take state as input and return a new state.

The closure you pass into memos will be called whenever the state you read inside the memo changes, but the memo you get will not rerun other parts of your app unless the output changes (`PartialEq` returns false).

That is a lot, lets dig into some examples to see how this works:

```rust
let count = use_state(|| 1);
// double_count will rerun when state we read inside the memo changes (count)
let double_count = use_memo(move || count() * 2);

// memos act a lot like a read only version of a signal. You can read them, display them, and move them around like any other signal
println!("{}", double_count); // Prints "2"

// But you can't write to them directly
double_count += 1;

// Instead, any time you write to a value the memo reads, the memo will rerun
count += 1;

println!("{}", double_count); // Prints "4"

// Lets create another memo that reads the value of double_count
let double_count_plus_one = use_memo(move || double_count() + 1);

println!("{}", double_count_plus_one); // Prints "5"

// Now if we write to count the double_count memo will rerun
// If that the output of double_count changes, then it will cause double_count_plus_one to rerun
count += 1;

println!("{}", double_count); // Prints "6"
println!("{}", double_count_plus_one); // Prints "7"

// However if the value of double_count doesn't change after a write, then it won't trigger double_count_plus_one to rerun
// Since we just write the same value, the doubled value is still 6 and we don't rerun double_count_plus_one
*double_count.write() = 3;

println!("{}", double_count); // Prints "6"
println!("{}", double_count_plus_one); // Prints "7"
```

That was a lot! In summary, memos let you derive state in your app that updates automatically. They memorize the output of the closure and only rerun other parts of your app when the output changes.

The good news is this is the core of the dioxus reactive system. Memos, Resources, and Effects all rerun in a very similar way. If you have a good grasp of how memos work, understanding the other two will be very easy.

# Derived Async State

Memos are great for deriving synchronous state, but sometimes you need to derive state that is asynchronous. In our previous example, we doubled the value of count. What if that doubling happened on the server? Instead of synchronously calling a function, we would need to start a request to the server and wait for it to finish and return a value.

Lets take a look at what that would look like in dioxus:

```rust
let count = use_state(|| 1);
let double_count = use_resource(move || async move {
    // Start a request to the server. We are reading the value of count to format it into the url
    // Since we are reading count, this resource will "subscribe" to changes to count (when count changes, the resource will rerun)
    let response = reqwest::get(format!("https://myserver.com/doubleme?count={count}")).await.unwrap();
    response.text().await.unwrap()
});

// Again, resources are similar to signals, but they have a bit of extra information. Unlike a memo, the resource may be in progress
// Calling .state() on a resource will return a Signal<UseResourceState> with information about the current status of the resource
println!("{:?}", double_count.state().read()); // Prints "UseResourceState::Pending"

// You can also try to get the last resolved value of the resource with the .value() method
println!("{:?}", double_count.value().read()); // Prints "None"

// Wait for the resource to finish and get the value
std::thread::sleep(std::time::Duration::from_secs(1));

// Now if we read the state, we will see that it is done
println!("{:?}", double_count.state().read()); // Prints "UseResourceState::Done"

// And we can get the value
println!("{:?}", double_count.value().read()); // Prints "Some(2)"

// Now if we write to count, the resource will rerun
count += 1; // count is now 2

// Wait for the resource to finish and get the value
std::thread::sleep(std::time::Duration::from_secs(1));

// Now if we read the state, we will see that it is done
println!("{:?}", double_count.state().read()); // Prints "UseResourceState::Done"

// And we can get the value
println!("{:?}", double_count.value().read()); // Prints "Some(4)"

// One more case, what happens if we write to the resource while it is in progress?
// The resource will rerun and the value will be None
count.write() += 1; // count is now 3

// If we write to a value the resource subscribes to again, it will cancel the current future and start a new one
count += 1; // count is now 4

println!("{:?}", double_count.state().read()); // Prints "UseResourceState::Stopped"
println!("{:?}", double_count.value().read()); // Prints the last resolved value "Some(4)"  

// After we wait for the resource to finish, we will get the value of only the latest future
std::thread::sleep(std::time::Duration::from_secs(1));

println!("{:?}", double_count.state().read()); // Prints "UseResourceState::Done"

println!("{:?}", double_count.value().read()); // Prints "Some(8)"
```

> Note: I made some analogies to memo, but unlike memos, resources do not memorize the output of the closure. They will always rerun any parts of your app that read the value of the resource when the future resolves even if the output doesn't change.

# Effects

When creating an app, you may need to interact with the outside world. This is where effects come in. Effects are reactive closures that run **after the component has finished rendering**. They are useful for things like manually updating the rendered DOM with web-sys or javascript.

**Effects are specifically created for side effects. If you are trying to derive state, use a [memo](#derived-state), or [resource](#derived-async-state) instead.**

If you are trying to update the DOM, you can use the [`use_effect`](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_effect.html) hook to run an effect after the component has finished rendering:

```rust
fn MyComponent() {
    let count = use_state(|| 0);

    use_effect(move || {
        // Effects are reactive just like memos. If you read a value inside the effect, the effect will rerun when that value changes
        let count = count.read();

        // You can use the count value to update the DOM
        eval(format!(
            "document.getElementById('count').innerText = {count}"
        ));
    });

    rsx! {
        button {
            // When you click the button, count will be incremented and the effect will rerun
            onclick: move |_| count += 1,
            "Increment"
        }
        div {
            id: "count",
            // This example has no good reason to manually manipulate the DOM, but if you do, you should do that in an effect
            // In real code, this should just be "{count}" and dioxus will update it automatically
            "0"
        }
    }
}
```

# Conclusion

Signals, memos, and resources are the core of state management in dioxus.

Signals store state, memos derive synchronous state, and resources derive asynchronous state.

Effects are used to run side effects after the component has finished rendering.

You can learn more about state management in dioxus in the [dioxus book](https://dioxuslabs.com/learn/0.5/reference).

If you want to see a more complex example of how state management comes together, check out the [todomvc example](https://github.com/DioxusLabs/dioxus/blob/main/examples/todomvc.rs).

If you have any questions, feel free to ask in the [dioxus discord](https://discord.gg/XgGxMSkvUM).
