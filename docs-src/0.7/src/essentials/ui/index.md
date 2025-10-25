# Building User Interfaces

Welcome to Dioxus! This first chapter teaches you how to build beautiful user interfaces in Rust and Dioxus.

Dioxus allows you to write user interfaces in HTML and CSS. On the web, your UI is native and accessible, and on desktop and mobile, your widgets are rendered either in a webview or natively with hybrid native components.

HTML and CSS allow you to build very beautiful, rich, interactive experiences. The world's top companies all use HTML and CSS for their sites and applications:

![Animated Homepage](/assets/07/anim-homepage.mp4)

## What Dioxus Gives You

Dioxus gives you all the tools you need to leverage HTML, CSS, and Rust in their full glory.

This includes:

- Hot-reloading for Rust code, UI, styles, and assets
- Cross-platform renderers for web, desktop, and mobile
- Reactivity system for managing and updating state
- Growing SDK of cross-platform APIs
- Backend integration for building fullstack web apps
- Tools for bundling and deploying to production

You can do anything in Dioxus that you could do with HTML and CSS alone.

## What HTML and CSS Gives You

HTML and CSS are powerful technologies (albet, with lots of baggage) that make it easy to build beautiful UIs that work across billions of devices.

HTML and CSS give you lots of tools:

- Multiple layout algorithms (flexbox, grid, table, float, block, etc)
- Stylesheets for app-level theming
- Per-element styling
- Great accessibility and screen-reader support
- Inline SVG support
- CSS Animations
- Multimedia elements like `img`, `video`, and `audio`
- and more!

HTML and CSS might not be the *absolute best* tools for UI work, but they are widely used, work nearly everywhere, and are extremely well documented.

## What Rust Gives You

Why write a UI framework in Rust? Rust was originally intended for systems programming, though we at Dioxus are working on making it suitable for high-level programming like app dev.

Rust has a rather steep learning curve, but the benefits are tremendous:

- Great, standardized tooling (cargo, rustc)
- Huge ecosystem and great package manager (crates.io, Cargo.toml)
- Runs everywhere (mostly) - Mac, Windows, Linux, iOS, Android, Web, etc
- Strong type system that prevents many logic bugs at compile time
- Great performance, rivaling languages like C / C++
- Reliable, even at massive scale

There are drawbacks too, like slower compile times, steep learning curve, and ergonomic papercuts. Our goal is to improve the "high-level Rust" experience to make it on par with languages like TypeScript, but with the added benefits of Rust.

<iframe style="width: 100%" height="500px" class="centered-overflow" src="https://www.youtube.com/embed/Kl90J5RmPxY" title="High-Level Rust" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
