use std::{collections::HashMap, path::*};
use jwalk::DirEntry;

type Entry = DirEntry<((), ())>;

#[derive(Debug, Clone)]
pub struct MatchInfo {
    pub match_type: String,
    pub weight: i32,
}

pub trait Heuristic {
    fn filter(&self, entry: &Entry) -> bool;
    fn update_matches(&self, path: PathBuf, data: &mut HashMap<PathBuf, Vec<MatchInfo>>);
}

mod cargo_target;

pub const ALL_HEURISTICS: [&dyn Heuristic; 1] = [
    &cargo_target::INSTANCE,
];
