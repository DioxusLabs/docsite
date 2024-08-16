use std::{env::current_dir, path::PathBuf};

use mdbook_gen::{generate_router, generate_router_as_file};
use mdbook_shared::MdBook;

fn main() {
    // re-run only if the "example-book" directory changes
    // println!("cargo:rerun-if-changed=../../example-book");

    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let mdbook_dir = manifest_dir.join("./example-book").canonicalize().unwrap();

    let out_dir = current_dir().unwrap().join("gen");

    let file_src = generate_router_as_file(mdbook_dir.clone(), MdBook::new(mdbook_dir).unwrap());

    std::fs::create_dir_all(&out_dir).unwrap();

    let prettifed = prettyplease::unparse(&file_src);

    std::fs::write(out_dir.join("router.rs"), prettifed).unwrap();
}
