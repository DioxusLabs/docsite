use dioxus::prelude::*;


pub static ExternalLinkIcon: FC<()> = |(cx, props)| {
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

pub static Stacks: FC<()> = |(cx, props)| {
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


pub static ArrowRight: FC<()> = |(cx, props)| {
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


pub static Icon1: FC<()> = |(cx, props)| {
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

pub static Icon2: FC<()> = |(cx, props)| {
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

pub static Icon3: FC<()> = |(cx, props)| {
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

pub static Icon4: FC<()> = |(cx, props)| {
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

pub static Icon5: FC<()> = |(cx, props)| {
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

pub static Icon6: FC<()> = |(cx, props)| {
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


pub static IconCheck: FC<()> = |(cx, props)| {
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
