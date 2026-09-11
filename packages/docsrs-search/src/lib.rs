use std::{collections::HashMap, ffi::OsStr};

use dioxus_search::*;

pub fn build_into(output_directory: &impl AsRef<OsStr>) {
    let versions = [
        "0.7.0-alpha.2",
        "0.6.3",
        "0.5.6",
        "0.4.3",
        "0.3.2",
        "0.2.4",
        "0.1.8",
    ];
    for version in versions {
        let flat_version = version.replace('.', "_").replace('-', "_");
        let major_version = flat_version
            .split('_')
            .take(2)
            .collect::<Vec<_>>()
            .join("_");
        let temp_dir =
            tempdir::TempDir::new(&major_version).expect("Failed to create temporary directory");
        let download_directory = temp_dir.path();
        let download_directory = download_directory.join(&major_version);
        let url_prefix = format!("https://docs.rs/dioxus/{version}/dioxus/");

        println!("Fetching dioxus documentation...");
        let data = reqwest::blocking::get(format!("https://docs.rs/crate/dioxus/{version}/download"))
            .expect("Failed to fetch data")
            .bytes()
            .expect("Failed to read bytes");

        println!("Extracting dioxus documentation...");

        // extract the zip file
        let mut archive =
            zip::ZipArchive::new(std::io::Cursor::new(data)).expect("Failed to create ZipArchive");

        // create a directory to extract the file
        std::fs::create_dir_all(&download_directory).expect("Failed to create output directory");

        println!("Extracting files to {:?}", download_directory);

        // extract the files
        archive
            .extract(&download_directory)
            .expect("Failed to extract zip archive");
        println!("Extraction complete!");

        let base_dir = download_directory.join("dioxus");
        let boring_paths = vec![
            "prelude/global_attributes",
            "prelude/svg_attributes",
            "prelude/attributes",
            "prelude/props",
            "prelude/keyboard_types",
            "prelude/server_fn",
            "events",
            "all.html",
        ];

        fn extract_item_name(path: &std::path::Path) -> Option<String> {
            let file_stem = path.file_stem()?;
            let file_stem_str = file_stem.to_string_lossy();
            let prefixes = [
                ("fn.", "fn"),
                ("struct.", "struct"),
                ("trait.", "trait"),
                ("enum.", "enum"),
                ("type.", "type"),
                ("constant.", "const"),
                ("macro.", "macro"),
                ("attr.", "attr"),
                ("derive.", "derive"),
            ];

            for (prefix, ty) in &prefixes {
                if let Some(name) = file_stem_str.strip_prefix(prefix) {
                    if *ty == "fn" {
                        if name.starts_with("use_") {
                            return Some(format!("hook {}", name));
                        } else if name
                            .chars()
                            .next()
                            .map_or(false, |c| c.is_ascii_uppercase())
                        {
                            return Some(format!("component {}", name));
                        }
                    }
                    return Some(format!("{} {}", ty, name));
                }
            }

            None
        }

        // Add all html files in the base directory and its subdirectories
        let files = walkdir::WalkDir::new(&base_dir)
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.expect("Failed to read directory entry");
                let path = entry.path();
                if boring_paths
                    .iter()
                    .any(|bp| path.starts_with(&base_dir.join(bp)))
                {
                    return None; // Skip files in the boring paths
                }
                if entry.file_type().is_file()
                    && path.extension().map_or(false, |ext| ext == "html")
                {
                    println!("Processing file: {:?}", path);
                    let Some(title) = extract_item_name(path) else {
                        println!("Skipping file without a valid title: {:?}", path);
                        return None; // Skip files without a valid title
                    };
                    let path = path
                        .strip_prefix(&base_dir)
                        .expect("Failed to strip base directory")
                        .to_string_lossy()
                        .to_string();
                    Some(File {
                        path: path,
                        url: format!(
                            "{}{}",
                            url_prefix,
                            entry
                                .path()
                                .strip_prefix(&base_dir)
                                .unwrap()
                                .to_string_lossy()
                        ),
                        title: title.to_string(),
                        fields: HashMap::new(),
                        explicit_source: None,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let config = Config {
            input: InputConfig {
                base_directory: base_dir.to_string_lossy().to_string(),
                url_prefix: url_prefix.into(),
                html_selector: Some(
                    ".top-doc .docblock, #implementations .docblock, .methods .docblock".into(),
                ),
                files,
                break_on_file_error: false,
                minimum_indexed_substring_length: 3,
                minimum_index_ideographic_substring_length: 1,
            },
        };
        write_index(
            &config,
            format!("docsrs_{}", major_version),
            &output_directory.as_ref(),
        );
    }
}
