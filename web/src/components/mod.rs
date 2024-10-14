mod header;
pub use header::*;

mod panes;
pub use panes::*;

mod modal;
pub use modal::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    Page,
    Logs,
}
