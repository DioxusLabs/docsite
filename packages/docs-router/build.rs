use mdbook_gen::generate_router_build_script;
use std::{env::current_dir, path::PathBuf};

fn main() {
    // re-run only if the "example-book" directory changes
    println!("cargo:rerun-if-changed=../../docs-src/");
    println!("cargo:rerun-if-changed=src/doc_examples/");

    make_docs("blog");
    make_docs("0.3");
    make_docs("0.4");
    make_docs("0.5");
    make_docs("0.6");
    make_docs("0.7");
}

fn make_docs(version: &str) {
    let mdbook_dir = PathBuf::from("../../docs-src").join(version);
    let out_dir = current_dir().unwrap().join("src/docs");
    let mut out = generate_router_build_script(mdbook_dir);
    out.push_str("\nuse super::*;\n");

    let version_flattened = version.replace(".", "");
    let filename = format!("router_{version_flattened}.rs");

    std::fs::write(out_dir.join(filename), out).unwrap();
}
