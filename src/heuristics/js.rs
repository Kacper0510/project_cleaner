use crate::heuristic;

heuristic!(JavaScript, "", "js", IconColor(221), state, {
    if state.has_directory("node_modules").is_some() {
        state.add_match("node_modules", "Found node_modules.");
    }
});
