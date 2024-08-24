use super::{
    model::TableData,
    popup::{DeletePopUpKind, PopUpKind, PopUpState},
};
use crate::{
    args::Args,
    core::{dir_rm_parallel, dir_stats_parallel, DirStats, MatchData},
    walk_directories,
};
use std::{
    env, error,
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};
use throbber_widgets_tui::ThrobberState;
use tracing::info;

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
    pub popup_state: PopUpState,

    pub dir_stats_channel: Channel<(usize, DirStats)>,
    pub walker_channel: Channel<MatchData>,
    pub handle: Vec<JoinHandle<()>>,
    pub del_handle: Vec<JoinHandle<()>>,

    pub info_path: Option<PathBuf>,
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
            popup_state: PopUpState::Closed,
            dir_stats_channel: std::sync::mpsc::channel(),
            walker_channel: std::sync::mpsc::channel(),
            handle: vec![],
            del_handle: vec![],
            info_path: None,
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
            info!("UI got new match path: {:?}", data.path);
            if !data.hidden() || self.args.dangerous {
                self.table.add_match(data);
            }
        }

        let mut updated = false;
        while let Ok((idx, data)) = self.dir_stats_channel.1.try_recv() {
            info!("UI got dir stats (idx={idx})");
            let res = self.table.update_match(idx, data);
            updated = updated || res;
        }
        if updated {
            self.table.sort();
        }

        if self.handle.iter().all(|h| h.is_finished()) {
            self.handle = vec![];
            self.state = match self.state {
                AppState::Scanning => {
                    self.handle = dir_stats_parallel(
                        self.table
                            .data
                            .clone()
                            .into_iter()
                            .flat_map(|ele| ele.matches)
                            .map(|ele| (ele.idx, ele.path))
                            .collect(),
                        self.dir_stats_channel.0.clone(),
                    );
                    AppState::Calculating
                },
                AppState::Done | AppState::Calculating => AppState::Done,
            }
        }

        if self.popup_state == PopUpState::Open(PopUpKind::Delete(DeletePopUpKind::Deleting))
            && self.del_handle.iter().all(|h| h.is_finished())
        {
            self.popup_state = PopUpState::Closed;
            self.del_handle = vec![];
            self.reload();
        }
    }

    pub fn quit(&mut self) {
        if self.table.is_any_selected() {
            self.popup_state = PopUpState::Open(PopUpKind::Exit);
        } else {
            self.running = false;
        }
    }

    pub fn force_quit(&mut self) {
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
        self.table = TableData::default();
        self.popup_state = PopUpState::Closed;
        self.del_handle = vec![];
        self.run();
    }

    pub fn show_info(&mut self) {
        if let Some(selected) = self.table.state.selected() {
            self.popup_state = PopUpState::Open(PopUpKind::Info);
            self.info_path = Some(self.table.data[selected].group_path.clone());
        }
    }

    pub fn hide_info(&mut self) {
        self.popup_state = PopUpState::Closed;
        self.info_path = None;
    }

    pub fn is_highlighted(&self) -> bool {
        self.table.state.selected().is_some()
    }

    pub fn toggle_select(&mut self) {
        self.table.toggle_select();
        self.list_down();
    }

    pub fn delete(&mut self) {
        if self.table.is_any_selected() {
            self.popup_state = PopUpState::Open(PopUpKind::Delete(DeletePopUpKind::Confirm));
        }
    }

    pub fn confirm_delete(&mut self) {
        self.del_handle = dir_rm_parallel(self.table.get_selected_path());
        self.popup_state = PopUpState::Open(PopUpKind::Delete(DeletePopUpKind::Deleting));
    }
}
