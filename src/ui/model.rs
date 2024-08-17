use ratatui::{
    style::{Color, Stylize},
    widgets::{Cell, Row, TableState},
};

use crate::core::{dir_stats::DirStats, MatchData};

#[derive(Debug, Clone, Default)]
pub struct TableData {
    pub state: TableState,
    pub data: Vec<MatchDataUI>,
}

#[derive(Debug, Clone)]
pub struct MatchDataUI {
    pub idx: usize,
    pub data: MatchData,
    pub dir_stats: DirStats,
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
                    Cell::new(if let Some(s) = &ele.dir_stats.last_mod_days() {
                        format!("{}d", s)
                    } else {
                        "---".to_owned()
                    }),
                    Cell::new(if let Some(s) = &ele.dir_stats.size { format!("{}", s) } else { "---".to_owned() }),
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
        let idx = self.data.len();
        self.data.push(MatchDataUI {
            idx,
            data,
            dir_stats: DirStats::default(),
            status: MatchDataUIStatus::Found,
        });
        self.resort();
    }

    pub fn resort(&mut self) {
        let idx = if let Some(selected) = self.state.selected() {
            let path = self.data[selected].idx;
            Some(path)
        } else {
            self.state.select(Some(0));
            None
        };

        self.data.sort_by(|a, b| b.dir_stats.size.partial_cmp(&a.dir_stats.size).unwrap());

        if let Some(idx) = idx {
            if let Some(idx) = self.data.iter().position(|ele| ele.idx == idx) {
                self.state.select(Some(idx))
            }
        }
    }

    pub fn get_by_idx(&self, idx: usize) -> Option<&MatchDataUI> {
        let idx = self.data.iter().position(|ele| ele.idx == idx);
        idx.map(|idx| &self.data[idx])
    }
}
