use super::app::{App, AppResult, AppState};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        },
        KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            app.quit();
        },

        // Table handlers
        KeyCode::Up => {
            app.list_up();
        },
        KeyCode::Down => {
            app.list_down();
        },

        // Reload
        KeyCode::Char('r') => {
            if app.state == AppState::Done {
                app.reload();
            }
        },

        // Delete
        KeyCode::Char('d') => {
            app.delete();
        },

        _ => {},
    }
    Ok(())
}

pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    match mouse_event.kind {
        MouseEventKind::ScrollDown => {
            app.list_down();
        },

        MouseEventKind::ScrollUp => {
            app.list_up();
        },

        _ => {},
    }

    Ok(())
}
