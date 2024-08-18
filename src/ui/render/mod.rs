use super::{
    app::App,
    popup::{PopUpKind, PopUpState},
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

mod header;
mod help;
mod popup;

mod spinner;
mod table;

pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(5), Constraint::Percentage(100), Constraint::Length(1)])
        .split(frame.size());

    header::render(app, frame, layout[0]);
    table::render(app, frame, layout[1]);
    help::render(app, frame, layout[2]);

    match app.popup_state.clone() {
        PopUpState::Open(kind) => match kind {
            PopUpKind::Info => {
                popup::info::render(app, frame, frame.size());
            },
            PopUpKind::Delete(kind) => {
                popup::del::render(app, frame, frame.size(), kind);
            },
            PopUpKind::Exit => {
                popup::exit::render(app, frame, frame.size());
            },
        },
        PopUpState::Closed => {},
    }
}
