this file is a dumping ground for old docs that might be useful in the future

## Why RSX and not HTML ?

If you've seen React's JSX or the `html!{}` Rust macro, you might be curious as to why Dioxus chose to use its own syntax instead of a syntax that looks more similar to HTML.

A few reasons:

- RSX gets token coloring and code-folding without additional tooling
- RSX is faster to type since curly braces are auto-closed
- Not all RSX is HTML - Dioxus can be used in non-HTML contexts
- HTML is not valid Rust - not all HTML can be used in html!{}
