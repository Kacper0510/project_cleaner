use super::{Entry, Heuristic, LangData, MatchData, Sender};
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    path::PathBuf,
};

struct PathData<'entry> {
    entry: &'entry mut Entry,
    weight: i32,
    reasons: Vec<LangData>,
}

pub struct MatchingState<'entries> {
    contents: HashMap<OsString, PathData<'entries>>,
    pub(super) current_heuristic: Option<&'static dyn Heuristic>,
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

    pub(super) fn process_collected_data(&mut self, sender: &Sender<MatchData>) {
        for (_, v) in self.contents.drain() {
            if v.weight <= 0 {
                continue;
            }
            v.entry.read_children_path = None;
            let data = MatchData {
                path: v.entry.path(),
                weight: v.weight,
                reasons: v.reasons,
            };
            sender.send(data).expect("Sender error (did UI panic?)");
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
            v.reasons.push(self.current_heuristic.unwrap().info());
        }
    }

    pub fn add_weight_with_comment(&mut self, name: &str, weight: i32, comment: &str) {
        if let Some(v) = self.contents.get_mut(OsStr::new(name)) {
            v.weight += weight;
            v.reasons.push(self.current_heuristic.unwrap().info().comment(comment));
        }
    }
}
