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
    extension: String,
    code: String,
    theme: String,
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
        Ok(CodeBlock {
            extension: extension.value(),
            code: code.value(),
            theme: theme.value(),
        })
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
///     "base16-ocean.dark",
/// };
#[proc_macro]
pub fn syntect_html(input: TokenStream) -> TokenStream {
    match syn::parse::<CodeBlock>(input) {
        Ok(block) => {
            let ss = SyntaxSet::load_defaults_newlines();
            let ts = ThemeSet::load_defaults();

            let theme = ts.themes.get(&block.theme);
            let theme = match theme.ok_or_else(|| {
                syn::Error::new(
                    Span::call_site(),
                    format!("No theme found for {}", block.theme),
                )
            }) {
                Ok(theme) => theme,
                Err(err) => return err.to_compile_error().into(),
            };
            let syntax = match ss
                .find_syntax_by_extension(&block.extension)
                .ok_or_else(|| {
                    syn::Error::new(
                        Span::call_site(),
                        format!("No syntax found for extension {}", block.extension),
                    )
                }) {
                Ok(syntax) => syntax,
                Err(err) => return err.to_compile_error().into(),
            };
            let html = syntect::html::highlighted_html_for_string(&block.code, &ss, &syntax, theme)
                .unwrap();

            quote! {
                #html
            }
            .into_token_stream()
            .into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}
