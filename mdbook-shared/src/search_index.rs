use std::{collections::HashMap, fmt::Debug, path::PathBuf, sync::atomic::AtomicU32};

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use stork_lib::{build_index, SearchError};

use crate::{get_book_content_path, MdBook, PageId};

static SEARCH_INDEX: AtomicU32 = AtomicU32::new(0);

fn take_index() -> u32 {
    SEARCH_INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

#[derive(Deserialize, Serialize)]
pub struct SearchIndex {
    index: Bytes,
    id: u32,
}

impl Debug for SearchIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchIndex").finish()
    }
}

impl SearchIndex {
    pub fn from_book(book_path: PathBuf, book: &MdBook<PathBuf>) -> Self {
        let en_path = get_book_content_path(book_path).unwrap();
        let asset_format = Config::from_book(en_path.to_string_lossy().into(), book);

        let toml = toml::to_string(&asset_format).unwrap();
        let bytes = build_index(&stork_lib::Config::try_from(&*toml).unwrap())
            .unwrap()
            .bytes;

        let id = take_index();
        stork_lib::register_index(&format!("index_{id}"), bytes.clone()).unwrap();

        Self { index: bytes, id }
    }

    pub fn from_bytes<T: Into<Bytes>>(bytes: T) -> Self {
        let bytes = bytes.into();
        let id = take_index();
        stork_lib::register_index(&format!("index_{id}"), bytes.clone()).unwrap();
        Self { index: bytes, id }
    }

    pub fn to_bytes(self) -> Bytes {
        self.index
    }

    pub fn search(&self, text: &str) -> Result<Vec<SearchResult>, SearchError> {
        let id = self.id;
        let output = stork_lib::search_from_cache(&format!("index_{id}"), text)?;
        let mut results = Vec::new();
        for result in output.results {
            let id = PageId(result.entry.url.parse().unwrap());
            let excerpts = result
                .excerpts
                .into_iter()
                .map(|excerpt| {
                    let mut segments = Vec::new();
                    let mut char_index = 0;
                    let mut chars = excerpt.text.chars();
                    let mut current_segment = String::new();
                    for highlight_range in excerpt.highlight_ranges {
                        let start = highlight_range.beginning;
                        while char_index < start.saturating_sub(1) {
                            if let Some(c) = chars.next() {
                                current_segment.push(c);
                                char_index += 1;
                            } else {
                                break;
                            }
                        }
                        // add the current segment as a plain text segment
                        if !current_segment.is_empty() {
                            segments.push(Segment {
                                text: std::mem::take(&mut current_segment),
                                highlighted: false,
                            });
                        }
                        let end = highlight_range.end;
                        while char_index < end {
                            if let Some(c) = chars.next() {
                                current_segment.push(c);
                                char_index += 1;
                            } else {
                                break;
                            }
                        }
                        // add the current segment as a highlighted segment
                        if !current_segment.is_empty() {
                            segments.push(Segment {
                                text: std::mem::take(&mut current_segment),
                                highlighted: true,
                            });
                        }
                    }
                    Excerpt {
                        text: segments,
                        score: excerpt.score,
                    }
                })
                .collect();
            results.push(SearchResult {
                id,
                excerpts,
                title: result.entry.title,
                score: result.score,
            })
        }

        results.sort_by_key(|result| !result.score);

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
    pub text: Vec<Segment>,
    pub score: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Segment {
    pub text: String,
    pub highlighted: bool,
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
