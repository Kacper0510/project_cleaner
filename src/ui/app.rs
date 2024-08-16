use std::error;

use ratatui::widgets::{Cell, Row, TableState};
use size::Size;
use throbber_widgets_tui::ThrobberState;

use crate::{
    args::Args,
    core::{LangData, MatchData},
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub args: Args, // TODO: use args
    pub running: bool,
    pub table: TableData,
    pub throbber_state: ThrobberState,
}
#[derive(Debug, Clone, Default)]
pub struct TableData {
    pub state: TableState,
    pub data: Vec<MatchDataUI>,
}

#[derive(Debug, Clone)]
pub struct MatchDataUI {
    pub data: MatchData,
    pub size: Option<Size>,
    pub status: MatchDataUIStatus,
}

#[derive(Debug, Clone)]
enum MatchDataUIStatus {
    FOUND,
    DELETED,
}

impl TableData {
    pub fn to_rows(&self) -> Vec<Row> {
        self.data
            .iter()
            .map(|ele| {
                let icons = ele.data.reasons.iter().map(|e| e.icon.to_owned()).collect::<Vec<String>>().join(" ");

                Row::new(vec![
                    Cell::new(icons),
                    Cell::new(ele.data.path.display().to_string()),
                    Cell::new(if let Some(s) = ele.size { format!("{}", s) } else { "---".to_owned() }),
                    Cell::new("lol"),
                ])
            })
            .collect()
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(args: Args) -> Self {
        Self {
            args,
            running: true,
            table: TableData::default(),
            throbber_state: ThrobberState::default(),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.throbber_state.calc_next();

        // TODO: read channel
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn list_up(&mut self) {
        if self.table.data.is_empty() {
            self.table.state.select(None);
            return;
        }

        self.table.state.select(self.table.state.selected().map(|e| if e == 0 { 0 } else { e - 1 }));
    }

    pub fn list_down(&mut self) {
        if self.table.data.is_empty() {
            self.table.state.select(None);
            return;
        }

        self.table.state.select(self.table.state.selected().map(|e| {
            if e >= self.table.data.len() - 1 {
                self.table.data.len() - 1
            } else {
                e + 1
            }
        }));
    }
}
