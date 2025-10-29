use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// We bring in the App struct from the parent module (src/app/mod.rs or src/app/state.rs)
use super::state::App;

/// Defines the return type for event handling, allowing us to signal if the application
/// should quit.
pub enum HandlerResult {
    Continue,
    Quit,
}

impl App {
    /// Processes a single KeyEvent and updates the application state accordingly.
    ///
    /// This method is the core command interpreter, translating user input
    /// into actionable state changes, aligning with the Command Pattern.
    pub fn handle_key_event(&mut self, key: KeyEvent) -> HandlerResult {
        let (code, modifiers) = (key.code, key.modifiers);

        match (code, modifiers) {
            // --- Global Control ---
            // Quit application on 'q'
            (KeyCode::Char('q'), _) => return HandlerResult::Quit,

            // --- Preview Panel Scrolling (Ctrl+j/k) ---
            (KeyCode::Char('j'), KeyModifiers::CONTROL) => self.scroll_preview_down(),
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => self.scroll_preview_up(),

            // --- Navigation & View Control ---

            // Toggle Recursive/Flat view on 'e'
            (KeyCode::Char('e'), _) => self.toggle_recursive_view(),

            // Toggle fold/unfold on 't' (only available in recursive view)
            (KeyCode::Char('t'), _) => self.toggle_fold(),

            // Move selection down (j or Down arrow)
            (KeyCode::Char('j'), _) | (KeyCode::Down, _) => self.select_next(),

            // Move selection up (k or Up arrow)
            (KeyCode::Char('k'), _) | (KeyCode::Up, _) => self.select_previous(),

            // Go up directory (h, Backspace, or Left arrow)
            (KeyCode::Char('h'), _) | (KeyCode::Backspace, _) | (KeyCode::Left, _) => {
                self.leave_directory()
            }

            // Enter directory (l, Enter, or Right arrow)
            (KeyCode::Char('l'), _) | (KeyCode::Enter, _) | (KeyCode::Right, _) => {
                self.enter_directory()
            }

            // Ignore all other keys
            _ => {}
        }

        // Default: continue the main loop
        HandlerResult::Continue
    }
}
