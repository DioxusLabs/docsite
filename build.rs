use std::{env::current_dir, path::PathBuf};

use mdbook_gen::generate_router;
use mdbook_shared::MdBook;

fn main() {
    // re-run only if the "example-book" directory changes
    println!("cargo:rerun-if-changed=docs-src");

    let mdbook_dir = PathBuf::from("docs-src/0.5");
    let out_dir = current_dir().unwrap().join("src/docs");

    let mut out = generate_router(mdbook_dir.clone(), MdBook::new(mdbook_dir).unwrap()).to_string();
    out.push_str("\n");
    out.push_str("use super::*;\n");

    std::fs::write(out_dir.join("router.rs"), out).unwrap();
}
