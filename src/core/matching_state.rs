use super::*;
use std::{
    any::Any,
    ffi::{OsStr, OsString},
    ops::DerefMut,
};

pub struct MatchingState<'entries> {
    contents: HashMap<OsString, (&'entries mut Entry, Vec<MatchDataBuilder>)>,
    parent_path: &'entries Path,
    pub(super) current_heuristic: Option<&'static dyn Heuristic>,
    inherited_files: &'entries mut InheritedFiles,
}

impl<'entries> MatchingState<'entries> {
    pub(super) fn new(
        children: &'entries mut [&mut Entry],
        files: &'entries mut InheritedFiles,
        path: &'entries Path,
    ) -> Self {
        Self {
            contents: children.iter_mut().map(|v| (v.file_name.clone(), (v.deref_mut(), vec![]))).collect(),
            current_heuristic: None,
            inherited_files: files,
            parent_path: path,
        }
    }

    pub(super) fn process_collected_data(&mut self, sender: &Sender<MatchData>) {
        for (_, (entry, md)) in self.contents.drain() {
            let md: MatchDataBuilder = md.into_iter().sum();
            if md.weight <= 0 {
                continue;
            }
            entry.read_children_path = None;
            let data = MatchData {
                path: entry.path(),
                group: self.parent_path.to_owned(),
                other_data: md,
            };
            sender.send(data).expect("Sender error (did UI panic?)");
        }
    }

    pub fn path(&self) -> &Path {
        self.parent_path
    }

    pub fn inherited_files(&mut self) -> &mut Vec<PathBuf> {
        self.inherited_files.entry(self.current_heuristic.type_id()).or_default()
    }

    pub fn has_file(&self, name: &str) -> Option<PathBuf> {
        self.contents.get(OsStr::new(name)).filter(|v| v.0.file_type.is_file()).map(|v| v.0.path())
    }

    pub fn has_directory(&self, name: &str) -> Option<PathBuf> {
        self.contents.get(OsStr::new(name)).filter(|v| v.0.file_type.is_dir()).map(|v| v.0.path())
    }

    pub fn get_all_directories(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.contents.values().filter(move |v| v.0.file_type.is_dir()).map(move |v| v.0.path())
    }

    pub fn add_match(&mut self, name: &str, comment: &str) -> &mut MatchDataBuilder {
        let new = MatchDataBuilder::new(self.current_heuristic.unwrap().info().comment(comment));
        if let Some((_, v)) = self.contents.get_mut(OsStr::new(name)) {
            v.push(new);
            v.last_mut().unwrap()
        } else {
            panic!("Heuristic \"{}\" tried to add invalid match: {}", self.current_heuristic.unwrap(), name)
        }
    }
}
