use std::{env::current_dir, path::PathBuf};

use mdbook_gen::generate_router_as_file;
use mdbook_shared::MdBook;

fn main() {
    // re-run only if the "example-book" directory changes
    println!("cargo:rerun-if-changed=../../example-book");

    let manifest_dir =
        PathBuf::from("/Users/jonkelley/Development/ecosystem-dioxus/include_mdbook/");

    let mdbook_dir = manifest_dir.join("./example-book").canonicalize().unwrap();
    let out_dir = current_dir().unwrap().join("gen");
    let file_src = generate_router_as_file(mdbook_dir.clone(), MdBook::new(mdbook_dir).unwrap());
    std::fs::create_dir_all(&out_dir).unwrap();
    let prettifed = prettyplease::unparse(&file_src);
    let as_file = syn::parse_file(&prettifed).unwrap();
    let fmts = dioxus_autofmt::try_fmt_file(&prettifed, &as_file, Default::default()).unwrap();
    let out = dioxus_autofmt::apply_formats(&prettifed, fmts);

    std::fs::write(out_dir.join("router.rs"), out).unwrap();
}
