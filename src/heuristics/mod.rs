use crate::core::Heuristic;

mod cmake;
mod direnv;
mod flutter;
mod git;
mod gradle;
mod hidden;
mod js;
mod python;
mod rust;
mod system;
mod unity;
mod venv;

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
    &venv::INSTANCE,
    &direnv::INSTANCE,
    &flutter::INSTANCE,
    &cmake::INSTANCE,
    &gradle::INSTANCE,
];
