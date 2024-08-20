use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    widgets::Clear,
    Frame,
};

pub mod del;
pub mod exit;
pub mod info;

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
