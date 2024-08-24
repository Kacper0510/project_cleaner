use ratatui::{
    layout::{self, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Padding, Paragraph, Row, Table},
    Frame,
};

use crate::ui::{app::App, model::MatchGroup};

use super::make_popup_layout;

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) -> Option<()> {
    let path = app.info_path.clone()?;
    let match_data = app.table.get_by_path(&path)?;

    let area = make_popup_layout(frame, area);
    let layout = Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
        .margin(2)
        .split(area);
    let small_style = Style::default().fg(Color::DarkGray);

    let mut text = vec![
        Line::from(vec![Span::styled(path.to_str().unwrap(), Style::default().bold().fg(Color::Cyan))]),
        Line::from(vec![]),
        Line::from(vec![Span::styled("Languages: ", Style::default().bold())]),
    ];
    let mut other: Vec<Line> = match_data
        .languages
        .iter()
        .flat_map(|ele| {
            let mut res = vec![Line::from(vec![Span::styled(
                if app.args.no_icons {
                    format!("- {}", ele.lang.name)
                } else {
                    format!("- {} {}", ele.lang.icon, ele.lang.name)
                },
                Color::Indexed(ele.lang.color_index),
            )])];
            for comment in &ele.comments {
                res.push(Line::from(vec![Span::styled(format!("  {}", comment), small_style)]))
            }
            res
        })
        .collect();
    text.append(&mut other);

    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan))
            .padding(Padding::uniform(1)),
        area,
    );

    let container = Paragraph::new(text);
    frame.render_widget(container, layout[0]);

    let widths = [Constraint::Percentage(100), Constraint::Length(10), Constraint::Length(10)];
    let table = Table::new(match_data_to_rows(match_data), widths)
        .column_spacing(1)
        .header(Row::new(vec!["Path", "LastMod", "Size"]).style(Style::default().add_modifier(Modifier::BOLD)));
    frame.render_widget(table, layout[1]);

    Some(())
}

fn match_data_to_rows(data: &MatchGroup) -> Vec<Row> {
    data.matches
        .iter()
        .map(|ele| {
            Row::new(vec![
                ele.path.display().to_string(),
                if let Some(s) = &ele.dir_stats.last_mod_days() { format!("{}d", s) } else { "---".to_owned() },
                if let Some(s) = &ele.dir_stats.size { format!("{}", s) } else { "---".to_owned() },
            ])
        })
        .collect::<Vec<_>>()
}
