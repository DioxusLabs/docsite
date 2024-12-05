//! Material Design Icons

use dioxus::prelude::*;

#[component]
pub fn Warning() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "#FFB11F",
            "viewBox": "0 -960 960 960",
            path { d: "m40-120 440-760 440 760H40Zm138-80h604L480-720 178-200Zm302-40q17 0 28.5-11.5T520-280q0-17-11.5-28.5T480-320q-17 0-28.5 11.5T440-280q0 17 11.5 28.5T480-240Zm-40-120h80v-200h-80v200Zm40-100Z" }
        }
    }
}

#[component]
pub fn ArrowDownIcon() -> Element {
    rsx! {
        svg {
            height: "16px",
            width: "16px",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "currentColor",
            "viewBox": "0 -960 960 960",
            path { d: "M480-344 240-584l56-56 184 184 184-184 56 56-240 240Z" }
        }
    }
}
