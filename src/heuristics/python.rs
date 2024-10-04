use crate::heuristic;


heuristic!(Python, "", "py", IconColor(32), state, {
    if state.has_directory("__pycache__").is_some() {
        state.add_match("__pycache__", "Found __pycache__.");
    }
});
