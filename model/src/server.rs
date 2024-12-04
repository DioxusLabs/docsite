//! Server-specific implementations

use crate::{
    BuildStage, CargoDiagnostic, CargoDiagnosticSpan, CargoLevel, SocketError, SocketMessage,
};
use axum::extract::ws;
use dioxus_dx_wire_format::{
    cargo_metadata::{diagnostic::DiagnosticLevel, CompilerMessage},
    BuildStage as DxBuildStage,
};

impl From<DxBuildStage> for BuildStage {
    fn from(value: DxBuildStage) -> Self {
        match value {
            DxBuildStage::RunningBindgen => BuildStage::RunningBindgen,
            DxBuildStage::Compiling {
                current,
                total,
                krate,
                ..
            } => Self::Compiling {
                crates_compiled: current,
                total_crates: total,
                current_crate: krate,
            },
            _ => BuildStage::Other,
        }
    }
}

impl TryFrom<ws::Message> for SocketMessage {
    type Error = SocketError;

    fn try_from(value: ws::Message) -> Result<Self, Self::Error> {
        let text = value.into_text()?;
        SocketMessage::try_from(text)
    }
}

impl From<axum::Error> for SocketError {
    fn from(value: axum::Error) -> Self {
        Self::Parse(Box::new(value))
    }
}

/// TryFrom that fails for data we don't care about from cargo.
impl TryFrom<CompilerMessage> for CargoDiagnostic {
    type Error = ();

    fn try_from(value: CompilerMessage) -> Result<Self, Self::Error> {
        let diagnostic = value.message;

        println!("{:?} | {}", diagnostic.level, diagnostic.message);

        let level = match diagnostic.level {
            DiagnosticLevel::Error => CargoLevel::Error,
            DiagnosticLevel::Warning => CargoLevel::Warning,
            _ => return Err(()),
        };

        // let message = match diagnostic.rendered {
        //     Some(m) => m,
        //     None => diagnostic.message,
        // };

        let message = diagnostic.message;

        // Collect spans
        let spans = diagnostic
            .spans
            .iter()
            .map(|s| CargoDiagnosticSpan {
                line_start: s.line_start,
                line_end: s.line_end,
                column_start: s.column_start,
                column_end: s.column_end,
                label: s.label.clone(),
            })
            .collect();

        Ok(Self {
            level,
            message,
            spans,
        })
    }
}
