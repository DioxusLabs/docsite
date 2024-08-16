use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use convert_case::{Case, Casing};
use mdbook_shared::MdBook;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use quote::ToTokens;
use syn::LitStr;

use mdbook_gen::*;

#[proc_macro]
pub fn mdbook_router(input: TokenStream) -> TokenStream {
    match syn::parse::<LitStr>(input).map(load_book_from_fs) {
        Ok(Ok((path, book))) => generate_router(path, book).into(),
        Ok(Err(err)) => write_book_err(err),
        Err(err) => err.to_compile_error().into(),
    }
}

fn write_book_err(err: anyhow::Error) -> TokenStream {
    let err = err.to_string();
    println!("{}", err);
    quote! { compile_error!(#err); }.to_token_stream().into()
}
