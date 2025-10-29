use ratatui::widgets::ListState;
use std::{fs, path::PathBuf};

pub enum PreviewState {
    None,
    Text(String), // For plain text info
    Markdown(String), // For markdown file content
}

// ## 1. Application State ##

// Represents an entry in the "Current" panel
pub struct TreeEntry {
    pub entry: fs::DirEntry,
    pub depth: usize,
}

// This struct holds all information about our app's current state.
pub struct App {
    pub should_quit: bool,
    pub recursive_view: bool, // NEW: Flag to toggle recursive view
    pub current_path: PathBuf,
    // State for the "Current" panel
    pub current_entries: Vec<TreeEntry>, // CHANGED: Now holds TreeEntry
    pub current_selected: ListState,
    // State for the "Parent" panel
    pub parent_entries: Vec<fs::DirEntry>,
    pub parent_selected: ListState,
    pub preview_content: PreviewState,
    pub preview_scroll_offset: u16, // CHANGED from preview_scroll
}
