use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Padding, Paragraph},
    Frame,
};

use crate::ui::{
    app::{App, DeletePopUpKind},
    render::spinner::make_spinner,
};

use super::make_popup_layout;

pub fn render(app: &mut App, frame: &mut Frame, area: Rect, kind: DeletePopUpKind) {
    let area = make_popup_layout(frame, area);

    let container = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::uniform(1));

    frame.render_widget(container, area);

    match kind {
        DeletePopUpKind::Confirm => {
            let count = app.table.get_selected_path().len();
            let p = Paragraph::new(vec![Line::from(vec![
                Span::from("Do you want to "),
                Span::styled("permanently", Style::default().underlined().fg(Color::Red)),
                Span::from(format!(" delete {} {}?", count, if count > 1 { "directories" } else { "directoire" })),
            ])
            .alignment(Alignment::Center)
            .style(Style::default().bold().fg(Color::Cyan))]);

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .flex(Flex::Center)
                .constraints(vec![Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)])
                .split(area);
            frame.render_widget(p, layout[0]);

            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::SpaceAround)
                .constraints(vec![Constraint::Length(15), Constraint::Length(15)])
                .split(layout[2]);

            frame.render_widget(
                Paragraph::new("No [N]").alignment(Alignment::Center).fg(Color::Gray).bg(Color::DarkGray),
                layout[0],
            );
            frame.render_widget(
                Paragraph::new("Yes [y]").alignment(Alignment::Center).fg(Color::Gray).bg(Color::DarkGray),
                layout[1],
            );
        },
        DeletePopUpKind::Deleting => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .flex(Flex::Center)
                .constraints(vec![Constraint::Length(1)])
                .split(area);
            make_spinner(app, frame, layout[0], "Deleting...")
        },
    }
}
