const SNIPPET_WELCOME: &str = include_str!("snippets/welcome.rs");
const SNIPPET_COUNTER: &str = include_str!("snippets/counter.rs");

pub const SNIPPETS: [(&str, &str); 2] =
    [("Welcome", SNIPPET_WELCOME), ("Counter", SNIPPET_COUNTER)];
