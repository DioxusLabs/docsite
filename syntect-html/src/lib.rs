use std::path::PathBuf;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::LitStr;
use syn::Token;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

struct CodeBlock {
    html: String,
}

impl Parse for CodeBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let code = input.parse::<LitStr>()?;
        let _ = input.parse::<Token![,]>();
        let extension = input
            .parse::<LitStr>()
            .unwrap_or_else(|_| LitStr::new("rs", Span::call_site()));
        let _ = input.parse::<Token![,]>();
        let theme = input
            .parse::<LitStr>()
            .unwrap_or_else(|_| LitStr::new("base16-ocean.dark", Span::call_site()));

        Self::new(code.value(), extension.value(), theme.value())
    }
}

impl CodeBlock {
    fn new(code: String, extension: String, theme: String) -> syn::Result<Self> {
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        let maybe_theme = ts.themes.get(&theme);
        let theme = maybe_theme.ok_or_else(|| {
            syn::Error::new(Span::call_site(), format!("No theme found for {}", theme))
        })?;
        let syntax = ss.find_syntax_by_extension(&extension).ok_or_else(|| {
            syn::Error::new(
                Span::call_site(),
                format!("No syntax found for extension {}", extension),
            )
        })?;
        let html = syntect::html::highlighted_html_for_string(&code, &ss, syntax, theme).map_err(
            |err| {
                syn::Error::new(
                    Span::call_site(),
                    format!("Error while generating HTML: {}", err),
                )
            },
        )?;
        Ok(CodeBlock { html })
    }
}

impl ToTokens for CodeBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let html = &self.html;

        tokens.extend(quote! {
            #html
        })
    }
}

struct CodeBlockFs {
    html: String,
}

impl Parse for CodeBlockFs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let code_path = input.parse::<LitStr>()?;
        let code_path = code_path.value();
        let _ = input.parse::<Token![,]>();
        let theme = input
            .parse::<LitStr>()
            .unwrap_or_else(|_| LitStr::new("base16-ocean.dark", Span::call_site()));
        let theme = theme.value();

        let path = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR")
                .map_err(|_| syn::Error::new(Span::call_site(), "CARGO_MANIFEST_DIR not found"))?,
        );
        let path = path.join(code_path);
        let code = std::fs::read_to_string(&path).map_err(|err| {
            syn::Error::new(
                Span::call_site(),
                format!(
                    "Error while reading file: {} while reading {}",
                    err,
                    path.display()
                ),
            )
        })?;
        let extension = path.extension();
        let extension = &*extension.unwrap().to_string_lossy();
        let html = syntect::html::highlighted_html_for_string(
            &code,
            &syntect::parsing::SyntaxSet::load_defaults_newlines(),
            syntect::parsing::SyntaxSet::load_defaults_newlines()
                .find_syntax_by_extension(extension)
                .unwrap(),
            syntect::highlighting::ThemeSet::load_defaults()
                .themes
                .get(&theme)
                .unwrap(),
        )
        .unwrap();

        Ok(CodeBlockFs { html })
    }
}

/// Generate a HTML string from a code block path.
#[proc_macro]
pub fn syntect_html_fs(input: TokenStream) -> TokenStream {
    match syn::parse::<CodeBlockFs>(input) {
        Ok(block) => {
            let html = &block.html;
            quote! {
                #html
            }
            .into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

/// Generate a HTML string from a code block.
///
/// # Example
///
/// ```rust
/// use syntect_html::syntect_html;
///
/// let html = syntect_html! {
///     r#"
///     fn main() {
///         println!("Hello, world!");
///     }
///     "#,
///     "rs",
///     "base16-ocean.dark"
/// };
#[proc_macro]
pub fn syntect_html(input: TokenStream) -> TokenStream {
    match syn::parse::<CodeBlock>(input) {
        Ok(block) => quote! {
            #block
        }
        .into(),
        Err(err) => err.to_compile_error().into(),
    }
}
