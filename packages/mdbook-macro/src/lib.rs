use proc_macro::TokenStream;
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
