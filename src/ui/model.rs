use crate::core::{CommentedLang, DirStats, Lang, MatchData};
use ratatui::widgets::TableState;
use size::Size;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct MatchGroup {
    pub hidden: bool,
    pub group_path: PathBuf,
    pub status: MatchDataUIStatus,
    pub languages: Vec<LangDataUI>,
    pub matches: Vec<MatchDataUI>,
}

impl MatchGroup {
    pub fn add_langs(mut self, langs: Vec<CommentedLang>) -> Self {
        for lang in langs {
            if let Some(lang_ui) = self.languages.iter_mut().find(|ele| ele.lang == lang.lang) {
                lang_ui.comments.push(lang.comment)
            } else {
                self.languages.push(LangDataUI {
                    lang: lang.lang,
                    comments: vec![lang.comment],
                })
            }
        }
        self
    }

    pub fn add_langs_ref(&mut self, langs: &[CommentedLang]) {
        for lang in langs {
            if let Some(lang_ui) = self.languages.iter_mut().find(|ele| ele.lang == lang.lang) {
                lang_ui.comments.push(lang.comment.to_owned())
            } else {
                self.languages.push(LangDataUI {
                    lang: lang.lang,
                    comments: vec![lang.comment.to_owned()],
                })
            }
        }
    }

    pub fn stats(&self) -> DirStats {
        self.matches.iter().map(|ele| ele.dir_stats).sum()
    }
}

#[derive(Debug, Clone)]
pub struct MatchDataUI {
    pub idx: usize,
    pub path: PathBuf,
    pub dir_stats: DirStats,
}

#[derive(Debug, Clone)]
pub struct LangDataUI {
    pub lang: &'static Lang,
    pub comments: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchDataUIStatus {
    Found,
    Selected,
}

#[derive(Debug, Clone)]
pub struct TableData {
    pub idx: usize,
    pub state: TableState,
    pub data: Vec<MatchGroup>,
    cleanable_space: Size,
}

impl Default for TableData {
    fn default() -> Self {
        Self {
            idx: 0,
            state: Default::default(),
            data: Default::default(),
            cleanable_space: Size::from_bytes(0),
        }
    }
}

impl TableData {
    pub fn add_match(&mut self, data: MatchData) {
        let path = data.group.clone();

        let ui_data = MatchDataUI {
            idx: self.idx,
            path: data.path.clone(),
            dir_stats: DirStats::default(),
        };
        self.idx += 1;

        if let Some(record) = self.data.iter_mut().find(|ele| ele.group_path == path) {
            record.add_langs_ref(data.languages());
            record.matches.push(ui_data);
        } else {
            self.data.push(
                MatchGroup {
                    hidden: data.hidden(),
                    group_path: path,
                    status: MatchDataUIStatus::Found,
                    languages: vec![],
                    matches: vec![ui_data],
                }
                .add_langs(data.languages().to_vec()),
            );
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

        self.data.sort_by_key(MatchGroup::stats);

        if let Some(path) = path {
            if let Some(idx) = self.data.iter().position(|ele| ele.group_path == path) {
                self.state.select(Some(idx))
            }
        }
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
