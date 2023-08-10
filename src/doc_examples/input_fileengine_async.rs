#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App(cx: Scope) -> Element {
    let files_uploaded: &UseRef<Vec<String>> = use_ref(cx, Vec::new);

    cx.render(rsx! {
        input {
            // we tell the component what to render
            value: "{name}",
            // tell the input to pick a file
            r#type:"file",
            // list the accepted extensions
            accept: ".txt, .rs",
            multiple: true,
            onchange: |evt| {
                // A helper macro to use hooks in async environments
                to_owned![files_uploaded];
                async move {
                    if let Some(file_engine) = &evt.files {
                        let files = file_engine.files();
                        for file_name in &files {
                            // Make sure to use async/await when doing heavy I/O operations,
                            // to not freeze the interface in the meantime
                            if let Some(file) = file_engine.read_file_to_string(file_name).await{
                                files_uploaded.write().push(file);
                            }
                        }
                    }
                }
            }
        }
    })
}
// ANCHOR_END: component
