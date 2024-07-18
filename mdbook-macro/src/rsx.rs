use std::{
    iter::Peekable,
    path::{Path, PathBuf},
    vec,
};

use dioxus_rsx::{
    AttributeType, BodyNode, CallBody, Element, ElementAttr, ElementAttrName, ElementAttrNamed,
    ElementAttrValue, IfmtInput,
};
use pulldown_cmark::{Alignment, Event, Options, Parser, Tag};
use syn::{Ident, __private::Span, parse_quote, parse_str, LitStr};

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
        CallBody {
            roots: vec![BodyNode::Text(IfmtInput::new_static(""))],
        }
    } else {
        CallBody {
            roots: rsx_parser.root_nodes,
        }
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
                self.create_node(BodyNode::Text(IfmtInput::new_static(&text)));
            }
            pulldown_cmark::Event::Code(code) => {
                self.create_node(BodyNode::Element(Element::new(
                    None,
                    dioxus_rsx::ElementName::Ident(Ident::new("code", Span::call_site())),
                    Vec::new(),
                    vec![BodyNode::Text(IfmtInput::new_static(&code))],
                    Default::default(),
                )));
            }
            pulldown_cmark::Event::Html(_) => {}
            pulldown_cmark::Event::FootnoteReference(_) => {}
            pulldown_cmark::Event::SoftBreak => {}
            pulldown_cmark::Event::HardBreak => {}
            pulldown_cmark::Event::Rule => {
                self.create_node(BodyNode::Element(Element::new(
                    None,
                    dioxus_rsx::ElementName::Ident(Ident::new("hr", Span::call_site())),
                    vec![],
                    vec![],
                    Default::default(),
                )));
            }
            pulldown_cmark::Event::TaskListMarker(value) => {
                self.write_checkbox(value);
            }
        }
        Ok(())
    }

    fn write_checkbox(&mut self, checked: bool) {
        let input = Ident::new("input", Span::call_site());
        let name = dioxus_rsx::ElementName::Ident(input);
        self.create_node(BodyNode::Element(Element::new(
            None,
            name.clone(),
            vec![AttributeType::Named(ElementAttrNamed {
                el_name: name,
                attr: dioxus_rsx::ElementAttr {
                    name: ElementAttrName::BuiltIn(Ident::new("type", Span::call_site())),
                    value: ElementAttrValue::AttrLiteral(if checked {
                        IfmtInput::new_static("true")
                    } else {
                        IfmtInput::new_static("false")
                    }),
                },
            })],
            vec![],
            Default::default(),
        )));
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

                    let text = BodyNode::Text(IfmtInput::new_static(&all_text));
                    self.create_node(text);
                }
                Some(pulldown_cmark::Event::Code(code)) => {
                    let code = BodyNode::Text(IfmtInput::new_static(code));
                    self.create_node(BodyNode::Element(Element::new(
                        None,
                        dioxus_rsx::ElementName::Ident(Ident::new("code", Span::call_site())),
                        Vec::new(),
                        vec![code],
                        Default::default(),
                    )));

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
                self.start_node(BodyNode::Element(Element::new(
                    None,
                    dioxus_rsx::ElementName::Ident(Ident::new("p", Span::call_site())),
                    Vec::new(),
                    Vec::new(),
                    Default::default(),
                )));
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
                let link_name = dioxus_rsx::ElementName::Ident(Ident::new("a", Span::call_site()));
                let name = dioxus_rsx::ElementName::Ident(Ident::new(
                    match level {
                        pulldown_cmark::HeadingLevel::H1 => "h1",
                        pulldown_cmark::HeadingLevel::H2 => "h2",
                        pulldown_cmark::HeadingLevel::H3 => "h3",
                        pulldown_cmark::HeadingLevel::H4 => "h4",
                        pulldown_cmark::HeadingLevel::H5 => "h5",
                        pulldown_cmark::HeadingLevel::H6 => "h6",
                    },
                    Span::call_site(),
                ));
                self.start_node(BodyNode::Element(Element::new(
                    None,
                    name.clone(),
                    vec![AttributeType::Named(ElementAttrNamed {
                        el_name: link_name.clone(),
                        attr: dioxus_rsx::ElementAttr {
                            name: ElementAttrName::BuiltIn(Ident::new("id", Span::call_site())),
                            value: ElementAttrValue::AttrLiteral(IfmtInput::new_static(&anchor)),
                        },
                    })],
                    vec![BodyNode::Element(Element::new(
                        None,
                        link_name,
                        vec![
                            AttributeType::Named(ElementAttrNamed {
                                el_name: name.clone(),
                                attr: dioxus_rsx::ElementAttr {
                                    name: ElementAttrName::BuiltIn(Ident::new(
                                        "class",
                                        Span::call_site(),
                                    )),
                                    value: ElementAttrValue::AttrLiteral(IfmtInput::new_static(
                                        "header",
                                    )),
                                },
                            }),
                            AttributeType::Named(ElementAttrNamed {
                                el_name: name,
                                attr: dioxus_rsx::ElementAttr {
                                    name: ElementAttrName::BuiltIn(Ident::new(
                                        "href",
                                        Span::call_site(),
                                    )),
                                    value: ElementAttrValue::AttrLiteral(IfmtInput::new_static(
                                        &format!(r##"#{anchor}"##),
                                    )),
                                },
                            }),
                        ],
                        vec![BodyNode::Text(IfmtInput::new_static(&text))],
                        Default::default(),
                    ))],
                    Default::default(),
                )));
            }
            Tag::BlockQuote => {
                self.start_node(BodyNode::Element(Element::new(
                    None,
                    dioxus_rsx::ElementName::Ident(Ident::new("blockquote", Span::call_site())),
                    Vec::new(),
                    Vec::new(),
                    Default::default(),
                )));
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
                    let code = transform_code_block(&self.path, raw_code)?;
                    let mut code_attrs = Vec::new();

                    let ss = SyntaxSet::load_defaults_newlines();
                    let ts = ThemeSet::load_defaults();

                    let theme = &ts.themes["base16-ocean.dark"];
                    let syntax = ss.find_syntax_by_extension("rs").unwrap();
                    let html =
                        syntect::html::highlighted_html_for_string(&code, &ss, syntax, theme)
                            .unwrap();
                    code_attrs.push(AttributeType::Named(ElementAttrNamed {
                        el_name: dioxus_rsx::ElementName::Ident(Ident::new(
                            "div",
                            Span::call_site(),
                        )),
                        attr: dioxus_rsx::ElementAttr {
                            name: ElementAttrName::BuiltIn(Ident::new(
                                "dangerous_inner_html",
                                Span::call_site(),
                            )),
                            value: ElementAttrValue::AttrLiteral(IfmtInput::new_static(&html)),
                        },
                    }));
                    let code_node = BodyNode::Element(Element::new(
                        None,
                        dioxus_rsx::ElementName::Ident(Ident::new("div", Span::call_site())),
                        code_attrs,
                        vec![],
                        Default::default(),
                    ));
                    let copy_node = BodyNode::Element(Element::new(
                        None,
                         dioxus_rsx::ElementName::Ident(Ident::new("button", Span::call_site())),
                         vec![
                            AttributeType::Named(

                                ElementAttrNamed {
                                    el_name:      dioxus_rsx::ElementName::Ident(Ident::new(
                                    "button",
                                    Span::call_site(),
                                )),
                                  attr: ElementAttr{
                                    name: ElementAttrName::BuiltIn( Ident::new("style", Span::call_site())),
                                    value: ElementAttrValue::AttrLiteral( IfmtInput::new_static("position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;")),
                                },
                }),
                            AttributeType::Named(ElementAttrNamed {
                                el_name: dioxus_rsx::ElementName::Ident(Ident::new(
                                    "button",
                                    Span::call_site(),
                                )),
                                 attr: dioxus_rsx::ElementAttr  {
                                    name: ElementAttrName::Custom(LitStr::new("onclick", Span::call_site())),
                                    value: ElementAttrValue::AttrLiteral( IfmtInput::new_static(
                                        r##"navigator.clipboard.writeText(this.previousElementSibling.innerText)"##,
                                    )),
                                },
                }),
                        ],
                        vec![
                            BodyNode::Text(IfmtInput::new_static("Copy"))
                        ],
                         Default::default(),
                    ));
                    self.start_node(BodyNode::Element(Element::new(
                        None,
                        dioxus_rsx::ElementName::Ident(Ident::new("div", Span::call_site())),
                        vec![AttributeType::Named(ElementAttrNamed {
                            el_name: dioxus_rsx::ElementName::Ident(Ident::new(
                                "button",
                                Span::call_site(),
                            )),
                            attr: dioxus_rsx::ElementAttr {
                                name: ElementAttrName::BuiltIn(Ident::new(
                                    "style",
                                    Span::call_site(),
                                )),
                                value: ElementAttrValue::AttrLiteral(IfmtInput::new_static(
                                    "position: relative;",
                                )),
                            },
                        })],
                        vec![code_node, copy_node],
                        Default::default(),
                    )));
                }
            }
            Tag::List(first) => {
                let name = match first {
                    Some(_) => "ol",
                    None => "ul",
                };
                self.start_node(BodyNode::Element(Element::new(
                    None,
                    dioxus_rsx::ElementName::Ident(Ident::new(name, Span::call_site())),
                    Vec::new(),
                    Vec::new(),
                    Default::default(),
                )))
            }
            Tag::Item => self.start_node(BodyNode::Element(Element::new(
                None,
                dioxus_rsx::ElementName::Ident(Ident::new("li", Span::call_site())),
                Vec::new(),
                Vec::new(),
                Default::default(),
            ))),
            Tag::FootnoteDefinition(_) => {}
            Tag::Table(alignments) => {
                self.current_table = alignments;
                self.start_node(BodyNode::Element(Element::new(
                    None,
                    dioxus_rsx::ElementName::Ident(Ident::new("table", Span::call_site())),
                    Vec::new(),
                    Vec::new(),
                    Default::default(),
                )))
            }
            Tag::TableHead => {
                self.in_table_header = true;
                self.start_node(BodyNode::Element(Element::new(
                    None,
                    dioxus_rsx::ElementName::Ident(Ident::new("thead", Span::call_site())),
                    Vec::new(),
                    Vec::new(),
                    Default::default(),
                )))
            }
            Tag::TableRow => self.start_node(BodyNode::Element(Element::new(
                None,
                dioxus_rsx::ElementName::Ident(Ident::new("tr", Span::call_site())),
                Vec::new(),
                Vec::new(),
                Default::default(),
            ))),
            Tag::TableCell => {
                let name = if self.in_table_header { "th" } else { "td" };
                self.start_node(BodyNode::Element(Element::new(
                    None,
                    dioxus_rsx::ElementName::Ident(Ident::new(name, Span::call_site())),
                    Vec::new(),
                    Vec::new(),
                    Default::default(),
                )))
            }
            Tag::Emphasis => self.start_node(BodyNode::Element(Element::new(
                None,
                dioxus_rsx::ElementName::Ident(Ident::new("em", Span::call_site())),
                Vec::new(),
                Vec::new(),
                Default::default(),
            ))),
            Tag::Strong => self.start_node(BodyNode::Element(Element::new(
                None,
                dioxus_rsx::ElementName::Ident(Ident::new("strong", Span::call_site())),
                Vec::new(),
                Vec::new(),
                Default::default(),
            ))),
            Tag::Strikethrough => self.start_node(BodyNode::Element(Element::new(
                None,
                dioxus_rsx::ElementName::Ident(Ident::new("s", Span::call_site())),
                Vec::new(),
                Vec::new(),
                Default::default(),
            ))),
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
                let name = Ident::new("a", Span::call_site());
                let mut attributes = vec![AttributeType::Named(ElementAttrNamed {
                    el_name: dioxus_rsx::ElementName::Ident(name.clone()),
                    attr: dioxus_rsx::ElementAttr {
                        name: ElementAttrName::BuiltIn(Ident::new("href", Span::call_site())),
                        value: ElementAttrValue::AttrLiteral(IfmtInput::new_static(&href)),
                    },
                })];

                if !title.is_empty() {
                    attributes.push(AttributeType::Named(ElementAttrNamed {
                        el_name: dioxus_rsx::ElementName::Ident(name.clone()),
                        attr: dioxus_rsx::ElementAttr {
                            name: ElementAttrName::BuiltIn(Ident::new("title", Span::call_site())),
                            value: ElementAttrValue::AttrLiteral(IfmtInput::new_static(&title)),
                        },
                    }));
                }

                self.start_node(BodyNode::Element(Element::new(
                    None,
                    dioxus_rsx::ElementName::Ident(name),
                    attributes,
                    Vec::new(),
                    Default::default(),
                )));

                self.write_text();
            }
            Tag::Image(_, dest, title) => {
                let name = Ident::new("img", Span::call_site());
                let alt = self.take_text();
                let dest: &str = &dest;

                #[cfg(not(feature = "manganis"))]
                let url: syn::Expr = parse_quote!(#dest);
                #[cfg(feature = "manganis")]
                let url: syn::Expr = syn::parse_quote! { manganis::mg!(file(#dest)) };

                self.start_node(BodyNode::Element(Element::new(
                    None,
                    dioxus_rsx::ElementName::Ident(name.clone()),
                    vec![
                        AttributeType::Named(ElementAttrNamed {
                            el_name: dioxus_rsx::ElementName::Ident(name.clone()),
                            attr: dioxus_rsx::ElementAttr {
                                name: ElementAttrName::BuiltIn(Ident::new(
                                    "src",
                                    Span::call_site(),
                                )),
                                value: ElementAttrValue::AttrExpr(url),
                            },
                        }),
                        AttributeType::Named(ElementAttrNamed {
                            el_name: dioxus_rsx::ElementName::Ident(name.clone()),
                            attr: dioxus_rsx::ElementAttr {
                                name: ElementAttrName::BuiltIn(Ident::new(
                                    "alt",
                                    Span::call_site(),
                                )),
                                value: ElementAttrValue::AttrLiteral(IfmtInput::new_static(&alt)),
                            },
                        }),
                        AttributeType::Named(ElementAttrNamed {
                            el_name: dioxus_rsx::ElementName::Ident(name),
                            attr: dioxus_rsx::ElementAttr {
                                name: ElementAttrName::BuiltIn(Ident::new(
                                    "title",
                                    Span::call_site(),
                                )),
                                value: ElementAttrValue::AttrLiteral(IfmtInput::new_static(&title)),
                            },
                        }),
                    ],
                    Vec::new(),
                    Default::default(),
                )));
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
            *last_text = last_text.clone().join(new_text.clone(), " ");
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
