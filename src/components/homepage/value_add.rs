use dioxus::prelude::*;

pub static ValueAdd: FC<()> = |(cx, _)| {
    rsx!(cx, section { class: "text-gray-600 body-font"
        div { class: "container mx-auto py-12 px-40",
            div { class: "flex flex-wrap sm:-m-4 -mx-4 -mb-10 -mt-4 md:space-y-0 space-y-6 ",
                {[
                    ("Declarative", "Easily describe the layout of your application with HTML or RSX syntax."),
                    ("Component-Based", "Build encapsulated components that manage their own state, then compose them to make complex UIs."),
                    ("Learn Once, Write Anywhere", "Components and hooks can be reused to render on the web, desktop, mobile, server, and more!"),
                    ("Concurrent and Async", "First-class support for asynchronous tasks, Suspense for data fetching, and pausable coroutines."),
                    ("Static Types Everywhere", "Eliminate a whole class of bugs at compile time with static typing for every library."),
                    ("If it compiles, it works", "Fearlessly refactor even the largest of apps with powerful compile-time guarantees."),
                    ("First-class error handling", "No more uncaught exceptions. Components can easily abort rendering without crashing the entire app."),
                    ("Incredible inline documentation", "Comprehensive doc comments provide MDN hints and guides right under your finger tips."),
                ].iter().enumerate().map(|(_, (title, content))| rsx!(
                    div { class: "p-4 md:w-1/4 flex",
                        div { class: "flex-grow pl-6",
                            // todo: find a cute icon for each value add
                            // people like icons
                            // icons::Icon3 {}
                            a { href: "#"
                                h2 { class: "text-black text-lg title-font font-medium mb-2",
                                    "{title}"
                                }
                            }
                            {content.split('\n').map(|line| rsx!{
                                p { class: "leading-relaxed text-base pb-4",
                                    "{line}"
                                }
                            })}
                        }
                    }
                ))}
            }
        }
    })
};
