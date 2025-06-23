use crate::{
    icons::{DarkModeIcon, LightModeIcon},
    DARK_MODE,
};
use dioxus::prelude::*;

fn set_theme(dark_mode: bool) {
    let theme = if dark_mode { "dark" } else { "light" };
    _ = document::eval(&format!(
        "document.documentElement.setAttribute('data-theme', '{}');",
        theme
    ));
}

#[component]
pub(crate) fn DarkModeToggle(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    // Set the initial value based on the system preference
    use_effect(move || {
        spawn(async move {
            let dark_theme =
                document::eval("return window.matchMedia('(prefers-color-scheme: dark)').matches;")
                    .join()
                    .await;
            if let Ok(preference) = dark_theme {
                *DARK_MODE.write() = preference;
            }
        });
    });

    // Set the theme based on the signal
    use_effect(move || {
        set_theme(DARK_MODE());
    });

    // Toggle with a button
    rsx! {
        button {
            onclick: move |_| {
                let new_mode = !DARK_MODE();
                *DARK_MODE.write() = new_mode;
            },
            aria_label: if DARK_MODE() { "Enable light mode" } else { "Enable dark mode" },
            ..attributes,

            if DARK_MODE() {
                LightModeIcon {}
            } else {
                DarkModeIcon {}
            }
        }
    }
}
