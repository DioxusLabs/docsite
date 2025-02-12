use crate::docs::router_06::BookRoute;
use crate::*;
use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use gloo_timers::future::sleep;
use std::time::Duration;

#[component]
fn LogoImage(src: String, #[props(default = "h-6")] class: &'static str) -> Element {
    let mut scale = use_motion(1.0f32);

    let spring_config = Spring {
        stiffness: 300.0,
        damping: 20.0,
        mass: 1.0,
        velocity: 0.0,
    };

    let onmouseenter = move |_| {
        scale.animate_to(
            1.15,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );
    };

    let onmouseleave = move |_| {
        scale.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );
    };

    rsx! {
        img {
            class: "{class} cursor-pointer transition-all",
            style: "transform: scale({scale.get_value()})",
            src: "{src}",
            onmouseenter,
            onmouseleave,
        }
    }
}

// Add a new component for staggered logo animation
#[component]
fn StaggeredLogoImage(
    src: String,
    delay: f32,
    #[props(default = "h-6")] class: &'static str,
) -> Element {
    let mut opacity = use_motion(0.0f32);
    let mut y_offset = use_motion(20.0f32);
    let mut scale = use_motion(1.0f32);

    let spring_config = Spring {
        stiffness: 300.0,
        damping: 20.0,
        mass: 1.0,
        velocity: 0.0,
    };

    use_resource(move || async move {
        // gloo_timers uses milliseconds directly
        sleep(Duration::from_millis((delay * 1000.0) as u64)).await;
        opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );
        y_offset.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );
    });

    let onmouseenter = move |_| {
        scale.animate_to(
            1.15,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );
    };

    let onmouseleave = move |_| {
        scale.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );
    };

    rsx! {
        img {
            class: "{class} cursor-pointer transition-all",
            style: "transform: translateY({y_offset.get_value()}px) scale({scale.get_value()}); opacity: {opacity.get_value()}",
            src: "{src}",
            onmouseenter,
            onmouseleave,
        }
    }
}

pub(crate) fn Hero() -> Element {
    // Define motion states for different elements
    let mut heading_y = use_motion(-20.0f32);
    let mut heading_opacity = use_motion(0.0f32);
    let mut description_x = use_motion(-30.0f32);
    let mut description_opacity = use_motion(0.0f32);
    let mut buttons_scale = use_motion(0.95f32);
    let mut image_scale = use_motion(0.9f32);
    let mut logos_y = use_motion(20.0f32);
    let mut logos_opacity = use_motion(0.0f32);
    let mut image_y = use_motion(0.0f32);
    let mut em_opacity = use_motion(0.0f32);
    let mut em_scale = use_motion(0.95f32);

    // Add these new motion states
    let mut heading_scale = use_motion(0.95f32);
    let mut button_glow = use_motion(0.0f32);
    let mut heading_rotation = use_motion(0.0f32);

    // Animation configuration
    let spring_config = Spring {
        stiffness: 100.0,
        damping: 10.0,
        mass: 1.0,
        velocity: 0.0,
    };

    // Animate elements on mount
    let animate_hero = move |_| {
        heading_y.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );
        heading_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );

        description_x.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );
        description_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );

        buttons_scale.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );

        image_scale.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );

        logos_y.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );
        logos_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );

        image_y.animate_to(
            10.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config))
                .with_loop(LoopMode::Infinite), // Change LoopMode to Loop
        );

        em_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 50.0,
                damping: 8.0,
                mass: 1.0,
                velocity: 0.0,
            })),
        );
        em_scale.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(spring_config)),
        );

        // Add their animations in the animate_hero function
        heading_scale.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 150.0,
                damping: 15.0,
                mass: 1.0,
                velocity: 0.0,
            })),
        );
    };

    rsx! {
        section {
            class: "w-full mx-auto dark:text-white flex flex-col justify-between items-center border-b border-gray-300 min-h-[760px] flex-1 dark:border-[#a4a9ac7d] max-h-[960px] px-4",
            onmounted: animate_hero,

            div { class: "flex w-full max-w-screen-xl flex-col text-center md:min-h-[520px] min-h-[760px] h-[calc(100vh-4rem)] gap-2 justify-evenly",
                div { class: "flex flex-col-reverse lg:flex-row items-center justify-end lg:justify-between lg:flex-1 flex-none",
                    div { class: "text-center lg:text-left lg:flex-1",
                        div {
                            class: "text-[2.5em] md:text-[3.5em] font-semibold dark:text-white text-ghdarkmetal font-sans leading-snug text-balance",
                            style: "transform: translateY({heading_y.get_value()}px) scale({heading_scale.get_value()}) rotate({heading_rotation.get_value()}deg); opacity: {heading_opacity.get_value()}",
                            span { "One codebase, " }
                            span { " every platform." }
                        }
                        h3 {
                            class: "text-[1.25em] dark:text-white font-light text-ghdarkmetal max-w-screen-sm md:max-w-screen-md md:text-left text-center flex flex-col",
                            style: "transform: translateX({description_x.get_value()}px); opacity: {description_opacity.get_value()}",
                            span { class: "max-w-screen-md leading-loose",
                                "Dioxus is "
                                em {
                                    style: "opacity: {em_opacity.get_value()}; transform: scale({em_scale.get_value()})",
                                    class: "text-blue-500 dark:text-blue-400 font-medium",
                                    "the"
                                }
                                " Rust framework for building fullstack web, desktop, and mobile apps. Iterate with live hotreloading, add server functions, and deploy in record time."
                            }
                        }
                        div {
                            class: "pt-8 lg:pt-16 text-[1em] flex flex-row space-x-4 mx-auto lg:mx-0 justify-center lg:justify-start",
                            style: "transform: scale({buttons_scale.get_value()})",
                            Link {
                                to: Route::Docs06 {
                                    child: BookRoute::Index {
                                        section: Default::default(),
                                    },
                                },
                                class: "bg-ghdarkmetal dark:bg-[#EDEDED] text-white dark:text-black border border-[#a4a9ac7d] m-0 p-2 px-4 rounded transition-all duration-300 w-full md:w-auto dark:shadow-white relative overflow-hidden group",
                                style: "transform: scale({buttons_scale.get_value()})",
                                div { class: "absolute inset-0 bg-gradient-to-r from-blue-500/30 to-purple-500/30 opacity-0 group-hover:opacity-100 transition-opacity duration-300" }
                                "Get started"
                            }
                            Link {
                                to: "https://www.youtube.com/watch?v=WgAjWPKRVlQ",
                                new_tab: true,
                                class: "bg-[#EDEDED] dark:bg-ghdarkmetal  text-black dark:text-white border border-[#a4a9ac7d]  m-0 p-2 px-4 rounded md:hover:-translate-y-1 transition-transform duration-300 w-full md:w-auto gap-2 flex flex-row items-center justify-center",
                                "Take a tour"
                                span {
                                    svg {
                                        fill: "none",
                                        "viewBox": "0 0 24 24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "1.5rem",
                                        height: "1.5rem",
                                        circle {
                                            "stroke-width": "1.5",
                                            cx: "12",
                                            r: "10",
                                            stroke: "currentColor",
                                            cy: "12",
                                        }
                                        path {
                                            stroke: "currentColor",
                                            fill: "currentColor",
                                            "stroke-width": "1.5",
                                            d: "M15.4137 10.941C16.1954 11.4026 16.1954 12.5974 15.4137 13.059L10.6935 15.8458C9.93371 16.2944 9 15.7105 9 14.7868L9 9.21316C9 8.28947 9.93371 7.70561 10.6935 8.15419L15.4137 10.941Z",
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "lg:pb-12 h-screen max-h-40 lg:max-h-80 my-8",
                        style: "transform: scale({image_scale.get_value()}) translateY({image_y.get_value()}px)",
                        img {
                            src: asset!("/assets/static/multiplatform-dark.svg"),
                            class: "dark:hidden w-full h-full",
                            alt: "Animated Icon",
                        }
                        img {
                            src: asset!("/assets/static/multiplatform-light.svg"),
                            class: "hidden dark:block w-full h-full",
                            alt: "Animated Icon",
                        }
                    }
                }
                div {
                    class: "flex max-w-screen-2xl flex-col justify-end md:flex gap-4 pb-8 items-center",
                    style: "transform: translateY({logos_y.get_value()}px); opacity: {logos_opacity.get_value()}",
                    h1 { class: "lg:text-left text-center font-extralight text-sm",
                        "Trusted By"
                    }
                    div { class: "flex flex-row flex-wrap lg:justify-start justify-center invert dark:invert-0 gap-8 min-h-0",
                        StaggeredLogoImage {
                            src: asset!("/assets/static/airbuslogo.svg"),
                            delay: 0.0,
                        }
                        StaggeredLogoImage {
                            src: asset!("/assets/static/ESA_logo.svg"),
                            delay: 0.1,
                        }
                        StaggeredLogoImage {
                            src: asset!("/assets/static/yclogo.svg"),
                            delay: 0.2,
                        }
                        StaggeredLogoImage {
                            src: asset!("/assets/static/futurewei_bw.png"),
                            delay: 0.3,
                        }
                        StaggeredLogoImage {
                            src: asset!("/assets/static/satellite.webp"),
                            delay: 0.4,
                        }
                    }
                }
            }
        }
    }
}
