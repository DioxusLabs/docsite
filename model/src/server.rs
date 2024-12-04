//! Server-specific implementations

use crate::{BuildStage, SocketError, SocketMessage};
use axum::extract::ws;
use dioxus_dx_wire_format::BuildStage as DxBuildStage;

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