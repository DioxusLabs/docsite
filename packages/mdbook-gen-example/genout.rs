
    h1 { id: "roadmap--feature-set",
        a { href: "#roadmap--feature-set", class: "header", "Roadmap & Feature-set" }
    }
    p { "This feature set and roadmap can help you decide if what Dioxus can do today works for you." }
    p {
        "If a feature that you need doesn't exist or you want to contribute to projects on the roadmap, feel free to get involved by "
        a { href: "https://discord.gg/XgGxMSkvUM", "" }
        "."
    }
    p { "Generally, here's the status of each platform:" }
    ul {
        li {
            p {
                strong { "Web" }
                ": Dioxus is a great choice for pure web-apps – especially for CRUD/complex apps. However, it does lack the ecosystem of React, so you might be missing a component library or some useful hook."
            }
        }
        li {
            p {
                strong { "SSR" }
                ": Dioxus is a great choice for pre-rendering, hydration, and rendering HTML on a web endpoint. Be warned – the VirtualDom is not (currently) "
                code { "Send + Sync" }
                "."
            }
        }
        li {
            p {
                strong { "Desktop" }
                ": You can build very competent single-window desktop apps right now. However, multi-window apps require support from Dioxus core and are not ready."
            }
        }
        li {
            p {
                strong { "Mobile" }
                ": Mobile support is very young. You'll be figuring things out as you go and there are not many support crates for peripherals."
            }
        }
        li {
            p {
                strong { "LiveView" }
                ": LiveView support is very young. You'll be figuring things out as you go. Thankfully, none of it is too hard and any work can be upstreamed into Dioxus."
            }
        }
    }
    h2 { id: "features",
        a { href: "#features", class: "header", "Features" }
    }
    hr {}
    table {
        thead {
            th { "Feature" }
            th { "Status" }
            th { "Description" }
        }
        tr {
            th { "Conditional Rendering" }
            th { "✅" }
            th { "if/then to hide/show component" }
        }
        tr {
            th { "Map, Iterator" }
            th { "✅" }
            th { "map/filter/reduce to produce rsx!" }
        }
        tr {
            th { "Keyed Components" }
            th { "✅" }
            th { "advanced diffing with keys" }
        }
        tr {
            th { "Web" }
            th { "✅" }
            th { "renderer for web browser" }
        }
        tr {
            th { "Desktop (webview)" }
            th { "✅" }
            th { "renderer for desktop" }
        }
        tr {
            th { "Shared State (Context)" }
            th { "✅" }
            th { "share state through the tree" }
        }
        tr {
            th { "Hooks" }
            th { "✅" }
            th { "memory cells in components" }
        }
        tr {
            th { "SSR" }
            th { "✅" }
            th { "render directly to string" }
        }
        tr {
            th { "Component Children" }
            th { "✅" }
            th { "cx.children() as a list of nodes" }
        }
        tr {
            th { "Headless components" }
            th { "✅" }
            th { "components that don't return real elements" }
        }
        tr {
            th { "Fragments" }
            th { "✅" }
            th { "multiple elements without a real root" }
        }
        tr {
            th { "Manual Props" }
            th { "✅" }
            th { "Manually pass in props with spread syntax" }
        }
        tr {
            th { "Controlled Inputs" }
            th { "✅" }
            th { "stateful wrappers around inputs" }
        }
        tr {
            th { "CSS/Inline Styles" }
            th { "✅" }
            th { "syntax for inline styles/attribute groups" }
        }
        tr {
            th { "Custom elements" }
            th { "✅" }
            th { "Define new element primitives" }
        }
        tr {
            th { "Suspense" }
            th { "✅" }
            th { "schedule future render from future/promise" }
        }
        tr {
            th { "Integrated error handling" }
            th { "✅" }
            th { "Gracefully handle errors with ? syntax" }
        }
        tr {
            th { "NodeRef" }
            th { "✅" }
            th { "gain direct access to nodes" }
        }
        tr {
            th { "Re-hydration" }
            th { "✅" }
            th { "Pre-render to HTML to speed up first contentful paint" }
        }
        tr {
            th { "Jank-Free Rendering" }
            th { "✅" }
            th { "Large diffs are segmented across frames for silky-smooth transitions" }
        }
        tr {
            th { "Effects" }
            th { "✅" }
            th { "Run effects after a component has been committed to render" }
        }
        tr {
            th { "Portals" }
            th { "🛠" }
            th { "Render nodes outside of the traditional tree structure" }
        }
        tr {
            th { "Cooperative Scheduling" }
            th { "🛠" }
            th { "Prioritize important events over non-important events" }
        }
        tr {
            th { "Server Components" }
            th { "🛠" }
            th { "Hybrid components for SPA and Server" }
        }
        tr {
            th { "Bundle Splitting" }
            th { "👀" }
            th { "Efficiently and asynchronously load the app" }
        }
        tr {
            th { "Lazy Components" }
            th { "👀" }
            th { "Dynamically load the new components as the page is loaded" }
        }
        tr {
            th { "1st class global state" }
            th { "✅" }
            th { "redux/recoil/mobx on top of context" }
        }
        tr {
            th { "Runs natively" }
            th { "✅" }
            th { "runs as a portable binary w/o a runtime (Node)" }
        }
        tr {
            th { "Subtree Memoization" }
            th { "✅" }
            th { "skip diffing static element subtrees" }
        }
        tr {
            th { "High-efficiency templates" }
            th { "✅" }
            th { "rsx! calls are translated to templates on the DOM's side" }
        }
        tr {
            th { "Compile-time correct" }
            th { "✅" }
            th { "Throw errors on invalid template layouts" }
        }
        tr {
            th { "Heuristic Engine" }
            th { "✅" }
            th { "track component memory usage to minimize future allocations" }
        }
        tr {
            th { "Fine-grained reactivity" }
            th { "👀" }
            th { "Skip diffing for fine-grain updates" }
        }
    }
    ul {
        li { "✅ = implemented and working" }
        li { "🛠 = actively being worked on" }
        li { "👀 = not yet implemented or being worked on" }
    }
    h2 { id: "roadmap",
        a { href: "#roadmap", class: "header", "Roadmap" }
    }
    p { "These Features are planned for the future of Dioxus:" }
    h3 { id: "core",
        a { href: "#core", class: "header", "Core" }
    }
    ul {
        li { "[" }
        li { "[" }
        li { "[" }
        li { "[" }
        li { "[" }
        li { "[" }
    }
    h3 { id: "ssr",
        a { href: "#ssr", class: "header", "SSR" }
    }
    ul {
        li { "[" }
        li { "[" }
    }
    h3 { id: "desktop",
        a { href: "#desktop", class: "header", "Desktop" }
    }
    ul {
        li { "[" }
        li { "[" }
        li { "[" }
    }
    h3 { id: "mobile",
        a { href: "#mobile", class: "header", "Mobile" }
    }
    ul {
        li {
            "["
            ul {
                li { "[" }
                li { "[" }
                li { "[" }
                li { "[" }
                li { "[" }
                li { "[" }
                li { "[" }
                li { "[" }
            }
        }
        li { "[" }
    }
    h3 { id: "bundling-cli",
        a { href: "#bundling-cli", class: "header", "Bundling (CLI)" }
    }
    ul {
        li { "[" }
        li { "[" }
        li { "[" }
        li { "[" }
        li { "[" }
        li { "[" }
        li { "[" }
        li { "[" }
        li { "[" }
    }
    h3 { id: "essential-hooks",
        a { href: "#essential-hooks", class: "header", "Essential hooks" }
    }
    ul {
        li { "[" }
        li { "[" }
        li { "[" }
    }
    h2 { id: "work-in-progress",
        a { href: "#work-in-progress", class: "header", "Work in Progress" }
    }
    h3 { id: "build-tool",
        a { href: "#build-tool", class: "header", "Build Tool" }
    }
    p {
        "We are currently working on our own build tool called "
        a { href: "https://github.com/DioxusLabs/dioxus/tree/master/packages/cli",
            ""
        }
        " which will support:"
    }
    ul {
        li { "an interactive TUI" }
        li { "on-the-fly reconfiguration" }
        li { "hot CSS reloading" }
        li { "two-way data binding between browser and source code" }
        li {
            "an interpreter for "
            code { "rsx!" }
        }
        li { "ability to publish to github/netlify/vercel" }
        li { "bundling for iOS/Desktop/etc" }
    }
    h3 { id: "server-component-support",
        a { href: "#server-component-support", class: "header", "Server Component Support" }
    }
    p {
        "While not currently fully implemented, the expectation is that LiveView apps can be a hybrid between Wasm and server-rendered where only portions of a page are \"live\" and the rest of the page is either server-rendered, statically generated, or handled by the host SPA."
    }
    h3 { id: "native-rendering",
        a { href: "#native-rendering", class: "header", "Native rendering" }
    }
    p {
        "We are currently working on a native renderer for Dioxus using WGPU called "
        a { href: "https://github.com/DioxusLabs/blitz/", "" }
        ". This will allow you to build apps that are rendered natively for iOS, Android, and Desktop."
    }