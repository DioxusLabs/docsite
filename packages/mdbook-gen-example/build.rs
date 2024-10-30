use std::{env::current_dir, path::PathBuf};

fn main() {
    // re-run only if the "example-book" directory changes
    // println!("cargo:rerun-if-changed=../../example-book");
    // println!("cargo:rerun-if-changed=../mdbook-gen/");

    let manifest_dir =
        PathBuf::from("/Users/jonkelley/Development/ecosystem-dioxus/include_mdbook/");

    let mdbook_dir = manifest_dir.join("./example-book").canonicalize().unwrap();
    let out_dir = current_dir().unwrap().join("gen");

    let out = mdbook_gen::generate_router_build_script(mdbook_dir);

    std::fs::write(out_dir.join("router.rs"), out).unwrap();
}
