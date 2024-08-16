use super::*;

#[derive(Default)]
pub struct CargoTarget;

pub const INSTANCE: CargoTarget = CargoTarget;

impl Heuristic for CargoTarget {
    fn info(&self) -> LangData {
        LangData::new("Rust", "")
    }

    fn check_for_matches(&self, state: &mut MatchingState) {
        if state.has_file("Cargo.toml").is_some() && state.has_directory("target").is_some() {
            state.add_weight("target", 1000);
        }
    }
}
