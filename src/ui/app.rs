use super::model::TableData;
use crate::{
    args::Args,
    core::{
        dir_stats::{dir_stats_parallel, DirStats},
        MatchData,
    },
    walk_directories,
};
use size::Size;
use std::{
    env, error,
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};
use throbber_widgets_tui::ThrobberState;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]

pub enum AppState {
    Scanning,
    Calculating,
    Done,
}

type Channel<T> = (Sender<T>, Receiver<T>);

#[derive(Debug)]
pub struct App {
    pub args: Args,
    pub running: bool,
    pub table: TableData,
    pub throbber_state: ThrobberState,
    pub state: AppState,
    pub dir_stats_channel: Channel<(usize, DirStats)>,
    pub walker_channel: Channel<MatchData>,
    pub handle: Vec<JoinHandle<()>>,
    pub cleanable_space: Size,
    pub saved_space: Size,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(args: Args) -> Self {
        Self {
            args,
            running: true,
            table: TableData::default(),
            throbber_state: ThrobberState::default(),
            state: AppState::Scanning,
            dir_stats_channel: std::sync::mpsc::channel(),
            walker_channel: std::sync::mpsc::channel(),
            handle: vec![],
            cleanable_space: Size::from_bytes(0),
            saved_space: Size::from_bytes(0),
        }
    }

    pub fn run(&mut self) {
        self.state = AppState::Scanning;
        self.handle = vec![];
        let path = self.args.path.clone().unwrap_or(env::current_dir().unwrap());

        let tx = self.walker_channel.0.clone();
        let handle = std::thread::spawn(move || walk_directories(&path, tx, |_path| {}));
        self.handle.push(handle);
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.throbber_state.calc_next();

        while let Ok(data) = self.walker_channel.1.try_recv() {
            self.table.add_match(data);
        }

        while let Ok((idx, data)) = self.dir_stats_channel.1.try_recv() {
            self.table.data[idx].dir_stats = data;
            if let Some(size) = data.size {
                self.cleanable_space += size;
            }
        }

        if self.handle.iter().all(|h| h.is_finished()) {
            self.handle = vec![];
            self.state = match self.state {
                AppState::Scanning => {
                    self.handle = dir_stats_parallel(
                        self.table.data.clone().into_iter().map(|ele| ele.data.path).enumerate().collect(),
                        self.dir_stats_channel.0.clone(),
                    );
                    AppState::Calculating
                },
                AppState::Done | AppState::Calculating => AppState::Done,
            }
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

    pub fn reload(&mut self) {
        self.cleanable_space = Size::from_bytes(0);
        self.saved_space = Size::from_bytes(0);
        self.table = TableData::default();
        self.run();
    }

    pub fn delete(&mut self) {}
}
