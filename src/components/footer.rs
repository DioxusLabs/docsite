use crate::icons::*;
use crate::sitemap::SECTIONS;
use dioxus::prelude::*;

pub fn Footer(cx: Context<()>) -> DomTree {
    let sections = SECTIONS.iter().map(|(section, raw_links)| {
        let links = raw_links.iter().map(|(link_name, href)| {
            rsx! (
                a { href: "{href}",
                    "{link_name}",
                    {href.starts_with("http").then(|| rsx!( ExternalLinkIcon {} ))}
                }
            )
        });
        rsx! {
            div {
                div { "{section}" }
                {links}
            }
        }
    });

    cx.render(rsx! {
        footer {
            div {
                div {
                    div {
                        {sections}
                    }
                    section {
                        a {
                            img {}
                        }
                        p {}
                    }
                }
            }
        }
    })
}
