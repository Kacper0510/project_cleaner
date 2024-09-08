use crate::heuristic;

struct SystemConstants {
    icon: &'static str,
    color: IconColor,
    paths: &'static [&'static str],
}

#[cfg(windows)]
const CONSTANTS: SystemConstants = SystemConstants {
    icon: "",
    color: IconColor(26),
    paths: &[
        "AppData",
    ],
};

#[cfg(all(unix, target_os = "macos"))]
const CONSTANTS: SystemConstants = SystemConstants {
    icon: "",
    color: IconColor(255),
    paths: &[
        "Applications",
    ],
};

#[cfg(all(unix, not(target_os = "macos")))]
const CONSTANTS: SystemConstants = SystemConstants {
    icon: "",
    color: IconColor(255),
    paths: &[
        "opt",
    ],
};

heuristic!(System, CONSTANTS.icon, "os", CONSTANTS.color, state, {
    for path in CONSTANTS.paths {
        if state.has_directory(path).is_some() {
            state.add_match(&path, &format!("{} is a system path.", path)).dangerous();
        }
    }
});