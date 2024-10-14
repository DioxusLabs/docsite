mod header;
pub use header::*;

mod panes;
pub use panes::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    Page,
    Logs,
}
