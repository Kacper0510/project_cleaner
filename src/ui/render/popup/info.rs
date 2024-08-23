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
    let path = app.info_path.clone()?;
    let match_data = app.table.get_by_path(&path)?;

    let area = make_popup_layout(frame, area);
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
            let mut res = vec![Line::from(vec![Span::from(if app.args.no_icons {
                format!("- {}", ele.lang.name)
            } else {
                format!("- {} {}", ele.lang.icon, ele.lang.name)
            })])];
            for comment in &ele.comments {
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
            Span::styled(match_data.stats().size.map(|s| format!("{}", s)).unwrap_or("---".to_owned()), small_style),
        ]),
        Line::from(vec![
            Span::from("Last modification: "),
            Span::styled(
                match_data.stats().last_mod_days().map(|s| format!("{}d", s)).unwrap_or("---".to_owned()),
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
