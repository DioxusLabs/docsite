fn main() {
    println!("cargo:rerun-if-changed=../../docs-src/blog");
    mdbook_gen::make_docs_from_ws("blog");
}
