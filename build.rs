use std::{env::current_dir, path::PathBuf};

use mdbook_gen::{generate_router_as_file, generate_router};
use mdbook_shared::MdBook;

fn main() {
    // re-run only if the "example-book" directory changes
    println!("cargo:rerun-if-changed=docs-src");

    let mdbook_dir = PathBuf::from("docs-src/0.5");
    let out_dir = current_dir().unwrap().join("src/docs");
    // let out = mdbook_gen::generate_router_build_script(mdbook_dir);
    // let router = generate_router(book_path, book);

    let mut out = generate_router(mdbook_dir.clone(), MdBook::new(mdbook_dir).unwrap());
    let mut out = out.to_string();

    // let mut out = generate_router_as_file(mdbook_dir.clone(), MdBook::new(mdbook_dir).unwrap());
    // let mut out = prettyplease::unparse(&file_src);
    out.push_str("\n");
    out.push_str("use super::*;\n");

    std::fs::write(out_dir.join("router.rs"), out).unwrap();
}
