use super::{App, AppResult};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        KeyCode::Char('c') | KeyCode::Char('C')
            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
        {
            app.quit();
        }

        // Counter handlers
        KeyCode::Up => {
            app.list_up();
        }
        KeyCode::Down => {
            app.list_down();
        }

        _ => {}
    }
    Ok(())
}
