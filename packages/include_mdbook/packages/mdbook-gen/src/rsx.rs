use mdbook_shared::get_book_content_path;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use std::{
    iter::Peekable,
    path::{Path, PathBuf},
    str::FromStr,
    vec,
};

use dioxus_rsx::{BodyNode, CallBody, TemplateBody};
use pulldown_cmark::{Alignment, Event, Options, Parser, Tag};
use syn::{parse_quote, parse_str, Ident};

use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

use crate::{
    path_to_route_enum, path_to_route_enum_with_section, to_upper_camel_case_for_ident,
    EmptyIdentError,
};

#[cfg(test)]
use pretty_assertions::assert_eq;

/// Convert a CallBody to a TokenStream
pub fn callbody_to_tokens(cb: CallBody) -> TokenStream2 {
    // Get the tokens
    let out = dioxus_autofmt::write_block_out(&cb).unwrap();

    // Parse the tokens
    TokenStream2::from_str(&out).unwrap()
}

pub(crate) struct Section {
    name: String,
}

impl Section {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub(crate) fn fragment(&self) -> String {
        self.name
            .trim()
            .to_lowercase()
            .chars()
            .filter_map(|char| match char {
                '-' | 'a'..='z' | '0'..='9' => Some(char),
                ' ' | '_' => Some('-'),
                _ => None,
            })
            .collect()
    }

    pub(crate) fn variant(&self) -> Result<String, EmptyIdentError> {
        to_upper_camel_case_for_ident(&self.fragment())
    }
}

pub(crate) struct ParsedMarkdown {
    pub(crate) body: CallBody,
    pub(crate) sections: Vec<Section>,
}

pub fn parse_markdown(
    book_path: PathBuf,
    path: PathBuf,
    markdown: &str,
) -> syn::Result<ParsedMarkdown> {
    let mut options = Options::empty();
    options.insert(
        Options::ENABLE_TABLES
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS,
    );

    let mut parser = Parser::new_ext(markdown, options);

    let mut rsx_parser = RsxMarkdownParser {
        element_stack: vec![],
        root_nodes: vec![],
        current_table: vec![],
        sections: vec![],
        in_table_header: false,
        iter: parser.by_ref().peekable(),
        book_path,
        path,
        phantom: std::marker::PhantomData,
    };
    rsx_parser.parse()?;
    while !rsx_parser.element_stack.is_empty() {
        rsx_parser.end_node();
    }

    let body = if rsx_parser.root_nodes.is_empty() {
        parse_quote! {}
    } else {
        CallBody::new(TemplateBody::new(rsx_parser.root_nodes))
    };

    Ok(ParsedMarkdown {
        body,
        sections: rsx_parser.sections,
    })
}

struct RsxMarkdownParser<'a, I: Iterator<Item = Event<'a>>> {
    element_stack: Vec<BodyNode>,
    root_nodes: Vec<BodyNode>,
    current_table: Vec<Alignment>,
    in_table_header: bool,
    iter: Peekable<I>,
    book_path: PathBuf,
    path: PathBuf,
    sections: Vec<Section>,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, I: Iterator<Item = Event<'a>>> RsxMarkdownParser<'a, I> {
    fn parse(&mut self) -> syn::Result<()> {
        while let Some(event) = self.iter.next() {
            self.parse_event(event)?;
        }
        Ok(())
    }

    fn parse_event(&mut self, event: Event) -> syn::Result<()> {
        match event {
            pulldown_cmark::Event::Start(start) => {
                self.start_element(start)?;
            }
            pulldown_cmark::Event::End(_) => self.end_node(),
            pulldown_cmark::Event::Text(text) => {
                let text = escape_text(&text);
                self.create_node(BodyNode::Text(parse_quote!(#text)));
            }
            pulldown_cmark::Event::Code(code) => {
                let code = escape_text(&code);
                self.create_node(parse_quote! {
                    code {
                        #code
                    }
                })
            }
            pulldown_cmark::Event::Html(node) => {
                let code = escape_text(&node);
                self.create_node(parse_quote! {
                    p {
                        class: "inline-html-block",
                        dangerous_inner_html: #code,
                    }
                })
            }
            pulldown_cmark::Event::FootnoteReference(_) => {}
            pulldown_cmark::Event::SoftBreak => {}
            pulldown_cmark::Event::HardBreak => {}
            pulldown_cmark::Event::Rule => self.create_node(parse_quote! {
                hr {}
            }),
            pulldown_cmark::Event::TaskListMarker(value) => {
                self.write_checkbox(value);
            }
        }
        Ok(())
    }

    fn write_checkbox(&mut self, checked: bool) {
        let type_value = if checked { "true" } else { "false" };
        self.create_node(parse_quote! {
            input {
                r#type: "checkbox",
                readonly: true,
                class: "mdbook-checkbox",
                value: #type_value,
            }
        })
    }

    fn take_code_or_text(&mut self) -> String {
        let mut current_text = String::new();
        loop {
            match self.iter.peek() {
                Some(pulldown_cmark::Event::Code(text) | pulldown_cmark::Event::Text(text)) => {
                    current_text += text;
                    self.iter.next().unwrap();
                }
                // Ignore any softbreaks
                Some(pulldown_cmark::Event::SoftBreak) => {}
                _ => break,
            }
        }
        current_text
    }

    fn write_text(&mut self) {
        loop {
            match self.iter.peek() {
                Some(pulldown_cmark::Event::Text(text)) => {
                    let mut all_text = text.to_string();

                    // Take the text or code event we just inserted
                    let _ = self.iter.next().unwrap();

                    // If the next block after this is a code block, insert the space in the text before the code block
                    if let Some(pulldown_cmark::Event::Code(_)) = self.iter.peek() {
                        all_text.push(' ');
                    }
                    let all_text = escape_text(&all_text);

                    let text = BodyNode::Text(parse_quote!(#all_text));
                    self.create_node(text);
                }
                Some(pulldown_cmark::Event::Code(code)) => {
                    let code = code.to_string();
                    let code = escape_text(&code);
                    self.create_node(parse_quote! {
                        code {
                            #code
                        }
                    });

                    // Take the text or code event we just inserted
                    let _ = self.iter.next().unwrap();
                }
                // Ignore any softbreaks
                Some(pulldown_cmark::Event::SoftBreak) => {
                    let _ = self.iter.next().unwrap();
                }
                _ => return,
            }
        }
    }

    fn take_text(&mut self) -> String {
        let mut current_text = String::new();
        // pulldown_cmark will create a new text node for each newline. We insert a space
        // between each newline to avoid two lines being rendered right next to each other.
        let mut insert_space = false;
        loop {
            match self.iter.peek() {
                Some(pulldown_cmark::Event::Text(text) | pulldown_cmark::Event::Code(text)) => {
                    let starts_with_space =
                        text.chars().next().filter(|c| c.is_whitespace()).is_some();
                    let ends_with_space =
                        text.chars().last().filter(|c| c.is_whitespace()).is_some();
                    if insert_space && !starts_with_space {
                        current_text.push(' ');
                    }
                    current_text += text;
                    insert_space = !ends_with_space;
                    _ = self.iter.next().unwrap();
                }
                // Ignore any softbreaks
                Some(pulldown_cmark::Event::SoftBreak) => {
                    _ = self.iter.next().unwrap();
                }
                _ => break,
            }
        }
        current_text
    }

    fn start_element(&mut self, tag: Tag) -> syn::Result<()> {
        match tag {
            Tag::Paragraph => {
                self.start_node(parse_quote! {
                    p {}
                });
                self.write_text();
            }
            Tag::Heading(level, _, _) => {
                let text = self.take_text();
                let section = Section::new(&text);
                let variant = section.variant();
                let section_variant = variant.and_then(|variant| {
                    path_to_route_enum_with_section(
                        &self.path,
                        Ident::new(&variant, Span::call_site()),
                    )
                });
                let section_variant = match section_variant {
                    Ok(section_variant) => section_variant,
                    Err(err) => err.to_token_stream(),
                };
                let anchor = section.fragment();
                self.sections.push(section);
                let element_name = match level {
                    pulldown_cmark::HeadingLevel::H1 => Ident::new("h1", Span::call_site()),
                    pulldown_cmark::HeadingLevel::H2 => Ident::new("h2", Span::call_site()),
                    pulldown_cmark::HeadingLevel::H3 => Ident::new("h3", Span::call_site()),
                    pulldown_cmark::HeadingLevel::H4 => Ident::new("h4", Span::call_site()),
                    pulldown_cmark::HeadingLevel::H5 => Ident::new("h5", Span::call_site()),
                    pulldown_cmark::HeadingLevel::H6 => Ident::new("h6", Span::call_site()),
                };
                let anchor = escape_text(&anchor);
                let text = escape_text(&text);
                let element = parse_quote! {
                    #element_name {
                        id: #anchor,
                        Link {
                            to: #section_variant,
                            class: "header",
                            #text
                        }
                    }
                };
                self.start_node(element);
            }
            Tag::BlockQuote => {
                self.start_node(parse_quote! {
                    blockquote {}
                });
                self.write_text();
            }
            Tag::CodeBlock(kind) => {
                let lang = match kind {
                    pulldown_cmark::CodeBlockKind::Indented => None,
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                        (!lang.is_empty()).then_some(lang)
                    }
                };
                let raw_code = self.take_code_or_text();

                if lang.as_deref() == Some("inject-dioxus") {
                    self.start_node(parse_str::<BodyNode>(&raw_code).unwrap());
                } else {
                    let (fname, html) = build_codeblock(raw_code, &self.path)?;
                    let fname = if let Some(fname) = fname {
                        quote! { name: #fname.to_string() }
                    } else {
                        quote! {}
                    };

                    self.start_node(parse_quote! {
                        CodeBlock {
                            contents: #html,
                            #fname
                        }
                    });
                }
            }
            Tag::List(first) => {
                let name = match first {
                    Some(_) => Ident::new("ol", Span::call_site()),
                    None => Ident::new("ul", Span::call_site()),
                };
                self.start_node(parse_quote! {
                    #name {}
                })
            }
            Tag::Item => self.start_node(parse_quote! {
                li {}
            }),
            Tag::FootnoteDefinition(_) => {}
            Tag::Table(alignments) => {
                self.current_table = alignments;
                self.start_node(parse_quote! {
                    table {}
                })
            }
            Tag::TableHead => {
                self.in_table_header = true;
                self.start_node(parse_quote! {
                    thead {}
                })
            }
            Tag::TableRow => self.start_node(parse_quote! {
                tr {}
            }),
            Tag::TableCell => {
                let name = if self.in_table_header { "th" } else { "td" };
                let ident = Ident::new(name, Span::call_site());
                self.start_node(parse_quote! {
                    #ident {}
                })
            }
            Tag::Emphasis => self.start_node(parse_quote! {
                em {}
            }),
            Tag::Strong => self.start_node(parse_quote! {
                strong {}
            }),
            Tag::Strikethrough => self.start_node(parse_quote! {
                s {}
            }),
            Tag::Link(ty, dest, title) => {
                let href = match ty {
                    pulldown_cmark::LinkType::Email => format!("mailto:{}", dest).to_token_stream(),
                    _ => {
                        if dest.starts_with("http") || dest.starts_with("https") {
                            escape_text(&dest).to_token_stream()
                        } else {
                            // If this is a relative link, resolve it relative to the current file
                            let content_path =
                                get_book_content_path(&self.book_path).ok_or_else(|| {
                                    syn::Error::new(
                                        Span::call_site(),
                                        "Failed to resolve the content path",
                                    )
                                })?;
                            let content_path = content_path.canonicalize().unwrap();
                            let current_file_path = content_path.join(&self.path);
                            let parent_of_current_file = current_file_path.parent().unwrap();
                            let hash;
                            let dest_without_hash = match dest.split_once('#') {
                                Some((without_hash, trailing_hash)) => {
                                    hash = Some(trailing_hash);
                                    if without_hash.is_empty() {
                                        current_file_path
                                            .strip_prefix(parent_of_current_file)
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                    } else {
                                        without_hash
                                    }
                                }
                                None => {
                                    hash = None;
                                    &dest
                                }
                            };
                            let path =
                                PathBuf::from(dest_without_hash.to_string()).with_extension("md");
                            if path.is_relative() {
                                let relative_to_current_folder = parent_of_current_file.join(&path);
                                match relative_to_current_folder
                                    .canonicalize()
                                    .map_err(|e| e.to_string())
                                    .and_then(|p| {
                                        p.strip_prefix(&content_path)
                                            .map(PathBuf::from)
                                            .map_err(|_| format!("failed to strip prefix {content_path:?} from {p:?}"))
                                    }) {
                                    Ok(resolved) if content_path.join(&resolved).is_file() => {
                                        let result = if let Some(hash) = hash {
                                            let section = Section::new(hash);
                                            match section.variant() {
                                                Ok(variant) => {
                                                    path_to_route_enum_with_section(&resolved, Ident::new(&variant, Span::call_site()))
                                                },
                                                Err(_) => {
                                                    Ok(quote! {
                                                        compile_error!("Fragment cannot be empty")
                                                    })
                                                }
                                            }
                                        } else {
                                            path_to_route_enum(&resolved)
                                        };

                                        match result {
                                            Ok(result) => result,
                                            Err(err) => {
                                                err.to_token_stream()
                                            }
                                        }
                                    },
                                    Ok(resolved) => {
                                        let err = format!("The file {resolved:?} linked to in {current_file_path:?} does not exist");
                                        quote! {
                                            compile_error!(#err)
                                        }
                                    },
                                    Err(e) => {
                                        let err = format!(
                                            "Failed to resolve link {} relative to {}: {}",
                                            path.display(), current_file_path.display(), e
                                        );
                                        quote! {
                                            compile_error!(#err)
                                        }
                                    }
                                }
                            } else {
                                escape_text(&dest).to_token_stream()
                            }
                        }
                    }
                };

                let title = escape_text(&title);
                let title_attr = if !title.is_empty() {
                    quote! {
                        title: #title,
                    }
                } else {
                    quote! {}
                };

                self.start_node(parse_quote! {
                    Link {
                        to: #href,
                        #title_attr
                    }
                });

                self.write_text();
            }
            Tag::Image(_, dest, title) => {
                let alt = escape_text(&self.take_text());
                let dest: &str = &dest;
                let title = escape_text(&title);

                let should_asset_it = cfg!(feature = "manganis")
                    && (dest.starts_with("/")
                        || !(dest.starts_with("https://") || dest.starts_with("http://")));

                let url = if should_asset_it {
                    // todo(jon): recognize the url by parsing it and checking if it's external/internal - these might be unreliable heuristics
                    if dest.ends_with(".png") || dest.ends_with(".jpg") || dest.ends_with(".jpeg") {
                        let res = quote::quote! {
                            asset!(#dest, ImageAssetOptions::new().with_webp())
                        };

                        res
                    } else {
                        quote::quote! {
                            asset!(#dest)
                        }
                    }
                } else {
                    let dest = escape_text(dest);
                    quote::quote!(#dest)
                };

                if dest.ends_with(".mp4") || dest.ends_with(".mov") {
                    self.start_node(parse_quote! {
                        video {
                            src: #url,
                            alt: #alt,
                            title: #title,
                            autoplay: true,
                            muted: true,
                            r#loop: true,
                            playsinline: true,
                            preload: "metadata"
                        }
                    })
                } else {
                    self.start_node(parse_quote! {
                        img {
                            src: #url,
                            alt: #alt,
                            title: #title,
                        }
                    })
                }
            }
        }
        Ok(())
    }

    fn start_node(&mut self, node: BodyNode) {
        self.element_stack.push(node);
    }

    fn end_node(&mut self) {
        if let Some(node) = self.element_stack.pop() {
            match self.last_mut() {
                Some(BodyNode::Element(element)) => {
                    element.children.push(node);
                }
                Some(BodyNode::Component(element)) => {
                    element.children.roots.push(node);
                }
                None => {
                    self.root_nodes.push(node);
                }
                _ => {}
            }
        }
    }

    fn create_node(&mut self, node: BodyNode) {
        // Find the list of elements we should add the node to
        let element_list = match self.last_mut() {
            Some(BodyNode::Element(element)) => &mut element.children,
            Some(BodyNode::Component(element)) => &mut element.children.roots,
            None => &mut self.root_nodes,
            _ => return,
        };

        // If the last element is a text node, add a space between them
        if let (Some(BodyNode::Text(last_text)), BodyNode::Text(new_text)) =
            (element_list.last_mut(), &node)
        {
            if !last_text
                .input
                .source
                .value()
                .chars()
                .last()
                .filter(|c| c.is_whitespace())
                .is_some()
                && !new_text
                    .input
                    .source
                    .value()
                    .chars()
                    .next()
                    .filter(|c| c.is_whitespace())
                    .is_some()
            {
                element_list.push(parse_quote! { " " });
            }
        }

        element_list.push(node);
    }

    fn last_mut(&mut self) -> Option<&mut BodyNode> {
        self.element_stack.last_mut()
    }
}

fn build_codeblock(
    raw_code: String,
    path: &PathBuf,
) -> Result<(Option<String>, String), syn::Error> {
    let mut fname = None;
    let code = transform_code_block(&path, raw_code, &mut fname)?;
    static THEME: once_cell::sync::Lazy<syntect::highlighting::Theme> =
        once_cell::sync::Lazy::new(|| {
            let raw = include_str!("../themes/MonokaiDark.thTheme").to_string();
            let mut reader = std::io::Cursor::new(raw.clone());
            ThemeSet::load_from_reader(&mut reader).unwrap()
        });

    let ss = SyntaxSet::load_defaults_newlines();
    let syntax = ss.find_syntax_by_extension("rs").unwrap();
    let html =
        syntect::html::highlighted_html_for_string(code.trim_end(), &ss, syntax, &THEME).unwrap();

    let html = escape_text(&html);

    Ok((fname, html))
}

fn transform_code_block(
    path: &Path,
    code_contents: String,
    fname: &mut Option<String>,
) -> syn::Result<String> {
    let segments = code_contents.split("{{#include");
    let segments = segments;
    let mut output = String::new();
    for (i, segment) in segments.enumerate() {
        // Skip the first segment which is before the first include
        if let Some((file, after)) = segment.split_once("}}").filter(|_| i > 0) {
            output += &resolve_extension(path, file, fname)?;
            output += after;
        } else {
            output += segment;
        }
    }
    Ok(output)
}

fn resolve_extension(path: &Path, file: &str, fname: &mut Option<String>) -> syn::Result<String> {
    let file = file.trim();
    let mut segment = None;
    let file = if let Some((file, file_segment)) = file.split_once(':') {
        segment = Some(file_segment);
        file
    } else {
        file
    };

    let result = std::fs::read_to_string(file).map_err(|e| {
        syn::Error::new(
            Span::call_site(),
            format!(
                "Failed to read file {}: {} from path {} at cwd {}",
                file,
                e,
                path.display(),
                std::env::current_dir().unwrap().display()
            ),
        )
    })?;
    *fname = Some(
        PathBuf::from(file)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(),
    );
    if let Some(segment) = segment {
        // get the text between lines with ANCHOR: segment and ANCHOR_END: segment
        let lines = result.lines();
        let mut output = String::new();
        let mut in_segment: bool = false;
        // normalize indentation to the first line
        let mut first_line_indent = 0;
        for line in lines {
            if let Some((_, remaining)) = line.split_once("ANCHOR:") {
                if remaining.trim() == segment {
                    in_segment = true;
                    first_line_indent = line.chars().take_while(|c| c.is_whitespace()).count();
                }
            } else if let Some((_, remaining)) = line.split_once("ANCHOR_END:") {
                if remaining.trim() == segment {
                    in_segment = false;
                }
            } else if in_segment {
                for (_, char) in line
                    .chars()
                    .enumerate()
                    .skip_while(|(i, c)| *i < first_line_indent && c.is_whitespace())
                {
                    output.push(char);
                }
                output += "\n";
            }
        }
        if output.ends_with('\n') {
            output.pop();
        }
        Ok(output)
    } else {
        Ok(result)
    }
}

fn escape_text(text: &str) -> String {
    text.replace('{', "{{").replace('}', "}}")
}

#[test]
fn parse_link() {
    let markdown = r#"
# Chapter 1
[Chapter 2](./chapter_2.md)

Some assets:
![some_external](https://avatars.githubusercontent.com/u/79236386?s=200&v=4)
![some_local](/example-book/assetsasd/logo)
![some_local1](/example-book/assets1/logo.png)
![some_local2](/example-book/assets2/logo.png)
"#;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    let mut parser = Parser::new_ext(markdown, options);

    let mut rsx_parser = RsxMarkdownParser {
        element_stack: vec![],
        root_nodes: vec![],
        current_table: vec![],
        sections: vec![],
        in_table_header: false,
        iter: parser.by_ref().peekable(),
        path: PathBuf::from("../../example-book/en/chapter_1.md"),
        book_path: PathBuf::from("../../example-book"),
        phantom: std::marker::PhantomData,
    };

    rsx_parser.parse().unwrap();
    while !rsx_parser.element_stack.is_empty() {
        rsx_parser.end_node();
    }

    let body = CallBody::new(TemplateBody::new(rsx_parser.root_nodes));

    dbg!(&body);

    let fmted = dioxus_autofmt::write_block_out(&body).unwrap();
    println!("{}", fmted);

    // Parse the tokens
    let tokens_out = TokenStream2::from_str(&fmted).unwrap();

    let out: syn::File = parse_quote! {
        #[component(no_case_check)]
        pub fn Hmm() -> dioxus::prelude::Element {
            use dioxus::prelude::*;
            rsx! {
                #tokens_out
            }
        }
    };

    let fmted = prettyplease::unparse(&out);

    println!("{}", fmted);
}

#[test]
fn parse_softbreaks() {
    let markdown = r#"
# Programmatic Navigation

Sometimes we want our application to navigate to another page without having the
user click on a link. This is called programmatic navigation.

## Using a Navigator

We can get a navigator with the `navigator` function which returns a `Navigator`.
"#;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    let mut parser = Parser::new_ext(markdown, options);

    let mut rsx_parser = RsxMarkdownParser {
        element_stack: vec![],
        root_nodes: vec![],
        current_table: vec![],
        sections: vec![],
        in_table_header: false,
        iter: parser.by_ref().peekable(),
        path: PathBuf::from("../../example-book/en/chapter_1.md"),
        book_path: PathBuf::from("../../example-book"),
        phantom: std::marker::PhantomData,
    };

    rsx_parser.parse().unwrap();
    while !rsx_parser.element_stack.is_empty() {
        rsx_parser.end_node();
    }

    let body = CallBody::new(TemplateBody::new(rsx_parser.root_nodes));
    let fmted = dioxus_autofmt::write_block_out(&body).unwrap();
    println!("{}", fmted);

    let expected_tokens: CallBody = parse_quote! {
        h1 { id: "programmatic-navigation",
            Link {
                to: BookRoute::ExampleBookEnChapter1 {
                    section: ExampleBookEnChapter1Section::ProgrammaticNavigation
                },
                class: "header",
                "Programmatic Navigation"
            }
        }
        p {
            "Sometimes we want our application to navigate to another page without having the"
            " "
            "user click on a link. This is called programmatic navigation."
        }
        h2 { id: "using-a-navigator",
            Link {
                to: BookRoute::ExampleBookEnChapter1 {
                    section: ExampleBookEnChapter1Section::UsingANavigator
                },
                class: "header",
                "Using a Navigator"
            }
        }
        p {
            "We can get a navigator with the  "
            code { "navigator" }
            " function which returns a  "
            code { "Navigator" }
            "."
        }
    };

    assert_eq!(expected_tokens.body, body.body);
}

#[test]
fn parse_code_headers() {
    let markdown = r#"# This `is` a header about `MdBook`"#;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    let mut parser = Parser::new_ext(markdown, options);

    let mut rsx_parser = RsxMarkdownParser {
        element_stack: vec![],
        root_nodes: vec![],
        current_table: vec![],
        sections: vec![],
        in_table_header: false,
        iter: parser.by_ref().peekable(),
        path: PathBuf::from("../../example-book/en/chapter_1.md"),
        book_path: PathBuf::from("../../example-book"),
        phantom: std::marker::PhantomData,
    };

    rsx_parser.parse().unwrap();
    while !rsx_parser.element_stack.is_empty() {
        rsx_parser.end_node();
    }

    let body = CallBody::new(TemplateBody::new(rsx_parser.root_nodes));
    let fmted = dioxus_autofmt::write_block_out(&body).unwrap();
    println!("{}", fmted);

    let expected_tokens: CallBody = parse_quote! {
        h1 { id: "this-is-a-header-about-mdbook",
            Link {
                to: BookRoute::ExampleBookEnChapter1 {
                    section: ExampleBookEnChapter1Section::ThisIsAHeaderAboutMdbook
                },
                class: "header",
                "This is a header about MdBook"
            }
        }
    };

    assert_eq!(expected_tokens.body, body.body);
}

#[test]
fn syn_parsing_race() {
    let alt1 = "some_alt_text";

    let res1 = quote::quote! {
        asset!(#alt1, ImageAssetOptions::new().with_avif())
    };

    let alt2 = "some_alt_text2";

    let res2 = quote::quote! {
        asset!(#alt2, ImageAssetOptions::new().with_avif())
    };

    println!("{}", res1.to_string());
    println!("{}", res2.to_string());

    let out_toks1: BodyNode = parse_quote! {
        img { alt: #alt1, src: #res1 }
    };

    let out_toks2: BodyNode = parse_quote! {
        img { alt: #alt2, src: #res2 }
    };

    println!("{:?}", out_toks1);
    println!("{:?}", out_toks2);
}

#[test]
fn parses_codeblocks() {
    let code = r##"
{"timestamp":"   9.927s","level":"INFO","message":"Bundled app successfully!","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"INFO","message":"App produced 2 outputs:","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"INFO","message":"app - [target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app]","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"INFO","message":"dmg - [target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg]","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"DEBUG","json":"{\"BundleOutput\":{\"bundles\":[\"target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app\"]}}"}
    "##;

    let (name, contents) = build_codeblock(
        code.to_string(),
        &PathBuf::from("../../example-book/en/chapter_1.md"),
    )
    .unwrap();

    println!("{:?}", name);
    println!("{}", contents);
}
