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
    let widths = [
        Constraint::Length(6),
        Constraint::Percentage(100),
        Constraint::Length(3),
        Constraint::Length(8),
        Constraint::Length(8),
    ];
    let table_data = app.table.clone();
    let table = Table::new(table_data_to_rows(&table_data, app.args.no_icons, app.table.state.selected()), widths)
        .column_spacing(1)
        .header(
            Row::new(vec!["", "Path", "", "LastMod", "Size"])
                .style(Style::default().bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        )
        .block(Block::bordered().border_type(BorderType::Rounded));

    frame.render_stateful_widget(table, area, &mut app.table.state);
}

fn table_data_to_rows(data: &TableData, no_icons: bool, selected: Option<usize>) -> Vec<Row> {
    data.data
        .iter()
        .enumerate()
        .map(|(idx, ele)| {
            let icons: Vec<_> = ele
                .languages
                .iter()
                .map(|e| {
                    Span::styled(
                        format!("{} ", if no_icons { e.lang.short } else { e.lang.icon }),
                        Color::from(e.lang.color),
                    )
                })
                .collect();

            let is_selected = selected.and_then(|s| if s == idx { Some(()) } else { None }).is_some();
            let fg = if is_selected { Color::Black } else { Color::White };
            let line = match ele.status {
                MatchDataUIStatus::Selected => {
                    vec![
                        Span::styled("[del]", Style::default().fg(Color::Red)),
                        Span::from(" "),
                        Span::styled(ele.group_path.display().to_string(), fg),
                    ]
                },
                MatchDataUIStatus::Found => vec![Span::styled(ele.group_path.display().to_string(), fg)],
            };

            Row::new(vec![
                Cell::new(Line::from(icons)),
                Cell::new(Line::from(line)).bg(if is_selected { Color::White } else { Color::Reset }),
                Cell::new(Line::from(if ele.hidden {
                    Span::styled(if no_icons { "(!)" } else { "  " }, Color::LightYellow)
                } else {
                    Span::from("")
                })),
                Cell::new(Span::from(if let Some(s) = &ele.stats().last_mod_days() {
                    format!("{}d", s)
                } else {
                    "---".to_owned()
                })),
                Cell::new(Span::from(if let Some(s) = &ele.stats().size {
                    format!("{}", s)
                } else {
                    "---".to_owned()
                })),
            ])
        })
        .collect()
}
