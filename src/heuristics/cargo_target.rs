use super::*;

#[derive(Default)]
pub struct CargoTarget;

pub const INSTANCE: CargoTarget = CargoTarget;

impl Heuristic for CargoTarget {
    fn name(&self) -> &'static str {
        "Rust/Cargo target directory"
    }
    
    fn check_for_matches(&self, state: &mut MatchingState) {
        if state.has_file("Cargo.toml").is_some() && state.has_directory("target").is_some() {
            state.add_weight("target", 1000);
        }
    }
}
