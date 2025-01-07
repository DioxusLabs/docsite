use std::path::PathBuf;

use mdbook_shared::MdBook;
use mdbook_shared::Page;
use mdbook_shared::Section;
use mdbook_shared::Summary;
use mdbook_shared::SummaryItem;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

use crate::path_to_route_enum;

/// Transforms the book to use enum routes instead of paths
pub fn write_book_with_routes(book: &mdbook_shared::MdBook<PathBuf>) -> TokenStream {
    let MdBook { summary, .. } = book;
    let summary = write_summary_with_routes(summary);
    let pages = book.pages().iter().map(|(id, v)| {
        let name = match path_to_route_enum(&v.url) {
            Ok(url) => url,
            Err(err) => {
                return err.to_token_stream();
            }
        };
        let page = write_page_with_routes(v);
        quote! {
            pages.push((#id, #page));
            page_id_mapping.insert(#name, ::use_mdbook::mdbook_shared::PageId(#id));
        }
    });

    let out = quote! {
        {
            let mut page_id_mapping = ::std::collections::HashMap::new();
            let mut pages = Vec::new();
            #(#pages)*
            ::use_mdbook::mdbook_shared::MdBook {
                summary: #summary,
                pages: pages.into_iter().collect(),
                page_id_mapping,
            }
        }
    };

    out.to_token_stream()
}

fn write_summary_with_routes(book: &mdbook_shared::Summary<PathBuf>) -> TokenStream {
    let Summary {
        title,
        prefix_chapters,
        numbered_chapters,
        suffix_chapters,
    } = book;

    let prefix_chapters = prefix_chapters.iter().map(write_summary_item_with_routes);
    let numbered_chapters = numbered_chapters.iter().map(write_summary_item_with_routes);
    let suffix_chapters = suffix_chapters.iter().map(write_summary_item_with_routes);
    let title = match title {
        Some(title) => quote! { Some(#title.to_string()) },
        None => quote! { None },
    };

    quote! {
        ::use_mdbook::mdbook_shared::Summary {
            title: #title,
            prefix_chapters: vec![#(#prefix_chapters),*],
            numbered_chapters: vec![#(#numbered_chapters),*],
            suffix_chapters: vec![#(#suffix_chapters),*],
        }
    }
}

fn write_summary_item_with_routes(item: &SummaryItem<PathBuf>) -> TokenStream {
    match item {
        SummaryItem::Link(link) => {
            let link = write_link_with_routes(link);
            quote! {
                ::use_mdbook::mdbook_shared::SummaryItem::Link(#link)
            }
        }
        SummaryItem::Separator => {
            quote! {
                ::use_mdbook::mdbook_shared::SummaryItem::Separator
            }
        }
        SummaryItem::PartTitle(title) => {
            quote! {
                ::use_mdbook::mdbook_shared::SummaryItem::PartTitle(#title.to_string())
            }
        }
    }
}

fn write_link_with_routes(book: &mdbook_shared::Link<PathBuf>) -> TokenStream {
    let mdbook_shared::Link {
        name,
        location,
        number,
        nested_items,
    } = book;

    let location = match location {
        Some(loc) => {
            let inner = match path_to_route_enum(loc) {
                Ok(url) => url,
                Err(err) => {
                    return err.to_token_stream();
                }
            };
            quote! { Some(#inner) }
        }
        None => quote! { None },
    };
    let number = match number {
        Some(number) => {
            let inner = write_number_with_routes(number);
            quote! { Some(#inner) }
        }
        None => quote! {None},
    };

    let nested_items = nested_items.iter().map(write_summary_item_with_routes);

    quote! {
        ::use_mdbook::mdbook_shared::Link {
            name: #name.to_string(),
            location: #location,
            number: #number,
            nested_items: vec![#(#nested_items,)*],
        }
    }
}

fn write_number_with_routes(number: &mdbook_shared::SectionNumber) -> TokenStream {
    let mdbook_shared::SectionNumber(number) = number;
    let numbers = number.iter().map(|num| {
        quote! {
            #num
        }
    });

    quote! {
        ::use_mdbook::mdbook_shared::SectionNumber(vec![#(#numbers),*])
    }
}

fn write_page_with_routes(book: &mdbook_shared::Page<PathBuf>) -> TokenStream {
    let Page {
        title,
        url,
        segments,
        sections,
        raw: _,
        id,
    } = book;

    let segments = segments.iter().map(|segment| {
        quote! {
            #segment.to_string()
        }
    });

    let sections = sections.iter().map(write_section_with_routes);

    let path = url;
    let url = match path_to_route_enum(path) {
        Ok(url) => url,
        Err(err) => {
            return err.to_token_stream();
        }
    };
    let id = id.0;

    quote! {
        {
            ::use_mdbook::mdbook_shared::Page {
                title: #title.to_string(),
                url: #url,
                segments: vec![#(#segments,)*],
                sections: vec![#(#sections,)*],
                raw: String::new(),
                id: ::use_mdbook::mdbook_shared::PageId(#id),
            }
        }
    }
}

fn write_section_with_routes(book: &mdbook_shared::Section) -> TokenStream {
    let Section { title, id, level } = book;

    quote! {
        ::use_mdbook::mdbook_shared::Section {
            title: #title.to_string(),
            id: #id.to_string(),
            level: #level,
        }
    }
}
