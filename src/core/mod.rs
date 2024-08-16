use jwalk::*;
use std::{
    path::{Path, PathBuf},
    sync::mpsc::Sender,
};

#[derive(Debug, Default, Clone)]
struct WalkerCache {
    important_files: Vec<PathBuf>,
    sender: Option<Sender<MatchData>>,
}

impl WalkerCache {
    fn new(sender: Sender<MatchData>) -> Self {
        let sender = Some(sender);
        Self {
            important_files: vec![],
            sender,
        }
    }
}

impl ClientState for WalkerCache {
    type DirEntryState = ();
    type ReadDirState = Self;
}

type Entry = DirEntry<WalkerCache>;

mod matching_state;
pub use matching_state::MatchingState;

pub trait Heuristic {
    fn info(&self) -> LangData;
    fn check_for_matches(&self, state: &mut MatchingState);
}

#[derive(Debug, Clone)]
pub struct MatchData {
    pub path: PathBuf,
    pub weight: i32,
    pub reasons: Vec<LangData>,
}

#[derive(Debug, Clone)]
pub struct LangData {
    pub name: &'static str,
    pub icon: &'static str,
    pub short: &'static str,
    pub comment: Option<String>,
}

impl LangData {
    pub fn new(name: &'static str, icon: &'static str, short: &'static str) -> Self {
        Self {
            name,
            icon,
            short,
            comment: None,
        }
    }

    pub fn comment(mut self, comment: &str) -> Self {
        self.comment = Some(comment.to_owned());
        self
    }
}

pub fn walk_directories<F>(root_path: &Path, sender: Sender<MatchData>, mut progress_callback: F)
where F: FnMut(Result<PathBuf>) {
    let iter = WalkDirGeneric::<WalkerCache>::new(root_path)
        .root_read_dir_state(WalkerCache::new(sender))
        .skip_hidden(false)
        .process_read_dir(|_depth, _path, read_dir_state, children| {
            let mut filtered_children: Vec<&mut Entry> =
                children.iter_mut().map(Result::as_mut).filter_map(|v| v.ok()).collect();
            let mut state = MatchingState::new(&mut filtered_children, &mut read_dir_state.important_files);
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
