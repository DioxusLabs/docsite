use quote::quote;
use std::{
    iter::Peekable,
    path::{Path, PathBuf},
    vec,
};

use dioxus_rsx::{BodyNode, CallBody, TemplateBody};
use pulldown_cmark::{Alignment, Event, Options, Parser, Tag};
use syn::{Ident, __private::Span, parse_quote, parse_str};

use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub fn parse(path: PathBuf, markdown: &str) -> syn::Result<CallBody> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    let mut parser = Parser::new_ext(markdown, options);

    let mut rsx_parser = RsxMarkdownParser {
        element_stack: vec![],
        root_nodes: vec![],
        current_table: vec![],
        in_table_header: false,
        iter: parser.by_ref().peekable(),
        path,
        phantom: std::marker::PhantomData,
    };
    rsx_parser.parse()?;
    while !rsx_parser.element_stack.is_empty() {
        rsx_parser.end_node();
    }

    Ok(if rsx_parser.root_nodes.is_empty() {
        parse_quote! {
            ""
        }
    } else {
        CallBody::new(TemplateBody::new(rsx_parser.root_nodes))
    })
}

struct RsxMarkdownParser<'a, I: Iterator<Item = Event<'a>>> {
    element_stack: Vec<BodyNode>,
    root_nodes: Vec<BodyNode>,

    current_table: Vec<Alignment>,
    in_table_header: bool,

    iter: Peekable<I>,

    path: PathBuf,

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
                let text = &*text;
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
            pulldown_cmark::Event::Html(_) => {}
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
                value: #type_value,
            }
        })
    }

    fn take_code_or_text(&mut self) -> String {
        let mut current_text = String::new();
        while let Some(pulldown_cmark::Event::Code(text) | pulldown_cmark::Event::Text(text)) =
            self.iter.peek()
        {
            current_text += text;
            let _ = self.iter.next().unwrap();
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

                    let text = BodyNode::Text(parse_quote!(#all_text));
                    self.create_node(text);
                }
                Some(pulldown_cmark::Event::Code(code)) => {
                    let code = code.to_string();
                    self.create_node(parse_quote! {
                        code {
                            #code
                        }
                    });

                    // Take the text or code event we just inserted
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
        while let Some(pulldown_cmark::Event::Text(text)) = self.iter.peek() {
            if insert_space {
                current_text.push(' ');
            }
            current_text += text;
            insert_space = true;
            let _ = self.iter.next().unwrap();
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
                let anchor: String = text
                    .trim()
                    .to_lowercase()
                    .chars()
                    .filter_map(|char| match char {
                        '-' | 'a'..='z' | '0'..='9' => Some(char),
                        ' ' | '_' => Some('-'),
                        _ => None,
                    })
                    .collect();
                let fragment = format!("#{}", anchor);
                let element_name = match level {
                    pulldown_cmark::HeadingLevel::H1 => Ident::new("h1", Span::call_site()),
                    pulldown_cmark::HeadingLevel::H2 => Ident::new("h2", Span::call_site()),
                    pulldown_cmark::HeadingLevel::H3 => Ident::new("h3", Span::call_site()),
                    pulldown_cmark::HeadingLevel::H4 => Ident::new("h4", Span::call_site()),
                    pulldown_cmark::HeadingLevel::H5 => Ident::new("h5", Span::call_site()),
                    pulldown_cmark::HeadingLevel::H6 => Ident::new("h6", Span::call_site()),
                };
                let element = parse_quote! {
                    #element_name {
                        id: #anchor,
                        a {
                            href: #fragment,
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
                let raw_code = escape_text(&self.take_code_or_text());

                if lang.as_deref() == Some("inject-dioxus") {
                    self.start_node(parse_str::<BodyNode>(&raw_code).unwrap());
                } else {
                    let code = transform_code_block(&self.path, raw_code)?;
                    // let mut code_attrs = Vec::new();

                    let ss = SyntaxSet::load_defaults_newlines();
                    let ts = ThemeSet::load_defaults();

                    let theme = &ts.themes["base16-ocean.dark"];
                    let syntax = ss.find_syntax_by_extension("rs").unwrap();
                    let html =
                        syntect::html::highlighted_html_for_string(&code, &ss, syntax, theme)
                            .unwrap();
                    self.start_node(parse_quote!{
                        div {
                            style: "position: relative;",
                            div {
                                dangerous_inner_html: #html
                            }
                            button {
                                style: "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                                "onclick": "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                                "Copy"
                            }
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
                let without_extension = dest.trim_end_matches(".md");
                let without_index = without_extension.trim_end_matches("/index");

                let href = match ty {
                    pulldown_cmark::LinkType::Email => format!("mailto:{}", without_index),
                    _ => {
                        if dest.starts_with("http") || dest.starts_with("https") {
                            dest.to_string()
                        } else {
                            // If this route ends with index.md, we need to prefix any routes relative to it with /route
                            if self.path.ends_with("index.md") {
                                if let Some(last_self_segment) =
                                    self.path.parent().and_then(|p| p.file_name())
                                {
                                    format!(
                                        "{}/{}",
                                        last_self_segment.to_string_lossy(),
                                        without_index
                                    )
                                } else {
                                    without_index.to_string()
                                }
                            } else {
                                without_index.to_string()
                            }
                        }
                    }
                };
                let title: &str = &title;
                let title_attr = if !title.is_empty() {
                    quote! {
                        title: #title,
                    }
                } else {
                    quote! {}
                };

                self.start_node(parse_quote! {
                    a {
                        href: #href,
                        #title_attr
                        #title
                    }
                });

                self.write_text();
            }
            Tag::Image(_, dest, title) => {
                let alt = self.take_text();
                let dest: &str = &dest;
                let title: &str = &title;

                #[cfg(not(feature = "manganis"))]
                let url: syn::Expr = syn::parse_quote!(#dest);
                #[cfg(feature = "manganis")]
                let url: syn::Expr = syn::parse_quote! { manganis::mg!(file(#dest)) };

                self.create_node(parse_quote! {
                    img {
                        src: #url,
                        alt: #alt,
                        title: #title,
                    }
                })
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
            None => &mut self.root_nodes,
            _ => return,
        };

        // If the last element is a text node, we can just join the text nodes together with a space
        if let (Some(BodyNode::Text(last_text)), BodyNode::Text(new_text)) =
            (element_list.last_mut(), &node)
        {
            last_text.input.push_ifmt(new_text.input.clone());
        } else {
            element_list.push(node);
        }
    }

    fn last_mut(&mut self) -> Option<&mut BodyNode> {
        self.element_stack.last_mut()
    }
}

fn transform_code_block(path: &Path, code_contents: String) -> syn::Result<String> {
    let segments = code_contents.split("{{#");
    let mut output = String::new();
    for segment in segments {
        if let Some((plugin, after)) = segment.split_once("}}") {
            if plugin.starts_with("include") {
                output += &resolve_extension(path, plugin)?;
                output += after;
            }
        } else {
            output += segment;
        }
    }
    Ok(output)
}

fn resolve_extension(_path: &Path, ext: &str) -> syn::Result<String> {
    if let Some(file) = ext.strip_prefix("include") {
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
                format!("Failed to read file {}: {}", file, e),
            )
        })?;
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
    } else {
        todo!("Unknown extension: {}", ext);
    }
}

fn escape_text(text: &str) -> String {
    text.replace('{', "{{")
        .replace('}', "}}")
}