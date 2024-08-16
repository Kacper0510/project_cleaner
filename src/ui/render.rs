use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, Clear, Padding, Paragraph, Row, Table},
    Frame,
};
use unicode_segmentation::UnicodeSegmentation;

use super::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(5), Constraint::Percentage(100), Constraint::Length(1)])
        .split(frame.size());

    render_header(app, frame, layout[0]);
    render_table(app, frame, layout[1]);
    render_help(app, frame, layout[2]);

    // TODO: Popup
    // render_popup(app, frame, frame.size())
}

fn render_header(app: &mut App, frame: &mut Frame, area: Rect) {
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

    let spinner_box = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Length(13)])
        .flex(Flex::Center)
        .split(info_header[1]);

    let logo = r#"   ___             _         __    _______                     
  / _ \_______    (_)__ ____/ /_  / ___/ /__ ___ ____  ___ ____
 / ___/ __/ _ \  / / -_) __/ __/ / /__/ / -_) _ `/ _ \/ -_) __/
/_/  /_/  \___/_/ /\__/\__/\__/  \___/_/\__/\_,_/_//_/\__/_/   
             |___/                                             "#;

    let logo = Paragraph::new(logo);
    frame.render_widget(logo, header[0]);

    let logo =
        Paragraph::new(format!("Cleanable space: {}GB\nSaved space: {}GB", 1.24, 0.24)).alignment(Alignment::Center);
    frame.render_widget(logo, info_header[0]);

    let spinner = throbber_widgets_tui::Throbber::default()
        .label("Scanning...")
        .throbber_set(throbber_widgets_tui::BRAILLE_SIX_DOUBLE)
        .use_type(throbber_widgets_tui::WhichUse::Spin);

    frame.render_stateful_widget(spinner, spinner_box[0], &mut app.throbber_state);
}

fn render_table(app: &mut App, frame: &mut Frame, area: Rect) {
    let widths = [Constraint::Length(6), Constraint::Percentage(100), Constraint::Length(10), Constraint::Length(10)];
    let table_data = app.table.clone();
    let table = Table::new(table_data.to_rows(), widths)
        .column_spacing(1)
        .header(
            Row::new(vec!["", "Path", "Size", "Rating"])
                .style(Style::default().bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        )
        .block(Block::bordered().border_type(BorderType::Rounded))
        .highlight_style(Style::default().reversed())
        .highlight_symbol(" ");

    frame.render_stateful_widget(table, area, &mut app.table.state);
}

fn render_help(_app: &mut App, frame: &mut Frame, area: Rect) {
    let help = ["Scroll [↑↓]", "Delete [d]", "Exit [q]"].iter().map(|e| format!(" {e} "));

    let constraints = help.clone().map(|e| Constraint::Length(e.graphemes(true).count().try_into().unwrap()));

    let line = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .horizontal_margin(1)
        .spacing(1)
        .flex(Flex::Start)
        .split(area);

    for (i, h) in help.enumerate() {
        let txt = Paragraph::new(h).fg(Color::Gray).bg(Color::DarkGray);
        frame.render_widget(txt, line[i]);
    }
}

fn render_popup(_app: &mut App, frame: &mut Frame, area: Rect) {
    let popup_l1 = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(60)])
        .flex(Flex::Center)
        .split(area);
    let popup_l2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(60)])
        .flex(Flex::Center)
        .split(popup_l1[0]);

    frame.render_widget(Clear, popup_l2[0]);
    let txt = Paragraph::new("Hello!").block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan))
            .padding(Padding::uniform(1)),
    );
    frame.render_widget(txt, popup_l2[0])
}
