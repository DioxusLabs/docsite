use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use mdbook_shared::MdBook;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::LitStr;
use convert_case::{Case, Casing};

#[proc_macro]
pub fn include_mdbook(input: TokenStream) -> TokenStream {
    match syn::parse::<LitStr>(input).map(load_book_from_fs) {
        Ok(Ok(book)) => write_book(book),
        Ok(Err(err)) => write_book_err(err),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn mdbook_router(input: TokenStream) -> TokenStream {
    match syn::parse::<LitStr>(input).map(load_book_from_fs) {
        Ok(Ok(book)) => generate_router(book).into(),
        Ok(Err(err)) => write_book_err(err),
        Err(err) => err.to_compile_error().into(),
    }
}

fn write_book_err(err: anyhow::Error) -> TokenStream {
    let err = err.to_string();
    quote! { compile_error!(#err) }.to_token_stream().into()
}

fn write_book(book: mdbook_shared::MdBook<PathBuf>) -> TokenStream {
    let sum = serde_json::to_string(&book).unwrap();

    let out = quote! {
        ::once_cell::sync::Lazy::new(|| ::use_mdbook::static_load(#sum))
    };

    out.to_token_stream().into()
}

/// Load an mdbook from the filesystem using the target tokens
/// ```ignore
///
///
/// ```
fn load_book_from_fs(input: LitStr) -> anyhow::Result<mdbook_shared::MdBook<PathBuf>> {
    let user_dir = input.value().parse::<PathBuf>()?;
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    MdBook::new(manifest_dir.join(user_dir))
}

const STATE_DIR: &'static str = env!("DIOXUS_ASSET_DIR");

/// Returns the path of the internal file that would be used to
/// store state for the specified key, as a [PathBuf](std::path::PathBuf).
/// You should never use this directly unless you know what you're doing.
fn state_file_path(key: &str) -> PathBuf {
    let filename = format!("mdbook_asset_{}", key);
    let mut buf = PathBuf::new();
    buf.push(STATE_DIR);
    buf.push(filename.as_str());
    buf
}

fn proc_append_state(key: &str, value: &str) -> std::io::Result<()> {
    let value = format!("{}\n", value.replace("\n", "\\n"));
    let state_file = state_file_path(key);
    match OpenOptions::new()
        .append(true)
        .create(true)
        .open(state_file)
    {
        Ok(mut file) => return file.write_all(value.as_bytes()),
        Err(e) => Err(e),
    }
}

fn clear_assets_file(key: &str) -> std::io::Result<()> {
    let state_file = state_file_path(key);
    match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(state_file)
    {
        Ok(mut file) => return file.write_all("".as_bytes()),
        Err(e) => Err(e),
    }
}

fn generate_router(book: mdbook_shared::MdBook<PathBuf>) -> TokenStream2 {
    let book_pages = book.pages.values().map(|page| {
        let name = title_to_ident(&page.url);
        let content = &page.content;
        // Rsx doesn't work very well in macros because the path for all the routes generated point to the same characters. We manulally expand rsx here to get around that issue.
        let template_name = format!("{}:0:0:0", page.url.to_string_lossy());
        quote! {
            #[dioxus::prelude::inline_props]
            pub fn #name(cx: dioxus::prelude::Scope) -> dioxus::prelude::Element {
                use dioxus::prelude::*;
                Some({
                    let __cx = cx;
                    static TEMPLATE: ::dioxus::core::Template = ::dioxus::core::Template {
                        name: #template_name,
                        roots: &[
                            ::dioxus::core::TemplateNode::Element {
                                tag: dioxus_elements::div::TAG_NAME,
                                namespace: dioxus_elements::div::NAME_SPACE,
                                attrs: &[
                                    ::dioxus::core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::dangerous_inner_html.0,
                                        namespace: dioxus_elements::div::dangerous_inner_html.1,
                                        value: #content,
                                    },
                                ],
                                children: &[],
                            },
                        ],
                        node_paths: &[],
                        attr_paths: &[],
                    };
                    ::dioxus::core::VNode {
                        parent: None,
                        key: None,
                        template: std::cell::Cell::new(TEMPLATE),
                        root_ids: Default::default(),
                        dynamic_nodes: __cx.bump().alloc([]),
                        dynamic_attrs: __cx.bump().alloc([]),
                    }
                })
            }
        }
    });

    let book_routes = book.pages.values().map(|page| {
        let name = title_to_ident(&page.url);
        let mut url = page.url.to_string_lossy().to_string();
        if !url.starts_with("/"){
            url = format!("/{}", url);
        }
        quote! {
            #[route(#url)]
            #name {},
        }
    });

    let sections = book.pages.values().map(|page| {
        let name = title_to_ident(&page.url);
        let sections = page.sections.iter().map(|section| {
            let title = section.title.to_string();
            let id = section.id.to_string();
            quote! {
                use_mdbook::mdbook_shared::Section {
                    title: #title.to_string(),
                    id: #id.to_string(),
                }
            }
        });
        
        quote! {
            sections.insert(BookRoute::#name {}, Box::new([#(#sections),*]) as Box<[use_mdbook::mdbook_shared::Section]>);
        }
    });

    quote!{
        #(
            #book_pages
        )*

        #[derive(Clone, Copy, dioxus_router::prelude::Routable, PartialEq, Eq, Hash, Debug)]
        pub enum BookRoute {
            #(#book_routes)*
        }

        impl BookRoute {
            pub fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
                static SECTIONS: use_mdbook::Lazy<std::collections::HashMap<BookRoute, Box<[use_mdbook::mdbook_shared::Section]>>> = use_mdbook::Lazy::new(|| {
                    let mut sections = std::collections::HashMap::new();
                    #(
                        #sections
                    )*
                    sections
                });
                match SECTIONS.get(self) {
                    Some(sections) => sections,
                    None => &[],
                }
            }
        }
    }
}

fn title_to_ident(path: &Path) -> Ident {
    let path_without_extension = path.with_extension("");
    let mut title = String::new();
    for segment in path_without_extension.components() {
        title.push(' ');
        title.push_str(&segment.as_os_str().to_string_lossy());
    }
    let title_filtered_alphanumeric = title.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '_')
        .collect::<String>();
    Ident::new(&title_filtered_alphanumeric.to_case(Case::UpperCamel), Span::call_site())
}