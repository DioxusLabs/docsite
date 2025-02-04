#![allow(unused)]

use dioxus::prelude::*;
pub(crate) static FERROUS_LOGO: &str = "https://rustacean.net/assets/rustacean-flat-gesture.png";

pub(crate) fn ArrowRight() -> Element {
    rsx! {
        svg {
            class: "w-4 h-4 ml-1",
            stroke_linejoin: "round",
            stroke: "currentColor",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke_linecap: "round",
            path { d: "M5 12h14M12 5l7 7-7 7" }
        }
    }
}
pub(crate) fn ChevronDownIcon() -> Element {
    rsx! {
        svg {
            height: "24",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            "stroke-width": "1.5",
            "viewBox": "0 0 24 24",
            "data-testid": "geist-icon",
            "shape-rendering": "geometricPrecision",
            stroke: "currentColor",
            width: "24",
            style: "color: currentcolor; width: 16px; height: 16px;",
            fill: "none",
            class: "with-icon_icon__MHUeb",
            path { d: "M6 9l6 6 6-6" }
        }
    }
}
pub(crate) fn ChevronRightIconSmall() -> Element {
    rsx! {
        svg {
            fill: "none",
            "stroke-linejoin": "round",
            "shape-rendering": "geometricPrecision",
            height: "24",
            "stroke-linecap": "round",
            "data-testid": "geist-icon",
            style: "color: currentcolor; width: 18px; height: 18px;",
            width: "24",
            stroke: "currentColor",
            "stroke-width": "1.5",
            "viewBox": "0 0 24 24",
            class: "with-icon_icon__MHUeb",
            path { d: "M9 18l6-6-6-6" }
        }
    }
}

pub(crate) fn ChevronRightIcon() -> Element {
    rsx! {
        span {
            svg {
                "stroke-width": "1.5",
                height: "24",
                "shape-rendering": "geometricPrecision",
                "stroke-linecap": "round",
                "data-testid": "geist-icon",
                fill: "none",
                stroke: "currentColor",
                "stroke-linejoin": "round",
                "viewBox": "0 0 24 24",
                width: "24",
                style: "color: currentcolor; width: 18px; height: 18px;",
                class: "with-icon_icon__MHUeb",
                path { d: "M9 18l6-6-6-6" }
            }
        }
    }
}
pub(crate) fn ChevronLeftIcon() -> Element {
    rsx! {
        span {
            svg {
                "stroke-width": "1.5",
                height: "24",
                "shape-rendering": "geometricPrecision",
                "stroke-linecap": "round",
                "data-testid": "geist-icon",
                fill: "none",
                stroke: "currentColor",
                "stroke-linejoin": "round",
                "viewBox": "0 0 24 24",
                width: "24",
                style: "color: currentcolor; width: 18px; height: 18px;",
                path { d: "M9 18l6-6-6-6" }
            }
        }
    }
}

pub(crate) fn DocumentIcon() -> Element {
    rsx! {
        div { class: "style_itemPrefix__HSCJc",
            svg {
                "data-testid": "geist-icon",
                height: "16",
                "viewBox": "0 0 16 16",
                width: "16",
                "stroke-linejoin": "round",
                style: "color: currentcolor;",
                path {
                    d: "M14.5 13.5V6.5V5.41421C14.5 5.149 14.3946 4.89464 14.2071 4.70711L9.79289 0.292893C9.60536 0.105357 9.351 0 9.08579 0H8H3H1.5V1.5V13.5C1.5 14.8807 2.61929 16 4 16H12C13.3807 16 14.5 14.8807 14.5 13.5ZM13 13.5V6.5H9.5H8V5V1.5H3V13.5C3 14.0523 3.44772 14.5 4 14.5H12C12.5523 14.5 13 14.0523 13 13.5ZM9.5 5V2.12132L12.3787 5H9.5ZM5.13 5.00062H4.505V6.25062H5.13H6H6.625V5.00062H6H5.13ZM4.505 8H5.13H11H11.625V9.25H11H5.13H4.505V8ZM5.13 11H4.505V12.25H5.13H11H11.625V11H11H5.13Z",
                    "fill-rule": "evenodd",
                    fill: "currentColor",
                    "clip-rule": "evenodd",
                }
            }
        }
    }
}

pub(crate) fn ExternalLinkIcon2() -> Element {
    rsx! {
        svg {
            style: "color: currentcolor; width: 14px; height: 14px;",
            fill: "none",
            stroke: "currentColor",
            "stroke-linejoin": "round",
            "shape-rendering": "geometricPrecision",
            width: "24",
            "stroke-width": "1.5",
            "data-testid": "geist-icon",
            height: "24",
            "stroke-linecap": "round",
            "viewBox": "0 0 24 24",
            path { d: "M7 17L17 7" }
            path { d: "M7 7h10v10" }
        }
    }
}

pub(crate) fn ExternalLinkIcon() -> Element {
    rsx! {
        svg {
            width: "15",
            height: "15",
            y: "0px",
            x: "0px",
            view_box: "0 0 100 100",
            path {
                fill: "currentColor",
                d: "M18.8,85.1h56l0,0c2.2,0,4-1.8,4-4v-32h-8v28h-48v-48h28v-8h-32l0, 0c-2.2,0-4,1.8-4,4v56C14.8,83.3,16.6,85.1,18.8,85.1z",
            }
            polygon {
                points: "45.7,48.7 51.3,54.3 77.2,28.5 77.2,37.2 85.2,37.2 85.2,14.9 62.8,14.9 62.8,22.9 71.5,22.9",
                fill: "currentColor",
            }
        }
    }
}

pub(crate) fn Stacks() -> Element {
    rsx! {
        svg {
            class: "w-10 h-10 text-white p-2 bg-indigo-500 rounded-full",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            fill: "none",
            stroke_linecap: "round",
            xmlns: "http://www.w3.org/2000/svg",
            stroke_linejoin: "round",
            stroke_width: "2",
            path { d: "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" }
        }
    }
}

pub(crate) static IconCheck: Component<()> = |cx| {
    rsx!(
        svg {
            class: "text-indigo-500 w-6 h-6 flex-shrink-0 mr-4",
            stroke_width: "3",
            fill: "none",
            stroke_linejoin: "round",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_linecap: "round",
            path { d: "M22 11.08V12a10 10 0 11-5.93-9.14" }
            path { d: "M22 4L12 14.01l-3-3" }
        }
    )
};

pub(crate) static TailwindLogo: Component<()> = |cx| {
    rsx!(
        svg { class: "w-auto h-7 sm:h-8", view_box: "0 0 248 31",
            path {
                clip_rule: "evenodd",
                fill: "#06B6D4",
                fill_rule: "evenodd",
                d: "M25.517 0C18.712 0 14.46 3.382 12.758 10.146c2.552-3.382 5.529-4.65 8.931-3.805 1.941.482 3.329 1.882 4.864 3.432 2.502 2.524 5.398 5.445 11.722 5.445 6.804 0 11.057-3.382 12.758-10.145-2.551 3.382-5.528 4.65-8.93 3.804-1.942-.482-3.33-1.882-4.865-3.431C34.736 2.92 31.841 0 25.517 0zM12.758 15.218C5.954 15.218 1.701 18.6 0 25.364c2.552-3.382 5.529-4.65 8.93-3.805 1.942.482 3.33 1.882 4.865 3.432 2.502 2.524 5.397 5.445 11.722 5.445 6.804 0 11.057-3.381 12.758-10.145-2.552 3.382-5.529 4.65-8.931 3.805-1.941-.483-3.329-1.883-4.864-3.432-2.502-2.524-5.398-5.446-11.722-5.446z",
            }
            path {
                fill: "#000",
                clip_rule: "evenodd",
                fill_rule: "evenodd",
                d: "M76.546 12.825h-4.453v8.567c0 2.285 1.508 2.249 4.453 2.106v3.463c-5.962.714-8.332-.928-8.332-5.569v-8.567H64.91V9.112h3.304V4.318l3.879-1.143v5.937h4.453v3.713zM93.52 9.112h3.878v17.849h-3.878v-2.57c-1.365 1.891-3.484 3.034-6.285 3.034-4.884 0-8.942-4.105-8.942-9.389 0-5.318 4.058-9.388 8.942-9.388 2.801 0 4.92 1.142 6.285 2.999V9.112zm-5.674 14.636c3.232 0 5.674-2.392 5.674-5.712s-2.442-5.711-5.674-5.711-5.674 2.392-5.674 5.711c0 3.32 2.442 5.712 5.674 5.712zm16.016-17.313c-1.364 0-2.477-1.142-2.477-2.463a2.475 2.475 0 012.477-2.463 2.475 2.475 0 012.478 2.463c0 1.32-1.113 2.463-2.478 2.463zm-1.939 20.526V9.112h3.879v17.849h-3.879zm8.368 0V.9h3.878v26.06h-3.878zm29.053-17.849h4.094l-5.638 17.849h-3.807l-3.735-12.03-3.771 12.03h-3.806l-5.639-17.849h4.094l3.484 12.315 3.771-12.315h3.699l3.734 12.315 3.52-12.315zm8.906-2.677c-1.365 0-2.478-1.142-2.478-2.463a2.475 2.475 0 012.478-2.463 2.475 2.475 0 012.478 2.463c0 1.32-1.113 2.463-2.478 2.463zm-1.939 20.526V9.112h3.878v17.849h-3.878zm17.812-18.313c4.022 0 6.895 2.713 6.895 7.354V26.96h-3.878V16.394c0-2.713-1.58-4.14-4.022-4.14-2.55 0-4.561 1.499-4.561 5.14v9.567h-3.879V9.112h3.879v2.285c1.185-1.856 3.124-2.749 5.566-2.749zm25.282-6.675h3.879V26.96h-3.879v-2.57c-1.364 1.892-3.483 3.034-6.284 3.034-4.884 0-8.942-4.105-8.942-9.389 0-5.318 4.058-9.388 8.942-9.388 2.801 0 4.92 1.142 6.284 2.999V1.973zm-5.674 21.775c3.232 0 5.674-2.392 5.674-5.712s-2.442-5.711-5.674-5.711-5.674 2.392-5.674 5.711c0 3.32 2.442 5.712 5.674 5.712zm22.553 3.677c-5.423 0-9.481-4.105-9.481-9.389 0-5.318 4.058-9.388 9.481-9.388 3.519 0 6.572 1.82 8.008 4.605l-3.34 1.928c-.79-1.678-2.549-2.749-4.704-2.749-3.16 0-5.566 2.392-5.566 5.604 0 3.213 2.406 5.605 5.566 5.605 2.155 0 3.914-1.107 4.776-2.749l3.34 1.892c-1.508 2.82-4.561 4.64-8.08 4.64zm14.472-13.387c0 3.249 9.661 1.285 9.661 7.89 0 3.57-3.125 5.497-7.003 5.497-3.591 0-6.177-1.607-7.326-4.177l3.34-1.927c.574 1.606 2.011 2.57 3.986 2.57 1.724 0 3.052-.571 3.052-2 0-3.176-9.66-1.391-9.66-7.781 0-3.356 2.909-5.462 6.572-5.462 2.945 0 5.387 1.357 6.644 3.713l-3.268 1.82c-.647-1.392-1.904-2.035-3.376-2.035-1.401 0-2.622.607-2.622 1.892zm16.556 0c0 3.249 9.66 1.285 9.66 7.89 0 3.57-3.124 5.497-7.003 5.497-3.591 0-6.176-1.607-7.326-4.177l3.34-1.927c.575 1.606 2.011 2.57 3.986 2.57 1.724 0 3.053-.571 3.053-2 0-3.176-9.66-1.391-9.66-7.781 0-3.356 2.908-5.462 6.572-5.462 2.944 0 5.386 1.357 6.643 3.713l-3.268 1.82c-.646-1.392-1.903-2.035-3.375-2.035-1.401 0-2.622.607-2.622 1.892z",
            }
        }
    )
};

pub(crate) static Copy: Component<()> = |cx| {
    rsx!(
        svg {
            width: "24",
            height: "24",
            stroke_width: "1.5",
            fill: "none",
            stroke: "currentColor",
            path { d: "M8 16c0 1.886 0 2.828.586 3.414C9.172 20 10.114 20 12 20h4c1.886 0 2.828 0 3.414-.586C20 18.828 20 17.886 20 16v-4c0-1.886 0-2.828-.586-3.414C18.828 8 17.886 8 16 8m-8 8h4c1.886 0 2.828 0 3.414-.586C16 14.828 16 13.886 16 12V8m-8 8c-1.886 0-2.828 0-3.414-.586C4 14.828 4 13.886 4 12V8c0-1.886 0-2.828.586-3.414C5.172 4 6.114 4 8 4h4c1.886 0 2.828 0 3.414.586C16 5.172 16 6.114 16 8" }
        }
    )
};

pub(crate) static GithubLogo: Component<()> = |cx| {
    rsx!(
        svg {
            height: "24",
            width: "24",
            fill: "currentColor",
            view_box: "0 0 16 16",
            path {
                fill_rule: "evenodd",
                d: "M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z",
            }
        }
    )
};

pub(crate) static DiscordLogo: Component<()> = |cx| {
    rsx!(
        svg {
            width: "24",
            height: "24",
            view_box: "0 -28.5 256 256",
            preserve_aspect_ratio: "xMidYMid",
            g {
                path {
                    d: "M216.856339,16.5966031 C200.285002,8.84328665 182.566144,3.2084988 164.041564,0 C161.766523,4.11318106 159.108624,9.64549908 157.276099,14.0464379 C137.583995,11.0849896 118.072967,11.0849896 98.7430163,14.0464379 C96.9108417,9.64549908 94.1925838,4.11318106 91.8971895,0 C73.3526068,3.2084988 55.6133949,8.86399117 39.0420583,16.6376612 C5.61752293,67.146514 -3.4433191,116.400813 1.08711069,164.955721 C23.2560196,181.510915 44.7403634,191.567697 65.8621325,198.148576 C71.0772151,190.971126 75.7283628,183.341335 79.7352139,175.300261 C72.104019,172.400575 64.7949724,168.822202 57.8887866,164.667963 C59.7209612,163.310589 61.5131304,161.891452 63.2445898,160.431257 C105.36741,180.133187 151.134928,180.133187 192.754523,160.431257 C194.506336,161.891452 196.298154,163.310589 198.110326,164.667963 C191.183787,168.842556 183.854737,172.420929 176.223542,175.320965 C180.230393,183.341335 184.861538,190.991831 190.096624,198.16893 C211.238746,191.588051 232.743023,181.531619 254.911949,164.955721 C260.227747,108.668201 245.831087,59.8662432 216.856339,16.5966031 Z M85.4738752,135.09489 C72.8290281,135.09489 62.4592217,123.290155 62.4592217,108.914901 C62.4592217,94.5396472 72.607595,82.7145587 85.4738752,82.7145587 C98.3405064,82.7145587 108.709962,94.5189427 108.488529,108.914901 C108.508531,123.290155 98.3405064,135.09489 85.4738752,135.09489 Z M170.525237,135.09489 C157.88039,135.09489 147.510584,123.290155 147.510584,108.914901 C147.510584,94.5396472 157.658606,82.7145587 170.525237,82.7145587 C183.391518,82.7145587 193.761324,94.5189427 193.539891,108.914901 C193.539891,123.290155 183.391518,135.09489 170.525237,135.09489 Z",
                    fill: "currentColor",
                    fill_rule: "nonzero",
                }
            }
        }
    )
};

pub(crate) static Search: Component<()> = |cx| {
    rsx!(
        svg {
            class: "text-gray-400 group-hover:text-gray-500 transition-colors duration-200",
            height: "24",
            fill: "none",
            width: "24",
            path {
                stroke_linecap: "round",
                stroke: "currentColor",
                stroke_width: "2",
                d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
                stroke_linejoin: "round",
            }
        }
    )
};

pub(crate) static Github2: Component<()> = |cx| {
    rsx!(
        svg {
            class: "w-5 h-5",
            "viewBox": "0 0 16 16",
            "aria-hidden": "true",
            fill: "currentColor",
            path { d: "M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z" }
        }
    )
};

pub(crate) static IconCheckGh: Component<()> = |cx| {
    rsx! {
        svg {
            class: "octicon octicon-check js-clipboard-check-icon d-inline-block d-none",
            fill: "rgb(26, 127, 55)",
            height: "24",
            version: "1.1",
            "aria_hidden": "true",
            width: "24",
            view_box: "0 0 16 16",
            "data_view_component": "true",
            path {
                d: "M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
};

pub(crate) fn DropdownChevrons() -> Element {
    rsx! {
        svg {
            "stroke-linecap": "round",
            width: "24",
            "shape-rendering": "geometricPrecision",
            height: "24",
            "data-testid": "geist-icon",
            stroke: "currentColor",
            "viewBox": "0 0 24 24",
            "stroke-width": "1.5",
            style: "color: currentcolor; width: 20px; height: 20px;",
            fill: "none",
            "aria-hidden": "true",
            "stroke-linejoin": "round",
            class: "with-icon_icon__MHUeb",
            path { d: "M17 8.517L12 3 7 8.517M7 15.48l5 5.517 5-5.517" }
        }
    }
}

pub(crate) fn VersionTagIcon() -> Element {
    rsx! {
        svg {
            style: "width: 16px; height: 16px; color: currentcolor;",
            "stroke-linejoin": "round",
            "viewBox": "0 0 16 16",
            width: "16",
            height: "16",
            class: "translate-x-px translate-y-px",
            path {
                "fill-rule": "evenodd",
                fill: "currentColor",
                d: "M1.5 1.5H6.34315C7.00619 1.5 7.64207 1.76339 8.11091 2.23223L13.8787 8L8 13.8787L2.23223 8.11091C1.76339 7.64207 1.5 7.00619 1.5 6.34315V1.5ZM16 8L14.9393 6.93934L9.17157 1.17157C8.42143 0.421427 7.40401 0 6.34315 0H1.5H0V1.5V6.34315C0 7.40401 0.421426 8.42143 1.17157 9.17157L6.93934 14.9393L8 16L9.06066 14.9393L14.9393 9.06066L16 8ZM4.5 5.25C4.91421 5.25 5.25 4.91421 5.25 4.5C5.25 4.08579 4.91421 3.75 4.5 3.75C4.08579 3.75 3.75 4.08579 3.75 4.5C3.75 4.91421 4.08579 5.25 4.5 5.25Z",
                "clip-rule": "evenodd",
            }
        }
    }
}

pub(crate) fn IconWarning() -> Element {
    rsx! {
        svg {
            "stroke-linejoin": "round",
            width: "16",
            "data-testid": "geist-icon",
            "viewBox": "0 0 16 16",
            style: "width: 16px; height: 16px; color: var(--ds-amber-900);",
            height: "16",
            path {
                "fill-rule": "evenodd",
                fill: "currentColor",
                "clip-rule": "evenodd",
                d: "M8.55846 2H7.44148L1.88975 13.5H14.1102L8.55846 2ZM9.90929 1.34788C9.65902 0.829456 9.13413 0.5 8.55846 0.5H7.44148C6.86581 0.5 6.34092 0.829454 6.09065 1.34787L0.192608 13.5653C-0.127943 14.2293 0.355835 15 1.09316 15H14.9068C15.6441 15 16.1279 14.2293 15.8073 13.5653L9.90929 1.34788ZM8.74997 4.75V5.5V8V8.75H7.24997V8V5.5V4.75H8.74997ZM7.99997 12C8.55226 12 8.99997 11.5523 8.99997 11C8.99997 10.4477 8.55226 10 7.99997 10C7.44769 10 6.99997 10.4477 6.99997 11C6.99997 11.5523 7.44769 12 7.99997 12Z",
            }
        }
    }
}
