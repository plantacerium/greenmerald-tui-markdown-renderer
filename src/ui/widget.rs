use ratatui::{
    style::{Color, Style},
    widgets::ListItem,
};


use crate::app::state::TreeEntry;
use std::fs;

// ## 5. UI Helpers ##

// Formats a flat directory entry (for Parent panel)

// NEW: Formats a TreeEntry, adding indentation (for Current panel)
pub fn format_entry_tree(tree_entry: &TreeEntry) -> ListItem {
    let entry = &tree_entry.entry;
    let file_name = entry.file_name().to_string_lossy().to_string();
    let metadata = entry.metadata().ok();
    let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);

    let (icon, style) =
    if is_dir {
        ("📁 ", Style::default().fg(Color::Cyan)) // Directory
    } else {
        ("📄 ", Style::default().fg(Color::White)) // File
    };

    // Add indentation based on depth
    let indent = "  ".repeat(tree_entry.depth);

    ListItem::new(format!("{indent}{icon}{file_name}")).style(style)
}


pub fn format_entry_flat(entry: &fs::DirEntry) -> ListItem {
    let file_name = entry.file_name().to_string_lossy().to_string();
    let metadata = entry.metadata().ok();
    let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);

    let (icon, style) = if is_dir {
        ("📁 ", Style::default().fg(Color::Cyan))
    } else {
        ("📄 ", Style::default().fg(Color::White))
    };

    ListItem::new(format!("{icon}{file_name}")).style(style)
}
