use std::{collections::HashMap, ffi::{OsString, OsStr}, path::PathBuf};
use super::{Entry, Heuristic};

struct PathData<'entry> {
    entry: &'entry Entry,
    weight: i32,
    reasons: Vec<String>,
}

pub struct MatchingState<'entries> {
    contents: HashMap<OsString, PathData<'entries>>,
    current_heuristic: &'static dyn Heuristic,
    pub important_files: Vec<PathBuf>,
}

impl<'entries> MatchingState<'entries> {
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
            v.reasons.push(self.current_heuristic.name().to_owned());
        }
    }

    pub fn add_weight_with_reason(&mut self, name: &str, weight: i32, reason: String) {
        if let Some(v) = self.contents.get_mut(OsStr::new(name)) {
            v.weight += weight;
            v.reasons.push(reason);
        }
    }
}
