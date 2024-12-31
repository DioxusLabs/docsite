use crate::*;
use anyhow::{Context, Ok};
use pulldown_cmark::{Event, Tag};
use serde::{Deserialize, Serialize};
use slab::Slab;
use std::{
    collections::HashMap,
    hash::Hash,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct MdBook<R>
where
    R: Hash + Eq,
{
    pub summary: Summary<R>,

    // a mapping between urls and page ids
    pub page_id_mapping: HashMap<R, PageId>,

    // rendered pages to HTML
    pub pages: Slab<Page<R>>,
}

impl<R: Hash + Eq + Clone> MdBook<R> {
    /// Get a page from the book
    pub fn get_page(&self, id: impl PageIndex<R>) -> &Page<R> {
        &self.pages[id.into_page_index(self).0]
    }

    /// Get the pages
    pub fn pages(&self) -> &Slab<Page<R>> {
        &self.pages
    }
}

pub trait PageIndex<T: Hash + Eq> {
    fn into_page_index(self, book: &MdBook<T>) -> PageId;
}

impl<T: Hash + Eq> PageIndex<T> for PageId {
    fn into_page_index(self, _book: &MdBook<T>) -> PageId {
        self
    }
}

impl<T: Hash + Eq> PageIndex<T> for &T {
    fn into_page_index(self, book: &MdBook<T>) -> PageId {
        book.page_id_mapping.get(self).copied().unwrap()
    }
}

pub fn get_book_content_path(mdbook_root: impl AsRef<Path>) -> Option<PathBuf> {
    let mdbook_root = mdbook_root.as_ref();
    let path = mdbook_root.join("en");
    if path.exists() {
        return Some(path);
    }
    let path = mdbook_root.join("src");
    if path.exists() {
        return Some(path);
    }
    None
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page<R> {
    pub title: String,

    pub url: R,

    pub segments: Vec<String>,

    // the raw markdown
    pub raw: String,

    // headers
    pub sections: Vec<Section>,

    pub id: PageId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub level: usize,
    pub title: String,
    pub id: String,
}

impl MdBook<PathBuf> {
    pub fn new(mdbook_root: PathBuf) -> anyhow::Result<Self> {
        let buf = get_summary_path(&mdbook_root)
            .context("No SUMMARY.md found")?
            .canonicalize()?;

        let summary = std::fs::read_to_string(buf).map_err(|e| {
            anyhow::anyhow!(
                "Failed to read SUMMARY.md. Make sure you are running this command in the root of the book. {e}"
            )
        })?;
        let summary = parse_summary(&mdbook_root, &summary)?;

        let mut book = Self {
            summary,
            page_id_mapping: Default::default(),
            pages: Default::default(),
        };

        book.populate(mdbook_root)?;

        Ok(book)
    }

    pub fn populate(&mut self, mdbook_root: PathBuf) -> anyhow::Result<()> {
        let summary = self.summary.clone();

        let chapters = summary
            .prefix_chapters
            .iter()
            .chain(summary.numbered_chapters.iter())
            .chain(summary.suffix_chapters.iter());

        for chapter in chapters {
            self.populate_page(mdbook_root.clone(), chapter)?;
        }

        Ok(())
    }

    fn populate_page(
        &mut self,
        mdbook_root: PathBuf,
        chapter: &SummaryItem<PathBuf>,
    ) -> anyhow::Result<()> {
        let Some(link) = chapter.maybe_link() else {
            return Ok(());
        };

        let url = link.location.as_ref().cloned().unwrap();
        let md_file = get_book_content_path(&mdbook_root)
            .context("No book content found")?
            .join(&url)
            .canonicalize()
            .map_err(|e| {
                anyhow::anyhow!("Failed to canonicalize file for page {:?}: {}", url, e)
            })?;

        // create the file if it doesn't exist
        if !md_file.exists() {
            std::fs::write(&md_file, "").map_err(|e| {
                anyhow::anyhow!(
                    "Failed to create file {:?} for page {:?}: {}",
                    md_file,
                    url,
                    e
                )
            })?;
        }

        let body = std::fs::read_to_string(&md_file).map_err(|e| {
            anyhow::anyhow!(
                "Failed to read file {:?} for page {:?}: {}",
                md_file,
                url,
                e
            )
        })?;

        let parser = pulldown_cmark::Parser::new(&body);

        let mut last_heading = None;

        let mut sections = Vec::new();

        parser.for_each(|event| match event {
            Event::Start(Tag::Heading(level, ..)) => {
                last_heading = Some(level);
            }
            Event::Text(text) => {
                if let Some(current_level) = &mut last_heading {
                    let anchor = text
                        .clone()
                        .into_string()
                        .trim()
                        .to_lowercase()
                        .replace(' ', "-");
                    sections.push(Section {
                        level: *current_level as usize,
                        title: text.to_string(),
                        id: anchor,
                    });

                    last_heading = None;
                }
            }
            _ => {}
        });

        let entry = self.pages.vacant_entry();
        let id = query::PageId(entry.key());
        entry.insert(Page {
            raw: body,
            segments: vec![],
            url: url.to_owned(),
            title: link.name.clone(),
            sections,
            id,
        });

        self.page_id_mapping.insert(url, id);

        for nested in link.nested_items.iter() {
            self.populate_page(mdbook_root.clone(), nested)?;
        }

        // proc_append_state("mdbook", &link.name).unwrap();
        Ok(())
    }

    // Insert a page via its path, autofilling the segments and title
    pub fn insert_page(&mut self, _path: PathBuf, markdown: String) {
        let parser = pulldown_cmark::Parser::new(&markdown);
        let mut out = String::new();
        pulldown_cmark::html::push_html(&mut out, parser);
    }
}

/// An id for a page
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub struct PageId(pub usize);
