use std::path::PathBuf;

use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::LitStr;

#[proc_macro]
pub fn include_mdbook(input: TokenStream) -> TokenStream {
    match syn::parse::<LitStr>(input).map(load_book_from_fs) {
        Ok(Ok(book)) => write_book(book),
        Ok(Err(err)) => write_book_err(err),
        Err(err) => err.to_compile_error().into(),
    }
}

fn write_book_err(err: anyhow::Error) -> TokenStream {
    let err = err.to_string();
    quote! { compile_error!(#err) }.to_token_stream().into()
}

fn load_book_from_fs(input: LitStr) -> anyhow::Result<mdbook_shared::Summary> {
    let buf = input.value().parse::<PathBuf>()?;

    // todo: languages?? might need to "discover" the FS
    let buf = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?)
        .join(buf)
        // .join("src")
        // .join("en")
        .join("SUMMARY.md")
        .canonicalize()?;

    let summary = std::fs::read_to_string(buf)?;

    Ok(mdbook_shared::parse_summary(&summary)?)
}

fn write_book(book: mdbook_shared::Summary) -> TokenStream {
    let sum = serde_json::to_string(&book).unwrap();

    let out = quote! {
        ::once_cell::sync::Lazy::new(|| {
            ::use_mdbook::MdBook::load(#sum)
        })
    };

    out.to_token_stream().into()
}
