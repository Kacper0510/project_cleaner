use std::path::PathBuf;

use size::Size;

#[derive(Debug, Clone)]
pub struct FolderData {
    pub path: PathBuf,
    pub rating: i16,
    pub size: Size,
    pub langs: Vec<LangData>,
}

impl FolderData {
    pub fn new(path: PathBuf, rating: i16, size: Size, langs: Vec<LangData>) -> Self {
        FolderData {
            path,
            rating,
            size,
            langs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LangData {
    pub name: &'static str,
    pub icon: &'static str,
}

impl LangData {
    pub const PYTHON: LangData = LangData {
        name: "Python",
        icon: "",
    };
    pub const RUST: LangData = LangData {
        name: "Rust",
        icon: "",
    };
    pub const GIT: LangData = LangData {
        name: "Git",
        icon: "",
    };
}
