use crate::heuristic;

heuristic!(Flutter, "", "flutter", IconColor(39), state, {
    if state.has_file(".metadata").is_some() && state.has_file("pubspec.yaml").is_some() {
        if state.has_directory("build").is_some() {
            state.add_match("build", "Found .metadata, pubspec.yaml files and build directory.");
        }
        if state.has_directory(".dart_tool").is_some() {
            state.add_match(".dart_tool", "Found .metadata, pubspec.yaml files and .dart_tool directory.").weight(2000);
        }
    }
});
