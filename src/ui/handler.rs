use super::{
    app::{App, AppResult, AppState},
    popup::{DeletePopUpKind, PopUpKind, PopUpState},
};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if app.popup_state == PopUpState::Open(PopUpKind::Delete(DeletePopUpKind::Deleting)) {
        return Ok(());
    }

    match &app.popup_state {
        PopUpState::Open(kind) => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('i') => app.hide_info(),

            KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                app.quit()
            },

            KeyCode::Char(' ') | KeyCode::Enter | KeyCode::Char('n') => match kind {
                PopUpKind::Info => {},
                PopUpKind::Delete(_) | PopUpKind::Exit => app.hide_info(),
            },
            KeyCode::Char('y') => match kind {
                PopUpKind::Info => {},
                PopUpKind::Delete(_) => app.confirm_delete(),
                PopUpKind::Exit => app.force_quit(),
            },

            KeyCode::Up if *kind == PopUpKind::Info => app.scroll_up(),
            KeyCode::Down if *kind == PopUpKind::Info => app.scroll_down(),

            _ => {},
        },
        PopUpState::Closed => {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => app.quit(),

                KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.quit()
                },

                // Table handlers
                KeyCode::Up => app.list_up(),
                KeyCode::Down => app.list_down(),

                // Reload
                KeyCode::Char('r') => {
                    if app.state == AppState::Done {
                        app.reload()
                    }
                },

                // Delete
                KeyCode::Char('d') => app.delete(),
                // Info
                KeyCode::Char('i') => app.show_info(),
                // Select
                KeyCode::Char(' ') => app.toggle_select(),

                _ => {},
            }
        },
    }

    Ok(())
}

pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    match app.popup_state {
        PopUpState::Closed => match mouse_event.kind {
            MouseEventKind::ScrollDown => app.list_down(),
            MouseEventKind::ScrollUp => app.list_up(),

            _ => {},
        },
        PopUpState::Open(PopUpKind::Info) => match mouse_event.kind {
            MouseEventKind::ScrollDown => app.scroll_down(),
            MouseEventKind::ScrollUp => app.scroll_up(),

            _ => {},
        },
        _ => {},
    }
    Ok(())
}
