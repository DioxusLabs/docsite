use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use convert_case::{Case, Casing};
use mdbook_shared::MdBook;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use quote::ToTokens;
use syn::LitStr;

use crate::transform_book::write_book_with_routes;

mod rsx;
mod transform_book;

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
        Ok(mut file) => return file.write_all(&[]),
        Err(e) => Err(e),
    }
}

fn generate_router(book: mdbook_shared::MdBook<PathBuf>) -> TokenStream2 {
    let mdbook = write_book_with_routes(&book);

    let book_pages = book.pages.values().map(|page| {
        let name = path_to_route_variant(&page.url);
        // Rsx doesn't work very well in macros because the path for all the routes generated point to the same characters. We manulally expand rsx here to get around that issue.
        let template_name = format!("{}:0:0:0", page.url.to_string_lossy());
        let rsx = rsx::parse(page.url.clone(), &page.raw).render_with_location(template_name);
        quote! {
            #[dioxus::prelude::inline_props]
            pub fn #name(cx: dioxus::prelude::Scope) -> dioxus::prelude::Element {
                use dioxus::prelude::*;
                cx.render(#rsx)
            }
        }
    });

    let default_impl = book
        .pages
        .values()
        .min_by_key(|page| page.url.to_string_lossy().len())
        .map(|page| {
            let name = path_to_route_enum(&page.url);
            quote! {
                impl Default for BookRoute {
                    fn default() -> Self {
                        #name
                    }
                }
            }
        });

    let book_routes = book.pages.values().map(|page| {
        let name = path_to_route_variant(&page.url);
        let route_without_extension = page.url.with_extension("");
        // remove any trailing "index"
        let route_without_extension = route_without_extension
            .to_string_lossy()
            .trim_end_matches("index")
            .to_string();
        let mut url = route_without_extension;
        if !url.starts_with("/") {
            url = format!("/{}", url);
        }
        quote! {
            #[route(#url)]
            #name {},
        }
    });

    let sections =book.pages.values().map(|page| {
        let variant = path_to_route_enum(&page.url);
        let sections = page.sections.iter().map(|section| {
            let title = section.title.to_string();
            let id = section.id.to_string();
            let level = section.level;
            quote! {
                use_mdbook::mdbook_shared::Section {
                    title: #title.to_string(),
                    id: #id.to_string(),
                    level: #level,
                }
            }
        });

        quote! {
            sections.insert(#variant, Box::new([#(#sections),*]) as Box<[use_mdbook::mdbook_shared::Section]>);
        }
    });

    quote! {
        #[derive(Clone, Copy, dioxus_router::prelude::Routable, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
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

        #default_impl

        pub static LAZY_BOOK: use_mdbook::Lazy<use_mdbook::mdbook_shared::MdBook<BookRoute>> = use_mdbook::Lazy::new(|| {
            #mdbook
        });

        #(
            #book_pages
        )*
    }
}

pub(crate) fn path_to_route_variant(path: &Path) -> Ident {
    let path_without_extension = path.with_extension("");
    let mut title = String::new();
    for segment in path_without_extension.components() {
        title.push(' ');
        title.push_str(&segment.as_os_str().to_string_lossy());
    }
    let title_filtered_alphanumeric = title
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '_')
        .collect::<String>();
    Ident::new(
        &title_filtered_alphanumeric.to_case(Case::UpperCamel),
        Span::call_site(),
    )
}

pub(crate) fn path_to_route_enum(path: &Path) -> TokenStream2 {
    let name = path_to_route_variant(&path);
    quote! {
        BookRoute::#name {}
    }
}
