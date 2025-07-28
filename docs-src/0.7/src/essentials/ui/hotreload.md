# Dioxus Hot-Reloading Reference

The Dioxus Hot-Reload engine is incredibly powerful and fast. When used properly, it provides the fastest development experience for building apps with Rust.

Dioxus 0.5 featured a limited form of hot-reloading, while Dioxus 0.6 drastically improved it. **Dioxus 0.7 introduces experimental Rust hot-reloading**, making it the first Rust framework to support hot-reloading of actual Rust code.

Dioxus provides three forms of hotreloading.

Element hot-reloading:

![Element Hot-reloading](/assets/07/dog_app_hotreload.mp4)

Asset hot-reloading:

![Element Hot-reloading](/assets/07/css-hotreload.mp4)

Rust hot-reloading:

![Element Hot-reloading](/assets/07/subsecond-tui.mp4)

We provide this text guide as a resource for the details of hot-reloading. This guide also has an accompanying video as well:

<iframe style="width: 100%" height="500px" class="centered-overflow" src="https://www.youtube.com/embed/Q4Xzz8OJEoc" title="Dioxus 0.6" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## RSX Hot-Reload

RSX hot-reloading is fundamental to the Dioxus development experience. It allows you to modify the structure, styling, and content of your user interface without recompiling your entire application.

### What Can Be Hot-Reloaded

**Elements and Structure:**
```rust
fn App() -> Element {
    rsx! {
        // You can add, remove, or modify any elements
        div { class: "container" }
        h1 { "My App" }
        button { "Click me" }
    }
}
```

**String Attributes:**
All string-based attributes support instant hot-reloading:
```rust
fn Button() -> Element {
    rsx! {
        button {
            // All of these can be hot-reloaded instantly
            class: "btn btn-primary",
            id: "my-button",
            title: "Click this button",
            style: "background: blue; color: white;",

            "Submit"
        }
    }
}
```

**Formatted Strings:**
Hot-reload supports complex formatted strings anywhere in your RSX:
```rust
fn Counter() -> Element {
    let count = use_signal(|| 0);

    rsx! {
        div {
            // Formatted strings in text content
            "Count: {count}"

            button {
                // Formatted strings in attributes
                class: "btn-{if count() > 5 { "large" } else { "small" }}",
                onclick: move |_| count += 1,

                // Formatted strings in button text
                "Increment ({count})"
            }
        }
    }
}
```

**Component Properties:**
You can hot-reload simple Rust expressions passed as component props:
```rust
fn App() -> Element {
    rsx! {
        MyButton {
            // These simple props can be hot-reloaded
            text: "Click me",
            enabled: true,
            color: "blue"
        }
    }
}

#[component]
fn MyButton(text: String, enabled: bool, color: String) -> Element {
    rsx! {
        button { class: "btn-{color}", disabled: !enabled, "{text}" }
    }
}
```

**Nested RSX Blocks:**
Hot-reloading works inside conditional blocks, loops, and component children:
```rust
fn TodoList() -> Element {
    let todos = use_signal(|| vec!["Learn Dioxus", "Build an app"]);

    rsx! {
        ul {
            // This entire loop body can be hot-reloaded
            for (i, todo) in todos().iter().enumerate() {
                li {
                    key: "{i}",
                    class: "todo-item",
                    "{todo}"
                }
            }
        }

        if todos().len() > 5 {
            // Conditional blocks are hot-reloadable too
            div { class: "warning", "You have many todos!" }
        }
    }
}
```

### What Requires a Full Rebuild

While RSX hot-reloading is very capable, some changes still require a full application rebuild:

- **New variables or expressions** not present in the last compilation
- **Logic changes** outside of RSX (function bodies, hooks, etc.)
- **Component signatures** (adding/removing props)
- **Import statements** and module structure
- **Complex Rust expressions** in attributes that involve function calls

### Performance

RSX hot-reloading is extremely fast:
- **Sub-second updates** for most changes
- **Preserves application state** (signals, context, etc.)
- **Works across all platforms** (web, desktop, mobile)
- **Batches multiple changes** for efficient updates

## Asset Hot-Reload

Asset hot-reloading allows you to modify CSS, images, and other static files without rebuilding your application. This works seamlessly with the `asset!()` macro system.

### CSS Hot-Reloading

CSS files are automatically watched and hot-reloaded when changed:

```rust
// In your Rust code
static STYLES: Asset = asset!("/assets/main.css");

fn App() -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        div { class: "my-component", "Hello World!" }
    }
}
```

```css
/* In assets/main.css - changes here are instantly applied */
.my-component {
    background: blue;
    color: white;
    padding: 20px;
    /* Change this to red and see instant updates! */
}
```

### SCSS/Sass Support

SCSS files are automatically compiled and hot-reloaded:

```rust
static STYLES: Asset = asset!("/assets/styles.scss");
```

```scss
// In assets/styles.scss
$primary-color: #3b82f6;

.button {
    background: $primary-color;

    &:hover {
        background: darken($primary-color, 10%);
    }
}
```

### Image and Static Asset Hot-Reloading

Static assets like images also support hot-reloading:

```rust
static LOGO: Asset = asset!("/assets/logo.png");

fn Header() -> Element {
    rsx! {
        img {
            src: LOGO,
            alt: "Logo",
            class: "logo"
        }
    }
}
```

When you replace `/assets/logo.png` with a new image, the change appears instantly in your running application.

### Asset Processing

The hot-reload system automatically handles:
- **CSS minification** and optimization
- **SCSS compilation** with source maps
- **Asset hashing** for cache busting
- **Image optimization** when configured
- **Cross-platform bundling** for desktop and mobile

### Tailwind CSS Integration

Tailwind CSS works perfectly with hot-reloading:

1. **Set up Tailwind** with the `@import "tailwindcss"` directive
2. **Run the Tailwind compiler** with `--watch`
3. **Use Tailwind classes** in your RSX
4. **See instant updates** when you modify classes

```rust
fn Button() -> Element {
    rsx! {
        button {
            // Add/remove/modify these classes for instant updates
            class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
            "Click me"
        }
    }
}
```

## Experimental: Rust Hot-Reload

**New in Dioxus 0.7**, you can enable experimental Rust code hot-reloading using the `--hotpatch` flag. This feature is revolutionary - allowing you to modify Rust logic and see changes instantly without rebuilding.

### Enabling Rust Hot-Reload

```bash
# Enable experimental Rust hot-reloading
dx serve --hotpatch

# For specific platforms
dx serve --webview --hotpatch
dx serve --web --hotpatch
```

### What Can Be Hot-Patched

The "subsecond" hot-patching system can reload many types of Rust code changes:

**Function Bodies:**
```rust
fn calculate_total(items: &[Item]) -> f64 {
    // You can modify this logic and see instant updates
    items.iter()
        .map(|item| item.price * item.quantity)
        .sum()
}
```

**Component Logic:**
```rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);

    // Modify this event handler logic instantly
    let increment = move |_| {
        count += 1;
        // Add logging, validation, etc. without rebuilding
        println!("Count is now: {}", count());
    };

    rsx! {
        button { onclick: increment, "Count: {count}" }
    }
}
```

**Hook Implementations:**
```rust
fn use_local_storage(key: &str, default: String) -> Signal<String> {
    let mut value = use_signal(|| default);

    // Modify hook logic with hot-patching
    use_effect(move || {
        // Change storage logic instantly
        if let Ok(stored) = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item(key)
        {
            if let Some(stored) = stored {
                value.set(stored);
            }
        }
    });

    value
}
```

### Platform Support

Rust hot-reloading works on:
- ✅ **Web** (all browsers)
- ✅ **Desktop** (Windows, macOS, Linux)
- ✅ **Mobile Android**
- ❌ **iOS** (not supported due to platform limitations)

### Performance: "Subsecond" Hot-Reloading

The hot-patching system is incredibly fast:
- **Under 100ms** for most code changes
- **Preserves all application state**
- **No compilation artifacts** to manage
- **Incremental patching** of only changed functions

### Limitations

While experimental Rust hot-reloading is powerful, it has some current limitations:

**Not Supported:**
- Changing function signatures
- Adding/removing struct fields
- Modifying type definitions
- Changing macro expansions
- Adding new dependencies

**Experimental Status:**
- May not work with all code patterns
- Could have edge cases or stability issues
- API may change in future releases
- Best effort compatibility with different Rust features

### How It Works

The hot-patching system uses advanced techniques:

1. **Binary Analysis** - Analyzes your compiled binary
2. **Function Isolation** - Identifies hot-patchable functions
3. **Runtime Patching** - Patches running code in memory
4. **State Preservation** - Maintains all application state during patches

This makes Dioxus the first major Rust framework to support true hot-reloading of Rust code!

### Tips for Best Results

**Structure for Hot-Reloading:**
```rust
// Good: Logic isolated in functions
fn business_logic(input: &str) -> String {
    // This entire function can be hot-patched
    input.to_uppercase()
}

#[component]
fn MyComponent(text: String) -> Element {
    let processed = business_logic(&text);

    rsx! {
        div { "{processed}" }
    }
}
```

**Avoid Hot-Patch Barriers:**
```rust
// Avoid: Complex generic constraints
fn complex_generic<T: Clone + Debug + Send + 'static>(value: T) -> T {
    // May not hot-patch reliably
    value
}

// Better: Simple, concrete functions
fn process_string(value: String) -> String {
    // Hot-patches reliably
    value.trim().to_string()
}
```

### Future Development

The experimental Rust hot-reloading will continue to improve:
- **Broader compatibility** with Rust language features
- **Better error handling** and diagnostics
- **Performance optimizations**
- **iOS support** when platform allows
- **Production debugging** capabilities

Try it out with `dx serve --hotpatch` and experience the future of Rust development!
