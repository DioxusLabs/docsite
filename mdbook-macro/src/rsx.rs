use std::{iter::Peekable, str::FromStr, vec};

use dioxus_rsx::{BodyNode, CallBody, Element, ElementAttrNamed, IfmtInput};
use pulldown_cmark::{Alignment, Event, Tag};
use syn::{Ident, __private::Span, parse_quote};

pub fn parse(markdown: &str) -> CallBody {
    let mut parser = pulldown_cmark::Parser::new(markdown);

    let mut rsx_parser = RsxMarkdownParser {
        element_stack: vec![],
        root_nodes: vec![],
        current_table: vec![],
        in_table_header: false,
        iter: parser.by_ref().peekable(),
        phantom: std::marker::PhantomData,
    };
    rsx_parser.parse();
    while !rsx_parser.element_stack.is_empty() {
        rsx_parser.end_node();
    }

    if rsx_parser.root_nodes.is_empty() {
        CallBody {
            roots: vec![BodyNode::Text(IfmtInput::new_static(""))],
        }
    } else {
        CallBody {
            roots: rsx_parser.root_nodes,
        }
    }
}

struct RsxMarkdownParser<'a, I: Iterator<Item = Event<'a>>> {
    element_stack: Vec<BodyNode>,
    root_nodes: Vec<BodyNode>,

    current_table: Vec<Alignment>,
    in_table_header: bool,

    iter: Peekable<I>,

    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, I: Iterator<Item = Event<'a>>> RsxMarkdownParser<'a, I> {
    fn parse(&mut self) {
        while let Some(event) = self.iter.next() {
            self.parse_event(event);
        }
    }

    fn parse_event(&mut self, event: Event) {
        match event {
            pulldown_cmark::Event::Start(start) => {
                self.start_element(start);
            }
            pulldown_cmark::Event::End(_) => self.end_node(),
            pulldown_cmark::Event::Text(text) => {
                self.create_node(BodyNode::Text(IfmtInput::new_static(&text)));
            }
            pulldown_cmark::Event::Code(code) => {
                self.create_node(BodyNode::Text(IfmtInput::new_static(&code)));
            }
            pulldown_cmark::Event::Html(_) => {}
            pulldown_cmark::Event::FootnoteReference(_) => {}
            pulldown_cmark::Event::SoftBreak => {}
            pulldown_cmark::Event::HardBreak => {}
            pulldown_cmark::Event::Rule => {
                self.create_node(BodyNode::Element(Element {
                    name: dioxus_rsx::ElementName::Ident(Ident::new("hr", Span::call_site())),
                    key: None,
                    attributes: vec![],
                    children: vec![],
                    brace: Default::default(),
                }));
            }
            pulldown_cmark::Event::TaskListMarker(value) => {
                self.write_checkbox(value);
            }
        }
    }

    fn write_checkbox(&mut self, checked: bool) {
        let input = Ident::new("input", Span::call_site());
        let name = dioxus_rsx::ElementName::Ident(input.clone());
        self.create_node(BodyNode::Element(Element {
            name: name.clone(),
            key: None,
            attributes: vec![ElementAttrNamed {
                el_name: name,
                attr: dioxus_rsx::ElementAttr::AttrText {
                    name: Ident::new("type", Span::call_site()),
                    value: if checked {
                        IfmtInput::new_static("true")
                    } else {
                        IfmtInput::new_static("false")
                    },
                },
            }],
            children: vec![],
            brace: Default::default(),
        }));
    }

    fn take_code_or_text(&mut self) -> String {
        let mut current_text = String::new();
        loop {
            match self.iter.peek() {
                Some(pulldown_cmark::Event::Code(_) | pulldown_cmark::Event::Text(_)) => {
                    match self.iter.next().unwrap() {
                        pulldown_cmark::Event::Text(text) | pulldown_cmark::Event::Code(text) => {
                            current_text += &text;
                        }
                        _ => break,
                    }
                }
                _ => break,
            }
        }
        current_text
    }

    fn write_text(&mut self) {
        loop {
            match self.iter.peek() {
                Some(pulldown_cmark::Event::Text(_)) => match self.iter.next().unwrap() {
                    pulldown_cmark::Event::Text(text) => {
                        self.create_node(BodyNode::Text(IfmtInput::new_static(&text)));
                    }
                    _ => return,
                },
                Some(pulldown_cmark::Event::Code(_)) => match self.iter.next().unwrap() {
                    pulldown_cmark::Event::Code(text) => {
                        let code = BodyNode::Text(IfmtInput::new_static(&text));
                        self.create_node(BodyNode::Element(Element {
                            name: dioxus_rsx::ElementName::Ident(Ident::new(
                                "code",
                                Span::call_site(),
                            )),
                            key: None,
                            attributes: Vec::new(),
                            children: vec![code],
                            brace: Default::default(),
                        }));
                    }
                    _ => return,
                },
                _ => return,
            }
        }
    }

    fn take_text(&mut self) -> String {
        let mut current_text = String::new();
        loop {
            match self.iter.peek() {
                Some(pulldown_cmark::Event::Text(_)) => match self.iter.next().unwrap() {
                    pulldown_cmark::Event::Text(text) => {
                        current_text += &text;
                    }
                    _ => break,
                },
                _ => break,
            }
        }
        current_text
    }

    fn start_element(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => {
                self.start_node(BodyNode::Element(Element {
                    name: dioxus_rsx::ElementName::Ident(Ident::new("p", Span::call_site())),
                    key: None,
                    attributes: Vec::new(),
                    children: Vec::new(),
                    brace: Default::default(),
                }));
                self.write_text();
            }
            Tag::Heading(level, _, _) => {
                let text = self.take_text();
                let anchor = text.trim().to_lowercase().replace(" ", "-");
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
                self.start_node(BodyNode::Element(Element {
                    name: name.clone(),
                    key: None,
                    attributes: vec![dioxus_rsx::ElementAttrNamed {
                        el_name: name.clone(),
                        attr: dioxus_rsx::ElementAttr::AttrText {
                            name: Ident::new("id", Span::call_site()),
                            value: IfmtInput::new_static(&anchor),
                        },
                    }],
                    children: vec![BodyNode::Element(Element {
                        name: link_name.clone(),
                        key: None,
                        attributes: vec![
                            dioxus_rsx::ElementAttrNamed {
                                el_name: name.clone(),
                                attr: dioxus_rsx::ElementAttr::AttrText {
                                    name: Ident::new("class", Span::call_site()),
                                    value: IfmtInput::new_static("header"),
                                },
                            },
                            dioxus_rsx::ElementAttrNamed {
                                el_name: name.clone(),
                                attr: dioxus_rsx::ElementAttr::AttrText {
                                    name: Ident::new("href", Span::call_site()),
                                    value: IfmtInput::new_static(&format!(r##"#{anchor}"##)),
                                },
                            },
                        ],
                        children: vec![BodyNode::Text(IfmtInput::new_static(&text))],
                        brace: Default::default(),
                    })],
                    brace: Default::default(),
                }));
            }
            Tag::BlockQuote => {
                self.start_node(BodyNode::Element(Element {
                    name: dioxus_rsx::ElementName::Ident(Ident::new(
                        "blockquote",
                        Span::call_site(),
                    )),
                    key: None,
                    attributes: Vec::new(),
                    children: Vec::new(),
                    brace: Default::default(),
                }));
                self.write_text();
            }
            Tag::CodeBlock(kind) => {
                let code = self.take_code_or_text();
                self.start_node(BodyNode::Element(Element {
                    name: dioxus_rsx::ElementName::Ident(Ident::new("code", Span::call_site())),
                    key: None,
                    attributes: Vec::new(),
                    children: vec![BodyNode::Element(Element {
                        name: dioxus_rsx::ElementName::Ident(Ident::new("pre", Span::call_site())),
                        key: None,
                        attributes: Vec::new(),
                        children: vec![BodyNode::Text(IfmtInput::new_static(&code))],
                        brace: Default::default(),
                    })],
                    brace: Default::default(),
                }));
            }
            Tag::List(first) => {
                let name = match first {
                    Some(_) => "ol",
                    None => "ul",
                };
                self.start_node(BodyNode::Element(Element {
                    name: dioxus_rsx::ElementName::Ident(Ident::new(name, Span::call_site())),
                    key: None,
                    attributes: Vec::new(),
                    children: Vec::new(),
                    brace: Default::default(),
                }))
            }
            Tag::Item => self.start_node(BodyNode::Element(Element {
                name: dioxus_rsx::ElementName::Ident(Ident::new("li", Span::call_site())),
                key: None,
                attributes: Vec::new(),
                children: Vec::new(),
                brace: Default::default(),
            })),
            Tag::FootnoteDefinition(_) => {}
            Tag::Table(alignments) => {
                self.current_table = alignments;
                self.start_node(BodyNode::Element(Element {
                    name: dioxus_rsx::ElementName::Ident(Ident::new("table", Span::call_site())),
                    key: None,
                    attributes: Vec::new(),
                    children: Vec::new(),
                    brace: Default::default(),
                }))
            }
            Tag::TableHead => {
                self.in_table_header = true;
                self.start_node(BodyNode::Element(Element {
                    name: dioxus_rsx::ElementName::Ident(Ident::new("thead", Span::call_site())),
                    key: None,
                    attributes: Vec::new(),
                    children: Vec::new(),
                    brace: Default::default(),
                }))
            }
            Tag::TableRow => self.start_node(BodyNode::Element(Element {
                name: dioxus_rsx::ElementName::Ident(Ident::new("tr", Span::call_site())),
                key: None,
                attributes: Vec::new(),
                children: Vec::new(),
                brace: Default::default(),
            })),
            Tag::TableCell => {
                let name = if self.in_table_header { "th" } else { "td" };
                self.start_node(BodyNode::Element(Element {
                    name: dioxus_rsx::ElementName::Ident(Ident::new(name, Span::call_site())),
                    key: None,
                    attributes: Vec::new(),
                    children: Vec::new(),
                    brace: Default::default(),
                }))
            }
            Tag::Emphasis => self.start_node(BodyNode::Element(Element {
                name: dioxus_rsx::ElementName::Ident(Ident::new("em", Span::call_site())),
                key: None,
                attributes: Vec::new(),
                children: Vec::new(),
                brace: Default::default(),
            })),
            Tag::Strong => self.start_node(BodyNode::Element(Element {
                name: dioxus_rsx::ElementName::Ident(Ident::new("strong", Span::call_site())),
                key: None,
                attributes: Vec::new(),
                children: Vec::new(),
                brace: Default::default(),
            })),
            Tag::Strikethrough => self.start_node(BodyNode::Element(Element {
                name: dioxus_rsx::ElementName::Ident(Ident::new("s", Span::call_site())),
                key: None,
                attributes: Vec::new(),
                children: Vec::new(),
                brace: Default::default(),
            })),
            Tag::Link(ty, dest, title) => {
                let without_extension = dest.trim_end_matches(".md");
                let href = match ty {
                    pulldown_cmark::LinkType::Email => format!("mailto:{}", without_extension),
                    _ => without_extension.to_string(),
                };
                let name = Ident::new("a", Span::call_site());
                let mut attributes = vec![dioxus_rsx::ElementAttrNamed {
                    el_name: dioxus_rsx::ElementName::Ident(name.clone()),
                    attr: dioxus_rsx::ElementAttr::AttrText {
                        name: Ident::new("href", Span::call_site()),
                        value: IfmtInput::new_static(&href),
                    },
                }];

                if !title.is_empty() {
                    attributes.push(dioxus_rsx::ElementAttrNamed {
                        el_name: dioxus_rsx::ElementName::Ident(name.clone()),
                        attr: dioxus_rsx::ElementAttr::AttrText {
                            name: Ident::new("title", Span::call_site()),
                            value: IfmtInput::new_static(&title),
                        },
                    });
                }

                self.start_node(BodyNode::Element(Element {
                    name: dioxus_rsx::ElementName::Ident(name.clone()),
                    key: None,
                    attributes,
                    children: Vec::new(),
                    brace: Default::default(),
                }));

                self.write_text();
            }
            Tag::Image(_, dest, title) => {
                let name = Ident::new("img", Span::call_site());
                self.start_node(BodyNode::Element(Element {
                    name: dioxus_rsx::ElementName::Ident(name.clone()),
                    key: None,
                    attributes: vec![
                        dioxus_rsx::ElementAttrNamed {
                            el_name: dioxus_rsx::ElementName::Ident(name.clone()),
                            attr: dioxus_rsx::ElementAttr::AttrText {
                                name: Ident::new("src", Span::call_site()),
                                value: IfmtInput::new_static(&dest),
                            },
                        },
                        dioxus_rsx::ElementAttrNamed {
                            el_name: dioxus_rsx::ElementName::Ident(name.clone()),
                            attr: dioxus_rsx::ElementAttr::AttrText {
                                name: Ident::new("title", Span::call_site()),
                                value: IfmtInput::new_static(&title),
                            },
                        },
                    ],
                    children: Vec::new(),
                    brace: Default::default(),
                }));
            }
        }
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

    fn last_mut(&mut self) -> Option<&mut BodyNode> {
        self.element_stack.last_mut()
    }
}
