use std::collections::HashMap;
use std::fmt::format;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use mdbook_shared::MdBook;
use mdbook_shared::Page;
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

fn write_book(book: mdbook_shared::MdBook) -> TokenStream {
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
fn load_book_from_fs(input: LitStr) -> anyhow::Result<mdbook_shared::MdBook> {
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
