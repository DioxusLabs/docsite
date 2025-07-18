# Building UIs with RSX

Dioxus renders to HTML, if you are not familiar with HTML, this guide will help you get started with the basics. For more detail, the [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML) are a great resource.




## Why RSX and not HTML ?

If you've seen React's JSX or the `html!{}` Rust macro, you might be curious as to why Dioxus chose to use its own syntax instead of a syntax that looks more similar to HTML.

A few reasons:

- RSX gets token coloring and code-folding without additional tooling
- RSX is faster to type since curly braces are auto-closed
- Not all RSX is HTML - Dioxus can be used in non-HTML contexts
- HTML is not valid Rust - not all HTML can be used in html!{}
