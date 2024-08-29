use crate::heuristic;
use std::ffi::OsStr;

heuristic!(Hidden, "󰘓", ".", IconColor(7), state, {
    let to_hide: Vec<_> = state
        .get_all_contents()
        .filter_map(|v| v.0.file_name().map(OsStr::to_owned))
        .filter(|name| name.to_str().is_some_and(|s| s.starts_with('.')))
        .collect();
    for path in to_hide {
        state.add_match(&path, &format!("{} starts with a dot.", path.to_string_lossy())).weight(0).hidden();
    }
});
