fn main() {
    println!(
        "cargo:rustc-env=DIOXUS_ASSET_DIR={}",
        std::env::var("OUT_DIR").unwrap()
    )
}
