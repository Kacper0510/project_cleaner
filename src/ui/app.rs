use std::error;

use ratatui::widgets::ListState;
use throbber_widgets_tui::ThrobberState;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub list_state: ListState,
    pub throbber_state: ThrobberState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            list_state: ListState::default().with_selected(Some(5)),
            throbber_state: ThrobberState::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.throbber_state.calc_next();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn list_up(&mut self) {
        self.list_state.select_previous();
    }

    pub fn list_down(&mut self) {
        self.list_state.select_next();
    }
}
