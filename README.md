# `use_mdbook` - hooks and components for loading mdbooks with Dioxus

This crate provides the `use_mdbook` hook and `include_mdbook` macro that allows access to the contents of MdBooks at compile time.

This crate will integrate with a future Dioxus Assets system that allows image bundling outside the final output binary.

The point of this project is to power the Dioxus MdBook component ecosystem which enables any Dioxus app to easily include and render an MdBook.

Planned features for this crate:
- MdBook components (search, navbars, renderers)
- Hotreloading for mdbooks
- Devtool integration for live mdbook editing

<!--
This crate provides the `use_mdbook()` macro hook that provides *live editing* of mdbooks from within a running Dioxus app.

To get started, you'll want to configure a global static mdbook using the `include_mdbook!` hook. In "dev" mode - with dioxus hotreload enabled - this will sync the rendered output with the filesystem. Whenever the mdbook is edited locally, your running app will be updated *without recompiles.*

This integrates with your app router, enabling static-site and sitemap generation using the mdbook manifest data.

```rust
#[derive(Routable)]
enum Routes {
    Home,

    // This will inject the "Mdbook" object as a context when rendering the "Docs" component
    #[dioxus_mdbook::from_fs("/docs")]
    Docs {
        version: String,
    },


    #[dioxus_blog::from_fs("/blog")]
    BlogPost {

    }
}

fn app(cx: Scope) -> Element {
    render! {
        Router {
            Nav {
                MdbookSearch {}
            }
            Outlet::<Routes> {}
            Footer {}
        }
    }
}

```


This macro builds on top of Dioxus-assets -->
