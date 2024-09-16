use crate::heuristic;
use regex::Regex;

heuristic!(CMake, "", "cmake", IconColor(34), state, {
    for path in state.match_directory(&Regex::new(r"build").expect("Regex error :)")) {
        if path.join("CMakeCache.txt").is_file() {
            state.add_match(path.file_name().expect("Expected path"), "Found cmake build directory.");
        }
    }
});
