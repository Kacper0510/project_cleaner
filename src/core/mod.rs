use jwalk::*;
use std::{
    any::TypeId, collections::HashMap, path::{Path, PathBuf}, sync::mpsc::Sender
};

type InheritedFiles = HashMap<TypeId, Vec<PathBuf>>;

#[derive(Debug, Default, Clone)]
struct WalkerCache {
    inherited_files: InheritedFiles,
    sender: Option<Sender<MatchData>>,
}

impl WalkerCache {
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

type Entry = DirEntry<WalkerCache>;

mod dir_rm;
pub use dir_rm::dir_rm_parallel;

mod dir_stats;
pub use dir_stats::{DirStats, dir_stats_parallel};

mod match_data;
pub use match_data::{MatchData, MatchDataBuilder};

mod matching_state;
pub use matching_state::MatchingState;

mod heuristic;
pub use heuristic::{Heuristic, LangData};

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
