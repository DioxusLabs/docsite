use dioxus_sdk::{theme::SystemTheme, utils::timing::UseDebounce};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/bindings/monaco.js")]
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
