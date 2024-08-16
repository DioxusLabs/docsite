use std::collections::{HashMap, VecDeque};
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use super::summary::{parse_summary, Link, SectionNumber, Summary, SummaryItem};
// use crate::build_opts::BuildOpts;
// use crate::config::Config;
use crate::errors::*;

/// Load a book into memory from its `src/` directory.
pub fn load_book<P: AsRef<Path>>(
    root_dir: P,
    cfg: &Config,
    build_opts: &BuildOpts,
) -> Result<LoadedBook> {
    if cfg.has_localized_dir_structure() {
        match build_opts.language_ident {
            // Build a single book's translation.
            Some(_) => Ok(LoadedBook::Single(load_single_book_translation(
                &root_dir,
                cfg,
                &build_opts.language_ident,
            )?)),
            // Build all available translations at once.
            None => {
                let mut translations = HashMap::new();
                for (lang_ident, _) in cfg.language.0.iter() {
                    let book =
                        load_single_book_translation(&root_dir, cfg, &Some(lang_ident.clone()))?;
                    translations.insert(lang_ident.clone(), book);
                }
                Ok(LoadedBook::Localized(LocalizedBooks(translations)))
            }
        }
    } else {
        Ok(LoadedBook::Single(load_single_book_translation(
            &root_dir, cfg, &None,
        )?))
    }
}

fn load_single_book_translation<P: AsRef<Path>>(
    root_dir: P,
    cfg: &Config,
    language_ident: &Option<String>,
) -> Result<Book> {
    let localized_src_dir = root_dir
        .as_ref()
        .join(cfg.get_localized_src_path(language_ident.as_ref()).unwrap());
    let fallback_src_dir = root_dir.as_ref().join(cfg.get_fallback_src_path());

    let summary_md = localized_src_dir.join("SUMMARY.md");

    let mut summary_content = String::new();
    File::open(&summary_md)
        .with_context(|| {
            format!(
                "Couldn't open SUMMARY.md in {:?} directory",
                localized_src_dir
            )
        })?
        .read_to_string(&mut summary_content)?;

    let summary = parse_summary(&summary_content)
        .with_context(|| format!("Summary parsing failed for file={:?}", summary_md))?;

    if cfg.build.create_missing {
        create_missing(&localized_src_dir, &summary)
            .with_context(|| "Unable to create missing chapters")?;
    }

    load_book_from_disk(&summary, localized_src_dir, fallback_src_dir, cfg)
}

fn create_missing(src_dir: &Path, summary: &Summary) -> Result<()> {
    let mut items: Vec<_> = summary
        .prefix_chapters
        .iter()
        .chain(summary.numbered_chapters.iter())
        .chain(summary.suffix_chapters.iter())
        .collect();

    while !items.is_empty() {
        let next = items.pop().expect("already checked");

        if let SummaryItem::Link(ref link) = *next {
            if let Some(ref location) = link.location {
                let filename = src_dir.join(location);
                if !filename.exists() {
                    create_missing_link(&filename, link)?;
                }
            }

            items.extend(&link.nested_items);
        }
    }

    Ok(())
}

fn create_missing_link(filename: &Path, link: &Link) -> Result<()> {
    if let Some(parent) = filename.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| {
                Error::from(format!(
                    "Unable to create missing directory {:?}: {}",
                    parent, e
                ))
            })?;
        }
    }
    debug!("Creating missing file {}", filename.display());

    let mut f = File::create(&filename)?;
    writeln!(f, "# {}", link.name)?;

    Ok(())
}

/// A dumb tree structure representing a book.
///
/// For the moment a book is just a collection of [`BookItems`] which are
/// accessible by either iterating (immutably) over the book with [`iter()`], or
/// recursively applying a closure to each section to mutate the chapters, using
/// [`for_each_mut()`].
///
///
/// [`iter()`]: #method.iter
/// [`for_each_mut()`]: #method.for_each_mut
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Book {
    /// The sections in this book.
    pub sections: Vec<BookItem>,
    /// Chapter title overrides for this book.
    #[serde(default)]
    pub chapter_titles: HashMap<PathBuf, String>,
    __non_exhaustive: (),
}

impl Book {
    /// Create an empty book.
    pub fn new() -> Self {
        Default::default()
    }

    /// Get a depth-first iterator over the items in the book.
    pub fn iter(&self) -> BookItems<'_> {
        BookItems {
            items: self.sections.iter().collect(),
        }
    }

    /// Recursively apply a closure to each item in the book, allowing you to
    /// mutate them.
    ///
    /// # Note
    ///
    /// Unlike the `iter()` method, this requires a closure instead of returning
    /// an iterator. This is because using iterators can possibly allow you
    /// to have iterator invalidation errors.
    pub fn for_each_mut<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut BookItem),
    {
        for_each_mut(&mut func, &mut self.sections);
    }

    /// Append a `BookItem` to the `Book`.
    pub fn push_item<I: Into<BookItem>>(&mut self, item: I) -> &mut Self {
        self.sections.push(item.into());
        self
    }
}

pub fn for_each_mut<'a, F, I>(func: &mut F, items: I)
where
    F: FnMut(&mut BookItem),
    I: IntoIterator<Item = &'a mut BookItem>,
{
    for item in items {
        if let BookItem::Chapter(ch) = item {
            for_each_mut(func, &mut ch.sub_items);
        }

        func(item);
    }
}

/// A collection of `Books`, each one a single localization.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LocalizedBooks(pub HashMap<String, Book>);

impl LocalizedBooks {
    /// Get a depth-first iterator over the items in the book.
    pub fn iter(&self) -> BookItems<'_> {
        let mut items = VecDeque::new();

        for (_, book) in self.0.iter() {
            items.extend(book.iter().items);
        }

        BookItems { items: items }
    }

    /// Recursively apply a closure to each item in the book, allowing you to
    /// mutate them.
    ///
    /// # Note
    ///
    /// Unlike the `iter()` method, this requires a closure instead of returning
    /// an iterator. This is because using iterators can possibly allow you
    /// to have iterator invalidation errors.
    pub fn for_each_mut<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut BookItem),
    {
        for (_, book) in self.0.iter_mut() {
            book.for_each_mut(&mut func);
        }
    }
}

/// A book which has been loaded and is ready for rendering.
///
/// This exists because the result of loading a book directory can be multiple
/// books, each one representing a separate translation, or a single book with
/// no translations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoadedBook {
    /// The book was loaded with all translations.
    Localized(LocalizedBooks),
    /// The book was loaded without any additional translations.
    Single(Book),
}

impl LoadedBook {
    /// Get a depth-first iterator over the items in the book.
    pub fn iter(&self) -> BookItems<'_> {
        match self {
            LoadedBook::Localized(books) => books.iter(),
            LoadedBook::Single(book) => book.iter(),
        }
    }

    /// Recursively apply a closure to each item in the book, allowing you to
    /// mutate them.
    ///
    /// # Note
    ///
    /// Unlike the `iter()` method, this requires a closure instead of returning
    /// an iterator. This is because using iterators can possibly allow you
    /// to have iterator invalidation errors.
    pub fn for_each_mut<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut BookItem),
    {
        match self {
            LoadedBook::Localized(books) => books.for_each_mut(&mut func),
            LoadedBook::Single(book) => book.for_each_mut(&mut func),
        }
    }

    /// Returns one of the books loaded. Used for compatibility.
    pub fn first(&self) -> &Book {
        match self {
            LoadedBook::Localized(books) => books.0.iter().next().unwrap().1,
            LoadedBook::Single(book) => &book,
        }
    }
}

/// Enum representing any type of item which can be added to a book.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BookItem {
    /// A nested chapter.
    Chapter(Chapter),
    /// A section separator.
    Separator,
    /// A part title.
    PartTitle(String),
}

impl From<Chapter> for BookItem {
    fn from(other: Chapter) -> BookItem {
        BookItem::Chapter(other)
    }
}

/// The representation of a "chapter", usually mapping to a single file on
/// disk however it may contain multiple sub-chapters.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Chapter<R> {
    /// The chapter's name.
    pub name: String,
    /// The chapter's contents.
    pub content: String,
    /// The chapter's section number, if it has one.
    pub number: Option<SectionNumber>,
    /// Nested items.
    pub sub_items: Vec<BookItem>,
    /// The chapter's route
    pub path: Option<R>,
    /// An ordered list of the names of each chapter above this one in the hierarchy.
    pub parent_names: Vec<String>,
}

impl<R> Chapter<R> {
    /// Create a new chapter with the provided content.
    pub fn new<P: Into<R>>(
        name: &str,
        content: String,
        p: P,
        parent_names: Vec<String>,
    ) -> Chapter {
        let path: R = p.into();
        Chapter {
            name: name.to_string(),
            content,
            path: Some(path.clone()),
            parent_names,
            ..Default::default()
        }
    }

    /// Create a new draft chapter that is not attached to a source markdown file (and thus
    /// has no content).
    pub fn new_draft(name: &str, parent_names: Vec<String>) -> Self {
        Chapter {
            name: name.to_string(),
            content: String::new(),
            path: None,
            parent_names,
            ..Default::default()
        }
    }

    /// Check if the chapter is a draft chapter, meaning it has no path to a source markdown file.
    pub fn is_draft_chapter(&self) -> bool {
        self.path.is_none()
    }
}

/// Use the provided `Summary` to load a `Book` from disk.
///
/// You need to pass in the book's source directory because all the links in
/// `SUMMARY.md` give the chapter locations relative to it.
pub(crate) fn load_book_from_disk<P: AsRef<Path>>(
    summary: &Summary,
    localized_src_dir: P,
    fallback_src_dir: P,
    cfg: &Config,
) -> Result<Book> {
    debug!("Loading the book from disk");

    let prefix = summary.prefix_chapters.iter();
    let numbered = summary.numbered_chapters.iter();
    let suffix = summary.suffix_chapters.iter();

    let summary_items = prefix.chain(numbered).chain(suffix);

    let mut chapters = Vec::new();

    for summary_item in summary_items {
        let chapter = load_summary_item(
            summary_item,
            localized_src_dir.as_ref(),
            fallback_src_dir.as_ref(),
            Vec::new(),
            cfg,
        )?;
        chapters.push(chapter);
    }

    Ok(Book {
        sections: chapters,
        chapter_titles: HashMap::new(),
        __non_exhaustive: (),
    })
}

fn load_summary_item<P: AsRef<Path> + Clone>(
    item: &SummaryItem,
    localized_src_dir: P,
    fallback_src_dir: P,
    parent_names: Vec<String>,
    cfg: &Config,
) -> Result<BookItem> {
    match item {
        SummaryItem::Separator => Ok(BookItem::Separator),
        SummaryItem::Link(ref link) => {
            load_chapter(link, localized_src_dir, fallback_src_dir, parent_names, cfg)
                .map(BookItem::Chapter)
        }
        SummaryItem::PartTitle(title) => Ok(BookItem::PartTitle(title.clone())),
    }
}

fn load_chapter<P: AsRef<Path>>(
    link: &Link,
    localized_src_dir: P,
    fallback_src_dir: P,
    parent_names: Vec<String>,
    cfg: &Config,
) -> Result<Chapter> {
    let src_dir_localized = localized_src_dir.as_ref();
    let src_dir_fallback = fallback_src_dir.as_ref();

    let mut ch = if let Some(ref link_location) = link.location {
        debug!("Loading {} ({})", link.name, link_location.display());

        let mut src_dir = src_dir_localized;
        let mut location = if link_location.is_absolute() {
            link_location.clone()
        } else {
            src_dir.join(link_location)
        };

        if !location.exists() && !link_location.is_absolute() {
            src_dir = src_dir_fallback;
            location = src_dir.join(link_location);
            debug!(
                "Falling back to default translation in path \"{}\"",
                location.display()
            );
        }
        if !location.exists() && cfg.build.create_missing {
            create_missing_link(&location, &link)
                .with_context(|| "Unable to create missing link reference")?;
        }

        let mut f = File::open(&location)
            .with_context(|| format!("Chapter file not found, {}", link_location.display()))?;

        let mut content = String::new();
        f.read_to_string(&mut content).with_context(|| {
            format!("Unable to read \"{}\" ({})", link.name, location.display())
        })?;

        if content.as_bytes().starts_with(b"\xef\xbb\xbf") {
            content.replace_range(..3, "");
        }

        let stripped = location
            .strip_prefix(&src_dir)
            .expect("Chapters are always inside a book");

        Chapter::new(&link.name, content, stripped, parent_names.clone())
    } else {
        Chapter::new_draft(&link.name, parent_names.clone())
    };

    let mut sub_item_parents = parent_names;

    ch.number = link.number.clone();

    sub_item_parents.push(link.name.clone());
    let sub_items = link
        .nested_items
        .iter()
        .map(|i| {
            load_summary_item(
                i,
                src_dir_localized,
                src_dir_fallback,
                sub_item_parents.clone(),
                cfg,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    ch.sub_items = sub_items;

    Ok(ch)
}

/// A depth-first iterator over the items in a book.
///
/// # Note
///
/// This struct shouldn't be created directly, instead prefer the
/// [`Book::iter()`] method.
pub struct BookItems<'a> {
    items: VecDeque<&'a BookItem>,
}

impl<'a> Iterator for BookItems<'a> {
    type Item = &'a BookItem;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.items.pop_front();

        if let Some(&BookItem::Chapter(ref ch)) = item {
            // if we wanted a breadth-first iterator we'd `extend()` here
            for sub_item in ch.sub_items.iter().rev() {
                self.items.push_front(sub_item);
            }
        }

        item
    }
}

impl Display for Chapter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref section_number) = self.number {
            write!(f, "{} ", section_number)?;
        }

        write!(f, "{}", self.name)
    }
}
