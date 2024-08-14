use jwalk::*;
use std::{
    path::{Path, PathBuf},
    sync::mpsc::{channel, Sender},
};
use crate::heuristics::ALL_HEURISTICS;

#[derive(Debug, Default, Clone)]
struct WalkerCache {
    important_files: Vec<PathBuf>,
    sender: Option<Sender<MarkedForDeletion>>,
}

impl WalkerCache {
    fn new(sender: Sender<MarkedForDeletion>) -> Self {
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
    fn name(&self) -> &'static str;
    fn check_for_matches(&self, state: &mut MatchingState);
}

#[derive(Debug)]
pub struct MarkedForDeletion {
    pub path: PathBuf,
    pub weight: i32,
    pub reasons: Vec<String>,
}

pub fn walk_directories(root_path: &Path) {
    let (sender, receiver) = channel();
    let iter = WalkDirGeneric::<WalkerCache>::new(root_path)
        .root_read_dir_state(WalkerCache::new(sender))
        .skip_hidden(false)
        .process_read_dir(|_depth, path, read_dir_state, children| {
            let mut children: Vec<&mut Entry> = children.iter_mut().map(Result::as_mut).filter_map(|v| v.ok()).collect();
            let mut state = MatchingState::new(&mut children, &mut read_dir_state.important_files);
            for heuristic in ALL_HEURISTICS {
                state.current_heuristic = Some(heuristic);
                heuristic.check_for_matches(&mut state);
            }
            state.process_collected_data(read_dir_state.sender.as_ref().unwrap());
        });
    todo!()
}
