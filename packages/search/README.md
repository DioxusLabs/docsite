# Dioxus Search

Dioxus search creates a prebaked search index for all your static Dioxus routes.

It integrates with the Dioxus router to find all the static routes in your application and search for any rendered HTML files for those files.

Example:

```rust
#[cfg(feature = "ssr")]
{
    // Generate all static routes in the ./static folder using Dioxus fullstack
    // ...

    // Create search index
    dioxus_search::SearchIndex::<Route>::create(
        "searchable",
        dioxus_search::BaseDirectoryMapping::new("./static")
    );
}

// After the first build the search index is cached at compile time inline in your program
static SEARCH_INDEX: dioxus_search::LazySearchIndex<Route> = dioxus_search::load_search_index! {
    "searchable"
};


#[component]
fn Homepage() -> Element {
    let search_text = use_signal(String::new);
    let results = SEARCH_INDEX.search(&search_text.get());

    render!{
        input {
            oninput: move |e| {
                search_text.set(e.value.clone());
            },
            value: "{search_text}",
        }
        ul {
            for result in results.map(|i| i.into_iter()).ok().into_iter().flatten() {
                li {
                    Link {
                        target: result.route.clone(),
                        "{result:#?}"
                    }
                }
            }
        }
    }
}
```

For a full working demo, see the [searchable example](./examples/searchable/).
