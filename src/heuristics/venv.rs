use crate::heuristic;

heuristic!(Venv, "", "venv", IconColor(42), state, {
    if let Some(path) = state.has_directory("venv").or(state.has_directory("env")) {
        if path.join("pyvenv.cfg").is_file() {
            state.add_match(path.file_name().expect("Expected path"), "Found python venv.");
        }
    }
});
