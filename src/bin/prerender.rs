use dioxus::prelude::*;
use dioxus_docs_site::*;

fn main() {
    let mut dom = VirtualDom::new(AppContainer);
    dom.rebuild_in_place();

    let out = dioxus::ssr::render_vdom(&dom, |c| c);

    use std::fs::File;
    use std::io::Write;

    let mut file = File::create("index.html").unwrap();
    file.write_all(out.as_bytes()).unwrap();
}

static AppContainer: FC<()> = |cx| {
    cx.render(rsx! {
        head {}
        body {
            App {
                route: "home".to_string()
            }
        }
    })
};
