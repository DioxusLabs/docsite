# Dioxus Documentation Style Guide

This guide captures the voice, tone, and conventions used in the Dioxus documentation. Follow these guidelines to write documentation that feels consistent with the existing body of work.

---

## Voice and Perspective

### Use "we" for the Dioxus project, "you" for the reader

- **"We"** represents the Dioxus team, the project's philosophy, or shared understanding
- **"You"** directly addresses the reader

```
✓ We built Dioxus because we believe the current standard of building apps is too complex.
✓ If you're coming to Dioxus as an experienced web developer, we hope you'll feel comfortable rather quickly.
✓ You can think of state management in Dioxus as a hybrid of React and SolidJS.
```

Never use "I" or refer to the documentation itself ("this document will show you...").

### Be direct but not terse

Write in active voice. State things plainly. Avoid hedging language like "might," "perhaps," or "it could be said that."

```
✗ It might be worth noting that hooks should be called at the top level.
✓ You should only call hooks at the top level of a component.

✗ One could potentially use the `use_signal` hook for this purpose.
✓ Use `use_signal` for reactive state.
```

### Be honest about trade-offs and limitations

Don't oversell. Acknowledge complexity when it exists. Readers trust documentation that's straightforward about downsides.

```
✓ Rust has a rather steep learning curve, but the benefits are tremendous.
✓ Unfortunately, it's not very beautiful yet.
✓ Managing this cycle is easy at first, but can become more challenging as apps grow.
✓ There are drawbacks too, like slower compile times, steep learning curve, and ergonomic papercuts.
```

---

## Sentence Structure

### Keep sentences medium-length

Aim for 12-20 words per sentence. Vary length for rhythm, but avoid both choppy fragments and run-on sentences.

```
✓ Dioxus is a framework for building cross-platform apps with the Rust programming language. With one codebase, you can build apps that run on web, desktop, and mobile platforms.
```

### Lead with the main point

Put the important information first. Don't bury the action in subordinate clauses.

```
✗ If you want to build apps that run on multiple platforms, Dioxus provides a framework for doing so.
✓ Dioxus is designed to be cross-platform by default.
```

### Use contrast patterns for comparisons

When comparing to other frameworks, use "Unlike X..." or "Just like X..." constructions:

```
✓ Unlike SolidJS, Dioxus does not do compile-time transformation of your state.
✓ Unlike Svelte, Dioxus does not do compile-time transformation of your state.
✓ Just like React, Dioxus will call your component function multiple times throughout its lifecycle.
```

---

## Paragraph Structure

### Start with a clear topic sentence

Each paragraph should open with its main point. The rest elaborates or provides examples.

```
✓ In Dioxus, state that is local to a component is stored in *hooks*.

  Dioxus hooks work similarly to React's hooks. If you haven't done much web development, hooks might seem particularly unusual...
```

### Keep paragraphs focused

3-6 sentences per paragraph. If you're covering multiple ideas, split them into separate paragraphs.

### Use single-sentence paragraphs for emphasis

When a point is critical, let it stand alone:

```
✓ **Managing state is, by far, the hardest part of building an app.**
```

---

## Headers and Organization

### Use descriptive, action-oriented headers

Headers should tell readers what they'll learn or do, not just label content.

```
✓ ## What will we be learning?
✓ ## Adding UI to our HotDog App
✓ ## Rules of Hooks
✓ ## For Experienced Web Developers
```

### Provide multiple entry points

Different readers have different backgrounds. Create distinct sections for different audiences when explaining core concepts:

```
## For Experienced Web Developers
If you're coming to Dioxus as an experienced web developer, we hope you'll feel comfortable rather quickly...

## For Experienced Rustaceans
If you're coming to Dioxus as an experienced Rustacean, you might be intimidated by our use of "unusual" primitives...
```

### Use progressive disclosure

Start simple, add complexity gradually. Don't front-load all the caveats and edge cases.

---

## Code Examples

### Show code frequently

Include a code example every 150-200 words of prose. Readers learn by seeing working code.

### Keep examples short and focused

5-20 lines is ideal. If an example exceeds 30 lines, consider whether it can be simplified or split.

### Use inline code for technical terms

Backticks around:
- Function and hook names: `use_signal`, `use_effect`, `spawn`
- Macros: `rsx!`, `#[component]`
- Methods: `.read()`, `.write()`, `.clone()`
- Types: `Signal`, `Element`, `Props`
- File names: `Cargo.toml`, `main.rs`
- Terminal commands: `dx serve`

### Introduce code, don't just drop it in

Provide context before code blocks. After the code, explain what matters:

```
✓ The `use_hook` primitive is a function that takes an initializer and returns a `.clone()` of the value.

  ```rust
  fn Simple() -> Element {
      let count = use_hook(|| 123);
      rsx! { "{count}" }
  }
  ```

  Whenever `use_hook` is called, one of two things happens...
```

### Use the include syntax for external examples

Reference runnable examples from the doc_examples directory:

```
{{#include ../docs-router/src/doc_examples/guide_rsx.rs:rsx_is_html}}
```

---

## Explaining Concepts

### Theory before practice

Introduce the concept, explain why it exists, then show how to use it:

```
✓ ## The Theory of State Management

  Ultimately, "state management" refers to the act of:
  1. Initializing data for the UI
  2. Handling events from the user
  3. Updating the data and re-rendering the UI

  [Then show code examples]
```

### Compare to familiar concepts

Reference React, SolidJS, Svelte, or Flutter when the analogy helps. Many readers come from these ecosystems:

```
✓ State management in Dioxus was heavily inspired by projects like React, Preact, SolidJS, and Svelte.
✓ Dioxus uses signal-based reactivity.
✓ You can think of Dioxus as a hybrid of Flutter, React Native, and NextJS.
```

### Explain the "why" before the "how"

Don't just document behavior—explain the reasoning:

```
✓ In Dioxus, we are transparent with the inner workings of the framework. Because hooks are implemented by walking an internal "hook list," they have certain rules that would cause walking the list to fail...
```

---

## Common Patterns and Phrases

### Transitional phrases

Use these to guide readers through the material:

- "Now that you know how to..."
- "Let's move on to..."
- "At this point, we should..."
- "Before we get too far..."
- "Our app is coming together!"

### Recommendation phrases

When suggesting best practices:

- "We recommend..."
- "We strongly recommend..."
- "We generally discourage this pattern since..."
- "Consider..."
- "Instead, prefer to..."

### Cross-references

Link to related content with clear intent:

```
✓ You can read more about Event Handlers in the [Event Handler reference](../essentials/basics/event_handlers.md).
✓ For an in-depth guide, check the [hot-reload guide](../essentials/ui/hotreload.md).
```

---

## Formatting Conventions

### Bold for critical points

Use bold sparingly for genuinely important warnings or key concepts:

```
✓ **Managing state is, by far, the hardest part of building an app.**
✓ **you should only call hooks at the top level of a component or another hook**
```

### Italics for terminology and emphasis

Use italics to introduce terms or add subtle emphasis:

```
✓ Dioxus is a _declarative_ framework.
✓ In Dioxus, state that is local to a component is stored in *hooks*.
✓ Unlike SolidJS, *reads* and *writes* of value are **explicit**.
```

### Numbered lists for sequences

Use numbered lists when order matters—steps, processes, or ranked items:

```
✓ Ultimately, "state management" refers to the act of:
  1. Initializing data for the UI
  2. Handling events from the user
  3. Updating the data and re-rendering the UI
```

### Bullet lists for collections

Use bullets for unordered groups of features, options, or points:

```
✓ The features of HotDog are fairly simple:
  - Engage with a stream of cute dog photos
  - Swipe right if we want to save the dog photo
  - Swipe left if we don't want to save the dog photo
```

### Checkmarks and X marks for dos and don'ts

In code examples showing correct vs. incorrect patterns:

```rust
// ❌ don't early return between hooks!
if name() == "jack" {
    return Err("wrong name".into())
}

// ✅ instead, prefer to early return *after* all hook functions are run
```

---

## Tutorial-Specific Conventions

### Use a consistent example project

The tutorial uses "HotDog" (Tinder for dogs) throughout. When writing tutorials, maintain a single coherent example app.

### Show progress visually

Include screenshots and videos at key milestones:

```
✓ ![Dog App Hotreloading](/assets/06_docs/dog_app_hotreload.mp4)
✓ Our app is coming together!
  ![Unstyled Dog App](/assets/06_docs/unstyled_dog_app.png)
```

### End sections with forward momentum

Close sections by pointing to what's next:

```
✓ Unfortunately, it's not very beautiful yet. Let's move on to [styling our app](assets.md).
```

---

## Technical Depth

### Assume Rust familiarity

Don't explain Rust basics. Link to external resources for readers who need them:

```
✓ This guide assumes you already know some Rust!
```

### Don't over-explain simple things

If it's obvious to a Rust developer, don't belabor it. Trust your reader's competence.

### Explain Dioxus-specific behavior thoroughly

When something works differently than readers might expect, take time to explain:

```
✓ Rust does not have an equivalent to JavaScript's Proxy, so reactivity is traced by calls to `.read()` and `.write()`.
```

---

## What to Avoid

### No filler phrases

Cut these:
- "It's worth noting that..."
- "It should be mentioned that..."
- "As you can see..."
- "Obviously..."
- "Simply..."

### No excessive enthusiasm

Avoid:
- "Exciting new feature!"
- "Amazing capabilities!"
- "You'll love this!"

### No apologies or self-deprecation about the documentation

Don't write:
- "This section is a work in progress"
- "Sorry if this is confusing"
- "We'll try to explain this better"

### No passive voice where active works

```
✗ The component will be re-rendered by Dioxus.
✓ Dioxus will re-render the component.
```

### No time estimates

Don't promise how long things take. What's "quick" for one reader isn't for another.

---

# No AI slop!

no AI slop! don't engage in common Ai-isms like emdashes, "this, not that", or anything that comes across as sychophantic. this is technical documentation writing.

---

## Quick Reference

| Aspect | Guideline |
|--------|-----------|
| Voice | First-person plural "we" for project, second-person "you" for reader |
| Tone | Professional, direct, honest about trade-offs |
| Sentences | Medium length (12-20 words), active voice |
| Paragraphs | 3-6 sentences, clear topic sentence |
| Code examples | Every 150-200 words, 5-20 lines |
| Inline code | All technical terms, function names, types |
| Bold | Critical warnings and key concepts only |
| Italics | Introducing terms, subtle emphasis |
| Comparisons | Reference React, SolidJS, Svelte, Flutter when helpful |
| Structure | Theory before practice, simple before complex |
