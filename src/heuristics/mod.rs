use crate::core::Heuristic;

mod git;
mod hidden;
mod rust;
mod unity;

/// A list of all heuristics implemented by default in this crate.
pub const ALL_HEURISTICS: [&dyn Heuristic; 4] = [&hidden::INSTANCE, &rust::INSTANCE, &unity::INSTANCE, &git::INSTANCE];
