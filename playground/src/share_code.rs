use crate::{editor::monaco, error::ShareError, hotreload::HotReload};
use base64::{prelude::BASE64_URL_SAFE, Engine};
use dioxus::prelude::*;
use dioxus_document::eval;

/// Use a share code, converting the share code to real code.
pub fn use_share_code(
    share_code: Option<String>,
    mut show_share_warning: Signal<bool>,
    mut hot_reload: HotReload,
) -> Memo<Option<String>> {
    use_memo(use_reactive((&share_code,), move |(share_code,)| {
        let share_code = share_code?;
        let decoded = decode_code(&share_code).ok()?;

        show_share_warning.set(true);

        // If monaco is initialized, set it now. Otherwise save it for monaco onload code.
        if monaco::is_ready() {
            monaco::set_current_model_value(&decoded);
            hot_reload.set_starting_code(&decoded);
            return None;
        }

        Some(decoded)
    }))
}

/// Copy a share link to the clipboard.
///
/// This will:
/// 1. Get the current code from the editor.
/// 2. Compress it using `miniz_oxide`.
/// 3. Encodes it in url-safe base64.
/// 4. Formats the code with the provided `location` url prefix.
/// 5. Copies the link to the clipboard.
///
/// This allows users to have primitve serverless sharing.
/// Links will be large and ugly but it works.
pub fn copy_share_link(location: &str) {
    let code = monaco::get_current_model_value();
    let encoded = encode_code(&code);

    let formatted = format!("{}/{}", location, encoded);
    let e = eval(
        r#"
        const data = await dioxus.recv();
        navigator.clipboard.writeText(data);
        "#,
    );

    let _ = e.send(formatted);
}

/// Encode a string into a share code.
fn encode_code(code: &str) -> String {
    let compressed = miniz_oxide::deflate::compress_to_vec(code.as_bytes(), 10);
    let mut encoded = String::new();
    BASE64_URL_SAFE.encode_string(compressed, &mut encoded);
    encoded
}

/// Decode the share code into code.
fn decode_code(share_code: &str) -> Result<String, ShareError> {
    let bytes = BASE64_URL_SAFE.decode(share_code)?;
    let decoded_bytes = miniz_oxide::inflate::decompress_to_vec(&bytes)?;
    let decoded = String::from_utf8(decoded_bytes)?;
    Ok(decoded)
}

#[cfg(test)]
mod test {
    use super::{decode_code, encode_code};
    const ENCODED: &str = "TYpNCoAgFIT3gXd42qYWXSEo6BTRyh8Q1CepEYR3T4mobzPfDNOuHK1HJ13cSKMcTN53PQwjLEbassJFGijs4aSvV4Q-_rXC5qSNAI5oIMSkFGXfIT9aIt8=";
    const DECODED: &str = "#[component]\r\nfn App() -> Element {\r\n    rsx! {\r\n        div {\r\n            \"Build cool stuff!\"\r\n        }\r\n    }\r\n}";

    #[test]
    fn test_encode_code() {
        let result = encode_code(DECODED);
        assert_eq!(ENCODED, result);
    }

    #[test]
    fn test_decode_share_link() {
        let result = decode_code(ENCODED).unwrap();
        assert_eq!(DECODED, result);
    }
}
