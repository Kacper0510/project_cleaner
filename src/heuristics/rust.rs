use super::*;

#[derive(Default)]
pub struct Rust;

pub const INSTANCE: Rust = Rust;

impl Heuristic for Rust {
    fn info(&self) -> LangData {
        LangData::new("Rust", "", "rs")
    }

    fn check_for_matches(&self, state: &mut MatchingState) {
        if state.has_file("Cargo.toml").is_some() && state.has_directory("target").is_some() {
            state.add_weight_with_comment("target", 1000, "Found Cargo.toml and target directory.");
        }
    }
}
