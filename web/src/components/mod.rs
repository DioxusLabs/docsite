mod header;
pub use header::*;

mod panes;
pub use panes::*;

mod modal;
pub use modal::*;

pub mod material_icons;

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    Page,
    Logs,
}
