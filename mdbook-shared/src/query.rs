use crate::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct MdBook<R> {
    pub summary: Summary<R>,

    // rendered pages to HTML
    pub pages: HashMap<PathBuf, Page>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub title: String,

    pub url: PathBuf,

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

        // let parser = parser.map(|event| {
        //     //

        //     match event {
        //         pulldown_cmark::Event::Start(_) => todo!(),
        //         pulldown_cmark::Event::End(_) => todo!(),
        //         pulldown_cmark::Event::Text(_) => todo!(),
        //         pulldown_cmark::Event::Code(_) => todo!(),
        //         pulldown_cmark::Event::Html(_) => todo!(),
        //         pulldown_cmark::Event::FootnoteReference(_) => todo!(),
        //         pulldown_cmark::Event::SoftBreak => todo!(),
        //         pulldown_cmark::Event::HardBreak => todo!(),
        //         pulldown_cmark::Event::Rule => todo!(),
        //         pulldown_cmark::Event::TaskListMarker(_) => todo!(),
        //     }
        //     event
        // });

        pulldown_cmark::html::push_html(&mut content, parser);

        self.pages.insert(
            url.to_owned(),
            Page {
                content,
                segments: vec![],
                url: url.to_owned(),
                title: link.name.clone(),
                sections: vec![
                    Section {
                        title: "Section 1".to_owned(),
                        id: "section-1".to_owned(),
                    },
                    Section {
                        title: "Section 2".to_owned(),
                        id: "section-2".to_owned(),
                    },
                    Section {
                        title: "Section 3".to_owned(),
                        id: "section-3".to_owned(),
                    },
                    Section {
                        title: "Section 4".to_owned(),
                        id: "section-4".to_owned(),
                    },
                ],
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

impl<R: From<PathBuf>+Clone> MdBook<R> {
    fn get_page(&self, path: &PathBuf) -> Option<&Page> {
        self.pages.get(path)
    }
}