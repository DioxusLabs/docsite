use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    path::PathBuf,
    str::FromStr,
};

use bytes::Bytes;
use dioxus_router::routable::Routable;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use stork_lib::{build_index, SearchError};

#[derive(Deserialize, Serialize)]
pub struct SearchIndex<R> {
    index: Bytes,
    name: String,
    mock: bool,
    _marker: std::marker::PhantomData<R>,
}

impl<R> Default for SearchIndex<R> {
    fn default() -> Self {
        Self {
            index: Bytes::new(),
            name: String::new(),
            mock: true,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<R> Debug for SearchIndex<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchIndex").finish()
    }
}

impl<R: Routable> SearchIndex<R>
where
    <R as FromStr>::Err: Display,
{
    pub fn create(name: impl AsRef<str>, mapping: impl SearchIndexMapping<R>) -> Self {
        let name = name.as_ref().to_string();
        let asset_format = Config::from_route(mapping);

        let toml = toml::to_string(&asset_format).unwrap();
        let bytes = build_index(&stork_lib::Config::try_from(&*toml).unwrap())
            .unwrap()
            .bytes;

        stork_lib::register_index(&format!("index_{name}"), bytes.clone()).unwrap();

        let myself = Self {
            index: bytes,
            name,
            mock: false,
            _marker: std::marker::PhantomData,
        };

        let target_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());

        let path = format!("{}/dioxus_search/index_{}.bin", target_dir, myself.name);
        std::fs::create_dir_all(std::path::Path::new(&path).parent().unwrap()).unwrap();
        let compressed = yazi::compress(
            &myself.index,
            yazi::Format::Zlib,
            yazi::CompressionLevel::Default,
        )
        .unwrap();
        std::fs::write(path, compressed).unwrap();

        myself
    }

    pub fn from_bytes<T: Into<Bytes>>(name: impl AsRef<str>, bytes: T) -> Self {
        let name = name.as_ref().to_string();
        let bytes = bytes.into();
        stork_lib::register_index(&format!("index_{name}"), bytes.clone()).unwrap();
        Self {
            index: bytes,
            name,
            mock: false,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn search(&self, text: &str) -> Result<Vec<SearchResult<R>>, SearchError> {
        if self.mock {
            return Ok(Vec::new());
        }

        let id = &self.name;
        let output = stork_lib::search_from_cache(&format!("index_{id}"), text)?;
        let mut results = Vec::new();
        for result in output.results {
            match result.entry.url.parse() {
                Ok(route) => {
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
                        route,
                        excerpts,
                        title: result.entry.title,
                        score: result.score,
                    })
                }
                Err(err) => {
                    log::error!("Failed to parse url ({}): {err}", result.entry.url);
                }
            }
        }

        results.sort_by_key(|result| !result.score);

        Ok(results)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult<R: Routable> {
    pub route: R,
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
    fn from_route<R: Routable>(mapping: impl SearchIndexMapping<R>) -> Self
    where
        <R as FromStr>::Err: Display,
    {
        let mut files = Vec::new();
        let base_directory = mapping.base_directory();

        // Collect all the static routes
        let static_routes = R::static_routes();
        // Add the routes to the index
        for route in static_routes {
            let url = route.to_string();
            if let Some(path) = mapping.map_route(route) {
                let path = &path.strip_prefix("/").unwrap_or(&path);
                let absolute_path = base_directory.join(path);
                log::trace!("Adding {:?} to search index", absolute_path);
                match std::fs::read_to_string(&absolute_path) {
                    Ok(contents) => {
                        let document = Html::parse_document(&contents);
                        let title = document
                            .select(&Selector::parse("h1").unwrap())
                            .next()
                            .map(|title| title.text().collect::<String>())
                            .unwrap_or_else(|| {
                                document
                                    .select(&Selector::parse("title").unwrap())
                                    .next()
                                    .map(|title| title.text().collect::<String>())
                                    .unwrap_or_else(|| {
                                        let mut title = String::new();
                                        for segment in path.iter() {
                                            title.push_str(&segment.to_string_lossy());
                                            title.push(' ');
                                        }
                                        title
                                    })
                            });
                        files.push(File {
                            path: path.to_string_lossy().into(),
                            url,
                            title,
                            fields: HashMap::new(),
                            explicit_source: None,
                        })
                    }
                    Err(err) => {
                        log::error!("Error reading file: {:?}: {}", absolute_path, err);
                    }
                }
            }
        }

        Self {
            input: InputConfig {
                base_directory: base_directory.to_string_lossy().into(),
                url_prefix: "".into(),
                html_selector: Some("#main".into()),
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

pub trait SearchIndexMapping<R: Routable> {
    fn base_directory(&self) -> PathBuf;
    fn map_route(&self, route: R) -> Option<PathBuf>;
}

pub struct Mapped<F: Fn(R) -> Option<PathBuf>, R: Routable> {
    base_directory: PathBuf,
    map: F,
    _marker: std::marker::PhantomData<R>,
}

impl<F: Fn(R) -> Option<PathBuf>, R: Routable> SearchIndexMapping<R> for Mapped<F, R> {
    fn base_directory(&self) -> PathBuf {
        self.base_directory.clone()
    }

    fn map_route(&self, route: R) -> Option<PathBuf> {
        (self.map)(route)
    }
}

pub struct BaseDirectoryMapping {
    base_directory: PathBuf,
}

impl<R: Routable> SearchIndexMapping<R> for BaseDirectoryMapping {
    fn base_directory(&self) -> PathBuf {
        self.base_directory.clone()
    }

    fn map_route(&self, route: R) -> Option<PathBuf> {
        let route = route.to_string();
        let (route, _) = route.split_once('#').unwrap_or((&route, ""));
        let (route, _) = route.split_once('?').unwrap_or((&route, ""));
        let route = PathBuf::from(route).join("index.html");
        Some(route)
    }
}

impl BaseDirectoryMapping {
    pub fn new(base_directory: impl Into<PathBuf>) -> Self {
        Self {
            base_directory: base_directory.into(),
        }
    }

    pub fn map<F: Fn(R) -> Option<PathBuf>, R: Routable>(self, map: F) -> Mapped<F, R> {
        Mapped {
            base_directory: self.base_directory,
            map,
            _marker: std::marker::PhantomData,
        }
    }
}

impl From<PathBuf> for BaseDirectoryMapping {
    fn from(base_directory: PathBuf) -> Self {
        Self::new(base_directory)
    }
}
