use std::{io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

// Import core modules from the local crate structure
use greenmerald_markdown_renderer::{
    App,
    HandlerResult,
    ui,
};


fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {

    loop {
        // Draw the UI
        terminal.draw(|f| ui::<B>(f, app))?;

        // Handle input events
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                // Call the new dedicated handler method
                match app.handle_key_event(key) {
                    HandlerResult::Quit => return Ok(()),
                    HandlerResult::Continue => {}
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
// ## 2. Main Function ##
// Entry point of the application
fn main() -> Result<(), io::Error> {
    // Setup the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}






