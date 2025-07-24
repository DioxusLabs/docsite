# Assets

Assets are files that are included in the final build of the application. They can be images, fonts, stylesheets, or any other file that is not a source file. Dioxus includes first class support for assets, and provides a simple way to include them in your application and automatically optimize them for production.

Assets in dioxus are also compatible with libraries! If you are building a library, you can include assets in your library and they will be automatically included in the final build of any application that uses your library.



## Including images

To include an image in your application, you can simply wrap the path to the asset in the `asset!` macro:

```rust
{{#include ../docs-router/src/doc_examples/assets.rs:images}}
```

The asset macro takes a path to the asset relative to the root of your app. The path is *not* absolute to your machine, making it possible to use the same asset paths across multiple machines.

```rust
// ❌ does not work!
let ferrous = asset!("/Users/dioxus/Downloads/image.png");
```

The asset macro is `const`, meaning we can use it inline or as a static/const item:
```rust
// as a static item
static FERROUS: Asset = asset!("/assets/static/ferrous_wave.png");

// or inline
rsx! {
    img { src: asset!("/assets/static/ferrous_wave.png") }
}
```

## Asset Hashes

The asset macro automatically attaches a hash to the name of the asset after it's bundled. This makes your app's bundled assets unique across time, allowing you to infinitely cache the asset in your webserver or on a [CDN](https://en.wikipedia.org/wiki/Content_delivery_network).

```rust
// prints "/assets/ferrous_wave-dxhx13xj2j.png"
println!("{}", asset!("/assets/static/ferrous_wave.png"))
```

Asset hashes are an extremely powerful feature of the asset system. Hashes integrate with CDNs and can greatly speedup your application's load performance and save you money in infrastructure costs.

However, you occasionally might want to disable them. We can customize the asset processing options with the `AssetOptions` builder:
```rust
let ferrous = asset!("/assets/static/ferrous_wave.png", AssetOptions::builder().with_hash_suffix(false));
```

## Asset Bundling and Loading

Unlike Rust's `include_bytes!` macro, the `asset!` macro *does not* copy the contents of the asset into your final application binary. Instead, it adds the asset path and options into the final binary's metadata. When you run `dx serve` or `dx build`, we automatically read that metadata and process the asset.

![Asset Bundling](/assets/07/asset-pipeline.png)

On the web, you

## Customizing Image Processing Options

You can also optimize, resize, and preload images using the `asset!` macro. Choosing an optimized file type (like Avif) and a reasonable quality setting can significantly reduce the size of your images which helps your application load faster. For example, you can use the following code to include an optimized image in your application:

```rust
{{#include ../docs-router/src/doc_examples/assets.rs:optimized_images}}
```

## Including arbitrary files

In dioxus desktop, you may want to include a file with data for your application. If you don't set any options for your asset and the file extension is not recognized, the asset will be copied without any changes. For example, you can use the following code to include a binary file in your application:

```rust
{{#include ../docs-router/src/doc_examples/assets.rs:arbitrary_files}}
```

These files will be automatically included in the final build of your application, and you can use them in your application as you would any other file.

## Including stylesheets

You can include stylesheets in your application using the `asset!` macro. Stylesheets will automatically be minified as they are bundled to speed up load times. For example, you can use the following code to include a stylesheet in your application:

```rust
{{#include ../docs-router/src/doc_examples/assets.rs:style_sheets}}
```

> The [tailwind guide](../../guides/utilities/tailwind.md) has more information on how to use tailwind with dioxus.

#### SCSS Support
SCSS is also supported through the `asset!` macro. Include it the same way as a regular CSS file.

## Conclusion

Dioxus provides first class support for assets, and makes it easy to include them in your application. You can include images, arbitrary files, and stylesheets in your application, and dioxus will automatically optimize them for production. This makes it easy to include assets in your application and ensure that they are optimized for production.

You can read more about assets and all the options available to optimize your assets in the [manganis documentation](https://docs.rs/manganis/0.6.0/manganis).
