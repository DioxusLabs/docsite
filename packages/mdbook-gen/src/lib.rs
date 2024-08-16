use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use convert_case::{Case, Casing};
use mdbook_shared::MdBook;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::LitStr;

use crate::transform_book::write_book_with_routes;

mod rsx;
mod transform_book;

/// Load an mdbook from the filesystem using the target tokens
/// ```ignore
///
///
/// ```
pub fn load_book_from_fs(
    input: LitStr,
) -> anyhow::Result<(PathBuf, mdbook_shared::MdBook<PathBuf>)> {
    let user_dir = input.value().parse::<PathBuf>()?;
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let path = manifest_dir.join(user_dir);
    let path = path.canonicalize().with_context(|| {
        anyhow::anyhow!(
            "Failed to canonicalize the path to the book at {:?}",
            path.display()
        )
    })?;

    Ok((path.clone(), MdBook::new(path)?))
}

pub fn generate_router_as_file(
    book_path: PathBuf,
    book: mdbook_shared::MdBook<PathBuf>,
) -> syn::File {
    let router = generate_router(book_path, book);

    syn::parse_quote! {
        #router
    }
}

pub fn generate_router(book_path: PathBuf, book: mdbook_shared::MdBook<PathBuf>) -> TokenStream2 {
    let mdbook = write_book_with_routes(book_path, &book);

    let book_pages = book.pages().iter().map(|(_, page)| {
        let name = path_to_route_variant(&page.url);

        // Rsx doesn't work very well in macros because the path for all the routes generated point to the same characters. We manually expand rsx here to get around that issue.
        match rsx::parse(page.url.clone(), &page.raw) {
            Ok(rsx) => {
                // for the sake of readability, we want to actuall convert the CallBody back to Tokens
                let rsx = rsx::callbody_to_tokens(rsx);

                quote! {
                    #[component(no_case_check)]
                    pub fn #name() -> dioxus::prelude::Element {
                        use dioxus::prelude::*;
                        rsx! {
                            #rsx
                        }
                    }
                }
            }
            Err(err) => err.to_compile_error(),
        }
    });

    let default_impl = book
        .pages()
        .iter()
        .min_by_key(|(_, page)| page.url.to_string_lossy().len())
        .map(|(_, page)| {
            let name = path_to_route_enum(&page.url);
            quote! {
                impl Default for BookRoute {
                    fn default() -> Self {
                        #name
                    }
                }
            }
        });

    let book_routes = book.pages().iter().map(|(_, page)| {
        let name = path_to_route_variant(&page.url);
        let route_without_extension = page.url.with_extension("");
        // remove any trailing "index"
        let route_without_extension = route_without_extension.to_string_lossy().to_string();
        let mut url = route_without_extension;
        if let Some(stripped) = url.strip_suffix("index") {
            url = stripped.to_string();
        }
        if let Some(stripped) = url.strip_suffix('/') {
            url = stripped.to_string();
        }
        if !url.starts_with('/') {
            url = format!("/{}", url);
        }
        quote! {
            #[route(#url)]
            #name {},
        }
    });

    let match_page_id = book.pages().iter().map(|(_, page)| {
        let id = page.id.0;
        let variant = path_to_route_enum(&page.url);
        quote! {
            #variant => use_mdbook::mdbook_shared::PageId(#id),
        }
    });

    quote! {
        #[derive(Clone, Copy, dioxus_router::prelude::Routable, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
        pub enum BookRoute {
            #(#book_routes)*
        }

        impl BookRoute {
            pub fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
                &self.page().sections
            }

            pub fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
                LAZY_BOOK.get_page(self)
            }

            pub fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
                match self {
                    #(
                        #match_page_id
                    )*
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
    let name = path_to_route_variant(path);
    quote! {
        BookRoute::#name {}
    }
}
