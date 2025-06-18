//! Server-specific implementations

use crate::{
    AppError, BuildStage, CargoDiagnostic, CargoDiagnosticSpan, CargoLevel, SocketError,
    SocketMessage,
};
use axum::http::StatusCode;
use axum::{extract::ws, response::IntoResponse};
use dioxus_dx_wire_format::{
    cargo_metadata::{diagnostic::DiagnosticLevel, CompilerMessage},
    BuildStage as DxBuildStage,
};
use dioxus_logger::tracing::{error, warn};

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

impl SocketMessage {
    pub fn into_axum(self) -> ws::Message {
        let msg = self
            .as_json_string()
            .expect("socket message should be valid json");
        ws::Message::Text(msg)
    }
}

impl TryFrom<ws::Message> for SocketMessage {
    type Error = SocketError;

    fn try_from(value: ws::Message) -> Result<Self, Self::Error> {
        let text = value.into_text()?;
        SocketMessage::try_from(text)
    }
}

/// TryFrom that fails for data we don't care about from cargo.
impl TryFrom<CompilerMessage> for CargoDiagnostic {
    type Error = ();

    fn try_from(value: CompilerMessage) -> Result<Self, Self::Error> {
        let diagnostic = value.message;

        let level = match diagnostic.level {
            DiagnosticLevel::Error => CargoLevel::Error,
            DiagnosticLevel::Warning => CargoLevel::Warning,
            _ => return Err(()),
        };

        let message = diagnostic.message;

        // Collect spans
        let spans = diagnostic
            .spans
            .iter()
            .map(|s| CargoDiagnosticSpan {
                is_primary: s.is_primary,
                line_start: s.line_start,
                line_end: s.line_end,
                column_start: s.column_start,
                column_end: s.column_end,
                label: s.label.clone(),
            })
            .collect();

        Ok(Self {
            target_crate: value.target.name,
            level,
            message,
            spans,
        })
    }
}

/// IntoResponse for app errors.
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::ResourceNotFound => StatusCode::NOT_FOUND,
            AppError::Parse(error) => {
                error!(?error, "parse error");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::Request(error) => {
                error!(?error, "request error");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            error => {
                warn!(?error, "unhandled `IntoResponse` server error");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        .into_response()
    }
}
