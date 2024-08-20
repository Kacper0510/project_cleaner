use crate::core::{Heuristic, LangData, MatchingState};

mod rust;
mod unity;

pub const ALL_HEURISTICS: [&dyn Heuristic; 2] = [&rust::INSTANCE, &unity::INSTANCE];
