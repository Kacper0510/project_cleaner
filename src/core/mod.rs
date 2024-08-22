use jwalk::*;
use std::{
    any::TypeId,
    collections::HashMap,
    path::{Path, PathBuf},
    sync::mpsc::Sender,
};

/// Type for storing files inherited from parent directories.
/// See [`MatchingState::inherited_files()`].
type InheritedFiles = HashMap<TypeId, Vec<PathBuf>>;

/// Directory walker cache for storing inherited files and a channel to send matches to.
#[derive(Debug, Default, Clone)]
struct WalkerCache {
    /// Files inherited from parent directories, hashed by heuristic type. Propagated by cloning.
    inherited_files: InheritedFiles,
    /// Channel to send matches to.
    sender: Option<Sender<MatchData>>,
}

impl WalkerCache {
    /// Create a new cache with a specified sender.
    ///
    /// Although this type implements [`Default`], the sender is required for the cache to work,
    /// as it is unwrapped in the [`MatchingState`] methods.
    #[inline]
    fn new(sender: Sender<MatchData>) -> Self {
        let sender = Some(sender);
        Self {
            inherited_files: HashMap::new(),
            sender,
        }
    }
}

impl ClientState for WalkerCache {
    type DirEntryState = ();
    type ReadDirState = Self;
}

/// Specialized type for directory entries with this module's walker cache.
type Entry = DirEntry<WalkerCache>;

mod dir_rm;
pub use dir_rm::dir_rm_parallel;

mod dir_stats;
pub use dir_stats::{dir_stats_parallel, DirStats};

mod match_data;
pub use match_data::{MatchData, MatchParameters};

mod matching_state;
pub use matching_state::MatchingState;

mod heuristic;
pub use heuristic::{Heuristic, LangData};

/// Traverses filesystem and sends heuristic matches to the specified channel.
///
/// This function walks the filesystem starting from the specified root path,
/// collecting matches for each heuristic and sending them to the specified channel.
/// It also calls the progress callback for each directory visited, whether it is matched or not.
/// Some directories may be skipped if they are already matched by a parent directory.
pub fn walk_directories<F>(root_path: &Path, sender: Sender<MatchData>, mut progress_callback: F)
where F: FnMut(Result<PathBuf>) {
    let iter = WalkDirGeneric::<WalkerCache>::new(root_path)
        .root_read_dir_state(WalkerCache::new(sender))
        .skip_hidden(false)
        .process_read_dir(|_depth, path, read_dir_state, children| {
            let mut filtered_children: Vec<&mut Entry> =
                children.iter_mut().map(Result::as_mut).filter_map(|v| v.ok()).collect();
            let mut state = MatchingState::new(&mut filtered_children, &mut read_dir_state.inherited_files, path);
            for heuristic in super::ALL_HEURISTICS {
                state.current_heuristic = Some(heuristic);
                heuristic.check_for_matches(&mut state);
            }
            state.process_collected_data(read_dir_state.sender.as_ref().unwrap());

            // Skip files in the progress iteration, yield only directories and errors
            children.retain(|f| if let Ok(f) = f { f.file_type.is_dir() } else { true });
        });
    for path in iter {
        progress_callback(path.map(|e| e.path()));
    }
}
