use crate::*;

/// Sample app definition
#[derive(Clone, Copy, PartialEq, Eq)]
struct SampleApp {
    name: &'static str,
    code: &'static str,
}

const SAMPLE_APPS: &[SampleApp] = &[
    SampleApp {
        name: "Counter",
        code: r#"use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "Counter" }
        p { "Count: {count}" }
        button { onclick: move |_| count -= 1, "−" }
        button { onclick: move |_| count += 1, "+" }
    }
}"#,
    },
    SampleApp {
        name: "Todo List",
        code: r#"use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let mut todos = use_signal(|| vec!["Learn Rust", "Build app"]);
    let mut input = use_signal(String::new);

    rsx! {
        h1 { "Todos" }
        input {
            value: "{input}",
            oninput: move |e| input.set(e.value())
        }
        button { onclick: move |_| todos.push(input()), "Add" }
        ul {
            for todo in todos.read().iter() {
                li { "{todo}" }
            }
        }
    }
}"#,
    },
    SampleApp {
        name: "Fetch Data",
        code: r#"use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let mut data = use_resource(|| async move {
        api::fetch_users().await
    });

    rsx! {
        h1 { "Async Data" }
        match data() {
            Some(d) => rsx! { p { "{d}" } },
            None => rsx! { p { "Loading..." } }
        }
    }
}"#,
    },
    SampleApp {
        name: "Server Fn",
        code: r#"use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let user = use_server_future(|| get_user(1))?;

    rsx! {
        h1 { "Server Functions" }
        p { "User: {user}" }
    }
}

#[get("/api/user/{id}")]
async fn get_user(id: i32) -> Result<String> {
    let user = db::get_user(id).await?;
    Ok(user.name)
}
    "#,
    },
    SampleApp {
        name: "Routing",
        code: r#"use dioxus::prelude::*;

fn main() {
    dioxus::launch(rsx! { Router::<Route> {} });
}

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/about")]
    About {},
}

fn Home() -> Element {
    rsx! {
        h1 { "Home" }
        Link { to: Route::About {}, "About" }
    }
}"#,
    },
];

pub(crate) fn Hero() -> Element {
    rsx! {
        // Hero section - full viewport height
        section { class: "relative w-full min-h-screen bg-white dark:bg-gray-950 overflow-visible",
            // Subtle grid background
            div { class: "absolute inset-0 technical-grid" }

            // Main hero content
            div { class: "relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Title section
                div { class: "pt-20 sm:pt-24 md:pt-32 lg:pt-40 pb-16 md:pb-20",
                    // Main headline - clean sans-serif
                    h1 { class: "text-5xl sm:text-6xl md:text-7xl lg:text-7xl font-bold tracking-tight text-gray-900 dark:text-white leading-tight flex flex-row gap-4",
                        div { class: "leading-[1.1]", "One codebase." }
                        div { class: "leading-[1.1] text-orange-500", "Every platform." }
                    }

                    // Subheadline
                    p { class: "mt-8 text-xl md:text-2xl text-gray-500 dark:text-gray-400 max-w-xl",
                        "Build native web, desktop, and mobile apps with Rust."
                    }

                    // Quick actions
                    div { class: "mt-10 flex flex-wrap items-center gap-4",
                        a {
                            href: "/learn",
                            class: "inline-flex items-center gap-2 px-5 py-2.5 text-sm font-medium text-white bg-gray-900 dark:bg-white dark:text-gray-900 hover:bg-gray-700 dark:hover:bg-gray-200 transition-colors",
                            "Get Started"
                            svg {
                                class: "w-4 h-4",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_width: "2",
                                    d: "M9 5l7 7-7 7",
                                }
                            }
                        }
                        a {
                            href: "https://github.com/DioxusLabs/dioxus",
                            class: "inline-flex items-center gap-2 px-5 py-2.5 text-sm font-medium text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors",
                            svg {
                                class: "w-5 h-5",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                path { d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" }
                            }
                            "Star on GitHub"
                        }
                    }
                }

                // Editor section - hangs off viewport
                div { class: "relative", InteractivePlayground {} }
            }

            // Scroll indicator
            div { class: "absolute bottom-6 left-1/2 -translate-x-1/2 text-gray-300 dark:text-gray-700 animate-bounce",
                svg {
                    class: "w-5 h-5",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M19 14l-7 7m0 0l-7-7m7 7V3",
                    }
                }
            }
        }

        // Trusted by section
        TrustedBy {}
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Viewport {
    Desktop,
    Web,
    Mobile,
}

#[component]
fn InteractivePlayground() -> Element {
    let mut selected_idx = use_signal(|| 0usize);
    let mut dropdown_open = use_signal(|| false);
    let mut animating = use_signal(|| false);
    let mut code = use_signal(|| SAMPLE_APPS[0].code.to_string());
    let mut viewport = use_signal(|| Viewport::Desktop);

    let selected_app = SAMPLE_APPS[selected_idx()];
    let line_count = code().lines().count().max(17);

    let mut select_app = move |idx: usize| {
        if idx != selected_idx() {
            animating.set(true);
            // Brief delay for exit animation, then update both index and code
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(150).await;
                selected_idx.set(idx);
                code.set(SAMPLE_APPS[idx].code.to_string());
                gloo_timers::future::TimeoutFuture::new(50).await;
                animating.set(false);
            });
        }
        dropdown_open.set(false);
    };

    rsx! {
        div {
            // Editor window
            div { class: "bg-[#0d1117] border border-gray-800 shadow-2xl overflow-hidden",
                // Window header
                div { class: "flex items-center justify-between px-4 py-3 bg-[#161b22] border-b border-gray-800",
                    div { class: "flex items-center gap-3",
                        // Traffic lights
                        div { class: "flex items-center gap-1.5",
                            div { class: "w-3 h-3 rounded-full bg-[#ff5f57]" }
                            div { class: "w-3 h-3 rounded-full bg-[#febc2e]" }
                            div { class: "w-3 h-3 rounded-full bg-[#28c840]" }
                        }

                        // Dropdown for selecting sample
                        div { class: "relative ml-3",
                            button {
                                class: "flex items-center gap-2 text-sm text-gray-400 hover:text-gray-200 transition-colors",
                                onclick: move |_| dropdown_open.toggle(),
                                span { "{selected_app.name}.rs" }
                                svg {
                                    class: "w-4 h-4",
                                    class: if dropdown_open() { "rotate-180" } else { "" },
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M19 9l-7 7-7-7",
                                    }
                                }
                            }

                            // Dropdown menu
                            if dropdown_open() {
                                div { class: "absolute top-full left-0 mt-2 py-1 bg-ghdarkmetal border border-gray-700 shadow-xl z-50 min-w-40",
                                    for (idx , app) in SAMPLE_APPS.iter().enumerate() {
                                        button {
                                            class: "w-full px-4 py-2 text-left text-sm transition-colors",
                                            class: if idx == selected_idx() { "text-orange-400 bg-orange-500/10" } else { "text-gray-400 hover:text-white hover:bg-gray-800" },
                                            onclick: move |_| select_app(idx),
                                            "{app.name}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Action buttons
                    div { class: "flex items-center gap-2",
                        // Viewport toggle
                        div { class: "hidden lg:flex items-center gap-1 p-1 bg-gray-800/50 border border-gray-700",
                            button {
                                class: "p-1.5 transition-colors",
                                class: if viewport() == Viewport::Desktop { "text-orange-400 bg-gray-700" } else { "text-gray-500 hover:text-gray-300" },
                                onclick: move |_| viewport.set(Viewport::Desktop),
                                title: "Desktop",
                                // Desktop icon (monitor with stand)
                                svg {
                                    class: "w-4 h-4",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
                                    }
                                }
                            }
                            button {
                                class: "p-1.5 transition-colors",
                                class: if viewport() == Viewport::Web { "text-orange-400 bg-gray-700" } else { "text-gray-500 hover:text-gray-300" },
                                onclick: move |_| viewport.set(Viewport::Web),
                                title: "Web",
                                // Globe icon
                                svg {
                                    class: "w-4 h-4",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9",
                                    }
                                }
                            }
                            button {
                                class: "p-1.5 transition-colors",
                                class: if viewport() == Viewport::Mobile { "text-orange-400 bg-gray-700" } else { "text-gray-500 hover:text-gray-300" },
                                onclick: move |_| viewport.set(Viewport::Mobile),
                                title: "Mobile",
                                // Mobile icon
                                svg {
                                    class: "w-4 h-4",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z",
                                    }
                                }
                            }
                        }

                        // Deploy button
                        button { class: "flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-white bg-orange-500 hover:bg-orange-600 transition-colors",
                            svg {
                                class: "w-3.5 h-3.5",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12",
                                }
                            }
                            "Deploy"
                        }
                    }
                }

                // Editor content - side by side
                div { class: "flex flex-col lg:flex-row",
                    // Left side - Code editor
                    div { class: "flex-1 lg:border-r border-gray-800",
                        div { class: "relative h-[600px] lg:h-[700px]",
                            // Line numbers
                            div { class: "absolute left-0 top-0 bottom-0 w-12 bg-[#0d1117] border-r border-gray-800/50 flex flex-col py-4 text-right pr-3 select-none overflow-hidden",
                                for i in 1..=line_count {
                                    span { class: "text-xs text-gray-600 font-mono leading-[1.7]",
                                        "{i}"
                                    }
                                }
                            }
                            textarea {
                                class: "w-full h-full pl-14 pr-4 py-4 bg-transparent text-gray-300 font-mono text-sm overflow-auto leading-[1.7] resize-none focus:outline-none",
                                spellcheck: false,
                                value: "{code}",
                                oninput: move |e| code.set(e.value()),
                            }
                        }
                    }

                    // Right side - Live preview with viewport frame
                    div { class: "flex-1 bg-gray-100 dark:bg-[#0a0a0a] border-t lg:border-t-0 border-gray-800 overflow-hidden",
                        div {
                            class: "h-[600px] lg:h-[700px] p-4 flex items-center justify-center transition-all duration-200",
                            class: if animating() { "opacity-0 scale-95" } else { "opacity-100 scale-100" },

                            // Viewport frame
                            div {
                                class: match viewport() {
                                    Viewport::Desktop => {
                                        "transition-all duration-300 ease-out flex flex-col w-full h-full bg-white dark:bg-[#0d1117] border border-gray-200 dark:border-gray-800 shadow-lg overflow-hidden rounded-xl"
                                    }
                                    Viewport::Web => {
                                        "transition-all duration-300 ease-out flex flex-col w-full h-full bg-white dark:bg-[#0d1117] border border-gray-200 dark:border-gray-800 shadow-lg overflow-hidden rounded-lg"
                                    }
                                    Viewport::Mobile => {
                                        "transition-all duration-300 ease-out flex flex-col w-[320px] h-[640px] bg-white dark:bg-[#0d1117] border-4 border-gray-800 rounded-3xl shadow-2xl overflow-hidden relative"
                                    }
                                },

                                // Desktop: macOS-style traffic lights header
                                if viewport() == Viewport::Desktop {
                                    div { class: "flex items-center gap-2 px-4 py-3 bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700",
                                        div { class: "flex items-center gap-1.5",
                                            div { class: "w-3 h-3 rounded-full bg-[#ff5f57] hover:bg-[#ff5f57]/80 transition-colors" }
                                            div { class: "w-3 h-3 rounded-full bg-[#febc2e] hover:bg-[#febc2e]/80 transition-colors" }
                                            div { class: "w-3 h-3 rounded-full bg-[#28c840] hover:bg-[#28c840]/80 transition-colors" }
                                        }
                                        div { class: "flex-1 text-center text-sm text-gray-500 dark:text-gray-400 font-medium",
                                            "Dioxus App"
                                        }
                                        // Invisible spacer to center title
                                        div { class: "w-12" }
                                    }
                                }

                                // Web: Browser chrome with URL bar
                                if viewport() == Viewport::Web {
                                    div { class: "flex items-center gap-3 px-4 py-2 bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700",
                                        // Navigation buttons
                                        div { class: "flex items-center gap-1",
                                            button { class: "p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                                                svg {
                                                    class: "w-4 h-4",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    view_box: "0 0 24 24",
                                                    path {
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        stroke_width: "2",
                                                        d: "M15 19l-7-7 7-7",
                                                    }
                                                }
                                            }
                                            button { class: "p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                                                svg {
                                                    class: "w-4 h-4",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    view_box: "0 0 24 24",
                                                    path {
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        stroke_width: "2",
                                                        d: "M9 5l7 7-7 7",
                                                    }
                                                }
                                            }
                                            button { class: "p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                                                svg {
                                                    class: "w-4 h-4",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    view_box: "0 0 24 24",
                                                    path {
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        stroke_width: "2",
                                                        d: "M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15",
                                                    }
                                                }
                                            }
                                        }
                                        // URL bar
                                        div { class: "flex-1 flex items-center gap-2 px-3 py-1.5 bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-md",
                                            svg {
                                                class: "w-3.5 h-3.5 text-gray-400",
                                                fill: "none",
                                                stroke: "currentColor",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z",
                                                }
                                            }
                                            span { class: "text-sm text-gray-600 dark:text-gray-400 font-mono",
                                                "myapp.dioxus.dev"
                                            }
                                        }
                                    }
                                }

                                // Mobile notch
                                if viewport() == Viewport::Mobile {
                                    div { class: "absolute top-0 left-1/2 -translate-x-1/2 w-28 h-7 bg-gray-800 rounded-b-2xl z-10" }
                                }

                                // Preview content
                                div {
                                    class: "flex-1 flex items-center justify-center p-6",
                                    class: if viewport() == Viewport::Mobile { "pt-10" } else { "" },
                                    LivePreview { app_type: selected_app.name }
                                }
                            }
                        }
                    }
                }
            }

            // Caption
            p { class: "mt-4 text-center text-sm text-gray-400",
                "Select a sample above to explore Dioxus features."
            }
        }
    }
}

/// Live preview component - renders different demos based on app type
#[component]
fn LivePreview(app_type: &'static str) -> Element {
    match app_type {
        "Counter" => rsx! {
            CounterPreview {}
        },
        "Todo List" => rsx! {
            TodoPreview {}
        },
        "Fetch Data" => rsx! {
            FetchPreview {}
        },
        "Server Fn" => rsx! {
            ServerFnPreview {}
        },
        "Routing" => rsx! {
            RoutingPreview {}
        },
        _ => rsx! {
            CounterPreview {}
        },
    }
}

#[component]
fn CounterPreview() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div { class: "w-full max-w-sm",
            div { class: "text-center p-8",
                h2 { class: "text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-4",
                    "Counter"
                }
                div { class: "text-7xl font-bold text-gray-900 dark:text-white my-10 tabular-nums",
                    "{count}"
                }
                div { class: "flex gap-4 justify-center",
                    button {
                        class: "w-14 h-14 flex items-center justify-center text-2xl font-medium bg-gray-200 dark:bg-gray-800 hover:bg-gray-300 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 transition-colors rounded-lg",
                        onclick: move |_| count -= 1,
                        "−"
                    }
                    button {
                        class: "w-14 h-14 flex items-center justify-center text-2xl font-medium bg-orange-500 hover:bg-orange-600 text-white transition-colors rounded-lg",
                        onclick: move |_| count += 1,
                        "+"
                    }
                }
            }
        }
    }
}

#[component]
fn TodoPreview() -> Element {
    let mut todos = use_signal(|| vec!["Learn Rust".to_string(), "Build app".to_string()]);
    let mut input = use_signal(String::new);

    rsx! {
        div { class: "w-full max-w-sm p-6",
            h2 { class: "text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-6",
                "Todos"
            }
            div { class: "flex gap-3 mb-6",
                input {
                    class: "flex-1 px-4 py-3 text-sm bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 text-gray-900 dark:text-white focus:outline-none focus:border-orange-500 rounded-lg",
                    placeholder: "Add todo...",
                    value: "{input}",
                    oninput: move |e| input.set(e.value()),
                }
                button {
                    class: "px-5 py-3 text-sm font-medium bg-orange-500 hover:bg-orange-600 text-white transition-colors rounded-lg",
                    onclick: move |_| {
                        if !input().is_empty() {
                            todos.push(input());
                            input.set(String::new());
                        }
                    },
                    "Add"
                }
            }
            ul { class: "space-y-3",
                for (i , todo) in todos.read().iter().enumerate() {
                    li {
                        key: "{i}",
                        class: "flex items-center gap-4 p-4 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg",
                        div { class: "w-3 h-3 bg-orange-500 rounded-full" }
                        span { class: "text-sm text-gray-700 dark:text-gray-300", "{todo}" }
                    }
                }
            }
        }
    }
}

#[component]
fn FetchPreview() -> Element {
    let mut loading = use_signal(|| true);
    let mut data = use_signal(|| None::<Vec<&'static str>>);

    // Simulate async fetch
    use_effect(move || {
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(1500).await;
            data.set(Some(vec!["User 1", "User 2", "User 3"]));
            loading.set(false);
        });
    });

    rsx! {
        div { class: "w-full max-w-sm p-6",
            h2 { class: "text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-6",
                "Async Data"
            }
            if loading() {
                div { class: "flex items-center justify-center py-12",
                    div { class: "flex flex-col items-center gap-4",
                        div { class: "w-8 h-8 border-3 border-orange-500 border-t-transparent rounded-full animate-spin" }
                        span { class: "text-sm text-gray-500 dark:text-gray-400", "Loading..." }
                    }
                }
            } else if let Some(users) = data() {
                ul { class: "space-y-3",
                    for user in users {
                        li { class: "flex items-center gap-4 p-4 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg",
                            div { class: "w-10 h-10 bg-linear-to-br from-orange-400 to-pink-500 flex items-center justify-center text-white text-sm font-bold rounded-full",
                                "{user.chars().next().unwrap_or('?')}"
                            }
                            span { class: "text-sm text-gray-700 dark:text-gray-300",
                                "{user}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ServerFnPreview() -> Element {
    let mut user = use_signal(|| None::<&'static str>);
    let mut loading = use_signal(|| false);

    let fetch_user = move |_| {
        loading.set(true);
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(800).await;
            user.set(Some("Alice Johnson"));
            loading.set(false);
        });
    };

    rsx! {
        div { class: "w-full max-w-sm p-6",
            h2 { class: "text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-6",
                "Server Functions"
            }
            div { class: "space-y-5",
                button {
                    class: "w-full px-5 py-4 text-sm font-medium bg-orange-500 hover:bg-orange-600 text-white transition-colors disabled:opacity-50 rounded-lg",
                    disabled: loading(),
                    onclick: fetch_user,
                    if loading() {
                        "Fetching..."
                    } else {
                        "Call Server Function"
                    }
                }
                if let Some(name) = user() {
                    div { class: "p-5 bg-green-500/10 border border-green-500/20 rounded-lg",
                        div { class: "text-xs text-green-600 dark:text-green-400 mb-2",
                            "Response:"
                        }
                        div { class: "text-base font-medium text-gray-900 dark:text-white",
                            "{name}"
                        }
                    }
                }
                div { class: "text-sm text-gray-400 text-center",
                    "Runs on the server, returns to client"
                }
            }
        }
    }
}

#[component]
fn RoutingPreview() -> Element {
    let mut current_route = use_signal(|| "/");

    rsx! {
        div { class: "w-full max-w-sm p-6",
            h2 { class: "text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-6",
                "Routing"
            }
            // Current route indicator
            div { class: "mb-6 px-4 py-2 bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg",
                span { class: "text-xs text-gray-500 dark:text-gray-400", "Current route: " }
                span { class: "text-sm font-mono text-orange-500", "{current_route}" }
            }
            // Page content
            div { class: "p-6 bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg",
                if current_route() == "/" {
                    div {
                        h3 { class: "text-xl font-bold text-gray-900 dark:text-white mb-3",
                            "Home"
                        }
                        p { class: "text-sm text-gray-600 dark:text-gray-400 mb-6",
                            "Welcome to the app!"
                        }
                        button {
                            class: "text-sm text-orange-500 hover:text-orange-600 font-medium",
                            onclick: move |_| current_route.set("/about"),
                            "Go to About →"
                        }
                    }
                } else {
                    div {
                        h3 { class: "text-xl font-bold text-gray-900 dark:text-white mb-3",
                            "About"
                        }
                        p { class: "text-sm text-gray-600 dark:text-gray-400 mb-6",
                            "Built with Dioxus!"
                        }
                        button {
                            class: "text-sm text-orange-500 hover:text-orange-600 font-medium",
                            onclick: move |_| current_route.set("/"),
                            "← Back to Home"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TrustedBy() -> Element {
    rsx! {
        div { class: "border-t border-gray-100 dark:border-gray-800 bg-white dark:bg-gray-950",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
                p { class: "text-center text-sm text-gray-400 mb-10", "Trusted by engineers at" }
                div { class: "flex flex-wrap items-center justify-center gap-x-12 gap-y-8 opacity-40 hover:opacity-60 transition-opacity",
                    img {
                        class: "h-8 dark:invert",
                        src: asset!("/assets/static/airbuslogo.svg"),
                        alt: "Airbus",
                    }
                    img {
                        class: "h-8 dark:invert",
                        src: asset!("/assets/static/ESA_logo.svg"),
                        alt: "ESA",
                    }
                    img {
                        class: "h-8 dark:invert",
                        src: asset!("/assets/static/yclogo.svg"),
                        alt: "Y Combinator",
                    }
                    img {
                        class: "h-8 dark:invert",
                        src: asset!("/assets/static/futurewei_bw.png"),
                        alt: "Futurewei",
                    }
                    img {
                        class: "h-7 dark:invert",
                        src: asset!("/assets/static/satellite.webp"),
                        alt: "Satellite",
                    }
                }
            }
        }
    }
}
