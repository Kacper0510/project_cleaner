use crate::core::{Heuristic, MatchingState};

mod cargo_target;

pub const ALL_HEURISTICS: [&dyn Heuristic; 1] = [
    &cargo_target::INSTANCE,
];
