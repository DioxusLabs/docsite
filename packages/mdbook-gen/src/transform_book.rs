use std::path::Path;
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
use mdbook_shared::get_book_content_path;
use mdbook_shared::get_summary_path;

/// Transforms the book to use enum routes instead of paths
pub fn write_book_with_routes(
    book_path: PathBuf,
    book: &mdbook_shared::MdBook<PathBuf>,
) -> TokenStream {
    let summary_path = get_summary_path(&book_path).expect("SUMMARY.md path not found");
    let index_path = summary_path.to_string_lossy();

    let MdBook { summary, .. } = book;
    let summary = write_summary_with_routes(summary);
    let pages = book.pages().iter().map(|(id, v)| {
        let name = path_to_route_enum(&v.url);
        let page = write_page_with_routes(&book_path, v);
        quote! {
            pages.push((#id, #page));
            page_id_mapping.insert(#name, ::use_mdbook::mdbook_shared::PageId(#id));
        }
    });

    let out = quote! {
        {
            // Let the compiler know that we care about the index file
            const _: &[u8] = include_bytes!(#index_path);
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
            let inner = path_to_route_enum(loc);
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

fn write_page_with_routes(book_path: &Path, book: &mdbook_shared::Page<PathBuf>) -> TokenStream {
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
    let url = path_to_route_enum(path);
    let full_path = get_book_content_path(book_path)
        .expect("No book content path found")
        .join(path);
    let path_str = full_path.to_str().unwrap();
    let id = id.0;

    quote! {
        {
            // This lets the rust compile know that we read the file
            const _: &[u8] = include_bytes!(#path_str);
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
