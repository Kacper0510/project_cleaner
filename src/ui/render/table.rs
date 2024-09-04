use std::collections::HashSet;

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
            let is_selected = selected.and_then(|s| if s == idx { Some(()) } else { None }).is_some();
            let fg: Color = if is_selected { Color::Black } else { Color::White };
            let bg = if is_selected { Color::White } else { Color::Reset };

            let icons: HashSet<_> = ele.matches.iter().flat_map(|e| &e.lang).map(|e| e.lang).collect();
            let mut icons: Vec<&&crate::core::Lang> = icons.iter().collect();
            icons.sort();

            let icons: Vec<_> = icons
                .iter()
                .map(|e| {
                    Span::styled(
                        if no_icons { format!("{} ", e) } else { e.icon.to_owned() },
                        Style::default().fg(if is_selected { e.color.selected() } else { e.color.normal() }),
                    )
                })
                .collect();

            let mut line = match ele.status {
                MatchDataUIStatus::Selected => {
                    vec![Span::styled("[del]", Style::default().fg(Color::Red)), Span::from(" ")]
                },
                MatchDataUIStatus::Found => vec![],
            };
            if ele.matches.len() == 1 {
                line.push(Span::styled(ele.matches[0].path.display().to_string(), fg));
            } else {
                line.push(Span::styled(ele.group_path.display().to_string(), fg));
                line.push(Span::styled(" {...}", Color::DarkGray));
            }

            let warn_color = if is_selected { Color::Yellow } else { Color::LightYellow };
            Row::new(vec![
                Cell::new(Line::from(icons)),
                Cell::new(Line::from(line)),
                Cell::new(Line::from(if ele.dangerous {
                    if no_icons {
                        Span::styled("(!)", warn_color)
                    } else {
                        Span::styled("  ", warn_color)
                    }
                } else {
                    Span::from("")
                })),
                Cell::new(Span::styled(
                    if let Some(s) = &ele.stats().last_mod_days() { format!("{}d", s) } else { "---".to_owned() },
                    fg,
                )),
                Cell::new(Span::styled(
                    if let Some(s) = &ele.stats().size { format!("{}", s) } else { "---".to_owned() },
                    fg,
                )),
            ])
            .bg(bg)
        })
        .collect()
}
