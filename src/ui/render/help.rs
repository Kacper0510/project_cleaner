use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Stylize},
    widgets::Paragraph,
    Frame,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::ui::app::{App, AppState, PopUpState};

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let help = construct_help(app);

    let constraints =
        help.clone().into_iter().map(|e| Constraint::Length(e.graphemes(true).count().try_into().unwrap()));

    let line = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .horizontal_margin(1)
        .spacing(1)
        .flex(Flex::Start)
        .split(area);

    for (i, h) in help.into_iter().enumerate() {
        let txt = Paragraph::new(h).fg(Color::Gray).bg(Color::DarkGray);
        frame.render_widget(txt, line[i]);
    }
}

fn construct_help(app: &App) -> Vec<String> {
    let mut res = vec![];

    match app.popup_state {
        PopUpState::Open(_) => {
            res.push((10, "Close [q]"));
        },
        PopUpState::Closed => {
            res.push((0, "Scroll [↑↓]"));
            res.push((10, "Exit [q]"));

            if app.state == AppState::Done {
                res.push((9, "Reload [r]"))
            }

            if app.is_highlighted() {
                res.push((1, "Info [i]"));
                if app.table.is_selected() {
                    res.push((2, "Unselect [˽]"));
                } else {
                    res.push((2, "Select [˽]"));
                }
            }

            if app.table.is_any_selected() {
                res.push((5, "Delete [d]"))
            }
        },
    };

    res.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    res.iter().map(|e| format!(" {} ", e.1)).collect()
}
