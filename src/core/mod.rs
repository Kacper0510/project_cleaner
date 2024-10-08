use std::{any::TypeId, collections::HashMap, path::PathBuf};

mod dir_rm;
pub use dir_rm::dir_rm_parallel;

mod dir_stats;
pub use dir_stats::{dir_stats_parallel, DirStats};

mod match_data;
pub use match_data::{MatchData, MatchParameters};

mod matching_state;
pub use matching_state::MatchingState;

mod heuristic;
pub use heuristic::Heuristic;

mod lang;
pub use lang::{CommentedLang, Lang};

mod color;
pub use color::IconColor;

mod scanner;
pub use scanner::Scanner;

const DEFAULT_THREAD_COUNT: usize = 1;
const PERCENTAGE_OF_USED_CORES: usize = 50;

const _CORE_MULTIPLIER: usize = 100 / PERCENTAGE_OF_USED_CORES; // Don't modify this line

/// Type for storing files inherited from parent directories.
/// See [`MatchingState::inherited_files()`].
type InheritedFiles = HashMap<TypeId, Vec<PathBuf>>;
