use super::{Heuristic, InheritedFiles, MatchData, MatchingState};
use jwalk::{ClientState, DirEntry, Result, WalkDirGeneric};
use std::{
    path::{Path, PathBuf},
    sync::mpsc::Sender,
};
use tracing::{info, trace};

/// Directory scanner cache for storing inherited files and a channel to send matches to.
#[derive(Debug, Default, Clone)]
pub(super) struct ScannerCache {
    /// Files inherited from parent directories, hashed by heuristic type. Propagated by cloning.
    inherited_files: InheritedFiles,
    /// Channel to send matches to.
    sender: Option<Sender<MatchData>>,
}

impl ScannerCache {
    /// Create a new cache with a specified sender.
    ///
    /// Although this type implements [`Default`], the sender is required for the cache to work,
    /// as it is unwrapped in the [`MatchingState`] methods.
    #[inline]
    fn new(sender: Sender<MatchData>) -> Self {
        let sender = Some(sender);
        Self {
            inherited_files: InheritedFiles::new(),
            sender,
        }
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
}

impl Scanner {
    /// Constructs a new [`Scanner`] with a default heuristics list.
    #[inline]
    pub fn new(root_path: &Path, sender: Sender<MatchData>) -> Self {
        Self {
            root: root_path.to_owned(),
            sender,
            heuristics: crate::ALL_HEURISTICS.to_vec(),
        }
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
                let mut filtered_children: Vec<&mut Entry> =
                    children.iter_mut().map(Result::as_mut).filter_map(|v| v.ok()).collect();
                let mut state = MatchingState::new(&mut filtered_children, &mut read_dir_state.inherited_files, path);
                for heuristic in &self.heuristics {
                    state.current_heuristic = Some(*heuristic);
                    trace!("Running heuristic {} for path {}", heuristic.info(), path.display());
                    heuristic.check_for_matches(&mut state);
                }
                state.process_collected_data(read_dir_state.sender.as_ref().unwrap());

                // Skip files in the progress iteration, yield only directories and errors
                children.retain(|f| if let Ok(f) = f { f.file_type.is_dir() } else { true });
            })
            .into_iter()
            .map(|entry| {
                let path = entry.map(|e| e.path());
                trace!("Scan progress: {path:#?}");
                path
            })
    }
}
