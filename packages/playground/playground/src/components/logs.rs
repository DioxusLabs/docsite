use crate::build::BuildState;
use dioxus::prelude::*;
use model::{CargoDiagnosticSpan, CargoLevel};

#[component]
pub fn Logs() -> Element {
    let build = use_context::<BuildState>();
    let diagnostics = build.diagnostics()();
    let err_message = build.stage().err_message();

    rsx! {
        div {
            id: "logs",

            // Main failure reason.
            if let Some(message) = err_message {
                Log {
                    level: CargoLevel::Error,
                    message,
                    spans: Vec::new(),
                }
            }

            // Log diagnostics
            for diagnostic in diagnostics {
                Log {
                    level: diagnostic.level,
                    message: diagnostic.message,
                    spans: diagnostic.spans,
                }
            }
        }
    }
}

#[component]
fn Log(level: CargoLevel, message: String, spans: Vec<CargoDiagnosticSpan>) -> Element {
    let level = match level {
        CargoLevel::Error => ("Error", "level-error"),
        CargoLevel::Warning => ("Warning", "level-warn"),
    };

    rsx! {
        div {
            class: "log",
            // Level
            p {
                class: "log-level",
                span {
                    class: "{level.1}",
                    "{level.0}"
                }
            }

            div {
                class: "log-codeblock",
                // Message
                p {
                    class: "log-message",
                    "{message}",
                }

                for span in spans {
                    if let Some(label) = span.label {
                        p {
                            class: "log-span",
                            "-"
                            span {
                                class: "level-info",
                                " {span.line_start}"
                            }
                            ":"
                            span {
                                class: "level-info",
                                "{span.column_start}"
                            }
                            " {label}"
                        }
                    }
                }
            }
        }
    }
}
