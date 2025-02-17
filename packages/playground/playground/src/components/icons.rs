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
pub fn MenuIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: "24px",
            "viewBox": "0 -960 960 960",
            fill: "currentColor",
            path { d: "M120-240v-80h720v80H120Zm0-200v-80h720v80H120Zm0-200v-80h720v80H120Z" }
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

/// From https://github.com/n3r4zzurr0/svg-spinners
/// MIT License
#[component]
pub fn LoadingSpinner() -> Element {
    const STYLE: &str = ".spinner_P7sC{transform-origin:center;animation:spinner_svv2 .75s infinite linear}@keyframes spinner_svv2{100%{transform:rotate(360deg)}}";
    rsx! {
        svg {
            "viewBox": "0 0 24 24",
            height: "24",
            width: "24",
            fill: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            style { "{STYLE}" }
            path {
                fill: "currentColor",
                d: "M10.14,1.16a11,11,0,0,0-9,8.92A1.59,1.59,0,0,0,2.46,12,1.52,1.52,0,0,0,4.11,10.7a8,8,0,0,1,6.66-6.61A1.42,1.42,0,0,0,12,2.69h0A1.57,1.57,0,0,0,10.14,1.16Z",
                class: "spinner_P7sC",
            }
        }
    }
}
