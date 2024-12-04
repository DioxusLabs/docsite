use uuid::Uuid;

pub mod builder;
pub mod watcher;

/// A message from the playground build process.
#[derive(Debug, Clone, PartialEq)]
pub enum BuildMessage {
    Building(model::BuildStage),
    Finished(Result<Uuid, String>),
    QueuePosition(usize),
}

// The DX CLI serves parseable JSON output with the regular tracing message and a parseable "json" field.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CliMessage {
    json: Option<String>,
}
