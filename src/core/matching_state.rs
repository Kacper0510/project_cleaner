use super::{
    scanner::{self, ScannerCache},
    CommentedLang, Heuristic, MatchData, MatchParameters,
};
use std::{
    any::Any,
    collections::HashMap,
    ffi::{OsStr, OsString},
    fs::FileType,
    ops::DerefMut,
    path::{Path, PathBuf},
};
use tracing::{debug, info, warn};

/// State passed to heuristics to manipulate matches and query current directory contents.
///
/// Only this type should be used to interact with the filesystem and return meaningful heuristic result to the user.
pub struct MatchingState<'entries> {
    /// Optimized storage for current directory contents.
    contents: HashMap<OsString, (&'entries mut scanner::Entry, Vec<MatchParameters>)>,
    /// Path of the current directory.
    parent_path: &'entries Path,
    /// Current heuristic being processed.
    pub(super) current_heuristic: Option<&'static dyn Heuristic>,
    /// [`ScannerCache`] associated with the current path.
    cache: &'entries mut ScannerCache,
}

impl<'entries> MatchingState<'entries> {
    /// Creates a new matching state for the specified directory, its entries and scanner cache.
    pub(super) fn new(
        children: &'entries mut [&mut scanner::Entry],
        cache: &'entries mut ScannerCache,
        path: &'entries Path,
    ) -> Self {
        Self {
            contents: children.iter_mut().map(|v| (v.file_name.clone(), (v.deref_mut(), vec![]))).collect(),
            current_heuristic: None,
            cache,
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
    pub(super) fn process_collected_data(&mut self, include_dangerous: bool) {
        for (entry_name, (entry, params)) in self.contents.drain() {
            let accumulated_params: MatchParameters = params.into_iter().sum();
            match accumulated_params.weight {
                nw @ ..=-1 if !accumulated_params.dangerous => {
                    info!("Negative weight of {}, but not dangerous: {:#?}", nw, entry_name);
                },
                nw @ ..=-1 if include_dangerous => {
                    if self.cache.dangerous {
                        warn!("{:#?} is already dangerous!", entry_name);
                    } else if entry.file_type.is_dir() {
                        info!("Negative weight of {}, marking as dangerous: {:#?}", nw, entry_name);
                        self.cache.marked_to_be_dangerous.insert(entry_name);
                    }
                },
                nw @ ..=-1 => {
                    info!("Negative weight of {}, skipping children: {:#?}", nw, entry_name);
                    entry.read_children_path = None;
                },
                0 => (),
                pw @ 1.. => {
                    entry.read_children_path = None;
                    let data = MatchData {
                        path: entry.path(),
                        group: self.parent_path.to_owned(),
                        params: MatchParameters {
                            dangerous: self.cache.dangerous,
                            ..accumulated_params
                        },
                    };
                    info!("Positive weight of {}, sending match: {:#?}", pw, entry_name);
                    debug!("{:#?}", data);
                    self.cache.sender.as_ref().unwrap().send(data).expect("Sender error (did UI panic?)");
                },
            }
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
        self.cache.inherited_files.entry(self.current_heuristic.type_id()).or_default()
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

    /// Returns an iterator over all files/subdirectories in the current directory.
    ///
    /// The second parameter in each entry contains information about the path type.
    pub fn get_all_contents(&self) -> impl Iterator<Item = (PathBuf, FileType)> + '_ {
        self.contents.values().map(|v| (v.0.path(), v.0.file_type))
    }

    /// Adds a match for the file or directory selected with the `name` parameter.
    ///
    /// The `comment` parameter is used to describe the match and is displayed to the user.
    /// Additional match options may be changed by calling methods of the returned reference.
    ///
    /// # Panics
    ///
    /// Panics if the specified file or directory does not exist in the current directory.
    pub fn add_match<S>(&mut self, name: &S, comment: &str) -> &mut MatchParameters
    where S: AsRef<OsStr> + ?Sized + std::fmt::Debug {
        let new = MatchParameters::new(CommentedLang {
            lang: self.current_heuristic.unwrap().info(),
            comment: comment.to_owned(),
        });
        if let Some((_, v)) = self.contents.get_mut(OsStr::new(name)) {
            v.push(new);
            v.last_mut().unwrap()
        } else {
            panic!("Heuristic \"{}\" tried to add invalid match: {:#?}", self.current_heuristic.unwrap(), name)
        }
    }
}
