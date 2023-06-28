// Build with:
// dioxus build --release --features web --example fullstack
// cargo run --example fullstack --features ssr --release

#[allow(unused)]

use dioxus_docs_site::*;

fn main() {
    dioxus_fullstack::launch_router!(@([127, 0, 0, 1], 8080), Route, {
        incremental,
    });
}
