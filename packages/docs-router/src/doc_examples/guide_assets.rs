mod stylesheet {
    use dioxus::prelude::*;

    // ANCHOR: css_asset
    static CSS: Asset = asset!("/assets/main.css");
    // ANCHOR_END: css_asset

    // ANCHOR: css_stylesheet
    fn App() -> Element {
        rsx! {
            document::Stylesheet { href: CSS }
        }
    }
    // ANCHOR_END: css_stylesheet
}

mod image_url {
    use dioxus::prelude::*;

    fn App() -> Element {
        // ANCHOR: url_image
        rsx! {
            // ...
            div {
                img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
            }
        }
        // ANCHOR_END: url_image
    }
}

mod image_asset {
    use dioxus::prelude::*;

    fn App() -> Element {
        // ANCHOR: asset_image
        static ICON: Asset = asset!("/assets/icon.png");

        rsx! {
            img { src: ICON }
        }
        // ANCHOR_END: asset_image
    }
}

mod asset_optimization {
    use dioxus::prelude::*;

    fn main() {
        // ANCHOR: asset_optimization
        // would output main-j1238nask123.css
        asset!("/assets/main.css").to_string();
        // ANCHOR_END: asset_optimization
        // ANCHOR: image_asset_expansion
        // outputs icon-j1238jd2.avif
        asset!("/assets/icon.png", AssetOptions::image().with_avif());
        // ANCHOR_END: image_asset_expansion
    }
}
