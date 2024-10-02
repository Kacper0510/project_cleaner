use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Cell, Row, Table},
    Frame,
};

use crate::ui::{
    app::App,
    model::{Field, MatchDataUIStatus, TableData},
    popup::{PopUpKind, PopUpState},
};

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let widths = [
        Constraint::Length(6),
        Constraint::Percentage(100),
        Constraint::Length(3),
        Constraint::Length(9),
        Constraint::Length(8),
    ];
    let table_data = app.table.clone();

    let sort_focused = app.popup_state == PopUpState::Open(PopUpKind::Sort);

    let table = Table::new(
        table_data_to_rows(&table_data, app.args.no_icons, app.table.state.selected(), sort_focused),
        widths,
    )
    .column_spacing(1)
    .header(make_header(&table_data, sort_focused))
    .block(Block::bordered().border_type(BorderType::Rounded));

    frame.render_stateful_widget(table, area, &mut app.table.state);
}

fn make_header(table: &TableData, sort_focused: bool) -> Row {
    let mut header_text =
        vec!["Lang".to_owned(), "Path".to_owned(), "".to_owned(), "LastMod".to_owned(), "Size".to_owned()];

    match table.sort_by {
        Field::Lang => header_text[0].push_str(if table.ascending { " ▲" } else { " ▼" }),
        Field::Path => header_text[1].push_str(if table.ascending { " ▲" } else { " ▼" }),
        Field::Size => header_text[4].push_str(if table.ascending { " ▲" } else { " ▼" }),
        Field::LastMod => header_text[3].push_str(if table.ascending { " ▲" } else { " ▼" }),
    }

    let selected_index = match table.selected {
        Field::Lang => 0,
        Field::Path => 1,
        Field::Size => 4,
        Field::LastMod => 3,
    };

    let header: Vec<_> = header_text
        .into_iter()
        .enumerate()
        .map(|(i, ele)| {
            Span::styled(
                ele,
                if selected_index == i && sort_focused {
                    Style::default().fg(Color::Cyan).bg(Color::White)
                } else {
                    Style::default()
                },
            )
        })
        .map(Cell::from)
        .collect();

    Row::new(header).style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Cyan))
}

fn table_data_to_rows(data: &TableData, no_icons: bool, selected: Option<usize>, sort_focused: bool) -> Vec<Row> {
    data.data
        .iter()
        .enumerate()
        .map(|(idx, ele)| {
            let icons: Vec<_> = ele
                .get_icons()
                .iter()
                .map(|e| Span::styled(format!("{} ", if no_icons { e.short } else { e.icon }), Color::from(e.color)))
                .collect();

            let is_selected = !sort_focused && selected.and_then(|s| if s == idx { Some(()) } else { None }).is_some();
            let fg = if is_selected { Color::Black } else { Color::White };
            let mut line: Vec<Span<'_>> = match ele.status {
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

            Row::new(vec![
                Cell::new(Line::from(icons)),
                Cell::new(Line::from(line)).bg(if is_selected { Color::White } else { Color::Reset }),
                Cell::new(Line::from(if ele.dangerous {
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
