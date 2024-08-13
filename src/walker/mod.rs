use std::path::{Path, PathBuf};
use jwalk::*;

type WalkerCache = ((), ());  // TODO
pub type Entry = DirEntry<WalkerCache>;

mod matching_state;
pub use matching_state::MatchingState;

pub trait Heuristic {
    fn name(&self) -> &'static str;
    fn check_for_matches(&self, state: &mut MatchingState);
}

#[derive(Debug)]
pub struct MarkedForDeletion {
    pub path: PathBuf,
    pub weight: u32,
    pub reasons: Vec<String>,
}

pub fn walk_directories(root_path: &Path) -> Vec<MarkedForDeletion> {
    WalkDirGeneric::<WalkerCache>::new(root_path).skip_hidden(false).process_read_dir(|_depth, path, read_dir_state, children| {
        
    });
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore]
    fn walk_home_dir() {
        let home = std::env::home_dir().unwrap().join("files").join("coding").join("rust");
        assert!(home.exists() && home.is_dir());
        let data = walk_directories(&home);
        assert!(!data.is_empty());
        println!("{:#?}", data);
    }
}