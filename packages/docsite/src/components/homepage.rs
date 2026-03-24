use crate::docs::router_07::BookRoute;
use crate::*;

pub(crate) mod featured_examples;
pub(crate) mod hero;
pub(crate) mod snippets;
use hero::Hero;

#[component]
pub(crate) fn Homepage() -> Element {
    rsx! {
        div { class: "w-full",
            Hero {}
            Features {}
            HotReloadShowcase {}
            PlatformShowcase {}
            Stats {}
            CallToAction {}
        }
    }
}

/// Features section highlighting key capabilities
#[component]
fn Features() -> Element {
    rsx! {
        section { class: "py-20 md:py-28 bg-gray-50 dark:bg-gray-950 border-t border-gray-200 dark:border-gray-800",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Section header
                div { class: "text-center max-w-3xl mx-auto mb-16",
                    h2 { class: "text-3xl sm:text-4xl md:text-5xl font-bold text-gray-900 dark:text-white mb-4",
                        "Everything you need to "
                        span { class: "text-orange-500", "ship fast" }
                    }
                    p { class: "text-lg text-gray-600 dark:text-gray-400",
                        "Dioxus combines the best of React, Solid, and Svelte with the power and safety of Rust."
                    }
                }

                // Feature grid
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 lg:gap-8",
                    FeatureCard {
                        icon: rsx! {
                            svg {
                                class: "w-6 h-6",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M13 10V3L4 14h7v7l9-11h-7z",
                                }
                            }
                        },
                        title: "Instant Hot Reload",
                        description: "Edit your code and see changes instantly. RSX, assets, and even Rust logic update in milliseconds without losing state.",
                        color: "orange",
                    }
                    FeatureCard {
                        icon: rsx! {
                            svg {
                                class: "w-6 h-6",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01",
                                }
                            }
                        },
                        title: "True Fullstack",
                        description: "Server functions, streaming SSR, static generation, and hydration. Build complete apps with a single, unified codebase.",
                        color: "blue",
                    }
                    FeatureCard {
                        icon: rsx! {
                            svg {
                                class: "w-6 h-6",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",
                                }
                            }
                        },
                        title: "Type Safe",
                        description: "Catch errors at compile time. No runtime surprises. Rust's type system ensures your UI logic is bulletproof.",
                        color: "green",
                    }
                    FeatureCard {
                        icon: rsx! {
                            svg {
                                class: "w-6 h-6",
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
                        },
                        title: "Cross Platform",
                        description: "Web, desktop, mobile, and more. Write once, deploy everywhere with native performance on each platform.",
                        color: "purple",
                    }
                    FeatureCard {
                        icon: rsx! {
                            svg {
                                class: "w-6 h-6",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M13 7h8m0 0v8m0-8l-8 8-4-4-6 6",
                                }
                            }
                        },
                        title: "Blazingly Fast",
                        description: "Optimized virtual DOM, fine-grained reactivity, and zero-cost abstractions. Performance that rivals native apps.",
                        color: "red",
                    }
                    FeatureCard {
                        icon: rsx! {
                            svg {
                                class: "w-6 h-6",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z",
                                }
                            }
                        },
                        title: "Developer Joy",
                        description: "Beautiful APIs, great docs, and first-class tooling. Building apps should be fun, not frustrating.",
                        color: "pink",
                    }
                }
            }
        }
    }
}

#[component]
fn FeatureCard(
    icon: Element,
    title: &'static str,
    description: &'static str,
    color: &'static str,
) -> Element {
    let icon_color = match color {
        "orange" => "text-orange-500",
        "blue" => "text-blue-400",
        "green" => "text-green-400",
        "purple" => "text-purple-400",
        "red" => "text-red-400",
        "pink" => "text-pink-400",
        _ => "text-gray-400",
    };

    rsx! {
        div { class: "group relative p-6 bg-white dark:bg-ghdarkmetal border border-gray-200 dark:border-gray-800 hover:border-gray-300 dark:hover:border-gray-600 transition-colors",
            div { class: "flex items-start gap-4",
                div { class: "flex-shrink-0 w-10 h-10 flex items-center justify-center bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 {icon_color}",
                    {icon}
                }
                div {
                    h3 { class: "text-lg font-semibold text-gray-900 dark:text-white mb-2",
                        "{title}"
                    }
                    p { class: "text-gray-600 dark:text-gray-400 leading-relaxed text-sm",
                        "{description}"
                    }
                }
            }
        }
    }
}

/// Hot reload showcase section
#[component]
fn HotReloadShowcase() -> Element {
    rsx! {
        section { class: "py-20 md:py-28 bg-white dark:bg-black border-t border-gray-200 dark:border-gray-800",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "grid lg:grid-cols-2 gap-12 lg:gap-16 items-center",
                    // Content
                    div {
                        div { class: "inline-flex items-center gap-2 px-3 py-1 bg-orange-500/10 border border-orange-500/20 mb-6",
                            span { class: "text-sm font-medium text-orange-500",
                                "Instant Feedback"
                            }
                        }
                        h2 { class: "text-3xl sm:text-4xl md:text-5xl font-bold text-gray-900 dark:text-white mb-6",
                            "Hot reload "
                            span { class: "text-orange-500", "everything" }
                        }
                        p { class: "text-lg text-gray-600 dark:text-gray-400 mb-8 leading-relaxed",
                            "Change your RSX markup, update styles, modify assets, and even tweak Rust logic. Watch your app update instantly without losing state. The fastest iteration cycle in the Rust ecosystem."
                        }
                        ul { class: "space-y-4",
                            HotReloadFeature { text: "RSX and styling changes in under 50ms" }
                            HotReloadFeature { text: "Preserves component state across reloads" }
                            HotReloadFeature { text: "Works on web, desktop, and mobile" }
                            HotReloadFeature { text: "Binary patching for Rust logic changes" }
                        }
                    }

                    // Visual demo - terminal style
                    div { class: "relative",
                        div { class: "bg-[#0d1117] border border-gray-800 shadow-2xl",
                            // Terminal header
                            div { class: "flex items-center gap-2 px-4 py-3 bg-ghdarkmetal border-b border-gray-800",
                                div { class: "flex gap-1.5",
                                    div { class: "w-3 h-3 rounded-full bg-[#ff5f57]" }
                                    div { class: "w-3 h-3 rounded-full bg-[#febc2e]" }
                                    div { class: "w-3 h-3 rounded-full bg-[#28c840]" }
                                }
                                span { class: "ml-3 text-sm text-gray-500", "dx serve" }
                            }
                            // Terminal content
                            div { class: "p-4 font-mono text-sm space-y-2",
                                div { class: "text-green-400", "Dioxus v0.7.0" }
                                div { class: "text-gray-400", "Hot reload enabled" }
                                div { class: "text-gray-600", "---" }
                                div { class: "flex items-center gap-2",
                                    span { class: "text-blue-400", "info:" }
                                    span { class: "text-gray-300", "Listening on http://127.0.0.1:8080" }
                                }
                                div { class: "flex items-center gap-2",
                                    span { class: "text-orange-400", "hot:" }
                                    span { class: "text-gray-300", "Reloaded app.rs in " }
                                    span { class: "text-green-400 font-bold", "23ms" }
                                }
                                div { class: "flex items-center gap-2",
                                    span { class: "text-orange-400", "hot:" }
                                    span { class: "text-gray-300", "Reloaded styles.css in " }
                                    span { class: "text-green-400 font-bold", "8ms" }
                                }
                                div { class: "flex items-center gap-2 animate-pulse",
                                    span { class: "text-gray-500", "Watching for changes..." }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HotReloadFeature(text: &'static str) -> Element {
    rsx! {
        li { class: "flex items-center gap-3",
            div { class: "flex-shrink-0 w-5 h-5 bg-green-500/10 flex items-center justify-center",
                svg {
                    class: "w-3 h-3 text-green-400",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "3",
                        d: "M5 13l4 4L19 7",
                    }
                }
            }
            span { class: "text-gray-700 dark:text-gray-300", "{text}" }
        }
    }
}

/// Platform showcase section
#[component]
fn PlatformShowcase() -> Element {
    rsx! {
        section { class: "py-20 md:py-28 bg-gray-50 dark:bg-gray-950 border-t border-gray-200 dark:border-gray-800",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div { class: "text-center max-w-3xl mx-auto mb-16",
                    h2 { class: "text-3xl sm:text-4xl md:text-5xl font-bold text-gray-900 dark:text-white mb-4",
                        "One codebase, "
                        span { class: "text-orange-500", "every platform" }
                    }
                    p { class: "text-lg text-gray-600 dark:text-gray-400",
                        "Build for web, desktop, and mobile from a single Rust codebase. No platform-specific code required."
                    }
                }

                // Platform cards
                div { class: "grid md:grid-cols-3 gap-6",
                    PlatformCard {
                        platform: "Web",
                        description: "WASM-powered web apps with SSR, hydration, and streaming. Faster than React, smaller than Vue.",
                        icon: rsx! {
                            svg {
                                class: "w-8 h-8",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "1.5",
                                    d: "M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9",
                                }
                            }
                        },
                    }
                    PlatformCard {
                        platform: "Desktop",
                        description: "Native windows with webview or GPU rendering. Sub-2MB binaries. macOS, Windows, and Linux.",
                        icon: rsx! {
                            svg {
                                class: "w-8 h-8",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "1.5",
                                    d: "M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
                                }
                            }
                        },
                    }
                    PlatformCard {
                        platform: "Mobile",
                        description: "iOS and Android apps with native performance. Share logic with your web and desktop apps.",
                        icon: rsx! {
                            svg {
                                class: "w-8 h-8",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "1.5",
                                    d: "M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z",
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn PlatformCard(platform: &'static str, description: &'static str, icon: Element) -> Element {
    rsx! {
        div { class: "group relative p-8 bg-white dark:bg-ghdarkmetal border border-gray-200 dark:border-gray-800 hover:border-orange-500/50 transition-colors",
            div { class: "w-12 h-12 flex items-center justify-center bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-orange-500 mb-6",
                {icon}
            }
            h3 { class: "text-xl font-bold text-gray-900 dark:text-white mb-3",
                "{platform}"
            }
            p { class: "text-gray-600 dark:text-gray-400 leading-relaxed text-sm",
                "{description}"
            }
        }
    }
}

/// Stats section
#[component]
fn Stats() -> Element {
    rsx! {
        section { class: "py-20 md:py-28 bg-[#0d1117] border-t border-gray-800",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div { class: "text-center max-w-3xl mx-auto mb-16",
                    h2 { class: "text-3xl sm:text-4xl md:text-5xl font-bold text-white mb-4",
                        "Loved by "
                        span { class: "text-orange-500", "developers" }
                    }
                    p { class: "text-lg text-gray-400",
                        "Join thousands of developers building production apps with Dioxus"
                    }
                }

                // Stats grid
                div { class: "grid grid-cols-2 lg:grid-cols-4 gap-6 lg:gap-8 mb-16",
                    StatCard { value: "25k+", label: "GitHub Stars" }
                    StatCard { value: "350k+", label: "Downloads" }
                    StatCard { value: "300+", label: "Contributors" }
                    StatCard { value: "2000+", label: "Discord Members" }
                }

                // Contributors
                div { class: "text-center",
                    p { class: "text-sm text-gray-400 mb-6", "Our amazing contributors" }
                    a {
                        href: "https://github.com/dioxuslabs/dioxus/graphs/contributors",
                        target: "_blank",
                        class: "inline-block hover:opacity-80 transition-opacity",
                        img {
                            src: "https://contrib.rocks/image?repo=dioxuslabs/dioxus&max=70&columns=14",
                            alt: "Dioxus Contributors",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(value: &'static str, label: &'static str) -> Element {
    rsx! {
        div { class: "text-center p-6 bg-ghdarkmetal border border-gray-800",
            div { class: "text-4xl sm:text-5xl font-bold text-white mb-2 tabular-nums", "{value}" }
            div { class: "text-gray-400 text-sm", "{label}" }
        }
    }
}

/// Final call to action
#[component]
fn CallToAction() -> Element {
    rsx! {
        section { class: "py-20 md:py-28 bg-white dark:bg-black border-t border-gray-200 dark:border-gray-800",
            div { class: "max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center",
                h2 { class: "text-3xl sm:text-4xl md:text-5xl font-bold text-gray-900 dark:text-white mb-6",
                    "Ready to build something "
                    span { class: "text-orange-500", "amazing" }
                    "?"
                }
                p { class: "text-lg text-gray-600 dark:text-gray-400 mb-10 max-w-2xl mx-auto",
                    "Get started with Dioxus in minutes. Our CLI handles all the setup so you can focus on building."
                }

                // Install command
                div { class: "bg-[#0d1117] border border-gray-800 p-6 mb-8 max-w-lg mx-auto",
                    div { class: "font-mono text-left text-sm space-y-2",
                        div { class: "flex items-center gap-2",
                            span { class: "text-gray-500", "$" }
                            span { class: "text-gray-300", "cargo install dioxus-cli" }
                        }
                        div { class: "flex items-center gap-2",
                            span { class: "text-gray-500", "$" }
                            span { class: "text-gray-300", "dx new my-app && cd my-app" }
                        }
                        div { class: "flex items-center gap-2",
                            span { class: "text-gray-500", "$" }
                            span { class: "text-gray-300", "dx serve" }
                        }
                    }
                }

                // CTA buttons
                div { class: "flex flex-col sm:flex-row gap-4 justify-center",
                    Link {
                        to: Route::Docs07 {
                            child: BookRoute::Index {
                                section: Default::default(),
                            },
                        },
                        class: "inline-flex items-center justify-center gap-2 px-8 py-4 text-lg font-semibold text-white bg-gray-900 dark:bg-white dark:text-gray-900 hover:bg-gray-700 dark:hover:bg-gray-200 transition-colors",
                        "Read the Docs"
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M13 7l5 5m0 0l-5 5m5-5H6",
                            }
                        }
                    }
                    Link {
                        to: "https://discord.gg/XgGxMSkvUM",
                        new_tab: true,
                        class: "inline-flex items-center justify-center gap-2 px-8 py-4 text-lg font-semibold text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors",
                        svg {
                            class: "w-5 h-5",
                            fill: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                d: "M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028 14.09 14.09 0 0 0 1.226-1.994.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z",
                            }
                        }
                        "Join Discord"
                    }
                }
            }
        }
    }
}
