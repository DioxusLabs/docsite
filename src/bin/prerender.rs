use dioxus::prelude::*;
use dioxus_docs_site::*;

fn main() {
    let mut dom = VirtualDom::new_with_props(
        App,
        AppProps {
            route: "".to_string(),
        },
    );
    dom.rebuild();

    let out = dioxus::ssr::render_vdom(&dom, |c| c.pre_render(true));

    use std::fs::File;
    use std::io::Write;

    let mut file = File::create("index.html").unwrap();
    file.write_all(out.as_bytes()).unwrap();
}

static AppContainer: FC<()> = |(cx, props)| {
    cx.render(rsx! {
        div {
            "hello world!"
        }
        // head {}
        // body {
        //     App {
        //         route: "home".to_string()
        //     }
        // }
    })
};
