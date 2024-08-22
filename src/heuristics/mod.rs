use crate::core::{Heuristic, LangData, MatchingState};

mod cargo_target;

/// A list of all heuristics implemented by default in this crate.
pub const ALL_HEURISTICS: [&dyn Heuristic; 1] = [&cargo_target::INSTANCE];
