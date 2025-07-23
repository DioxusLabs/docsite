fn main() {
    println!("cargo:rerun-if-changed=../../docs-src/0.7");
    mdbook_gen::make_docs_from_ws("0.7");
}
