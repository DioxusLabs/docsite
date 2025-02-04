use base64::{prelude::BASE64_URL_SAFE, Engine};
use include_dir::DirEntry;
use once_cell::sync::Lazy;
use uuid::Uuid;

static EXAMPLES: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/examples");

pub fn get_welcome_project() -> ExampleProject {
    get_example_projects()
        .iter()
        .find(|p| p.path == "welcome.rs")
        .unwrap()
        .clone()
}

/// Returns a list of all example projects.
pub fn get_example_projects() -> &'static [ExampleProject] {
    static LIST: Lazy<Vec<ExampleProject>> = once_cell::sync::Lazy::new(|| {
        let mut projects = Vec::new();

        for entry in EXAMPLES.entries() {
            let DirEntry::File(entry) = entry else {
                continue;
            };

            let path = entry.path();
            let contents = entry.contents();
            let contents = String::from_utf8(contents.to_vec()).unwrap();

            let mut description = String::new();

            for line in contents.lines() {
                if line.starts_with("//!") {
                    description.push_str(&line[3..]);
                    description.push('\n');
                } else {
                    break;
                }
            }

            // Remove the trailing newline
            description.pop();

            let mut project =
                ExampleProject::new(contents, description, path.to_string_lossy().to_string());

            project.prebuilt = true;

            projects.push(project);
        }

        projects
    });

    LIST.as_ref()
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExampleProject {
    pub description: String,
    pub path: String,
    pub contents: String,
    pub prebuilt: bool,
    id: Uuid,
}

impl ExampleProject {
    pub fn new(contents: String, description: String, path: String) -> Self {
        // Generate a unique id for the example.
        let id = Uuid::new_v3(&Uuid::NAMESPACE_URL, contents.as_bytes());
        Self {
            prebuilt: false,
            contents,
            description,
            path,
            id,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn from_compressed_base64(share_code: String) -> anyhow::Result<Self> {
        let bytes = BASE64_URL_SAFE.decode(share_code)?;
        let decoded_bytes = miniz_oxide::inflate::decompress_to_vec(&bytes)?;
        let contents = String::from_utf8(decoded_bytes)?;

        Ok(Self::new(contents, "".into(), "example.rs".into()))
    }

    pub fn to_compressed_base64(&self) -> String {
        let bytes = self.contents.as_bytes();
        let compressed_bytes = miniz_oxide::deflate::compress_to_vec(bytes, 9);
        BASE64_URL_SAFE.encode(compressed_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_projects() {
        assert!(!dbg!(get_example_projects()).is_empty());
    }

    #[test]
    fn has_welcome() {
        dbg!(get_welcome_project());
    }

    const ENCODED: &str = "TYpNCoAgFIT3gXd42qYWXSEo6BTRyh8Q1CepEYR3T4mobzPfDNOuHK1HJ13cSKMcTN53PQwjLEbassJFGijs4aSvV4Q-_rXC5qSNAI5oIMSkFGXfIT9aIt8=";
    const DECODED: &str = "#[component]\r\nfn App() -> Element {\r\n    rsx! {\r\n        div {\r\n            \"Build cool stuff!\"\r\n        }\r\n    }\r\n}";

    #[test]
    fn test_encode_code() {
        let result = ExampleProject::new(DECODED.into(), "".into(), "".into());
        assert_eq!(ENCODED, result.to_compressed_base64());
    }

    #[test]
    fn test_decode_share_link() {
        let result = ExampleProject::from_compressed_base64(ENCODED.into()).unwrap();
        assert_eq!(DECODED, result.contents);
    }
}
