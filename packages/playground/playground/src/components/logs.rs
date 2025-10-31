use std::collections::HashMap;

use crate::build::{BuildState, BuildStateStoreExt};
use crate::dx_components::accordion::*;
use ansi_parser::AnsiParser;
use dioxus::prelude::*;
use model::{CargoDiagnosticSpan, CargoLevel};

#[component]
pub fn Logs() -> Element {
    let build = use_context::<Store<BuildState>>();
    let diagnostics = build.diagnostics();
    let diagnostics = diagnostics.read();
    let diagnostics_with_spans = diagnostics.iter().filter(|d| !d.spans.is_empty());
    // Deduplicate diagnostics
    let diagnostics_with_spans: HashMap<_, _> = diagnostics_with_spans
        .enumerate()
        .map(|(i, item)| (item, i))
        .collect();
    let mut diagnostics_with_spans: Vec<_> = diagnostics_with_spans.into_iter().collect();
    diagnostics_with_spans.sort_by_key(|(_, id)| *id);
    let diagnostics_with_spans = diagnostics_with_spans
        .into_iter()
        .map(|(item, _)| item)
        .cloned();
    let err_message = build.stage().read().err_message();

    rsx! {
        // Main failure reason.
        if let Some(message) = err_message {
            h2 {
                "{message}"
            }
        }
        // Diagnostics
        Accordion {
            allow_multiple_open: true,
            horizontal: false,

            // Log diagnostics
            for (i, diagnostic) in diagnostics_with_spans.enumerate() {
                Log {
                    index: i,
                    level: diagnostic.level,
                    message: diagnostic.message,
                    spans: diagnostic.spans,
                    rendered: diagnostic.rendered
                }
            }
        }
    }
}

#[component]
fn Log(
    index: usize,
    level: CargoLevel,
    message: String,
    spans: Vec<CargoDiagnosticSpan>,
    rendered: Option<String>,
) -> Element {
    let level = match level {
        CargoLevel::Error => ("Error", "level-error"),
        CargoLevel::Warning => ("Warning", "level-warn"),
    };

    rsx! {
        AccordionItem { index,
            AccordionTrigger {
                display: "flex",
                justify_content: "space-between",
                align_items: "center",
                padding: "0.5rem 1rem",
                color: "black",
                "{message}"
                span {
                    "{level.0}"
                }
            }
            AccordionContent {
                RenderedLog {
                    rendered
                }
            }
        }
    }
}

#[component]
fn RenderedLog(rendered: Option<String>) -> Element {
    let Some(rendered) = rendered else {
        return rsx! {};
    };

    let mut fg_color = [0u8, 0, 0];
    let mut bg_color = [255u8, 255, 255];
    let mut bold = false;
    let iter = rendered.ansi_parse().filter_map(|output| match output {
        ansi_parser::Output::TextBlock(text) => {
            let background_color =
                format!("rgb({}, {}, {})", bg_color[0], bg_color[1], bg_color[2]);
            let color = format!("rgb({}, {}, {})", fg_color[0], fg_color[1], fg_color[2]);
            Some(rsx! {
                span {
                    background_color,
                    color,
                    font_weight: if bold { 400 },
                    {text}
                }
            })
        }
        ansi_parser::Output::Escape(ansi_parser::AnsiSequence::SetGraphicsMode(mode)) => {
            match mode.as_slice() {
                [0] => {
                    fg_color = [0u8, 0, 0];
                    bg_color = [255u8, 255, 255];
                    bold = false;
                }
                [1] => {
                    bold = true;
                }
                [38, 5, rgb_color] => fg_color = color_index_to_rgb(*rgb_color),
                [48, 5, rgb_color] => bg_color = color_index_to_rgb(*rgb_color),
                _ => {}
            }
            None
        }
        other => {
            tracing::info!("other: {other:?}");
            None
        }
    });

    rsx! {pre { {iter} }}
}

fn color_index_to_rgb(index: u8) -> [u8; 3] {
    match index {
        // Standard colors (0-15)
        0 => [0, 0, 0],        // Black
        1 => [128, 0, 0],      // Red
        2 => [0, 128, 0],      // Green
        3 => [128, 128, 0],    // Yellow
        4 => [0, 0, 128],      // Blue
        5 => [128, 0, 128],    // Magenta
        6 => [0, 128, 128],    // Cyan
        7 => [192, 192, 192],  // White
        8 => [128, 128, 128],  // Bright Black (Gray)
        9 => [255, 0, 0],      // Bright Red
        10 => [0, 255, 0],     // Bright Green
        11 => [255, 255, 0],   // Bright Yellow
        12 => [0, 0, 255],     // Bright Blue
        13 => [255, 0, 255],   // Bright Magenta
        14 => [0, 255, 255],   // Bright Cyan
        15 => [255, 255, 255], // Bright White

        // 216-color cube (16-231)
        16..=231 => {
            let cube_index = index - 16;
            let r = cube_index / 36;
            let g = (cube_index % 36) / 6;
            let b = cube_index % 6;

            // Map 0-5 to RGB values
            let value_map = [0, 95, 135, 175, 215, 255];
            [
                value_map[r as usize],
                value_map[g as usize],
                value_map[b as usize],
            ]
        }

        // Grayscale ramp (232-255)
        232..=255 => {
            let gray = 8 + (index - 232) * 10;
            [gray, gray, gray]
        }
    }
}
