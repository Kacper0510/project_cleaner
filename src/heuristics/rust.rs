use crate::heuristic;

heuristic!(Rust, "", "rs", IconColor(166), state, {
    if state.has_file("Cargo.toml").is_some() && state.has_directory("target").is_some() {
        state.add_match("target", "Found Cargo.toml and target directory.");
    }
});
