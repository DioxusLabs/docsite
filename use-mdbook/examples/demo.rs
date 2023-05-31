use use_mdbook::{include_mdbook, LazyMdbook};

// Loading mdbooks with the macro is lazy - it only happens when the LazyMdbook is used.
// Hence the type "LazyMdbook" is a Lazy<MdBook> and not a MdBook.
// The macro will do all the processing at compile time, so the LazyMdbook is a static and pre-processed
static DOCS: LazyMdbook = include_mdbook!("../example-book");

fn main() {
    println!("{:#?}", DOCS.summary);
}
