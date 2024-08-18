use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Cell, Row, TableState},
};
use size::Size;

use crate::core::{dir_stats::DirStats, MatchData};

#[derive(Debug, Clone)]
pub struct TableData {
    pub state: TableState,
    pub data: Vec<MatchDataUI>,
    cleanable_space: Size,
}

impl Default for TableData {
    fn default() -> Self {
        Self {
            state: Default::default(),
            data: Default::default(),
            cleanable_space: Size::from_bytes(0),
        }
    }
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
    Selected,
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

                let line = match ele.status {
                    MatchDataUIStatus::Selected => {
                        vec![
                            Span::styled("[del]", Style::default().fg(Color::Red)),
                            Span::from(" "),
                            Span::from(ele.data.path.display().to_string()),
                        ]
                    },
                    MatchDataUIStatus::Found => vec![Span::from(ele.data.path.display().to_string())],
                };

                Row::new(vec![
                    Cell::new(icons),
                    Cell::new(Line::from(line)),
                    Cell::new(if let Some(s) = &ele.dir_stats.last_mod_days() {
                        format!("{}d", s)
                    } else {
                        "---".to_owned()
                    }),
                    Cell::new(if let Some(s) = &ele.dir_stats.size { format!("{}", s) } else { "---".to_owned() }),
                ])
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

    pub fn update_match(&mut self, idx: usize, data: DirStats) -> bool {
        if let Some(ele) = self.get_by_idx_mut(idx) {
            ele.dir_stats = data;
            if let Some(size) = data.size {
                self.cleanable_space += size;
                return true;
            }
        }
        false
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

    pub fn get_by_idx_mut(&mut self, idx: usize) -> Option<&mut MatchDataUI> {
        let idx = self.data.iter().position(|ele| ele.idx == idx);
        idx.map(|idx| &mut self.data[idx])
    }

    pub fn toggle_select(&mut self) {
        if let Some(selected) = self.state.selected() {
            let ele = &self.data[selected];
            self.data[selected].status = match ele.status {
                MatchDataUIStatus::Found => MatchDataUIStatus::Selected,
                MatchDataUIStatus::Selected => MatchDataUIStatus::Found,
            }
        }
    }

    pub fn is_selected(&self) -> bool {
        if let Some(selected) = self.state.selected() {
            self.data[selected].status == MatchDataUIStatus::Selected
        } else {
            false
        }
    }

    pub fn is_any_selected(&self) -> bool {
        self.data.iter().any(|ele| ele.status == MatchDataUIStatus::Selected)
    }

    pub fn cleanable_space(&self) -> Size {
        self.cleanable_space
    }

    pub fn selected_space(&self) -> Size {
        self.data
            .iter()
            .filter(|ele| ele.status == MatchDataUIStatus::Selected)
            .filter_map(|ele| ele.dir_stats.size)
            .fold(Size::from_bytes(0), |prev, current| prev + current)
    }
}
