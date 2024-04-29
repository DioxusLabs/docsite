use dioxus::prelude::*;
use dioxus_material_icons::MaterialIconColor;

#[derive(Clone, Copy, PartialEq)]
enum Platform {
    Windows,
    MacOS,
    Linux,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let platform = match self {
            Platform::Windows => "Windows",
            Platform::MacOS => "MacOS",
            Platform::Linux => "Linux",
        };
        write!(f, "{}", platform)
    }
}

impl Platform {
    const ALL: [Platform; 3] = [Platform::Windows, Platform::MacOS, Platform::Linux];
}

pub(crate) fn DesktopDependencies() -> Element {
    let mut current_platform = use_signal(|| Platform::Windows);

    #[cfg(feature = "web")]
    {
        if generation() == 0 {
            needs_update();
        }
        if generation() == 1 {
            use web_sys::window;
            let platform = window().unwrap().navigator().platform();
            if let Ok(platform) = platform {
                if platform.contains("Win") {
                    current_platform.set(Platform::Windows);
                } else if platform.contains("Mac") {
                    current_platform.set(Platform::MacOS);
                } else if platform.contains("X11") || platform.contains("Linux") {
                    current_platform.set(Platform::Linux);
                }
            }
        }
    }

    let dependencies = match current_platform() {
        Platform::Windows => rsx! {
            WindowsDependencies {}
        },
        Platform::MacOS => rsx! {
            MacOSDependencies {}
        },
        Platform::Linux => rsx! {
            LinuxDependencies {}
        },
    };

    let mut active = use_signal(|| false);

    rsx! {
        button {
            class: "p-2 border-0 hover:text-blue-400 text-2xl",
            onclick: move |_| {
                active.toggle();
            },
            if active() {
                dioxus_material_icons::MaterialIcon {
                    name: "expand_more",
                    color: MaterialIconColor::Custom("gray".to_string())
                }
            } else {
                dioxus_material_icons::MaterialIcon {
                    name: "chevron_right",
                    color: MaterialIconColor::Custom("gray".to_string())
                }
            },
            "Desktop Specific Dependencies"
        }
        if active() {
            div {
                class: "flex flex-col items-center justify-center",
                div {
                    class: "flex flex-col width-full items-center justify-center space-y-4 divide-y-2 border-white m-5 rounded-lg",
                    div {
                        class: "flex flex-row items-center justify-center m-2",
                        "Your OS: "
                        for platform in Platform::ALL {
                            button {
                                class: if platform == current_platform() { "text-blue-500" },
                                class: "p-2 hover:text-blue-400 border-0",
                                onclick: move |_| {
                                    current_platform.set(platform);
                                },
                                "{platform}"
                            }
                        }
                    }
                    div {
                        class: "p-4",
                        {dependencies}
                    }
                }
            }
        }
    }
}

fn WindowsDependencies() -> Element {
    rsx! {
        div {
            p {
                "Windows apps depend on WebView2 – a library that should be installed in all modern Windows distributions. If you have Edge installed, then Dioxus will work fine. If you _don't_ have WebView2, "
                Link {
                    "then you can install it through Microsoft"
                    to: "https://developer.microsoft.com/en-us/microsoft-edge/webview2/"
                }
                ". MS provides 3 options:"
            }
            ol {
                li { "A tiny \"evergreen\" _bootstrapper_ that fetches an installer from Microsoft's CDN." }
                li { "A tiny _installer_ that fetches WebView2 from Microsoft's CDN." }
                li { "A statically linked version of WebView2 in your final binary for offline users." }
            }

            p { "For development purposes, use Option 1." }
        }
    }
}

fn LinuxDependencies() -> Element {
    rsx! {
        div {
            p {
                "WebView Linux apps require WebkitGtk. When distributing, this can be part of your dependency tree in your `.rpm` or `.deb`. However, likely, your users will already have WebkitGtk."
            }
            p {
                "If you run into issues, make sure you have all the basics installed, as outlined in the "
                Link {
                    "Tauri docs"
                    to: "https://beta.tauri.app/start/prerequisites/"
                }
                "."
            }
        }
    }
}

fn MacOSDependencies() -> Element {
    rsx! {
        div {
            p { "Currently – everything for macOS is built right in! However, you might run into an issue if you're using nightly Rust due to some permissions issues in our Tao dependency (which have been resolved but not published)." }
        }
    }
}
