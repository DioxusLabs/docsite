pub mod builder;
pub mod watcher;

/// Represents a message from the build process.
#[derive(Debug)]
pub enum BuildMessage {
    Compiling {
        current_crate: usize,
        total_crates: usize,
        krate: String,
    },
    Finished,
    BuildError(String),

    /// A new queue position.
    QueuePosition(usize),
}

// The DX CLI serves parseable JSON output with the regular tracing message and a parseable "json" field.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CliMessage {
    json: String,
}
