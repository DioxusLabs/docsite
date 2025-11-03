//! Theme utilities.
//!
//! Access the system's theme to use for common tasks such as automatically setting your app styling.
//!
//! Most apps will need to choose a default theme in the event of an error.
//! We recommend using either [`Result::unwrap_or`] or  [`Result::unwrap_or_default`] to do this.
//!
//! #### Platform Support
//! Theme is available for Web, Windows, & Mac. Linux is unsupported and Android/iOS has not been tested.
//!
//! # Examples
//! An example of using the theme to determine which class to use.
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_window::theme::{use_system_theme, Theme};
//!
//! #[component]
//! fn App() -> Element {
//!     let theme = use_system_theme();
//!
//!     // Default to a light theme in the event of an error.
//!     let class = match theme().unwrap_or(Theme::Light) {
//!         Theme::Light => "bg-light",
//!         Theme::Dark => "bg-dark",
//!     };
//!
//!     rsx! {
//!         div {
//!             class: "{class}",
//!             "the current theme is: {theme().unwrap_or(Theme::Light)}"
//!         }
//!     }
//! }
//! ```
use dioxus::{core::provide_root_context, prelude::*};
use std::{error::Error, fmt::Display};

/// A color theme.
///
/// For any themes other than `light` and `dark`, a [`ThemeError::UnknownTheme`] will be returned.
/// We may be able to support custom themes in the future.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Theme {
    /// A light theme, better in direct sunlight.
    #[default]
    Light,
    /// A dark theme, better for the night owls.
    Dark,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Light => write!(f, "light"),
            Self::Dark => write!(f, "dark"),
        }
    }
}

/// Possible theme errors.
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeError {
    /// Theme is not supported on this platform.
    Unsupported,
    /// Failed to get the system theme.
    CheckFailed,
    /// System returned an unknown theme.
    UnknownTheme,
}

impl Error for ThemeError {}
impl Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unsupported => write!(f, "the current platform is not supported"),
            Self::CheckFailed => write!(
                f,
                "the system returned an error while checking the color theme"
            ),
            Self::UnknownTheme => write!(
                f,
                "the system provided a theme other than `light` or `dark`"
            ),
        }
    }
}

type ThemeResult = Result<Theme, ThemeError>;

/// Get a signal to the system theme.
///
/// On first run, the result will be [`ThemeError::Unsupported`]. This is to prevent hydration from failing.
/// After the client runs, the theme will be tracked and updated with accurate values.
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_window::theme::{use_system_theme, Theme};
///
/// #[component]
/// fn App() -> Element {
///     let theme = use_system_theme();
///
///     rsx! {
///         p {
///             "the current theme is: {theme().unwrap_or(Theme::Light)}"
///         }
///     }
/// }
/// ```
pub fn use_system_theme() -> ReadSignal<ThemeResult> {
    let mut system_theme = match try_use_context::<Signal<ThemeResult>>() {
        Some(s) => s,
        // This should only run once.
        None => {
            let signal = Signal::new_in_scope(Err(ThemeError::Unsupported), ScopeId::ROOT);
            provide_root_context(signal)
        }
    };

    // Only start the listener on the client.
    use_effect(move || {
        system_theme.set(get_theme());

        #[cfg(target_arch = "wasm32")]
        listen(system_theme);
    });

    use_hook(|| ReadSignal::new(system_theme))
}

// The listener implementation for wasm targets.
#[cfg(target_arch = "wasm32")]
fn listen(mut theme: Signal<ThemeResult>) {
    use wasm_bindgen::{closure::Closure, JsCast};
    use web_sys::MediaQueryList;

    let Some(window) = web_sys::window() else {
        theme.set(Err(ThemeError::Unsupported));
        return;
    };

    // Get the media query
    let Ok(query) = window.match_media("(prefers-color-scheme: dark)") else {
        theme.set(Err(ThemeError::CheckFailed));
        return;
    };

    let Some(query) = query else {
        theme.set(Err(ThemeError::UnknownTheme));
        return;
    };

    // Listener that is called when the media query changes.
    // https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/change_event
    let listener = Closure::wrap(Box::new(move |query: MediaQueryList| {
        match query.matches() {
            true => theme.set(Ok(Theme::Dark)),
            false => theme.set(Ok(Theme::Light)),
        };
    }) as Box<dyn FnMut(MediaQueryList)>);

    let cb = listener.as_ref().clone();
    listener.forget();
    query.set_onchange(Some(cb.unchecked_ref()));
}

/// Get the current theme.
///
///
/// **Note**
///
/// This function will cause hydration to fail if not used inside an effect, task, or event handler.
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_window::theme::{Theme, get_theme};
///
/// #[component]
/// fn App() -> Element {
///     let theme = use_signal(get_theme);
///
///     let class_name = match theme().unwrap_or(Theme::Light) {
///         Theme::Dark => "dark-theme",
///         Theme::Light => "light-theme",
///     };
///
///     rsx! {
///         div {
///             style: "width: 100px; height: 100px;",
///             class: "{class_name}",
///         }
///     }
/// }
/// ```
pub fn get_theme() -> ThemeResult {
    #[cfg(target_arch = "wasm32")]
    return get_theme_platform();

    #[cfg(not(target_arch = "wasm32"))]
    return Ok(Theme::Light);
}

// The wasm implementation to get the system theme.
#[cfg(target_arch = "wasm32")]
fn get_theme_platform() -> ThemeResult {
    let Some(window) = web_sys::window() else {
        return Err(ThemeError::Unsupported);
    };

    // Check the color theme with a media query
    let Some(query) = window
        .match_media("(prefers-color-scheme: dark)")
        .or(Err(ThemeError::CheckFailed))?
    else {
        return Err(ThemeError::UnknownTheme);
    };

    match query.matches() {
        true => Ok(Theme::Dark),
        false => Ok(Theme::Light),
    }
}
