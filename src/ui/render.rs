use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListDirection, ListState, Paragraph},
    Frame,
};
use throbber_widgets_tui::{Throbber, ASCII};

use super::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Percentage(100)])
        .split(frame.size());

    let spinner = throbber_widgets_tui::Throbber::default()
        .label("Running...")
        .throbber_set(throbber_widgets_tui::BRAILLE_SIX_DOUBLE)
        .use_type(throbber_widgets_tui::WhichUse::Spin);
    frame.render_stateful_widget(spinner, layout[0], &mut app.throbber_state);

    let items: Vec<String> = (0..100).map(|e| format!("Item number {e}")).collect();
    let list = List::new(items)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title_alignment(Alignment::Center)
                .title(" Folders "),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, layout[1], &mut app.list_state);
}
