// src/lib.rs

// 1. Module Declarations: Define the crate's internal structure.
// These lines tell the Rust compiler to look for the code in
// src/app/mod.rs, src/fs/mod.rs, src/ui/mod.rs, and src/utils/mod.rs.
pub mod app;
pub mod fs;
pub mod ui;
pub mod utils;

// 2. Convenience Re-exports (The Public API):
// This allows users (like main.rs) to import key components directly from the
// crate root (e.g., `use file_explorer_tui::App`) instead of the deep path
// (e.g., `use file_explorer_tui::app::App`).

// Re-export the core application state (structs/enums)
pub use app::state::{App, PreviewState, TreeEntry};

// Re-export the main event handler result
pub use app::handler::HandlerResult;

// Re-export the main UI drawing function
pub use ui::ui;
