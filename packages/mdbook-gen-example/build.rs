use std::{env::current_dir, path::PathBuf};

fn main() {
    // re-run only if the "example-book" directory changes
    println!("cargo:rerun-if-changed=example-book");

    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let mdbook_dir = manifest_dir.join("./example-book").canonicalize().unwrap();
    let out_dir = current_dir().unwrap().join("src");

    let mut out = mdbook_gen::generate_router_build_script(mdbook_dir);
    out.push_str("\n");
    out.push_str("use super::*;\n");

    std::fs::write(out_dir.join("router.rs"), out).unwrap();
}
