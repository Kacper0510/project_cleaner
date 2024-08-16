use ratatui::{
    style::{Color, Stylize},
    widgets::{Cell, Row, TableState},
};
use size::Size;

use crate::core::MatchData;

#[derive(Debug, Clone, Default)]
pub struct TableData {
    pub state: TableState,
    pub data: Vec<MatchDataUI>,
}

#[derive(Debug, Clone)]
pub struct MatchDataUI {
    pub data: MatchData,
    pub size: Option<Size>,
    status: MatchDataUIStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum MatchDataUIStatus {
    Found,
    Deleted,
}

impl TableData {
    pub fn to_rows(&self, no_icons: bool) -> Vec<Row> {
        self.data
            .iter()
            .map(|ele| {
                let icons = ele
                    .data
                    .reasons
                    .iter()
                    .map(|e| if no_icons { e.short.to_owned() } else { e.icon.to_owned() })
                    .collect::<Vec<String>>()
                    .join(" ");

                let row = Row::new(vec![
                    Cell::new(icons),
                    Cell::new(ele.data.path.display().to_string()),
                    Cell::new(if let Some(s) = ele.size { format!("{}", s) } else { "---".to_owned() }),
                    Cell::new("lol"),
                ]);
                if ele.status == MatchDataUIStatus::Deleted {
                    row.bg(Color::Red)
                } else {
                    row
                }
            })
            .collect()
    }

    pub fn add_match(&mut self, data: MatchData) {
        if self.state.selected().is_none() {
            self.state.select(Some(0))
        }

        self.data.push(MatchDataUI {
            data,
            size: None,
            status: MatchDataUIStatus::Found,
        });
    }
}
