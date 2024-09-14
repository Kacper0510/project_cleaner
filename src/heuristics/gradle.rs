use crate::heuristic;
use regex::Regex;

heuristic!(Gradle, "", "gradle", IconColor(25), state, {
    if !state.match_file(Regex::new(r"^gradlew(\.bat)?$").expect("Regex error :)")).is_empty() {
        if state.has_directory("build").is_some() {
            state.add_match("build", "Found gradlew and build directory.");
        }
        if state.has_directory(".gradle").is_some() {
            state.add_match(".gradle", "Found gradlew and .gradle directory.").weight(2000);
        }
    }
});
