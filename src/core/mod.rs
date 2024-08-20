use jwalk::*;
use matching_state::MatchDataBuilder;
use std::{
    any::TypeId, collections::HashMap, fmt, path::{Path, PathBuf}, sync::mpsc::Sender
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

pub mod dir_rm;
pub mod dir_stats;
mod matching_state;
pub use matching_state::MatchingState;

pub trait Heuristic {
    fn info(&self) -> LangData;
    fn check_for_matches(&self, state: &mut MatchingState);
}

impl fmt::Debug for dyn Heuristic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.info())
    }
}

impl fmt::Display for dyn Heuristic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.info())
    }
}

#[derive(Debug, Clone)]
pub struct MatchData {
    pub path: PathBuf,
    pub group: PathBuf,
    other_data: MatchDataBuilder,
}

impl MatchData {
    pub fn weight(&self) -> u32 {
        self.other_data.weight as u32
    }

    pub fn languages(&self) -> &[LangData] {
        &self.other_data.reasons
    }

    pub fn hidden(&self) -> bool {
        self.other_data.hidden
    }
}

#[derive(Debug, Clone)]
pub struct LangData {
    pub name: &'static str,
    pub icon: &'static str,
    pub short: &'static str,
    pub comment: Option<String>,
}

impl fmt::Display for LangData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(comment) = &self.comment {
            write!(f, " - {}", comment)
        } else {
            Ok(())
        }
    }
}

impl LangData {
    pub const fn new(name: &'static str, icon: &'static str, short: &'static str) -> Self {
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
