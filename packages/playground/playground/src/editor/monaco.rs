use crate::hotreload::HotReload;
use dioxus::prelude::*;
// use dioxus_sdk::utils::timing::UseDebounce;
use model::{CargoDiagnostic, CargoLevel};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// #[cfg(target_arch = "wasm32")]
// use dioxus_sdk::theme::SystemTheme;

/// Get the path prefix for the `/vs` folder inside the Monaco folder.
pub fn monaco_vs_prefix(folder: Asset) -> String {
    let monaco_vs_prefix = format!("{}/vs", folder);
    monaco_vs_prefix
}

/// Get the full path to the `loader.js` script.
pub fn monaco_loader_src(folder: Asset) -> String {
    let prefix = monaco_vs_prefix(folder);
    format!("{prefix}/loader.js")
}

/// Use monaco code markers for build diagnostics.
pub fn set_monaco_markers(diagnostics: Signal<Vec<CargoDiagnostic>>) {
    let mut markers = Vec::new();
    for diagnostic in diagnostics.read().iter() {
        let severity = match diagnostic.level {
            CargoLevel::Error => MarkerSeverity::Error,
            CargoLevel::Warning => MarkerSeverity::Warning,
        };

        for span in diagnostic.spans.iter() {
            let diagnostic_message = diagnostic.message.clone();
            let message = match span.label.clone() {
                Some(label) => format!("{}\n{}", diagnostic_message, label),
                None => diagnostic_message,
            };

            let marker = Marker {
                message,
                severity,
                start_line_number: span.line_start,
                end_line_number: span.line_end,
                start_column: span.column_start,
                end_column: span.column_end,
            };

            markers.push(marker);
        }
    }

    set_markers(&markers);
}

// /// Initialize Monaco once the loader script loads.
// #[cfg(target_arch = "wasm32")]
// pub fn on_monaco_load(
//     folder: Asset,
//     system_theme: SystemTheme,
//     contents: &str,
//     mut hot_reload: HotReload,
//     mut monaco_ready: Signal<bool>,
//     mut on_model_changed: UseDebounce<String>,
// ) {
//     let on_ready_callback = Closure::new(move || monaco_ready.set(true));
//     let monaco_prefix = monaco_vs_prefix(folder);
//     init(
//         &monaco_prefix,
//         super::EDITOR_ELEMENT_ID,
//         system_theme,
//         contents,
//         &on_ready_callback,
//     );

//     hot_reload.set_starting_code(contents);

//     let model_change_callback =
//         Closure::new(move |new_code: String| on_model_changed.action(new_code));
//     register_model_change_event(&model_change_callback);

//     on_ready_callback.forget();
//     model_change_callback.forget();
// }

#[derive(Serialize, Deserialize, Clone)]
pub struct Marker {
    pub message: String,
    pub severity: MarkerSeverity,

    #[serde(rename = "startLineNumber")]
    pub start_line_number: usize,

    #[serde(rename = "endLineNumber")]
    pub end_line_number: usize,

    #[serde(rename = "startColumn")]
    pub start_column: usize,

    #[serde(rename = "endColumn")]
    pub end_column: usize,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum MarkerSeverity {
    Hint,
    Info,
    Warning,
    Error,
}

pub fn set_markers(markers: &[Marker]) {
    let data = serde_wasm_bindgen::to_value(markers).unwrap();
    set_model_marker(data);
}

// Bindings
#[wasm_bindgen(module = "/src/editor/monaco.js")]
extern "C" {
    #[wasm_bindgen(js_name = initMonaco)]
    fn init_monaco(
        vs_path_prefix: &str,
        element_id: &str,
        initial_theme: &str,
        initial_snippet: &str,
        on_ready_callback: &Closure<dyn FnMut()>,
    );

    #[wasm_bindgen(js_name = getCurrentModelValue)]
    pub fn get_current_model_value() -> String;

    #[wasm_bindgen(js_name = setCurrentModelvalue)]
    pub fn set_current_model_value(value: &str);

    #[wasm_bindgen(js_name = isReady)]
    pub fn is_ready() -> bool;

    #[wasm_bindgen(js_name = setTheme)]
    fn set_monaco_theme(theme: &str);

    #[wasm_bindgen(js_name = setModelMarkers)]
    fn set_model_marker(markers: JsValue);

    #[wasm_bindgen(js_name = registerPasteAsRSX)]
    fn register_paste_as_rsx(convertHtmlToRSX: &Closure<dyn Fn(String) -> Option<String>>);

    #[wasm_bindgen(js_name = registerModelChangeEvent)]
    fn register_model_change_event(callback: &Closure<dyn FnMut(String)>);
}

#[cfg(target_arch = "wasm32")]
pub fn init(
    vs_path_prefix: &str,
    element_id: &str,
    initial_theme: SystemTheme,
    initial_snippet: &str,
    on_ready_callback: &Closure<dyn FnMut()>,
) {
    let theme = system_theme_to_string(initial_theme);
    init_monaco(
        vs_path_prefix,
        element_id,
        &theme,
        initial_snippet,
        on_ready_callback,
    );
    register_paste_as_rsx_action();
}

#[cfg(target_arch = "wasm32")]
pub fn set_theme(theme: SystemTheme) {
    let theme = system_theme_to_string(theme);
    set_monaco_theme(&theme);
}

fn register_paste_as_rsx_action() {
    let callback = Closure::new(|html: String| {
        let dom = dioxus_rsx_rosetta::Dom::parse(&html).ok()?;
        let rsx_callbody = dioxus_rsx_rosetta::rsx_from_html(&dom);
        dioxus_autofmt::write_block_out(&rsx_callbody)
    });

    register_paste_as_rsx(&callback);
    callback.forget();
}

#[cfg(target_arch = "wasm32")]
fn system_theme_to_string(theme: SystemTheme) -> String {
    match theme {
        SystemTheme::Light => "dx-vs",
        SystemTheme::Dark => "dx-vs-dark",
    }
    .to_string()
}
