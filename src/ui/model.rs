use crate::core::{CommentedLang, DirStats, Lang, MatchData};
use ratatui::widgets::TableState;
use size::Size;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Debug, Clone)]
pub struct MatchGroup {
    pub dangerous: bool,
    pub group_path: PathBuf,
    pub status: MatchDataUIStatus,
    pub matches: Vec<MatchDataUI>,
}

impl MatchGroup {
    pub fn stats(&self) -> DirStats {
        self.matches.iter().map(|ele| ele.dir_stats).sum()
    }

    pub fn path(&self) -> PathBuf {
        self.group_path.clone()
    }

    pub fn size(&self) -> Size {
        self.stats().size.unwrap_or(Size::from_bytes(0))
    }

    pub fn last_mod(&self) -> SystemTime {
        self.stats().last_mod.unwrap_or(SystemTime::UNIX_EPOCH)
    }

    pub fn lang(&self) -> String {
        self.get_icons().iter().map(|ele| ele.name).collect()
    }

    pub fn get_icons(&self) -> Vec<Lang> {
        let icons: HashSet<_> = self.matches.iter().flat_map(|e| &e.lang).map(|e| e.lang.clone()).collect();
        let mut icons: Vec<_> = icons.into_iter().collect();
        icons.sort();
        icons
    }
}

#[derive(Debug, Clone)]
pub struct MatchDataUI {
    pub idx: usize,
    pub path: PathBuf,
    pub dir_stats: DirStats,
    pub lang: Vec<CommentedLang>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchDataUIStatus {
    Found,
    Selected,
}

#[derive(Debug, Clone, Default)]
pub struct TableData {
    pub idx: usize,
    pub state: TableState,
    pub data: Vec<MatchGroup>,
    pub sort_by: Field,
    pub ascending: bool,
    pub selected: Field,
    cleanable_space: Size,
}

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub enum Field {
    Path,
    Lang,
    #[default]
    Size,
    LastMod,
}

impl TableData {
    pub fn add_match(&mut self, data: MatchData) {
        let path = data.group().to_owned();

        let ui_data = MatchDataUI {
            idx: self.idx,
            path: data.path.clone(),
            dir_stats: DirStats::default(),
            lang: data.languages().to_vec(),
        };
        self.idx += 1;

        if let Some(record) = self.data.iter_mut().find(|ele| ele.group_path == path) {
            record.dangerous |= data.dangerous();
            record.matches.push(ui_data);
        } else {
            self.data.push(MatchGroup {
                dangerous: data.dangerous(),
                group_path: path,
                status: MatchDataUIStatus::Found,
                matches: vec![ui_data],
            });
            self.sort();
        }
    }

    pub fn update_match(&mut self, idx: usize, data: DirStats) -> bool {
        if let Some(ele) = self.get_match_by_idx_mut(idx) {
            ele.dir_stats = data;
            if let Some(size) = data.size {
                self.cleanable_space += size;
                return true;
            }
        }
        false
    }

    pub fn sort(&mut self) {
        let path = if let Some(selected) = self.state.selected() {
            Some(self.data[selected].group_path.clone())
        } else {
            self.state.select(Some(0));
            None
        };

        match self.sort_by {
            Field::Path => self.data.sort_by_key(MatchGroup::path),
            Field::Lang => self.data.sort_by_key(MatchGroup::lang),
            Field::Size => self.data.sort_by_key(MatchGroup::size),
            Field::LastMod => self.data.sort_by_key(MatchGroup::last_mod),
        };

        if !self.ascending {
            self.data.reverse();
        }

        if let Some(path) = path {
            if let Some(idx) = self.data.iter().position(|ele| ele.group_path == path) {
                self.state.select(Some(idx))
            }
        }
    }

    pub fn sort_toggle(&mut self) {
        if self.selected == self.sort_by {
            self.ascending = !self.ascending;
        } else {
            self.sort_by = self.selected;
            self.ascending = false;
        }
        self.sort();
    }

    pub fn sort_left(&mut self) {
        self.selected = match self.selected {
            Field::Lang => Field::Size,
            Field::Path => Field::Lang,
            Field::LastMod => Field::Path,
            Field::Size => Field::LastMod,
        };
    }

    pub fn sort_right(&mut self) {
        self.selected = match self.selected {
            Field::Lang => Field::Path,
            Field::Path => Field::LastMod,
            Field::LastMod => Field::Size,
            Field::Size => Field::Lang,
        };
    }

    pub fn get_by_path(&self, path: &Path) -> Option<&MatchGroup> {
        self.data.iter().find(|ele| ele.group_path == path)
    }

    pub fn get_match_by_idx_mut(&mut self, idx: usize) -> Option<&mut MatchDataUI> {
        self.data.iter_mut().flat_map(|ele| ele.matches.iter_mut()).find(|ele| ele.idx == idx)
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

    pub fn get_selected_path(&self) -> Vec<PathBuf> {
        self.data
            .iter()
            .filter(|ele| ele.status == MatchDataUIStatus::Selected)
            .flat_map(|ele| ele.matches.iter().map(|e| e.path.clone()).collect::<Vec<_>>())
            .collect()
    }

    pub fn cleanable_space(&self) -> Size {
        self.cleanable_space
    }

    pub fn selected_space(&self) -> Size {
        self.data
            .iter()
            .filter(|ele| ele.status == MatchDataUIStatus::Selected)
            .flat_map(|ele| ele.matches.iter().filter_map(|e| e.dir_stats.size).collect::<Vec<_>>())
            .fold(Size::from_bytes(0), |prev, current| prev + current)
    }
}
