use dioxus::prelude::*;

pub static NavBar: FC<()> = |cx| {
    cx.render(rsx! {
        header {
            a {
                href: "/"
                img { /*logo*/ }
                span {}
            }
            nav {
                a { href: "/community/support", "Community" }
                a { href: "/docs/getting-started", "Docs" }
                a { href: "/tutorial/tutorial", "Tutorial" }
                a { href: "/blog/", "Blog" }
            }
            form {}
            div {}
        }
    })
};
