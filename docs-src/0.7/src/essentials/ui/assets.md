# Assets

Assets are files that are included in the final build of the application. They can be images, fonts, stylesheets, or any other file that is not a source file. Dioxus includes first class support for assets, and provides a simple way to include them in your application and automatically optimize them for production.

Assets in dioxus are also compatible with libraries! If you are building a library, you can include assets in your library and they will be automatically included in the final build of any application that uses your library.

## Including images

To include an image in your application, you can simply wrap the path to the asset in the `asset!` macro:

```rust
{{#include ../docs-router/src/doc_examples/assets.rs:images}}
```

The asset macro takes a path to the asset relative to the root of your app. The path is _not_ absolute to your machine, making it possible to use the same asset paths across multiple machines.

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

## Customizing Image Processing Options

You can also optimize, resize, and preload images using the `asset!` macro. Choosing an optimized file type (like Avif) and a reasonable quality setting can significantly reduce the size of your images which helps your application load faster. For example, you can use the following code to include an optimized image in your application:

```rust
{{#include ../docs-router/src/doc_examples/assets.rs:optimized_images}}
```

## Including stylesheets

You can include stylesheets in your application using the `asset!` macro. Stylesheets will automatically be minified as they are bundled to speed up load times. For example, you can use the following code to include a stylesheet in your application:

```rust
{{#include ../docs-router/src/doc_examples/assets.rs:style_sheets}}
```

> The [tailwind guide](../../guides/utilities/tailwind.md) has more information on how to use tailwind with dioxus.

## SCSS Support

SCSS is also supported through the `asset!` macro. Include it the same way as a regular CSS file.

You can read more about assets and all the options available to optimize your assets in the [manganis documentation](https://docs.rs/manganis/latest/manganis).

## Including arbitrary files

In dioxus desktop, you may want to include a file with data for your application. If you don't set any options for your asset and the file extension is not recognized, the asset will be copied without any changes. For example, you can use the following code to include a binary file in your application:

```rust
{{#include ../docs-router/src/doc_examples/assets.rs:arbitrary_files}}
```

These files will be automatically included in the final build of your application, and you can use them in your application as you would any other file.

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

## Linker-based Asset Bundling

Unlike Rust's `include_bytes!` macro, the `asset!` macro _does not_ copy the contents of the asset into your final application binary. Instead, it adds the asset path and options into the final binary's metadata. When you run `dx serve` or `dx build`, we automatically read that metadata and process the asset.

![Asset Bundling](/assets/07/asset-pipeline-full.png)

The metadata for each asset is automatically embedded in the final executable by serializing its path and properties using the [const-serialize](https://crates.io/crates/const-serialize) crate. When DX builds the executable, it then searches the output binary for asset metadata. After the build is complete, DX computes asset hashes and writes them back into the binary.

```rust
#[link_section = "dx-assets"]
static SERIALIZED_ASSET_OPTIONS: &[u8] = r#"{"path": "/assets/main.css","minify":"true","hash":"dxh0000"}"#;
```

This means that assets are not permanently baked into your final executable. The final executable is smaller, loads faster, and asset loading is much more flexible. This is important on platforms like the browser where assets are fetched in parallel over the network.

To dynamically load the asset's contents, you can use the [dioxus-asset-resolver](https://crates.io/crates/dioxus-asset-resolver) crate which properly understands the app's bundle format and loads an asset given its `Display` impl.

```rust
let contents = dioxus_asset_resolver::serve_asset(&asset!("/assets/main.css").to_string());
```

## Assets Must Be Used, Assets in Libraries

Because Dioxus uses the program's linker to save asset metadata, the resulting asset must be used somewhere in your application. If you forget to use the returned Asset, the Rust compiler is free to optimize away the call, and the asset metadata won't end up in the final output:

```rust
let ferrous = asset!("/assets/static/ferrous_wave.png");
rsx! {
    // our ferrous png won't be included since we forgot to use it!
    img { src: "..." }
}
```

This is expected behavior. We designed the asset system to automatically prune unused assets, making it possible for 3rd party libraries to export their own assets as part of their public API. For example, we could write a library that includes multiple stylesheets:

```rust
// crate: cool_dioxus_library
pub static GREEN_STYLES: Asset = asset!("/assets/red.css");
pub static RED_STYLES: Asset = asset!("/assets/green.css");
```

When a user builds an application using our library, they just need to import the stylesheet they want to use:

```rust
fn main() {
    dioxus::launch(|| rsx!{
        link { href: cool_dioxus_library::GREEN_STYLES, rel: "stylesheet" }
    })
}
```

Because the `RED_STYLES` asset is never referenced by the user's application, it won't be bundled in the final output.

However, you might want to include an asset even if you never reference it directly. Rust's [`#[used]`](https://doc.rust-lang.org/reference/abi.html#the-used-attribute) attribute is useful here, annotating to the compiler that asset _is_ used, even if we can't prove so at compile time.

```rust
#[used]
static CERTS: Asset = asset!("/assets/keys.cert");
```

## Including Folders

The asset macro also supports importing entire folders of content! The folder itself won't be copied into the final bundle. Instead, you join the name of the files in the folder against the folder's path. For example, you might need to include a folder of 3rd-party JavaScript into your app and don't want to use an `asset!` call for every file in the folder.

```rust
let logging_js_path = format!("{}/logging.js", asset!("/assets/posthog-js"));
```

Note that we need to format the `Asset` returned by the `asset!()` macro here because the actual folder name will receive an asset hash.

## Reading Assets

When you use an asset in an element like the `img` tag, the browser automatically fetches the asset for you. However, sometimes you might want to read the contents of an asset directly in your application code. For example, you might want to read a JSON file and parse it into a data structure.

To read assets at runtime, you can use the [`read_asset_bytes`](https://docs.rs/dioxus-asset-resolver/latest/dioxus_asset_resolver/fn.read_asset_bytes.html) from the [`asset_resolver`](https://docs.rs/dioxus-asset-resolver/latest/dioxus_asset_resolver/) module. This will either fetch the asset from the network (for the web target) or read it from the bundle (for native targets):

```rust
{{#include ../docs-router/src/doc_examples/assets.rs:read_assets}}
```

If you are targeting a platform like windows or macos where assets are bundled alongside the executable in the filesystem, you can also get the path to the asset using the `asset_path` function. Keep in mind this will not work in the browser or in android bundles since assets are not stored in the filesystem:

```rust
{{#include ../docs-router/src/doc_examples/assets.rs:resolve_assets}}
```

## The Public folder

If you're deploying your app to the web, then DX will automatically copy any files from your app's `/public` directory into the output `/public` directory.

This can be useful to copy files like `robots.txt` into the output directory since they're not referenced by any code in your app.

```
├── assets
├── src
└── public
    └── robots.txt
```

Note that this `/public` directory is *merged* into the output, allowing you to manually insert files into the output `/public/assets` directory.
