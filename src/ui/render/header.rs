use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::ui::app::{App, AppState};

use super::spinner::make_spinner;

const LOGO: &str = include_str!("logo.txt");

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let header = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Length(64), Constraint::Length(30)])
        .flex(Flex::SpaceBetween)
        .split(area);

    let info_header = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(2), Constraint::Length(1)])
        .flex(Flex::Center)
        .split(header[1]);

    let logo = Paragraph::new(LOGO);
    frame.render_widget(logo, header[0]);

    let accent = Style::default().fg(Color::Cyan);
    let text = vec![
        Line::from(vec![
            Span::from("Cleanable space: "),
            Span::styled(format!("{}", app.table.cleanable_space()), accent),
        ]),
        Line::from(vec![Span::from("Selected: "), Span::styled(format!("{}", app.table.selected_space()), accent)]),
    ];

    let logo = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(logo, info_header[0]);

    match app.state {
        AppState::Scanning => make_spinner(app, frame, info_header[1], "Scanning..."),
        AppState::Calculating => make_spinner(app, frame, info_header[1], "Calculating..."),
        AppState::Done => {
            let logo = Paragraph::new("Done!").alignment(Alignment::Center).fg(Color::Green);
            frame.render_widget(logo, info_header[1]);
        },
    };
}
