use dioxus::prelude::*;

#[derive(PartialEq, Debug)]
pub struct Snippet {
    pub title: &'static str,
    pub body: String,
    pub code: String,
    pub caller_id: usize,
}

pub fn build_snippets() -> Vec<Snippet> {
    let mut lines = SnippetCode.lines().peekable();
    let mut snips: Vec<Snippet> = Vec::new();

    let mut caller_id = 0;
    while let Some(line) = lines.next() {
        // just skip over the first few lines
        if line.starts_with("/// #") {
            let (_, title) = line.split_once("# ").unwrap();
            let mut body = String::new();
            let mut code = String::new();

            'inner: loop {
                let line = lines.next().unwrap();
                let mut should_commit = false;

                if line.starts_with("///") {
                    body += line.trim_start_matches("///");
                    if line == "///" {
                        body += "\n";
                    }
                } else {
                    // process the code
                    code += line;
                    code += "\n";
                    should_commit = match lines.peek() {
                        Some(line) => line.starts_with("/// #"),
                        None => true,
                    };
                }

                if should_commit {
                    snips.push(Snippet {
                        body,
                        code,
                        title,
                        caller_id,
                    });
                    caller_id += 1;
                    break 'inner;
                }
            }
        }
    }
    snips
}

static SnippetCode: &str = include_str!("../../snippets/mod.rs");

#[test]
fn render_s() {
    let snips = build_snippets();
    dbg!(&snips[0]);
}
