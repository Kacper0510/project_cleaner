use crate::heuristic;

heuristic!(Direnv, "", "direnv", IconColor(126), state, {
    if state.has_directory(".direnv").is_some() && state.has_file(".envrc").is_some() {
        state.add_match(".direnv", "Found .direnv directory").weight(2000);
    }
});
