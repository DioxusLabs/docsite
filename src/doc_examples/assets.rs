/// ANCHOR: images
use dioxus::prelude::*;
use manganis::*;

fn App() -> Element {
    // You can link to assets that are relative to the package root or even link to an asset from a url
    // These assets will automatically be picked up by the dioxus cli, optimized, and bundled with your final applications
    const ASSET: manganis::ImageAsset = manganis::mg!(image("./public/static/ferrous_wave.png"));
    rsx! {
        img { src: "{ASSET}" }
    }
}
/// ANCHOR_END: images

/// ANCHOR: optimized_images
pub const ENUM_ROUTER_IMG: manganis::ImageAsset =
    manganis::mg!(image("./public/static/enum_router.png")
        // Manganis uses the builder pattern inside the macro. You can set the image size in pixels at compile time to send the smallest possible image to the client
        .size(52, 52)
        // You can also convert the image to a web friendly format at compile time. This can make your images significantly smaller
        .format(ImageType::Avif)
        // You can even tell manganis to preload the image so it's ready to be displayed as soon as it's needed
        .preload());

fn EnumRouter() -> Element {
    rsx! {
        img { src: "{ENUM_ROUTER_IMG}" }
    }
}
/// ANCHOR_END: optimized_images

// ANCHOR: arbitrary_files
// You can also collect arbitrary files. Relative paths are resolved relative to the package root
const PATH_TO_BUNDLED_CARGO_TOML: &str = manganis::mg!(file("./Cargo.toml"));
// ANCHOR_END: arbitrary_files

// ANCHOR: style_sheets
// You can also bundle stylesheets with your application
// Any files that end with .css will be minified and bundled with your application even if you don't explicitly include them in your <head>
const _: &str = manganis::mg!(file("./tailwind.css"));
// ANCHOR_END: style_sheets
