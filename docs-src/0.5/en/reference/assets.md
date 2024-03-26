# Assets

> ⚠️ Support: Manganis is currently in alpha. API changes are planned and bugs are more likely

Assets are files that are included in the final build of the application. They can be images, fonts, stylesheets, or any other file that is not a source file. Dioxus includes first class support for assets, and provides a simple way to include them in your application and automatically optimize them for production.

Assets in dioxus are also compatible with libraries! If you are building a library, you can include assets in your library and they will be automatically included in the final build of any application that uses your library.

First, you need to add the `manganis` crate to your `Cargo.toml` file:

```sh
cargo add manganis
```

## Including images

To include an asset in your application, you can simply wrap the path to the asset in a `mg!` call. For example, to include an image in your application, you can use the following code:

```rust
{{#include src/doc_examples/assets.rs:images}}
```

You can also optimize, resize, and preload images using the `mg!` macro. Choosing an optimized file type (like WebP) and a reasonable quality setting can significantly reduce the size of your images which helps your application load faster. For example, you can use the following code to include an optimized image in your application:

```rust
{{#include src/doc_examples/assets.rs:optimized_images}}
```

## Including arbitrary files

In dioxus desktop, you may want to include a file with data for your application. You can use the `file` function to include arbitrary files in your application. For example, you can use the following code to include a file in your application:

```rust
{{#include src/doc_examples/assets.rs:arbitrary_files}}
```

These files will be automatically included in the final build of your application, and you can use them in your application as you would any other file.

## Including stylesheets

You can include stylesheets in your application using the `mg!` macro. For example, you can use the following code to include a stylesheet in your application:

```rust
{{#include src/doc_examples/assets.rs:style_sheets}}
```

> The [tailwind guide](../cookbook/tailwind.md) has more information on how to use tailwind with dioxus.

## Conclusion

Dioxus provides first class support for assets, and makes it easy to include them in your application. You can include images, arbitrary files, and stylesheets in your application, and dioxus will automatically optimize them for production. This makes it easy to include assets in your application and ensure that they are optimized for production.

You can read more about assets and all the options available to optimize your assets in the [manganis documentation](https://docs.rs/manganis/0.2.2/manganis/).
