use std::{collections::HashMap, fmt::Debug, path::PathBuf};

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use stork_lib::{build_index, SearchError};

use crate::{MdBook, PageId};

#[derive(Deserialize, Serialize)]
pub struct SearchIndex {
    index: Bytes,
}

impl Debug for SearchIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchIndex").finish()
    }
}

impl SearchIndex {
    pub fn from_book(book_path: PathBuf, book: &MdBook<PathBuf>) -> Self {
        let en_path = book_path.join("en");
        let asset_format = Config::from_book(en_path.to_string_lossy().into(), book);

        let toml = toml::to_string(&asset_format).unwrap();
        let bytes = build_index(&stork_lib::Config::try_from(&*toml).unwrap())
            .unwrap()
            .bytes;

        stork_lib::register_index("index", bytes.clone()).unwrap();

        Self { index: bytes }
    }

    pub fn from_bytes<T: Into<Bytes>>(bytes: T) -> Self {
        let bytes = bytes.into();
        stork_lib::register_index("index", bytes.clone()).unwrap();
        Self { index: bytes }
    }

    pub fn to_bytes(self) -> Bytes {
        self.index
    }

    pub fn search(&self, text: &str) -> Result<Vec<SearchResult>, SearchError> {
        let output = stork_lib::search_from_cache("index", text)?;
        let mut results = Vec::new();
        for result in output.results {
            let id = PageId(result.entry.url.parse().unwrap());
            let excerpts = result
                .excerpts
                .into_iter()
                .map(|excerpt| Excerpt {
                    text: excerpt.text,
                    score: excerpt.score,
                })
                .collect();
            results.push(SearchResult {
                id,
                excerpts,
                title: result.entry.title,
                score: result.score,
            })
        }

        results.sort_by_key(|result| result.score);

        Ok(results)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    pub id: PageId,
    pub title: String,
    pub excerpts: Vec<Excerpt>,
    pub score: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Excerpt {
    pub text: String,
    pub score: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub input: InputConfig,
}

impl Config {
    fn from_book(base_directory: String, book: &MdBook<PathBuf>) -> Self {
        let mut files = Vec::new();
        for (_, page) in book.pages() {
            files.push(File {
                path: page.url.to_string_lossy().to_string(),
                url: { page.id.0.to_string() },
                title: page.title.clone(),
                fields: HashMap::new(),
                explicit_source: None,
            })
        }

        Self {
            input: InputConfig {
                base_directory,
                url_prefix: "".into(),
                html_selector: None,
                files,
                break_on_file_error: false,
                minimum_indexed_substring_length: 3,
                minimum_index_ideographic_substring_length: 1,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputConfig {
    base_directory: String,
    url_prefix: String,
    html_selector: Option<String>,

    files: Vec<File>,

    break_on_file_error: bool,

    minimum_indexed_substring_length: u8,

    minimum_index_ideographic_substring_length: u8,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct File {
    path: String,
    url: String,
    title: String,
    #[serde(flatten, default)]
    pub fields: HashMap<String, String>,
    #[serde(flatten)]
    pub explicit_source: Option<DataSource>,
}

#[derive(Serialize, Deserialize)]
pub enum DataSource {
    #[serde(rename = "contents")]
    Contents(String),

    #[serde(rename = "src_url")]
    #[allow(clippy::upper_case_acronyms)]
    URL(String),

    #[serde(rename = "path")]
    FilePath(String),
}
