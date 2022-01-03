//! Pre-render the page so it loads faster.
//! In SPA, we have hydration enabled so we can pick up from the pre-rendered page.

use std::io::Write;

use dioxus::prelude::*;
use dioxus_docs_site::app;

fn main() {
    let mut dom = VirtualDom::new_with_props(app, ());
    dom.rebuild();

    let out = dioxus::ssr::render_vdom_cfg(&dom, |c| c.pre_render(true));

    let mut file = std::fs::File::create("prerender.html").unwrap();
    file.write_all(out.as_bytes()).unwrap();
}
