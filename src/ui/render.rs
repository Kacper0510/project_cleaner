use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Padding, Paragraph, Row, Table},
    Frame,
};
use unicode_segmentation::UnicodeSegmentation;

use super::app::{App, AppState, DeletePopUpKind, PopUpKind, PopUpState};

const LOGO: &str = r#"   ___             _         __    _______                     
  / _ \_______    (_)__ ____/ /_  / ___/ /__ ___ ____  ___ ____
 / ___/ __/ _ \  / / -_) __/ __/ / /__/ / -_) _ `/ _ \/ -_) __/
/_/  /_/  \___/_/ /\__/\__/\__/  \___/_/\__/\_,_/_//_/\__/_/   
           |___/                                             "#;

pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(5), Constraint::Percentage(100), Constraint::Length(1)])
        .split(frame.size());

    render_header(app, frame, layout[0]);
    render_table(app, frame, layout[1]);
    render_help(app, frame, layout[2]);

    match app.popup_state.clone() {
        PopUpState::Open(kind) => match kind {
            PopUpKind::Info => {
                render_info_popup(app, frame, frame.size());
            },
            PopUpKind::Delete(kind) => {
                render_del_popup(app, frame, frame.size(), kind);
            },
            PopUpKind::Exit => {
                render_exit_popup(app, frame, frame.size());
            },
        },
        PopUpState::Closed => {},
    }
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

    let logo = Paragraph::new(LOGO);
    frame.render_widget(logo, header[0]);

    let accent = Style::default().fg(Color::Cyan);
    let text = vec![
        Line::from(vec![
            Span::from("Cleanable space: "),
            Span::styled(format!("{}", app.table.cleanable_space()), accent),
        ]),
        Line::from(vec![Span::from("Selected: "), Span::styled(format!("{}", app.table.selected_space()), accent)]),
    ];

    let logo = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(logo, info_header[0]);

    match app.state {
        AppState::Scanning => make_spinner(app, frame, info_header[1], "Scanning..."),
        AppState::Calculating => make_spinner(app, frame, info_header[1], "Calculating..."),
        AppState::Done => {
            let logo = Paragraph::new("Done!").alignment(Alignment::Center).fg(Color::Green);
            frame.render_widget(logo, info_header[1]);
        },
    };
}

fn make_spinner(app: &mut App, frame: &mut Frame, area: Rect, name: &str) {
    let spinner_box = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Length((name.len() + 2).try_into().unwrap())])
        .flex(Flex::Center)
        .split(area);

    let spinner = throbber_widgets_tui::Throbber::default()
        .style(Style::default().fg(Color::Cyan))
        .label(name)
        .throbber_set(throbber_widgets_tui::BRAILLE_SIX_DOUBLE)
        .use_type(throbber_widgets_tui::WhichUse::Spin);

    frame.render_stateful_widget(spinner, spinner_box[0], &mut app.throbber_state);
}

fn render_table(app: &mut App, frame: &mut Frame, area: Rect) {
    let widths = [Constraint::Length(6), Constraint::Percentage(100), Constraint::Length(10), Constraint::Length(10)];
    let table_data = app.table.clone();
    let table = Table::new(table_data.to_rows(app.args.no_icons), widths)
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

fn render_help(app: &mut App, frame: &mut Frame, area: Rect) {
    let help = construct_help(app);

    let constraints =
        help.clone().into_iter().map(|e| Constraint::Length(e.graphemes(true).count().try_into().unwrap()));

    let line = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .horizontal_margin(1)
        .spacing(1)
        .flex(Flex::Start)
        .split(area);

    for (i, h) in help.into_iter().enumerate() {
        let txt = Paragraph::new(h).fg(Color::Gray).bg(Color::DarkGray);
        frame.render_widget(txt, line[i]);
    }
}

fn make_popup_layout(frame: &mut Frame, area: Rect) -> Rect {
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
    popup_l2[0]
}

fn render_info_popup(app: &mut App, frame: &mut Frame, area: Rect) -> Option<()> {
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
        Line::from(vec![Span::styled("Reasons: ", Style::default().bold())]),
    ];
    let mut other: Vec<Line> = match_data
        .data
        .reasons
        .iter()
        .flat_map(|ele| {
            let mut res = vec![Line::from(vec![Span::from(if app.args.no_icons {
                format!("- {}", ele.name)
            } else {
                format!("- {} {}", ele.icon, ele.name)
            })])];
            if let Some(comment) = &ele.comment {
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

fn render_del_popup(app: &mut App, frame: &mut Frame, area: Rect, kind: DeletePopUpKind) {
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

fn render_exit_popup(app: &mut App, frame: &mut Frame, area: Rect) {
    let area = make_popup_layout(frame, area);

    let container = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::uniform(1));

    frame.render_widget(container, area);

    let count = app.table.get_selected_path().len();
    let p = Paragraph::new(vec![
        Line::from(vec![Span::from("Do you want to exit?")])
            .alignment(Alignment::Center)
            .style(Style::default().bold().fg(Color::Cyan)),
        Line::from(vec![
            Span::from(format!(
                "Selected delete {} {} will ",
                count,
                if count > 1 { "directories" } else { "directoire" }
            )),
            Span::styled("not", Style::default().underlined().fg(Color::Red)),
            Span::from(" be deleted."),
        ])
        .alignment(Alignment::Center),
    ]);
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .flex(Flex::Center)
        .constraints(vec![Constraint::Length(2), Constraint::Length(1), Constraint::Length(1)])
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
}

fn construct_help(app: &App) -> Vec<String> {
    let mut res = vec![];

    match app.popup_state {
        super::app::PopUpState::Open(_) => {
            res.push((10, "Close [q]"));
        },
        super::app::PopUpState::Closed => {
            res.push((0, "Scroll [↑↓]"));
            res.push((10, "Exit [q]"));

            if app.state == AppState::Done {
                res.push((9, "Reload [r]"))
            }

            if app.is_highlighted() {
                res.push((1, "Info [i]"));
                if app.table.is_selected() {
                    res.push((2, "Unselect [˽]"));
                } else {
                    res.push((2, "Select [˽]"));
                }
            }

            if app.table.is_any_selected() {
                res.push((5, "Delete [d]"))
            }
        },
    };

    res.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    res.iter().map(|e| format!(" {} ", e.1)).collect()
}
