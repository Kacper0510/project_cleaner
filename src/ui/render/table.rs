use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Cell, Row, Table},
    Frame,
};

use crate::ui::{
    app::App,
    model::{MatchDataUIStatus, TableData},
};

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let widths = [Constraint::Length(6), Constraint::Percentage(100), Constraint::Length(10), Constraint::Length(10)];
    let table_data = app.table.clone();
    let table = Table::new(table_data_to_rows(&table_data, app.args.no_icons), widths)
        .column_spacing(1)
        .header(
            Row::new(vec!["", "Path", "LastMod", "Size"])
                .style(Style::default().bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        )
        .block(Block::bordered().border_type(BorderType::Rounded))
        .highlight_style(Style::default().reversed())
        .highlight_symbol(" ");

    frame.render_stateful_widget(table, area, &mut app.table.state);
}

fn table_data_to_rows(data: &TableData, no_icons: bool) -> Vec<Row> {
    data.data
        .iter()
        .map(|ele| {
            let icons = ele
                .data
                .reasons
                .iter()
                .map(|e| if no_icons { e.short.to_owned() } else { e.icon.to_owned() })
                .collect::<Vec<String>>()
                .join(" ");

            let line = match ele.status {
                MatchDataUIStatus::Selected => {
                    vec![
                        Span::styled("[del]", Style::default().fg(Color::Red)),
                        Span::from(" "),
                        Span::from(ele.data.path.display().to_string()),
                    ]
                },
                MatchDataUIStatus::Found => vec![Span::from(ele.data.path.display().to_string())],
            };

            Row::new(vec![
                Cell::new(icons),
                Cell::new(Line::from(line)),
                Cell::new(if let Some(s) = &ele.dir_stats.last_mod_days() {
                    format!("{}d", s)
                } else {
                    "---".to_owned()
                }),
                Cell::new(if let Some(s) = &ele.dir_stats.size { format!("{}", s) } else { "---".to_owned() }),
            ])
        })
        .collect()
}
