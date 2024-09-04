use crate::core::Heuristic;

mod git;
mod hidden;
mod js;
mod python;
mod rust;
mod system;
mod unity;

/// A list of all heuristics implemented by default in this crate.
///
/// According to [the Compiler Explorer](https://godbolt.org/),
/// a slice generates even less assembly code than an array (weird, isn't it?).
pub const ALL_HEURISTICS: &[&dyn Heuristic] = &[
    &hidden::INSTANCE,
    &system::INSTANCE,
    &rust::INSTANCE,
    &unity::INSTANCE,
    &js::INSTANCE,
    &python::INSTANCE,
    &git::INSTANCE,
];
