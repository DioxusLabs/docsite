use crate::editor::monaco;
use dioxus_document::eval;
use model::Project;

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
    // let code = monaco::get_current_model_value();
    // let encoded = Project::new(code, "".into(), "".into()).to_compressed_base64();

    // let formatted = format!("{}/{}", location, encoded);
    // let e = eval(
    //     r#"
    //     const data = await dioxus.recv();
    //     navigator.clipboard.writeText(data);
    //     "#,
    // );

    // let _ = e.send(formatted);
}
