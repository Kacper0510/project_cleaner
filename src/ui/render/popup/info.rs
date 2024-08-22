use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Padding, Paragraph},
    Frame,
};

use crate::ui::app::App;

use super::make_popup_layout;

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) -> Option<()> {
    let match_data_idx = app.info_index?;
    let match_data = app.table.get_by_idx(match_data_idx)?;

    let area = make_popup_layout(frame, area);
    let small_style = Style::default().fg(Color::DarkGray);

    let mut text = vec![
        Line::from(vec![Span::styled(
            match_data.data.path.to_str().unwrap_or("---").to_string(),
            Style::default().bold().fg(Color::Cyan),
        )]),
        Line::from(vec![]),
        Line::from(vec![Span::styled("Languages: ", Style::default().bold())]),
    ];
    let mut other: Vec<Line> = match_data
        .data
        .languages()
        .iter()
        .flat_map(|ele| {
            let mut res = vec![Line::from(vec![Span::from(if app.args.no_icons {
                format!("- {}", ele.name())
            } else {
                format!("- {} {}", ele.icon(), ele.name())
            })])];
            if let Some(comment) = &ele.comment() {
                res.push(Line::from(vec![Span::styled(format!("  {}", comment), small_style)]))
            }
            res
        })
        .collect();
    text.append(&mut other);
    text.append(&mut vec![
        Line::from(vec![]),
        Line::from(vec![Span::styled("Stats: ", Style::default().bold())]),
        Line::from(vec![
            Span::from("Size: "),
            Span::styled(match_data.dir_stats.size.map(|s| format!("{}", s)).unwrap_or("---".to_owned()), small_style),
        ]),
        Line::from(vec![
            Span::from("Last modification: "),
            Span::styled(
                match_data.dir_stats.last_mod_days().map(|s| format!("{}d", s)).unwrap_or("---".to_owned()),
                small_style,
            ),
        ]),
    ]);

    let container = Paragraph::new(text).block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan))
            .padding(Padding::uniform(1)),
    );
    frame.render_widget(container, area);
    Some(())
}
