use crate::heuristic;

heuristic!(Rust, "", "rs", state, {
    if state.has_file("Cargo.toml").is_some() && state.has_directory("target").is_some() {
        state.add_weight_with_comment("target", 1000, "Found Cargo.toml and target directory.");
    }
});
