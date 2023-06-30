use crate::{search_index::{Embedding, SearchIndex}, *};
use pulldown_cmark::{Event, Tag};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash, path::PathBuf};

#[derive(Debug)]
pub struct MdBook<R>
where
    R: Hash + Eq,
{
    pub summary: Summary<R>,

    // rendered pages to HTML
    pub pages: HashMap<R, Page<R>>,

    // search index
    pub search_index: Option<SearchIndex<R>>,
}

impl<R: Hash + Eq + Clone> MdBook<R> {
    /// Build the search index for the book
    pub fn build_search_index(&mut self) {
        let search_index = SearchIndex::from_book(self);

        self.search_index = Some(search_index);
    }
}

#[derive(Debug)]
pub struct Page<R> {
    pub title: String,

    pub url: R,

    pub segments: Vec<String>,

    // the raw markdown
    pub raw: String,

    // headers
    pub sections: Vec<Section>,

    pub embedding: Embedding,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    pub level: usize,
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
            search_index: None,
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

        // create the embeddings
        #[cfg(features = "build_embeddings")]
        {
            let model = crate::load_model::download();
            for page in self.pages.values_mut() {
                let inference_parameters = llm::InferenceParameters::default();
                let new_embedding =
                    get_embeddings(model.as_ref(), &inference_parameters, page.title);
                let mut session = model.start_session(Default::default());
                let mut output_request = llm::OutputRequest {
                    all_logits: None,
                    embeddings: Some(Vec::new()),
                };
                let _ = session.feed_prompt(
                    model,
                    inference_parameters,
                    embed,
                    &mut output_request,
                    |_| Ok::<_, std::convert::Infallible>(llm::InferenceFeedback::Halt),
                );

                page.embedding = Embedding {
                    vector: output_request.embeddings.unwrap(),
                };
            }
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
                        .replace(" ", "-");
                    sections.push(Section {
                        level: *current_level as usize,
                        title: text.to_string(),
                        id: anchor.clone(),
                    });

                    last_heading = None;
                }
            }
            _ => {}
        });

        self.pages.insert(
            url.to_owned(),
            Page {
                raw: body,
                segments: vec![],
                url: url.to_owned(),
                title: link.name.clone(),
                sections,
                embedding: Embedding::default(),
            },
        );

        for nested in link.nested_items.iter() {
            self.populate_page(mdbook_root.clone(), &nested);
        }

        // proc_append_state("mdbook", &link.name).unwrap();
    }

    // Insert a page via its path, autofilling the segments and title
    pub fn insert_page(&mut self, _path: PathBuf, markdown: String) {
        let parser = pulldown_cmark::Parser::new(&markdown);
        let mut out = String::new();
        pulldown_cmark::html::push_html(&mut out, parser);
    }
}
