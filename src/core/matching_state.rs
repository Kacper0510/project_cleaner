use super::*;
use std::{
    any::Any,
    ffi::{OsStr, OsString},
    ops::DerefMut,
};

/// State passed to heuristics to manipulate matches and query current directory contents.
///
/// Only this type should be used to interact with the filesystem and return meaningful heuristic result to the user.
pub struct MatchingState<'entries> {
    /// Optimized storage for current direcotry contents.
    contents: HashMap<OsString, (&'entries mut Entry, Vec<MatchDataBuilder>)>,
    /// Path of the current directory.
    parent_path: &'entries Path,
    /// Current heuristic being processed.
    pub(super) current_heuristic: Option<&'static dyn Heuristic>,
    /// Files inherited from parent directories, hashed by heuristic type.
    inherited_files: &'entries mut InheritedFiles,
}

impl<'entries> MatchingState<'entries> {
    /// Creates a new matching state for the specified directory, its entries and inherited files.
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

    /// Function to be called after every heuristic has done its job.
    ///
    /// This function filters and reorganizes all collected data in order to send it to the specified channel.
    ///
    /// # Panics
    ///
    /// Panics if the channel is closed, which should not happen in normal operation.
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

    /// Returns the path of the current directory.
    ///
    /// This may be used to exclude certain directories from the search.
    #[inline]
    pub fn path(&self) -> &Path {
        self.parent_path
    }

    /// Returns saved file paths for the current heuristic.
    ///
    /// This vector is inherited from parent directories and propagated while traversing filesystem.
    /// It is used to check for matches in files that are not in the current directory
    /// and/or store additional data for future calls.
    pub fn inherited_files(&mut self) -> &mut Vec<PathBuf> {
        self.inherited_files.entry(self.current_heuristic.type_id()).or_default()
    }

    /// Returns the path of the specified file in the current directory if it exists and is accesible.
    ///
    /// The result may be used to read the file contents.
    pub fn has_file(&self, name: &str) -> Option<PathBuf> {
        self.contents.get(OsStr::new(name)).filter(|v| v.0.file_type.is_file()).map(|v| v.0.path())
    }

    /// Returns the path of the specified directory in the current directory if it exists and is accesible.
    ///
    /// The result may be used to traverse the directory contents, but it is recommended to use
    /// caching via [`inherited_files()`](Self::inherited_files()) instead (if possible).
    pub fn has_directory(&self, name: &str) -> Option<PathBuf> {
        self.contents.get(OsStr::new(name)).filter(|v| v.0.file_type.is_dir()).map(|v| v.0.path())
    }

    /// Returns an iterator over all files in the current directory.
    pub fn get_all_directories(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.contents.values().filter(move |v| v.0.file_type.is_dir()).map(move |v| v.0.path())
    }

    /// Adds a match for the file or directory selected with the `name` parameter.
    ///
    /// The `comment` parameter is used to describe the match and is displayed to the user.
    /// Additional match options may be changed by calling methods on the returned builder.
    ///
    /// # Panics
    ///
    /// Panics if the specified file or directory does not exist in the current directory.
    pub fn add_match(&mut self, name: &str, comment: &str) -> &mut MatchDataBuilder {
        let new = MatchDataBuilder::new(self.current_heuristic.unwrap().info().with_comment(comment));
        if let Some((_, v)) = self.contents.get_mut(OsStr::new(name)) {
            v.push(new);
            v.last_mut().unwrap()
        } else {
            panic!("Heuristic \"{}\" tried to add invalid match: {}", self.current_heuristic.unwrap(), name)
        }
    }
}
