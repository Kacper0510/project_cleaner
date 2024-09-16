use super::{Heuristic, InheritedFiles, MatchData, MatchingState};
use jwalk::{ClientState, DirEntry, Result, WalkDirGeneric};
use std::{
    collections::HashSet,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::mpsc::Sender,
};
use tracing::{error, info, trace};

/// Directory scanner cache for storing inherited files and a channel to send matches to.
#[derive(Debug, Default, Clone)]
pub(super) struct ScannerCache {
    /// Files inherited from parent directories, hashed by heuristic type. Propagated by cloning.
    pub(super) inherited_files: InheritedFiles,
    /// Channel to send matches to.
    pub(super) sender: Option<Sender<MatchData>>,
    /// True if any of the parent paths was marked as dangerous.
    pub(super) dangerous: bool,
    /// Trick to propagate dangerous flag properly using [`jwalk`]'s weird data structures...
    pub(super) marked_to_be_dangerous: HashSet<OsString>,
}

impl ScannerCache {
    /// Create a new cache with a specified sender.
    ///
    /// Although this type implements [`Default`], the sender is required for the cache to work,
    /// as it is unwrapped in the [`MatchingState`] methods.
    #[inline]
    fn new(sender: Sender<MatchData>) -> Self {
        let sender = Some(sender);
        Self { sender, ..Default::default() }
    }
}

impl ClientState for ScannerCache {
    type DirEntryState = ();
    type ReadDirState = Self;
}

/// Specialized type for directory entries with this module's scanner cache.
pub(super) type Entry = DirEntry<ScannerCache>;

/// Helper struct for setting all the necessary information for a filesystem scan.
#[derive(Debug, Clone)]
pub struct Scanner {
    /// Specifies where the scan should start.
    pub root: PathBuf,
    /// List of heuristics to use while scanning.
    ///
    /// By default, all the heuristics which are bundled with this crate are used.
    pub heuristics: Vec<&'static dyn Heuristic>,
    /// Matched paths will be sent to this channel while scanning.
    pub sender: Sender<MatchData>,
    /// Enables the dangerous mode, which may find more files, while also possibly returning system paths.
    ///
    /// When false, excludes matches with negative weights from further search (mainly for hiding system files).
    /// When true, only matches with positive weights skip their subdirectories.
    pub dangerous: bool,
}

impl Scanner {
    /// Constructs a new [`Scanner`] with a default heuristics list.
    #[inline]
    pub fn new(root_path: &Path, sender: Sender<MatchData>) -> Self {
        Self { root: root_path.to_owned(), sender, heuristics: crate::ALL_HEURISTICS.to_vec(), dangerous: false }
    }

    /// Starts a scan. This is a blocking operation.
    ///
    /// Some directories may be skipped if they are already matched by a parent directory.
    /// If you want to receive updates about current progress, use [`Self::scan_with_progress()`].
    #[inline]
    pub fn scan(self) {
        self.scan_with_progress().for_each(|_| ())
    }

    /// Starts a scan by returning an iterator containing progress information.
    /// Getting new elements from this iterator is blocking.
    ///
    /// Some directories may be skipped if they are already matched by a parent directory.
    pub fn scan_with_progress(self) -> impl Iterator<Item = Result<PathBuf>> {
        info!("Starting scan: {:#?}", self);
        WalkDirGeneric::<ScannerCache>::new(self.root)
            .root_read_dir_state(ScannerCache::new(self.sender))
            .skip_hidden(false)
            .process_read_dir(move |_depth, path, read_dir_state, children| {
                trace!("Scanning directory: {:#?}", path);

                if path.file_name().is_some_and(|name| read_dir_state.marked_to_be_dangerous.contains(name)) {
                    read_dir_state.dangerous = true;
                    read_dir_state.marked_to_be_dangerous.clear();
                }

                let mut filtered_children: Vec<&mut Entry> =
                    children.iter_mut().map(Result::as_mut).filter_map(|v| v.ok()).collect();
                let mut state = MatchingState::new(&mut filtered_children, read_dir_state, path);
                for heuristic in &self.heuristics {
                    state.current_heuristic = Some(*heuristic);
                    trace!("Running heuristic {} for path {}", heuristic.info(), path.display());
                    heuristic.check_for_matches(&mut state);
                }

                if state.process_collected_data(self.dangerous).is_err() {
                    // Stop iteration as fast as possible
                    children.clear();
                    error!("Sender failure!");
                } else {
                    // Skip files in the progress iteration, yield only directories and errors
                    children.retain(|f| if let Ok(f) = f { f.file_type.is_dir() } else { true });
                }
            })
            .into_iter()
            .map(|entry| {
                let path = entry.map(|e| e.path());
                trace!("Scan progress: {path:#?}");
                path
            })
    }
}
