
### Bump allocator


Dioxus is incredibly optimized, both for performance and memory efficiency. Due to Rust's type system and low-level abilities, we can precisely control the performance in ways that JavaScript simply cannot. In some aspects, using Rust with Dioxus is faster than plain JavaScript. With direct access over memory allocation and efficient reuse of strings, we can bypass dynamic memory allocation entirely for components after their initial render.

All `Factory` calls use a bump memory allocator to allocate new objects like Elements, Listeners, Attributes, and even strings. Bump memory allocators are the fastest possible memory allocators - significantly faster than the default allocator used in JavaScript runtimes. The original research in bump allocators for Rust-based UI comes from Dodrio (@fitzgen), a now-archived project that demonstrated the insane speeds of low-level memory control.

![static/Screen_Shot_2021-08-17_at_2.24.39_AM.png](static/Screen_Shot_2021-08-17_at_2.24.39_AM.png)


### Static subtree optimization


Because the rsx! macro is deeply integrated with Dioxus, we can apply extremely aggressive optimizations, pushing performance to the absolute maximum. Every rsx! call will generate a static hash, informing the diffing algorithm if the element needs to be checked after it's been mounted. This means that static substructures in complex components will never need to be diffed, saving many precious clock cycles at runtime. For instance, the "div" element in the below example will never be checked once the component has rendered for the first time. Dioxus will only bother computing the differences of attributes and children that it knows *can* change, like the text contents of the paragraph.

```rust
let val = 10;
rsx!{
	div { 
		style: { background_color: "red" }
		"This is a static subtree"
		h1 {"These elements will not be diffed"}
	}
  p { "This content _can_ change: {val}" }
}
```

### Automatic Memoization and Managed Lifetimes


Dioxus provides a very simple form of memoization for components: if a component's props borrow directly from its parent, it is not memoized. This means that Dioxus is the only UI framework in Rust that lets components borrow data from their parents. 

A memoized component is a component that does not borrow any data from its parent. Either it "copies" or "clones" data from the parent into its own props, or generates new data on the fly. These Props must implement `PartialEq` - where you can safely implement your own memoization strategy.

```rust
static App: FC<()> = |cx| rsx!(in cx, Child { name: format!("world") });

#[derive(Props, PartialEq)]
struct ChildProps {
    name: String,
}
static Child: FC<MyProps> = |cx| rsx!(in cx, div {"Hello, {cx.name}"});
```

For components that are not valid for the `'static` lifetime, they do not need to implement `PartialEq` . This lets you choose between dynamic memory allocation or longer diffs. Just like in React, it's better to avoid memory allocation if you know that a child will always change when its parent changes. 

To make components with props that borrow data, we need to use the regular function syntax, and specify the appropriate lifetime:

```rust
struct ChildProps<'a> {
		name: &'a str
}
fn Child<'a>( ChildProps>) -> Element<'a> {
		rsx!(cx, div {"Hello, {cx.name}"})
}
```

### Cooperative Fiber-Like Scheduling


Rendering with Dioxus is entirely asynchronous and supports the same pause/resume functionality that the new React Fiber rewrite introduced. Internally, Dioxus uses a priority-based scheduler to pause long-running diffs to handle higher-priority work if new events are ready. With the priority scheduler, Dioxus will never block the main thread long enough to cause dropped frames or "jank": event handling is scheduled during idle times and DOM modification is scheduled during animation periods. 

A cool note: Pause/resume uses Rust's efficient Future machinery under the hood, accomplishing the exact same functionality as React fiber with no additional code overhead. 

On top of Pause/Resume, asynchronous rendering enables Suspense, fetch-as-you-render, and even Signals/Streams support.


### Listener Multiplexing / Event delegation


On the web, event delegation is a technique that makes highly-interactive web pages more performant by batching together listeners of the same type. For instance, a single app with more than 100 "onclick" listeners will only have a single listener mounted to the root node. Whenever the listener is triggered, the listener multiplexer will analyze the event and pass it off to the correct listener inside the VirtualDOM. This leads to more performant apps. Dioxus uses an event delegation system, but does not currently have support for proper bubbling or the ability to capture events. Dioxus will always bubble your events (manually) but does not (yet) provide any mechanism to prevent bubbling.


### Patch Batching


When updating the DOM to match your Dioxus component declaration, Dioxus separates its work into two separate phases: diff and commit. During the diff phase, Dioxus will compare an old version of your UI against a new version and figure out exactly which changes need to be made. Unlike other frameworks, Dioxus will not actually modify the DOM during this phase - modifying the DOM is an expensive operation that causes considerable context switching out of "hot codepaths." 

Instead, Dioxus returns a "Mutations" object to the renderer to handle:

```rust
#[derive(Debug)]
pub struct Mutations<'a> {
    pub edits: Vec<DomEdit<'a>>,
    pub noderefs: Vec<NodeRefMutation<'a>>,
}
```

These modifications give the renderer a list of changes that need to be made to modify the real DOM to match the Virtual DOM. Our "DomEdit" type is just a simple enum that can be serialized and sent across the network - making it possible for us to support Liveview and remote clients.

```rust
pub enum DomEdit<'bump> {
    PushRoot { id: u64 },
    PopRoot,
    AppendChildren { many: u32 },
    ReplaceWith { root: u64, m: u32 },
    InsertAfter { root: u64, n: u32 },
		// ....more variants
}
```




### Support for Suspense


Dioxus makes it dead-easy to work with asynchronous values. Simply provide a future to `cx.suspend`, and Dioxus will schedule that future as a task which occurs outside of the regular diffing process. Once the future is complete, that subtree will be rendered. This is a different, more "Rusty" approach to React's "Suspense" mechanism. Because Dioxus uses an asynchronous diffing algorithm, you can easily "fetch as you render." Right now, suspense is very low-level and there aren't a ton of ergonomic hooks built around it. Feel free to make your own!

```rust
#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}
const ENDPOINT: &str = "https://dog.ceo/api/breeds/image/random";

const App: FC<()> = |cx| {
		let req = use_ref(surf::get(ENDPOINT).recv_json::<DogApi>());

		let doggo = cx.suspend(req, |cx, res| match res {
	      Ok(res) => rsx!(in cx, img { src: "{res.message}" }),
		    Err(_) => rsx!(in cx, div { "No doggos for you :(" }),
		});

    rsx!(
        h1 {"Waiting for a doggo..."}
        {doggo}
    ))
};
```




### Built-in coroutines make apps easier to scale


Rust’s async story, while young, does have several highlights. Every future in Rust essentially has a built-in zero-cost AbortHandler - allowing us to pause, start, and restart asynchronous tasks without any additional logic. Dioxus gives full control over these tasks with the use_task hook, making it easy to spawn long-running event loops - essentially coroutines. Coroutines are very popular in game development for their ease of development and high performance even in the largest of apps. In Dioxus, a coroutine would look something like:

```rust
static App: FC<()> = |cx| {
	let websocket_coroutine = use_task(async move {
			let mut socket = connect_to_websocket().await.unwrap();
			while let Some(msg) = socket.recv().await {
					// update our global state
			}
	});
	// render code
};
```

In fact, coroutines allow you to implement true multithreading from within your UI. It's possible to spin up a Tokio/Async_std task or interact with a threadpool from a `use_task` handler.




### Inline styles


A small feature - but welcome one: Dioxus supports inline styles! This gives you 3 ways of styling components: dedicated CSS files, through `style` tags, and inline styles. Here:

```rust
// dedicated file
link { rel: "stylesheet", href: "style.css" }

// style tags
let style = include_str!("style.css");
rsx!(style { "{style}" });

// inline styles
rsx!(div { background_color: "red" });
```

Right now, a dedicated "Style" object is not supported for style merging, but we plan to add it in the future.



### Support for pre-rendering and hydration


As part of the mutation paradigm, Dioxus also supports pre-rendering and hydration of pages  - so you can easily generate your site statically and hydrate it with interactivity once it gets to the client. Rust already runs smoothly on the server, so you can easily build isomorphic apps that achieve amazing Lighthouse scores. Enabling hydration is as simple as:

```rust
// On the server
let pre_rendered = dioxus::ssr::render(App, |cfg| cfg.pre_render(true));

// On the client bundle
dioxus::web::launch(App, |cfg| cfg.hydrate(true));
```


### Support for global shared state


With Dioxus, shared state (React.Context) is simple. We don't need providers, consumers, wrapping components, or anything special. Creating a shared state is as simple as:

```rust
struct SharedState(String);
static App: FC<()> = |cx| {
	// Define it with the dedicated API on Context
	cx.use_create_shared_state(|| SharedState("Hello".to_string()));

	// We can even immediately consume it in the same component
	let my_state = cx.use_shared_state::<SharedState>();
}
```

With Rust's memory safety guarantees and smart pointers, we can share state across the component tree without worrying about accidental mutations or usage errors. 

In fact, Dioxus is shipping with a 1st-class state management solution modeled after Recoil.JS. The name is still in flux, but we're going with `Recoil` for now until we find something better.

In Recoil, shared state is declared with an "Atom" or "AtomFamily". From there, "Selectors" and "SelectorFamilies" make it possible to select, combine, and memoize computations across your app. The two fundamental hooks `use_read` and `use_write` provide an API that matches `use_state` and work across your whole app. The documentation on Recoil is currently very thin, but the API is simple to grok.

```rust
static COUNT: Atom<u32> = |_| 0;

static Incr: FC<()> = |cx| {
    let mut count = use_write(COUNT);
    rsx!(in cx, button { onclick: move |_| count += 1, "increment" })
};

static Decr: FC<()> = |cx| {
    let mut count = use_write(COUNT);
    rsx!(in cx, button { onclick: move |_| count -= 1, "decrement" })
};

static App: FC<()> = |cx| {
    let count = use_read(COUNT);
    rsx!(in cx, "Count is {count}", Incr {}, Decr {})
};
```



### Custom hooks


Just like React, Dioxus supports custom hooks. With the `use_hook` method on `Context` , it's easy to write any new hook. However, unlike React, Dioxus manages the mutability of hook data for you, automatically. Calling `use_hook` is basically just adding a field to a managed "bag of state." This lets you obtain an `&mut T` to any data in use-hook - just like if you were writing a regular struct-based component in other frameworks:

```rust
let counter: &mut u32 = cx.use_hook(|_| 0, |val| val, |_| {});
```



## FAQ:


*"I thought the overhead of Rust to JS makes Rust apps slow?"*

Wasm-bindgen is *just* as fast Vanilla JS, beating out nearly every JS framework in the [framework benchmark](https://krausest.github.io/js-framework-benchmark/2021/table_chrome_91.0.4472.77.html). The biggest bottleneck of Rust interacting with JS is the overhead of translating Rust's UTF-8 strings into JS's UTF-16 strings. Dioxus uses string-interning (caching) and Bloomfilters to effectively circumvent this overhead, meaning even this bottleneck is basically invisible. In fact, Dioxus actually beats the wasm-bindgen benchmark, approaching near vanilla-JS speeds on the JS framework benchmark.

*"Isn't it much more difficult to write Rust than JS?"*

Frankly, the type of code used to write UI is not that complex. When dealing with complex problems, Rust code may end up more complex. React, Redux, and Immer all struggle with mutability issues that Rust naturally prevents. With Rust, it's impossible to accidentally mutate a field, saving developers not only from memory safety bugs, but logic bugs. Plus, we truly believe that Dioxus code will be even easier to write and maintain than its JS counterpart:

```rust
// The Rust-Dioxus version
fn App(cx: Context, props: &()) -> Element {
    let mut count = use_state(|| 0);
    rsx!{
        h1 { "Count: {count}" }
        button { onclick: move |_| count += 1, "+" }
        button { onclick: move |_| count -= 1, "-" }
    })
}

// The TypeScript-React version:
const App: FC<()> = (props) => {
	let [count, set_count] = use_state(0);
	return (
		<>
            <h1> Count: {count} </h1>	
            <button onclick={() => set_count(count + 1)}> "+" </button>
            <button onclick={() => set_count(count - 1)}> "-" </button>			
		</>
	);
};
```

"Doesn't Rust take forever to compile?"

Have you ever used Webpack? 🙂 It's not uncommon for a large Webpack builds to push 3-5 minutes with hot iterations in 20-30 seconds. We've found that Rust's compiler has gotten much faster than it once was, WASM projects have fewer big dependencies, and the new Cranelift backend generates WASM code at blindingly-fast speeds. Smaller WASM projects will compile nearly instantly and the bigger projects might take 5-10 seconds for a hot-reload.

"Aren't Rust binaries too big for the web?"

Dioxus' gzipped "Hello world" clocks in at around 50 kB - more than Preact/Inferno but on par with React. However, WASM code is compiled as it is downloaded (in parallel) - and it compiles incredibly quickly!  By the time the 50 kB is downloaded, the app is already running at full speed. In this way, it's hard to compare WASM and JS; JS needs time to be JIT-ed and cannot be JIT-ed until after it's downloaded completely. Typically, the JIT process takes much longer than the WASM compile process, so it's hard compare kilobytes to kilobytes. 

In short - WASM apps launch just as fast, if not faster, than JS apps - which is typically the main concern around code size.
