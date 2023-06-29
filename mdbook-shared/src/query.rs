use crate::*;
use pulldown_cmark::{CowStr, Event, Tag};
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap, hash::Hash, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct MdBook<R>
where
    R: Hash + Eq,
{
    pub summary: Summary<R>,

    // rendered pages to HTML
    pub pages: HashMap<R, Page<R>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page<R> {
    pub title: String,

    pub url: R,

    pub segments: Vec<String>,

    // rendered as HTML
    pub content: String,

    // headers
    pub sections: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    pub title: String,
    pub id: String,
}

impl MdBook<PathBuf> {
    pub fn new(mdbook_root: PathBuf) -> anyhow::Result<Self> {
        let buf = mdbook_root.join("SUMMARY.md").canonicalize()?;

        let summary = std::fs::read_to_string(buf)?;
        let summary = parse_summary(&summary)?;

        let mut book = Self {
            summary,
            pages: HashMap::new(),
        };

        book.populate(mdbook_root);

        Ok(book)
    }

    pub fn populate(&mut self, mdbook_root: PathBuf) {
        let summary = self.summary.clone();

        let chapters = summary
            .prefix_chapters
            .iter()
            .chain(summary.numbered_chapters.iter())
            .chain(summary.suffix_chapters.iter());

        for chapter in chapters {
            self.populate_page(mdbook_root.clone(), chapter);
        }
    }

    fn populate_page(&mut self, mdbook_root: PathBuf, chapter: &SummaryItem<PathBuf>) {
        let Some(link) = chapter.maybe_link() else { return };

        let url = link.location.as_ref().cloned().unwrap();
        let md_file = mdbook_root.join("en").join(&url).canonicalize();

        let md_file = match md_file {
            Ok(f) => f,
            Err(e) => panic!("Failed to include mdbook - invalid path: {}", e),
        };

        let body = std::fs::read_to_string(md_file).unwrap();
        let mut content = String::new();

        let parser = pulldown_cmark::Parser::new(&body);

        let mut last_heading = None;

        let mut sections = Vec::new();

        let parser = parser.filter_map(|event| match event {
            Event::Start(Tag::Heading(level,..)) => {
                last_heading = Some(level);
                None
            }
            Event::Text(text) => {
                if let Some(current_level) = &mut last_heading {
                    let anchor = text
                            .clone()
                            .into_string()
                            .trim()
                            .to_lowercase()
                            .replace(" ", "-");
                    sections.push(Section {
                        title: text.to_string(),
                        id: anchor.clone(),
                    });
                    let event = Event::Html(CowStr::from(format!(
                        r##"<{current_level} id="{anchor}"><a class="header" href="#{anchor}">{text}</a>"##,
                    )))
                    .into();
                    last_heading = None;
                    return event;
                }
                Some(Event::Text(text))
            }
            _ => Some(event),
        });

        pulldown_cmark::html::push_html(&mut content, parser);

        self.pages.insert(
            url.to_owned(),
            Page {
                content,
                segments: vec![],
                url: url.to_owned(),
                title: link.name.clone(),
                sections,
            },
        );

        for nested in link.nested_items.iter() {
            self.populate_page(mdbook_root.clone(), &nested);
        }

        // proc_append_state("mdbook", &link.name).unwrap();
    }

    // Insert a page via its path, autofilling the segments and title
    pub fn insert_page(&mut self, path: PathBuf, markdown: String) {
        let parser = pulldown_cmark::Parser::new(&markdown);
        let mut out = String::new();
        pulldown_cmark::html::push_html(&mut out, parser);
    }
}

impl<R> MdBook<R>
where
    R: Hash + Eq,
{
    fn get_page(&self, path: &R) -> Option<&Page<R>> {
        self.pages.get(path)
    }
}
