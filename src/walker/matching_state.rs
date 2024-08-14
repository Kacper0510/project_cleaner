use super::{Entry, Heuristic, MarkedForDeletion, Sender};
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    path::PathBuf,
};

struct PathData<'entry> {
    entry: &'entry mut Entry,
    weight: i32,
    reasons: Vec<String>,
}

pub struct MatchingState<'entries> {
    contents: HashMap<OsString, PathData<'entries>>,
    pub current_heuristic: Option<&'static dyn Heuristic>,
    pub important_files: &'entries mut Vec<PathBuf>,
}

impl<'entries> MatchingState<'entries> {
    pub(super) fn new(children: &'entries mut [&mut Entry], files: &'entries mut Vec<PathBuf>) -> Self {
        Self {
            contents: children
                .iter_mut()
                .map(|v| {
                    (v.file_name.clone(), PathData {
                        entry: v,
                        weight: 0,
                        reasons: vec![],
                    })
                })
                .collect(),
            current_heuristic: None,
            important_files: files,
        }
    }

    pub(super) fn process_collected_data(&mut self, sender: &Sender<MarkedForDeletion>) {
        for v in self.contents.values_mut() {
            // TODO
            v.entry.read_children_path = None;
        }
    }

    pub fn has_file(&self, name: &str) -> Option<PathBuf> {
        self.contents.get(OsStr::new(name)).filter(|v| v.entry.file_type.is_file()).map(|v| v.entry.path())
    }

    pub fn has_directory(&self, name: &str) -> Option<PathBuf> {
        self.contents.get(OsStr::new(name)).filter(|v| v.entry.file_type.is_dir()).map(|v| v.entry.path())
    }

    pub fn get_all_directories(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.contents.values().filter(move |v| v.entry.file_type.is_dir()).map(move |v| v.entry.path())
    }

    pub fn add_weight(&mut self, name: &str, weight: i32) {
        if let Some(v) = self.contents.get_mut(OsStr::new(name)) {
            v.weight += weight;
            v.reasons.push(self.current_heuristic.unwrap().name().to_owned());
        }
    }

    pub fn add_weight_with_reason(&mut self, name: &str, weight: i32, reason: String) {
        if let Some(v) = self.contents.get_mut(OsStr::new(name)) {
            v.weight += weight;
            v.reasons.push(reason);
        }
    }
}
