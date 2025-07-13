use std::io::{self, stdout, Stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal for TUI mode
///
/// This function switches to alternate screen and enables raw mode
pub fn init() -> io::Result<Tui> {
    log::info!("Initializing terminal for TUI mode");
    execute!(stdout(), EnterAlternateScreen).map_err(|e| {
        log::error!("Failed to enter alternate screen: {}", e);
        e
    })?;
    enable_raw_mode().map_err(|e| {
        log::error!("Failed to enable raw mode: {}", e);
        e
    })?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout())).map_err(|e| {
        log::error!("Failed to create terminal: {}", e);
        e
    })?;
    log::info!("Terminal initialized successfully");
    Ok(terminal)
}

/// Restore the terminal to its original state
///
/// This function exits alternate screen and disables raw mode
pub fn restore() -> io::Result<()> {
    log::info!("Restoring terminal to original state");
    execute!(stdout(), LeaveAlternateScreen).map_err(|e| {
        log::error!("Failed to leave alternate screen: {}", e);
        e
    })?;
    disable_raw_mode().map_err(|e| {
        log::error!("Failed to disable raw mode: {}", e);
        e
    })?;
    log::info!("Terminal restored successfully");
    Ok(())
}
