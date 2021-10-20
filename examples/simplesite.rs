use dioxus::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let mut dom = VirtualDom::new_with_props(App, AppProps {
        route: "home"
    });
    let out = dioxus::ssr::render_vdom(&dom, |c| c);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::web::launch_with_props(App, AppProps {
        route: "home"
    }, |c| c);
}

struct AppProps {
    route: &'static str
}

static App: FC<AppProps> = |(cx, props)| {
    //
    let content = match props.route.split("/").next().unwrap_or("home") {
        "posts" => rsx!(cx, Nav {}, Posts {}),
        _ =>  rsx!(cx, Header {}, RecentPost {}, Description {})
    };

    cx.render(rsx! {
        head {}
        body {
            div {
                class: "container"
                {content}
                footer {}
            }
        }
    })
};

static RecentPost: FC<()> = |(cx, props)| {
    rsx!(cx, div {
        
    })
};

static Description: FC<()> = |(cx, props)| {
    rsx!(cx, div {
        
    })
};

static Nav: FC<()> = |(cx, props)| {
    cx.render(rsx!{
        div {

        }
    })
};

static Header: FC<()> = |(cx, _)| {
    //
    let title = "";
    let tagline = "";

    cx.render(rsx! {
        header {
            img { src: "/images/lettre.png", style: "float: left; width: 8em; margin-right: 2em;", alt: "lettre logo", }
            div {
                h1 { "{title}" }
                p { "{tagline}" }
                div {
                    style: "font-size: 0.9rem;",
                    a {
                        href: "https://github.com/lettre/lettre",
                        "repository"
                    }
                    "⋅"
                    a {
                        href: "https://docs.rs/lettre",
                        "documentation"
                    }
                    "⋅"
                    a {
                        href: "https://github.com/lettre/lettre/discussions",
                        "discussions"
                    }
                    "⋅"
                    a {
                        href: "https://twitter.com/lettre_rs",
                        "twitter"
                    }
                }
            }
        }        
    })
};

static Posts: FC<()> = |(cx, _)| {
    //
    cx.render(rsx! {
        article {
            class: "posts"
        }
    })
};
