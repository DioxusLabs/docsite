use dioxus::prelude::*;


pub static ExternalLinkIcon: Component<()> = |cx| {
    cx.render(rsx! {
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
    })
};

pub static Stacks: Component<()> = |cx| {
    cx.render(rsx!{
        svg { 
            class: "w-10 h-10 text-white p-2 bg-indigo-500 rounded-full",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            fill: "none",
            stroke_linecap: "round",
            xmlns: "http://www.w3.org/2000/svg",
            stroke_linejoin: "round",
            stroke_width: "2",
            path { 
                d: "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5",
            }
        }
    })
};


pub static ArrowRight: Component<()> = |cx| {
    cx.render(rsx! {
        svg { 
            class: "w-4 h-4 ml-1",
            stroke_linejoin: "round",
            stroke: "currentColor",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke_linecap: "round",
            path { 
                d: "M5 12h14M12 5l7 7-7 7",
            }
        }
    })
};


pub static Icon1: Component<()> = |cx| {
cx.render(rsx!(
svg { class: "w-6 h-6",
    stroke_linecap: "round",
    stroke_linejoin: "round",
    fill: "none",
    stroke_width: "2",
    view_box: "0 0 24 24",
    stroke: "currentColor",
    path { 
        d: "M22 12h-4l-3 9L9 3l-3 9H2",
    }
})) };

pub static Icon2: Component<()> = |cx| {
cx.render(rsx!(svg { class: "w-6 h-6",
    stroke_linejoin: "round",
    stroke_width: "2",
    view_box: "0 0 24 24",
    stroke_linecap: "round",
    stroke: "currentColor",
    fill: "none",
    circle { 
        r: "3",
        cx: "6",
        cy: "6",
    }
    circle { 
        cx: "6",
        cy: "18",
        r: "3",
    }
    path { 
        d: "M20 4L8.12 15.88M14.47 14.48L20 20M8.12 8.12L12 12",
    }
})) };

pub static Icon3: Component<()> = |cx| {
cx.render(rsx!(svg { class: "w-6 h-6",
    stroke_linecap: "round",
    fill: "none",
    stroke_linejoin: "round",
    stroke_width: "2",
    stroke: "currentColor",
    view_box: "0 0 24 24",
    path { 
        d: "M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2",
    }
    circle { 
        cx: "12",
        cy: "7",
        r: "4",
    }
})) };

pub static Icon4: Component<()> = |cx| {
cx.render(rsx!(svg { class: "w-6 h-6",
    stroke: "currentColor",
    view_box: "0 0 24 24",
    stroke_linecap: "round",
    fill: "none",
    stroke_linejoin: "round",
    stroke_width: "2",
    path { 
        d: "M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1zM4 22v-7",
    }
})) 
};

pub static Icon5: Component<()> = |cx| {
    cx.render(rsx!(
    svg { class: "w-6 h-6",
        fill: "none",
        stroke: "currentColor",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        view_box: "0 0 24 24",
        stroke_width: "2",
        path { d: "M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z" }
    })) 
};

pub static Icon6: Component<()> = |cx| {
    cx.render(rsx!(svg { class: "w-6 h-6",
        stroke: "currentColor",
        view_box: "0 0 24 24",
        stroke_linejoin: "round",
        fill: "none",
        stroke_width: "2",
        stroke_linecap: "round",
        path { 
            d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z",
        }
    })) 
};


pub static IconCheck: Component<()> = |cx| {
    cx.render(rsx!(
        svg { class: "text-indigo-500 w-6 h-6 flex-shrink-0 mr-4",
            stroke_width: "3",
            fill: "none",
            stroke_linejoin: "round",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_linecap: "round",
            path { d: "M22 11.08V12a10 10 0 11-5.93-9.14" }
            path { d: "M22 4L12 14.01l-3-3" }
        }
    ))
};

pub static TailwindLogo: Component<()> = |cx| {
    cx.render(rsx!(
        svg { class: "w-auto h-7 sm:h-8",
            view_box: "0 0 248 31",
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
    ))
};

pub static Copy: Component<()> = |cx| {
    cx.render(rsx!(
        svg { 
            width: "24",
            stroke_width: "1.5",
            height: "24",
            fill: "none",
            stroke: "currentColor",
            path { 
                d: "M8 16c0 1.886 0 2.828.586 3.414C9.172 20 10.114 20 12 20h4c1.886 0 2.828 0 3.414-.586C20 18.828 20 17.886 20 16v-4c0-1.886 0-2.828-.586-3.414C18.828 8 17.886 8 16 8m-8 8h4c1.886 0 2.828 0 3.414-.586C16 14.828 16 13.886 16 12V8m-8 8c-1.886 0-2.828 0-3.414-.586C4 14.828 4 13.886 4 12V8c0-1.886 0-2.828.586-3.414C5.172 4 6.114 4 8 4h4c1.886 0 2.828 0 3.414.586C16 5.172 16 6.114 16 8",
            }
        }
    ))
};


pub static GithubLogo: Component<()> = |cx| {
    cx.render(rsx!(
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
    ))
};


pub static Search: Component<()> = |cx| {
    cx.render(rsx!(
        svg { class: "text-gray-400 group-hover:text-gray-500 transition-colors duration-200",
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
    ))
};
