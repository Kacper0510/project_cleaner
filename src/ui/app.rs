use std::{error, path::PathBuf};

use ratatui::widgets::{Cell, Row, TableState};
use size::Size;
use throbber_widgets_tui::ThrobberState;

use crate::core::{FolderData, LangData};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub table: TableData,
    pub throbber_state: ThrobberState,
}
#[derive(Debug, Clone)]
pub struct TableData {
    pub state: TableState,
    pub data: Vec<FolderData>,
}

impl TableData {
    fn default() -> Self {
        TableData {
            state: TableState::default().with_selected(0),
            data: vec![
                FolderData::new(
                    PathBuf::from("/mnt/dane/Programowanie/rust/project_cleaner/src/ui/"),
                    10,
                    Size::from_mb(12),
                    vec![LangData::PYTHON],
                ),
                FolderData::new(
                    PathBuf::from("/mnt/dane/Programowanie/rust/project_cleaner/src/ui/"),
                    3,
                    Size::from_kb(14),
                    vec![LangData::RUST, LangData::GIT],
                ),
                FolderData::new(
                    PathBuf::from("/mnt/dane/Programowanie/rust/project_cleaner/src/ui/"),
                    7,
                    Size::from_gb(1.2),
                    vec![],
                ),
                FolderData::new(
                    PathBuf::from("/mnt/dane/Programowanie/rust/project_cleaner/src/ui/"),
                    2,
                    Size::from_mb(343),
                    vec![LangData::GIT],
                ),
                FolderData::new(
                    PathBuf::from("/mnt/dane/Programowanie/rust/project_cleaner/src/ui/"),
                    8,
                    Size::from_mb(2148),
                    vec![],
                ),
            ],
        }
    }

    pub fn to_rows(&self) -> Vec<Row> {
        self.data
            .iter()
            .map(|ele| {
                let icons = ele
                    .langs
                    .iter()
                    .map(|e| e.icon.to_owned())
                    .collect::<Vec<String>>()
                    .join(" ");

                Row::new(vec![
                    Cell::new(icons),
                    Cell::new(ele.path.display().to_string()),
                    Cell::new(format!("{}", ele.size)),
                    Cell::new(format!("{}", ele.rating)),
                ])
            })
            .collect()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            table: TableData::default(),
            throbber_state: ThrobberState::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.throbber_state.calc_next();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn list_up(&mut self) {
        self.table.state.select(
            self.table
                .state
                .selected()
                .map(|e| if e == 0 { 0 } else { e - 1 }),
        );
    }

    pub fn list_down(&mut self) {
        self.table
            .state
            .select(self.table.state.selected().map(|e| {
                if e >= self.table.data.len() - 1 {
                    self.table.data.len() - 1
                } else {
                    e + 1
                }
            }));
    }
}
