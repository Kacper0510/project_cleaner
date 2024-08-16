use super::model::TableData;
use crate::{args::Args, core::MatchData, walk_directories};
use std::{env, error, sync::mpsc::Receiver};
use throbber_widgets_tui::ThrobberState;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]

pub enum AppState {
    Scanning,
    Done,
}

#[derive(Debug)]
pub struct App {
    pub args: Args, // TODO: use args
    pub running: bool,
    pub table: TableData,
    pub throbber_state: ThrobberState,
    pub state: AppState,
    pub receiver: Receiver<MatchData>,
    pub handle: std::thread::JoinHandle<()>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(args: Args) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        let path = args.path.clone().unwrap_or(env::current_dir().unwrap());

        let handle = std::thread::spawn(move || walk_directories(&path, sender, |_path| {}));

        Self {
            args,
            running: true,
            table: TableData::default(),
            throbber_state: ThrobberState::default(),
            state: AppState::Scanning,
            receiver,
            handle,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.throbber_state.calc_next();

        while let Ok(data) = self.receiver.try_recv() {
            self.table.add_match(data);
        }

        if self.handle.is_finished() && self.state == AppState::Scanning {
            self.state = AppState::Done;
        }
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
