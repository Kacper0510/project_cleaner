use crate::heuristic;

const DIRS: [&str; 14] = [
    "Library",
    "library",
    "Logs",
    "logs",
    "Obj",
    "obj",
    "Temp",
    "temp",
    "UserSettings",
    "userSettings",
    "MemoryCaptures",
    "memoryCaptures",
    "Recordings",
    "recordings",
];

heuristic!(Unity, "󰚯", "unity", IconColor(15), state, {
    if state.has_directory("Assets").is_some()
        && state.has_directory("Packages").is_some()
        && state.has_directory("ProjectSettings").is_some()
    {
        for dir in DIRS {
            if state.has_directory(dir).is_some() {
                state.add_match(
                    dir,
                    &format!("Found Assets, Packages, ProjectSettings and {dir} directory."),
                );
            }
        }
    }
});
