use mdbook_gen::generate_router_build_script;
use std::{env::current_dir, path::PathBuf};

fn main() {
    // re-run only if the "example-book" directory changes
    println!("cargo:rerun-if-changed=docs-src");

    // make_docs("0.5");
    make_docs("0.6");
}

fn make_docs(version: &str) {
    let mdbook_dir = PathBuf::from("docs-src").join(version);
    let out_dir = current_dir().unwrap().join("src/docs");
    let mut out = generate_router_build_script(mdbook_dir);
    out.push_str("\n");
    out.push_str("use super::*;\n");

    let version_flattened = version.replace(".", "");
    let filename = format!("router_{version_flattened}.rs");

    std::fs::write(out_dir.join(filename), out).unwrap();
}
