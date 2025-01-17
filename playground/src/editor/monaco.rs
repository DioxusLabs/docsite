use crate::{
    hotreload::HotReload,
    snippets::{self, SelectedExample},
    EXTRA_LINE_COUNT,
};
use dioxus::prelude::*;
use dioxus_sdk::{theme::SystemTheme, utils::timing::UseDebounce};
use model::{CargoDiagnostic, CargoLevel};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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
pub fn use_monaco_markers(diagnostics: Signal<Vec<CargoDiagnostic>>) {
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
                start_line_number: span.line_start.saturating_sub(EXTRA_LINE_COUNT()),
                end_line_number: span.line_end.saturating_sub(EXTRA_LINE_COUNT()),
                start_column: span.column_start,
                end_column: span.column_end,
            };

            markers.push(marker);
        }
    }

    set_markers(&markers);
}

/// Initialize Monaco once the loader script loads.
pub fn on_monaco_load(
    folder: Asset,
    system_theme: SystemTheme,
    shared_code: Option<String>,
    mut selected_example: Signal<SelectedExample>,
    mut hot_reload: HotReload,
    on_model_changed: UseDebounce<String>,
) {
    // If shared code is available, use it, otherwise replace with starter snippet.
    let snippet = match shared_code {
        Some(c) => c,
        None => {
            selected_example.set(SelectedExample::Welcome);
            snippets::EXAMPLES[0].1.to_string()
        }
    };

    let monaco_prefix = monaco_vs_prefix(folder);
    init(
        &monaco_prefix,
        super::EDITOR_ELEMENT_ID,
        system_theme,
        &snippet,
    );

    hot_reload.set_starting_code(&snippet);
    register_model_change(on_model_changed);
}

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

pub fn init(
    vs_path_prefix: &str,
    element_id: &str,
    initial_theme: SystemTheme,
    initial_snippet: &str,
) {
    let theme = system_theme_to_string(initial_theme);
    init_monaco(vs_path_prefix, element_id, &theme, initial_snippet);
    register_paste_as_rsx_action();
}

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

pub fn register_model_change(mut debounce: UseDebounce<String>) {
    let callback = Closure::new(move |new_code: String| {
        debounce.action(new_code);
    });

    register_model_change_event(&callback);
    callback.forget();
}

fn system_theme_to_string(theme: SystemTheme) -> String {
    match theme {
        SystemTheme::Light => "dx-vs",
        SystemTheme::Dark => "dx-vs-dark",
    }
    .to_string()
}
