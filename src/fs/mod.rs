use std::{
    fs, io,
    path::Path,
};
use crate::app::state::TreeEntry;
use crate::utils;

pub fn read_dir_entries(path: &Path) -> io::Result<Vec<fs::DirEntry>> {
       let mut entries = fs::read_dir(path)?
       .filter_map(|res| res.ok())
       .collect::<Vec<_>>();

       // Sort: directories first, then files, alphabetically
       entries.sort_by_key(|entry| {
           let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
           (!is_dir, entry.file_name())
       });
       Ok(entries)
   }

   // NEW: Helper function to recursively build the tree
pub fn build_recursive_tree(
       path: &Path,
       current_depth: usize,
       entries_list: &mut Vec<TreeEntry>,
   ) -> io::Result<()> {
       let entries = read_dir_entries(path)?;

       for entry in entries {
           let path = entry.path();
           let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

           // Add current entry
           entries_list.push(TreeEntry {
               entry, // entry is moved here
               depth: current_depth,
           });

           // If it was a directory, recurse
           if is_dir {
               // Ignore errors on recursion (e.g., permission denied)
               let _ = build_recursive_tree(&path, current_depth + 1, entries_list);
           }
       }
       Ok(())
   }


    // Gets "relevant information" for the preview panel (UNCHANGED)
    pub fn get_entry_info(entry: &fs::DirEntry) -> String {
        let mut info = String::new();
        info.push_str(&format!("Name: {}\n", entry.file_name().to_string_lossy()));

        if let Ok(metadata) = entry.metadata() {
            let file_type = if metadata.is_dir() {
                "Directory"
            } else if metadata.is_file() {
                "File"
            } else if metadata.is_symlink() {
                "Symlink"
            } else {
                "Other"
            };
            info.push_str(&format!("Type: {}\n", file_type));

            if metadata.is_file() {
                info.push_str(&format!("Size: {}\n", utils::format_size(metadata.len())));
            }

            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = modified.duration_since(std::time::SystemTime::UNIX_EPOCH) {
                    info.push_str(&format!("Modified (epoch): {}\n", duration.as_secs()));
                }
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let perms = metadata.permissions();
                info.push_str(&format!("Perms: {:o}\n", perms.mode() & 0o777));
            }
        } else {
            info.push_str("Could not read metadata.\n");
        }

        info
    }




