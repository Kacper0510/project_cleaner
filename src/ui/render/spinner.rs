use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    Frame,
};

use crate::ui::app::App;

pub fn make_spinner(app: &mut App, frame: &mut Frame, area: Rect, name: &str) {
    let spinner_box = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Length((name.len() + 2).try_into().unwrap())])
        .flex(Flex::Center)
        .split(area);

    let spinner = throbber_widgets_tui::Throbber::default()
        .style(Style::default().fg(Color::Cyan))
        .label(name)
        .throbber_set(throbber_widgets_tui::BRAILLE_SIX_DOUBLE)
        .use_type(throbber_widgets_tui::WhichUse::Spin);

    frame.render_stateful_widget(spinner, spinner_box[0], &mut app.throbber_state);
}
