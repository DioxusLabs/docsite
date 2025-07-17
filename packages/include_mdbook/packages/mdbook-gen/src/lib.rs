use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use convert_case::{Case, Casing};
use mdbook_shared::MdBook;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use syn::LitStr;

use crate::transform_book::write_book_with_routes;

mod rsx;
mod transform_book;

pub fn make_docs_from_ws(version: &str) {
    let mdbook_dir = PathBuf::from("../../docs-src").join(version);
    let out_dir = std::env::current_dir().unwrap().join("src");
    let mut out = generate_router_build_script(mdbook_dir);
    out.push_str("use dioxus_docs_examples::*;\n");
    out.push_str("use dioxus::prelude::*;\n");
    let version_flattened = version.replace(".", "");
    let filename = format!("docsgen.rs");
    std::fs::write(out_dir.join(filename), out).unwrap();
}


/// Generate the contents of the mdbook from a router
pub fn generate_router_build_script(mdbook_dir: PathBuf) -> String {
    let file_src = generate_router_as_file(mdbook_dir.clone(), MdBook::new(mdbook_dir).unwrap());
    prettyplease::unparse(&file_src)
}

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
    mdbook_dir: PathBuf,
    book: mdbook_shared::MdBook<PathBuf>,
) -> syn::File {
    let router = generate_router(mdbook_dir, book);

    syn::parse_quote! {
        #router
    }
}

pub fn generate_router(mdbook_dir: PathBuf, book: mdbook_shared::MdBook<PathBuf>) -> TokenStream2 {
    let mdbook = write_book_with_routes(&book);

    let mut page_markdown_map = HashMap::new();

    let book_pages = book.pages().iter().map(|(_, page)| {
        let name = path_to_route_variant(&page.url).unwrap();

        // Rsx doesn't work very well in macros because the path for all the routes generated point to the same characters. We manually expand rsx here to get around that issue.
        match rsx::parse_markdown(mdbook_dir.clone(), page.url.clone(), &page.raw) {
            Ok(parsed) => {
                // insert the parsed markdown into the page_markdown map
                page_markdown_map.insert(page.id.0, parsed.resolved_markdown);

                // for the sake of readability, we want to actually convert the CallBody back to Tokens
                let rsx = rsx::callbody_to_tokens(parsed.body);

                // Create the fragment enum for the section
                let section_enum = path_to_route_section(&page.url).unwrap();
                let mut error_message = format!("Invalid section name. Expected one of {}", section_enum);
                for (i, section) in parsed.sections.iter().enumerate() {
                    if i > 0 {
                        error_message.push_str(", ");
                    }
                    error_message.push_str(&section.fragment());
                }
                let section_idents: Vec<_> = parsed
                    .sections
                    .iter()
                    .filter_map(|section| Some(Ident::new(&section.variant().ok()?, Span::call_site())))
                    .collect();
                let section_names: Vec<_> = parsed
                    .sections
                    .iter()
                    .map(|section| section.fragment())
                    .collect();
                let fragment = quote! {
                    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize)]
                    pub enum #section_enum {
                        #[default]
                        Empty,
                        #(#section_idents),*
                    }

                    impl std::str::FromStr for #section_enum {
                        type Err = &'static str;

                        fn from_str(s: &str) -> Result<Self, Self::Err> {
                            match s {
                                "" => Ok(Self::Empty),
                                #(
                                    #section_names => Ok(Self::#section_idents),
                                )*
                                _ => Err(#error_message)
                            }
                        }
                    }

                    impl std::fmt::Display for #section_enum {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            match self {
                                Self::Empty => f.write_str(""),
                                #(
                                    Self::#section_idents => f.write_str(#section_names),
                                )*
                            }
                        }
                    }
                };

                quote! {
                    #fragment

                    #[component(no_case_check)]
                    pub fn #name(section: #section_enum) -> Element {
                        rsx! {
                            #rsx
                        }
                    }
                }
            }
            Err(err) => err.to_compile_error(),
        }
    });
    let book_pages = quote! {#(#book_pages)*};

    let default_impl = book
        .pages()
        .iter()
        .min_by_key(|(_, page)| page.url.to_string_lossy().len())
        .map(|(_, page)| {
            let name = path_to_route_enum(&page.url).unwrap();
            quote! {
                impl Default for BookRoute {
                    fn default() -> Self {
                        #name
                    }
                }
            }
        });

    let book_routes = book.pages().iter().map(|(_, page)| {
        let name = path_to_route_variant(&page.url).unwrap();
        let section = path_to_route_section(&page.url).unwrap();
        let route_without_extension = page.url.with_extension("");
        // remove any trailing "index"
        let route_without_extension = route_without_extension.to_string_lossy().to_string();
        let mut url = route_without_extension;
        if let Some(stripped) = url.strip_suffix("index") {
            url = stripped.to_string();
        }
        if !url.starts_with('/') {
            url = format!("/{}", url);
        }
        url += "#:section";
        quote! {
            #[route(#url)]
            #name {
                section: #section
            },
        }
    });

    let match_page_id = book.pages().iter().map(|(_, page)| {
        let id = page.id.0;
        let variant = path_to_route_variant(&page.url).unwrap();
        quote! {
            BookRoute::#variant { .. } => use_mdbook::mdbook_shared::PageId(#id),
        }
    });

    let page_markdown = {
        let match_page = page_markdown_map.iter().map(|(id, markdown)| {
            let id = *id;
            quote! {
                #id => #markdown,
            }
        });

        quote! {
            /// Get the markdown for a page by its ID
            pub const fn page_markdown(id: use_mdbook::mdbook_shared::PageId) -> &'static str {
                match id.0 {
                    #(
                        #match_page
                    )*
                    _ => {
                        panic!("Invalid page ID:")
                    }
                }
            }
        }
    };

    quote! {
        #[derive(Clone, Copy, dioxus_router::Routable, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
        pub enum BookRoute {
            #(#book_routes)*
        }

        impl BookRoute {
            #page_markdown

            pub fn sections(&self) -> &'static [use_mdbook::mdbook_shared::Section] {
                &self.page().sections
            }

            pub fn page(&self) -> &'static use_mdbook::mdbook_shared::Page<Self> {
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

        #book_pages
    }
}

pub(crate) fn path_to_route_variant_name(path: &Path) -> Result<String, EmptyIdentError> {
    let path_without_extension = path.with_extension("");
    let mut title = String::new();
    for segment in path_without_extension.components() {
        title.push(' ');
        title.push_str(&segment.as_os_str().to_string_lossy());
    }
    let filtered = title
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '_')
        .collect::<String>();

    to_upper_camel_case_for_ident(&filtered)
}

/// Convert a string to an upper camel case which will be a valid Rust identifier. Any leading numbers will be skipped.
pub(crate) fn to_upper_camel_case_for_ident(title: &str) -> Result<String, EmptyIdentError> {
    let upper = title.to_case(Case::UpperCamel);
    Ok(
        if upper.chars().next().ok_or(EmptyIdentError)?.is_numeric() {
            format!("_{}", upper)
        } else {
            upper
        },
    )
}

#[derive(Debug)]
pub(crate) struct EmptyIdentError;

impl ToTokens for EmptyIdentError {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let err = self.to_string();
        tokens.extend(quote! {
            compile_error!(#err)
        })
    }
}

impl std::error::Error for EmptyIdentError {}

impl std::fmt::Display for EmptyIdentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Empty identifiers are not allowed")
    }
}

pub(crate) fn path_to_route_variant(path: &Path) -> Result<Ident, EmptyIdentError> {
    let title = path_to_route_variant_name(path)?;
    Ok(Ident::new(&title, Span::call_site()))
}

pub(crate) fn path_to_route_section(path: &Path) -> Result<Ident, EmptyIdentError> {
    let title = path_to_route_variant_name(path)?;
    Ok(Ident::new(&format!("{}Section", title), Span::call_site()))
}

pub(crate) fn path_to_route_enum(path: &Path) -> Result<TokenStream2, EmptyIdentError> {
    path_to_route_enum_with_section(path, Ident::new("Empty", Span::call_site()))
}

pub(crate) fn path_to_route_enum_with_section(
    path: &Path,
    section_variant: Ident,
) -> Result<TokenStream2, EmptyIdentError> {
    let name = path_to_route_variant(path)?;
    let section = path_to_route_section(path)?;
    Ok(quote! {
        BookRoute::#name {
            section: #section::#section_variant
        }
    })
}

fn rustfmt_via_cli(input: &str) -> String {
    let tmpfile = std::env::temp_dir().join(format!("mdbook-gen-{}.rs", std::process::id()));
    std::fs::write(&tmpfile, input).unwrap();

    let file = std::fs::File::open(&tmpfile).unwrap();
    let output = std::process::Command::new("rustfmt")
        .arg("--edition=2021")
        .stdin(file)
        .stdout(std::process::Stdio::piped())
        .output()
        .unwrap();

    _ = std::fs::remove_file(tmpfile);

    String::from_utf8(output.stdout).unwrap()
}
