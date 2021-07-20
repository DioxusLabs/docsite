use dioxus::prelude::*;

pub mod components;
pub mod icons;
pub mod sitemap;
pub mod snippets;
pub mod utils;

fn main() {
    let mut dom = VirtualDom::new(App);
    dioxus::ssr::render_vdom(&mut dom);
}

static App: FC<()> = |cx| {
    let url = use_state(cx, || "");

    use components::*;
    let body = match *url {
        "community" => rsx!(in cx, Community {}),
        "tutorial" => rsx!(in cx, Tutorial {}),
        "blog" => rsx!(in cx, Blog {}),
        "docs" => rsx!(in cx, Docs {}),
        _ => rsx!(in cx, Home {}),
    };

    cx.render(rsx! {
        div {
            NavBar {}
            {body}
            Footer {}
        }
    })
};
