use super::{
    model::TableData,
    popup::{DeletePopUpKind, PopUpKind, PopUpState},
};
use crate::{
    args::Args,
    core::{dir_rm_parallel, dir_stats_parallel, DirStats, MatchData},
    Scanner,
};
use std::{
    env, error,
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};
use throbber_widgets_tui::ThrobberState;
use tracing::info;
use tui_scrollview::ScrollViewState;

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
    pub scanner: Scanner,

    pub table: TableData,
    pub throbber_state: ThrobberState,
    pub scroll_state: ScrollViewState,

    pub state: AppState,
    pub popup_state: PopUpState,

    pub dir_stats_channel: Channel<(usize, DirStats)>,
    pub scanner_receiver: Receiver<MatchData>,
    pub handle: Vec<JoinHandle<()>>,
    pub del_handle: Vec<JoinHandle<()>>,

    pub info_path: Option<PathBuf>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(args: Args) -> Self {
        let root_path = args.path.clone().unwrap_or(env::current_dir().unwrap());
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut scanner = Scanner::new(&root_path, sender);
        scanner.dangerous = args.dangerous;
        Self {
            args,
            running: true,
            scanner,
            table: TableData::default(),
            throbber_state: ThrobberState::default(),
            scroll_state: ScrollViewState::new(),
            state: AppState::Scanning,
            popup_state: PopUpState::Closed,
            dir_stats_channel: std::sync::mpsc::channel(),
            scanner_receiver: receiver,
            handle: vec![],
            del_handle: vec![],
            info_path: None,
        }
    }

    pub fn run(&mut self) {
        self.state = AppState::Scanning;
        self.handle = vec![];

        let scanner = self.scanner.clone();
        let handle = std::thread::spawn(|| scanner.scan());
        self.handle.push(handle);
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.throbber_state.calc_next();

        while let Ok(data) = self.scanner_receiver.try_recv() {
            info!("UI got new match path: {:?}", data.path);
            self.table.add_match(data);
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
                    info!(
                        "UI scan finished with {} matches which created {} records.",
                        self.table.idx - 1,
                        self.table.data.len()
                    );
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
                AppState::Calculating => {
                    info!("UI stats calculation finished.");
                    AppState::Done
                },
                AppState::Done => AppState::Done,
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

    pub fn scroll_up(&mut self) {
        self.scroll_state.scroll_up()
    }

    pub fn scroll_down(&mut self) {
        self.scroll_state.scroll_down()
    }

    pub fn reload(&mut self) {
        self.table = TableData::default();
        self.popup_state = PopUpState::Closed;
        self.del_handle = vec![];
        self.run();
    }

    pub fn focus_sort(&mut self) {
        self.popup_state = PopUpState::Open(PopUpKind::Sort);
    }

    pub fn show_info(&mut self) {
        if let Some(selected) = self.table.state.selected() {
            self.popup_state = PopUpState::Open(PopUpKind::Info);
            self.info_path = Some(self.table.data[selected].group_path.clone());
        }
    }

    pub fn close_popup(&mut self) {
        match &self.popup_state {
            PopUpState::Open(kind) => match kind {
                PopUpKind::Info => {
                    self.scroll_state.scroll_to_top();
                    self.info_path = None;
                },
                PopUpKind::Sort => {},
                PopUpKind::Delete(_) | PopUpKind::Exit => {},
            },
            PopUpState::Closed => {},
        }
        self.popup_state = PopUpState::Closed;
    }

    pub fn sort_left(&mut self) {
        self.table.sort_left();
    }

    pub fn sort_right(&mut self) {
        self.table.sort_right();
    }

    pub fn sort_toggle(&mut self) {
        self.table.sort_toggle();
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
