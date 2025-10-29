use std::{
    env, fs,
    path::PathBuf,
    ffi::OsStr,
};
use ratatui::widgets::ListState;
use state::{App, TreeEntry, PreviewState};
use crate::fs::{build_recursive_tree, get_entry_info, read_dir_entries};

pub mod state;
pub mod handler;



impl App {
    // Create a new application instance
    pub fn new() -> App {
        let mut app = App {
            should_quit: false,
            recursive_view: false,
            current_path: env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            current_entries: Vec::new(),
            current_selected: ListState::default(),
            parent_entries: Vec::new(),
            parent_selected: ListState::default(),
            preview_content: PreviewState::None,
            preview_scroll_offset: 0, // CHANGED
        };
        app.current_selected.select(Some(0));
        app.update_panels(); // This will call update_preview() for us
        app
    }


    // Update the contents of the parent and current panels
    pub fn update_panels(&mut self) {
        // -- Update current entries --
        self.current_entries.clear(); // Clear the old list

        if self.recursive_view {
            // Build the full recursive tree
            let _ = build_recursive_tree(&self.current_path, 0, &mut self.current_entries);
        } else {
            // Original flat view logic
            if let Ok(entries) = read_dir_entries(&self.current_path) {
                for entry in entries {
                    self.current_entries.push(TreeEntry { entry, depth: 0 });
                }
            }
        }

        if self.current_entries.is_empty() {
            self.current_selected.select(None);
        } else {
            // Select 0, or clamp to new max
            let new_max = self.current_entries.len() - 1;
            if let Some(selected) = self.current_selected.selected() {
                if selected > new_max {
                    self.current_selected.select(Some(new_max));
                }
            } else {
                self.current_selected.select(Some(0));
            }
        }

        // -- Update parent entries (unchanged) --
        if let Some(parent_path) = self.current_path.parent() {
            self.parent_entries = read_dir_entries(parent_path).unwrap_or_default();
        } else {
            self.parent_entries.clear();
        }
        self.parent_selected.select(Some(0));
        self.update_preview();
    }

    // -- Event Handlers --

    // NEW: Toggle the recursive view
    pub fn toggle_recursive_view(&mut self) {
        self.recursive_view = !self.recursive_view;
        self.update_panels();
    }

    // In src/main.rs, inside impl App { ... }

    // NEW: Toggles the fold state of the selected directory
    pub fn toggle_fold(&mut self) {
        let selected_idx = match self.current_selected.selected() {
            Some(i) => i,
            None => return, // Nothing selected, do nothing
        };

        // Get info about the selected item
        let (current_path, current_depth) = {
            // Need to scope this borrow
            let selected_item = &self.current_entries[selected_idx];

            // Check if it's a directory. If not, do nothing.
            if !selected_item
                .entry
                .file_type()
                .map(|ft| ft.is_dir())
                .unwrap_or(false)
                {
                    return;
                }
                (selected_item.entry.path(), selected_item.depth)
        };

        // Check if it's already unfolded by peeking at the next item
        let is_unfolded = self
        .current_entries
        .get(selected_idx + 1)
        .map_or(false, |next_item| next_item.depth > current_depth);

        if is_unfolded {
            // --- Collapse (Refold) ---
            // Find the end of the items to remove.
            // We remove everything after this item until we hit an item with
            // a depth <= to the current item's depth.
            let end_range = self
            .current_entries
            .iter()
            .skip(selected_idx + 1)
            // Find the index of the *first* item that is NOT a child
            .position(|item| item.depth <= current_depth)
            // If we find one, its index is relative to (selected_idx + 1)
            .map_or(self.current_entries.len(), |i| i + selected_idx + 1);

            if end_range > selected_idx + 1 {
                self.current_entries.drain(selected_idx + 1..end_range);
            }
        } else {
            // --- Expand (Unfold) ---
            // We need to recursively build the tree for this folder
            let mut new_entries = Vec::new();

            // We use our existing recursive builder, starting at depth + 1
            // We ignore errors here (e.g., permissions)
            let _ = build_recursive_tree(&current_path, current_depth + 1, &mut new_entries);

            if !new_entries.is_empty() {
                // Insert the new entries right after the selected item
                self.current_entries
                .splice(selected_idx + 1..selected_idx + 1, new_entries);
            }
        }
    }
    // Go "into" a directory (like `cd <dir>` or `l`)
    pub fn enter_directory(&mut self) {
        if let Some(selected_idx) = self.current_selected.selected() {
            if let Some(tree_entry) = self.current_entries.get(selected_idx) {
                if tree_entry
                    .entry
                    .file_type()
                    .map(|ft| ft.is_dir())
                    .unwrap_or(false)
                    {
                        // Use the entry's full path
                        self.current_path = tree_entry.entry.path();
                        // Entering a directory always resets the view to flat
                        self.recursive_view = false;
                        self.update_panels();
                    }
            }
        }
    }

    // Go "up" a directory (like `cd ..` or `h`)
    pub fn leave_directory(&mut self) {
        if self.current_path.pop() {
            // Leaving a directory also resets the view to flat
            self.recursive_view = false;
            self.update_panels();
        }
    }

    // Move selection down (like `j`)
    pub fn select_next(&mut self) {
        let i = match self.current_selected.selected() {
            Some(i) => {
                if i >= self.current_entries.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.current_selected.select(Some(i));
        self.update_preview();
    }

    // Move selection up (like `k`)
    pub fn select_previous(&mut self) {
        let i = match self.current_selected.selected() {
            Some(i) => {
                if i == 0 {
                    if self.current_entries.is_empty() {
                        0
                    } else {
                        self.current_entries.len() - 1
                    }
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.current_selected.select(Some(i));
        self.update_preview();
    }

    // Get the currently selected entry (if any)
    pub fn get_selected_entry(&self) -> Option<&fs::DirEntry> {
        self.current_selected
        .selected()
        .and_then(|i| self.current_entries.get(i))
        .map(|tree_entry| &tree_entry.entry) // Get the inner entry
    }

    // ... inside impl App { ... }
    pub fn update_preview(&mut self) {
        // Reset scroll when selection changes
        self.preview_scroll_offset = 0; // CHANGED

        let selected_entry = match self.get_selected_entry() {
            Some(entry) => entry,
            None => {
                self.preview_content = PreviewState::None;
                return;
            }
        };

        // Check if it's a directory
        if selected_entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
            self.preview_content = PreviewState::Text(get_entry_info(selected_entry));
            return;
        }

        // Check if it's a Markdown file
        let is_markdown = selected_entry
        .path()
        .extension()
        .and_then(OsStr::to_str)
        .map_or(false, |ext| ext.eq_ignore_ascii_case("md"));

        if is_markdown {
            match fs::read_to_string(selected_entry.path()) {
                Ok(content) => {
                    self.preview_content = PreviewState::Markdown(content);
                }
                Err(e) => {
                    self.preview_content =
                    PreviewState::Text(format!("Error reading file:\n{}", e));
                }
            }
        } else {
            // Not markdown, just show regular info
            self.preview_content = PreviewState::Text(get_entry_info(selected_entry));
        }
    }


    // CHANGED: Scroll preview panel down
    pub fn scroll_preview_down(&mut self) {
        self.preview_scroll_offset = self.preview_scroll_offset.saturating_add(1);
    }

    // CHANGED: Scroll preview panel up
    pub fn scroll_preview_up(&mut self) {
        self.preview_scroll_offset = self.preview_scroll_offset.saturating_sub(1);
    }
}
