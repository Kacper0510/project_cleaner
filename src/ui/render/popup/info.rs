use std::path::PathBuf;

use crate::ui::{app::App, model::MatchGroup};
use ratatui::{
    layout::{self, Constraint, Layout, Rect, Size},
    prelude::StatefulWidget,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Padding, Paragraph, Row, Table},
    Frame,
};
use tui_scrollview::{ScrollView, ScrollViewState};

use super::make_popup_layout;

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) -> Option<()> {
    let area = make_popup_layout(frame, area);
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan))
            .padding(Padding::uniform(1)),
        area,
    );

    let layout = Layout::default().margin(2).constraints(vec![Constraint::Fill(1)]).split(area);
    let path = app.info_path.clone()?;
    let match_data = app.table.get_by_path(&path)?.clone();
    let popup = InfoPopup::new(path, match_data, app.args.no_icons);

    frame.render_stateful_widget(popup, layout[0], &mut app.scroll_state);
    Some(())
}

struct InfoPopup {
    path: PathBuf,
    match_data: MatchGroup,
    no_icons: bool,
}

impl InfoPopup {
    pub fn new(path: PathBuf, match_data: MatchGroup, no_icons: bool) -> Self {
        InfoPopup {
            path,
            match_data,
            no_icons,
        }
    }

    fn match_data_to_rows(&self) -> Vec<Row<'static>> {
        self.match_data
            .matches
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
}

impl StatefulWidget for InfoPopup {
    type State = ScrollViewState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State)
    where Self: Sized {
        let small_style = Style::default().fg(Color::DarkGray);

        let mut text = vec![
            Line::from(vec![Span::styled(self.path.to_str().unwrap(), Style::default().bold().fg(Color::Cyan))]),
            Line::from(vec![]),
            Line::from(vec![Span::styled("Languages: ", Style::default().bold())]),
        ];
        let mut other: Vec<Line> = self
            .match_data
            .languages
            .iter()
            .flat_map(|ele| {
                let mut res = vec![Line::from(vec![Span::styled(
                    if self.no_icons {
                        format!("- {}", ele.lang.name)
                    } else {
                        format!("- {} {}", ele.lang.icon, ele.lang.name)
                    },
                    Style::default().fg(ele.lang.color.into()),
                )])];
                for comment in &ele.comments {
                    res.push(Line::from(vec![Span::styled(format!("  {}", comment), small_style)]));
                }
                res
            })
            .collect();
        text.append(&mut other);

        let table_rows = self.match_data_to_rows();

        let text_h: u16 = (text.len() + 1).try_into().unwrap();
        let table_h: u16 = (table_rows.len() + 1).try_into().unwrap();
        let wight = if area.width > 0 { area.width - 1 } else { 0 };
        let text_area = Rect::new(0, 0, wight, text_h);
        let table_area = Rect::new(0, text_h, wight, table_h);
        let mut scrollview = ScrollView::new(Size::new(wight, text_h + table_h));

        scrollview.render_widget(Paragraph::new(text), text_area);
        if self.match_data.hidden {
            scrollview.render_widget(
                Paragraph::new(if self.no_icons {
                    Span::styled("(!)  ", Color::LightYellow)
                } else {
                    Span::styled("    ", Color::LightYellow)
                })
                .alignment(layout::Alignment::Right),
                text_area,
            );
        }

        let widths = [Constraint::Percentage(100), Constraint::Length(10), Constraint::Length(10)];
        let table = Table::new(table_rows, widths)
            .column_spacing(1)
            .header(Row::new(vec!["Path", "LastMod", "Size"]).style(Style::default().add_modifier(Modifier::BOLD)));
        scrollview.render_widget(table, table_area);
        scrollview.render(area, buf, state);
    }
}
