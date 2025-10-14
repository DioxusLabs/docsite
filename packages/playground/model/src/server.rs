//! Server-specific implementations

use crate::{
    AppError, BuildStage, CargoDiagnostic, CargoDiagnosticSpan, CargoLevel, SocketError,
    SocketMessage,
};
use axum::http::StatusCode;
use axum::{extract::ws, response::IntoResponse};
use dioxus_dx_wire_format::cargo_metadata::diagnostic::{Diagnostic, DiagnosticSpan};
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
        ws::Message::Text(msg.into())
    }
}

impl TryFrom<ws::Message> for SocketMessage {
    type Error = SocketError;

    fn try_from(value: ws::Message) -> Result<Self, Self::Error> {
        let text = value.into_data();
        Ok(serde_json::from_slice(&text)?)
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
            .map(|s| s.clone().into())
            .collect();

        Ok(Self {
            target_crate: Some(value.target.name),
            level,
            message,
            spans,
            rendered: diagnostic.rendered,
        })
    }
}

/// TryFrom that fails for data we don't care about from cargo.
impl TryFrom<Diagnostic> for CargoDiagnostic {
    type Error = ();

    fn try_from(value: Diagnostic) -> Result<Self, Self::Error> {
        let level = CargoLevel::Error;

        let message = value.message;

        // Collect spans
        let spans = value
            .spans
            .iter()
            .map(|s| s.clone().into())
            .collect();

        Ok(Self {
            target_crate: None,
            level,
            message,
            spans,
            rendered: value.rendered,
        })
    }
}


impl From<DiagnosticSpan> for CargoDiagnosticSpan{
    fn from(value: DiagnosticSpan) -> Self {
        Self {
            is_primary: value.is_primary,
            line_start: value.line_start,
            line_end: value.line_end,
            column_start: value.column_start,
            column_end: value.column_end,
            label: value.label,
            file_name: value.file_name,
        }
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
