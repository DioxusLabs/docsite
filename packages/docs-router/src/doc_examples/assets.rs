/// ANCHOR: images
use dioxus::prelude::*;

fn App() -> Element {
    // You can link to assets that are relative to the package root or even link to an asset from a url
    // These assets will automatically be picked up by the dioxus cli, optimized, and bundled with your final applications
    let ferrous = asset!("/assets/static/ferrous_wave.png");

    rsx! {
        img { src: "{ferrous}" }
    }
}
/// ANCHOR_END: images

/// ANCHOR: optimized_images
pub const ENUM_ROUTER_IMG: Asset = asset!(
    "/assets/static/enum_router.png",
    // You can pass a second argument to the asset macro to set up options for the asset
    ImageAssetOptions::new()
        // You can set the image size in pixels at compile time to send the smallest possible image to the client
        .with_size(ImageSize::Manual {
            width: 52,
            height: 52
        })
        // You can also convert the image to a web friendly format at compile time. This can make your images significantly smaller
        .with_format(ImageFormat::Avif)
);

fn EnumRouter() -> Element {
    rsx! {
        img { src: "{ENUM_ROUTER_IMG}" }
    }
}
/// ANCHOR_END: optimized_images

// ANCHOR: arbitrary_files
// You can also collect arbitrary files. Relative paths are resolved relative to the package root
const PATH_TO_BUNDLED_CARGO_TOML: Asset = asset!("/Cargo.toml");
// ANCHOR_END: arbitrary_files

// ANCHOR: style_sheets
// You can also bundle stylesheets with your application
// Any files that end with .css will be minified and bundled with your application even if you don't explicitly include them in your <head>
const _: Asset = asset!("/assets/tailwind.css");
// ANCHOR_END: style_sheets

async fn read_assets() {
    // ANCHOR: read_assets
    use dioxus::prelude::*;

    // Bundle the static JSON asset into the application
    static JSON_ASSET: Asset = asset!("assets/data.json");

    // Read the bytes of the JSON asset
    let bytes = dioxus::asset_resolver::read_asset_bytes(&JSON_ASSET)
        .await
        .unwrap();

    // Deserialize the JSON data
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["key"].as_str(), Some("value"));
    // ANCHOR_END: read_assets
}

fn resolve_assets() {
    // ANCHOR: resolve_assets
    use dioxus::prelude::*;

    // Bundle the static JSON asset into the application
    static JSON_ASSET: Asset = asset!("assets/data.json");

    // Resolve the path of the asset. This will not work in web or Android bundles
    let path = dioxus::asset_resolver::asset_path(&JSON_ASSET).unwrap();

    println!("Asset path: {:?}", path);

    // Read the bytes of the JSON asset
    let bytes = std::fs::read(path).unwrap();

    // Deserialize the JSON data
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["key"].as_str(), Some("value"));
    // ANCHOR_END: resolve_assets
}
