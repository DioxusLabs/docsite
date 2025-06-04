use dioxus::prelude::*;

pub fn Deploy() -> Element {
    rsx! {
        section { class: "max-w-screen-2xl mx-auto px-4 py-16 my-8 dark:text-white",
            div { class: "max-w-screen-md mx-auto border-b py-8 ",
                div { class: "flex flex-row",
                    div { class: "flex-1" }
                    h1 { class: "text-5xl text-center font-bold pb-8 px-8 flex-2",
                        "Dioxus Deploy"
                    }
                    div { class: "flex-1 mx-auto",
                        div { class: "flex-row hidden md:flex",
                            h3 { class: "text-white bg-green-600 py-1 px-2 rounded ",
                                "Coming soon!"
                            }
                            div { class: "flex-1 " }
                        }
                    }
                }
                h2 { class: "text-2xl text-center",
                    "A deployment platform designed for app developers"
                }
            }

            div { class: "max-w-screen-sm mx-auto p-8 text-gray-600 dark:text-gray-300 items-center flex flex-col",
                p { class: "pb-8",
                    "The batteries-included platform for deploying desktop, web, and mobile apps. Iterate quickly, "
                    "collaborate with your team, and scale to millions."
                }
                a {
                    target: "_blank",
                    class: "text-white text-2xl bg-red-600 py-2 px-4 rounded hover:-translate-y-2 transition-transform duration-300",
                    href: "https://forms.gle/zeBZmrjSkajqg7hUA",
                    "Join the Waitlist"
                }
            }
            div { class: "max-w-screen-lg mx-auto px-4 py-16 flex flex-col  md:flex-row justify-center gap-8 dark:text-white",
                div { class: "max-w-lg dark:bg-gray-800 bg-gray-100 flex-1 my-4 p-8 rounded-lg shadow-xl mx-auto text-center ",
                    h1 { class: "text-3xl pb-4", "Build" }
                    p { class: "text-gray-500 dark:text-gray-300 font-light1 border-b border-gray-200 pb-4",
                        "Iterate on your app with first-class tooling designed for the Dioxus ecosystem. "
                    }
                    ul { class: "text-left dark:text-gray-300 text-gray-600 pt-4 pl-4",
                        li { class: "pb-4", "📜 Automatic previews" }
                        li { class: "pb-4", "👥 Real-time collaboration" }
                        li { class: "pb-4", "🏠 Localhost ready" }
                        li { class: "pb-4", "🔄 Integrated hotreloading" }
                        li { class: "pb-4", "💪 Powerful devtools" }
                    }
                }
                div { class: "max-w-lg dark:bg-gray-800 bg-gray-100 flex-1 my-4 p-8 rounded-lg shadow-xl mx-auto text-center",
                    h1 { class: "text-3xl pb-4", "Deploy" }
                    p { class: "text-gray-500 dark:text-gray-300 font-light1 border-b border-gray-200 pb-4",
                        "Get your app on the internet in seconds, with previews, and powerful real-time collaboration"
                    }
                    ul { class: "text-left dark:text-gray-300 text-gray-600 pt-4 pl-4",
                        li { class: "pb-4", "🔆 Push-to-deploy" }
                        li { class: "pb-4", "👩‍💻 Deploy to App Store" }
                        li { class: "pb-4", "📊 Hosted LiteFS + Sqlite" }
                        li { class: "pb-4", "🧵 Cross-platform bundler" }
                        li { class: "pb-4", "🧯 Automated backups" }
                    }
                }
                div { class: "max-w-lg dark:bg-gray-800 bg-gray-100 flex-1 my-4 p-8 rounded-lg shadow-xl mx-auto text-center",
                    h1 { class: "text-3xl pb-4", "Scale" }
                    p { class: "text-gray-500 dark:text-gray-300 font-light1 border-b border-gray-200 pb-4",
                        "Monitor your app, track down bugs, and grow to as big or as small as you need."
                    }
                    ul { class: "text-left dark:text-gray-300 text-gray-600 pt-4 pl-4",
                        li { class: "pb-4", "🔬 Tracing Log Visualizer" }
                        li { class: "pb-4", "📈 Automatic Dashboards" }
                        li { class: "pb-4", "💥 Crash Analytics" }
                        li { class: "pb-4", "👝 Scale-to-zero" }
                        li { class: "pb-4", "🫣 A/B Testing" }
                    }
                }
            }
        }
    }
}
