use crate::{
    dark_mode,
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
            let mut dark_theme = document::eval(
                "const query = window.matchMedia('(prefers-color-scheme: dark)');
                dioxus.send(query.matches);
                query.addEventListener('change', (e) => dioxus.send(e.matches));",
            );
            while let Ok(preference) = dark_theme.recv().await {
                *DARK_MODE.write() = preference;
            }
        });
    });

    // Set the theme based on the signal
    use_effect(move || {
        set_theme(dark_mode());
    });

    // Toggle with a button
    rsx! {
        button {
            onclick: move |_| {
                let new_mode = !dark_mode();
                *DARK_MODE.write() = Some(new_mode);
            },
            aria_label: if dark_mode() { "Enable light mode" } else { "Enable dark mode" },
            cursor: "pointer",
            ..attributes,

            if dark_mode() {
                DarkModeIcon {}
            } else {
                LightModeIcon {}
            }
        }
    }
}
