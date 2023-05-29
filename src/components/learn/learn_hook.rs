use dioxus::prelude::ScopeState;
use include_dir::Dir;

use mdbook_shared::{parse_summary, Summary};

pub struct Book {
    pub summary: Summary,
}

static BOOK: include_dir::Dir = include_dir::include_dir!("docs");

pub fn use_mdbook(cx: &ScopeState) -> &mut Book {
    cx.use_hook(|| Book::new(&BOOK).unwrap())
}

impl Book {
    fn new(dir: &Dir) -> anyhow::Result<Self> {
        let root = dir
            .get_file("index.md")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string();

        let summary = dir
            .get_file("SUMMARY.md")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string();

        Ok(Book {
            summary: parse_summary(&summary)?,
        })
    }
}

#[test]
fn mdbook_parses() {
    let book = Book::new(&BOOK).unwrap();

    dbg!(&book.summary);
}
