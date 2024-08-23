use std::{
    collections::HashSet,
    iter::Sum,
    ops::Add,
    os::linux::fs::MetadataExt,
    path::PathBuf,
    sync::mpsc::Sender,
    thread::{self, available_parallelism, JoinHandle},
    time::SystemTime,
};
use tracing::{error, info};

use jwalk::WalkDir;
use size::Size;

#[derive(Debug, Clone, Copy, Default)]
pub struct DirStats {
    pub size: Option<Size>,
    pub last_mod: Option<SystemTime>,
}

impl Add for DirStats {
    type Output = DirStats;

    fn add(self, rhs: Self) -> Self::Output {
        let size = if let Some(s) = rhs.size {
            Some(if let Some(self_s) = self.size { self_s + s } else { s })
        } else {
            self.size
        };

        let last_mod = [self.last_mod, rhs.last_mod].iter().flatten().max().copied();
        DirStats {
            size,
            last_mod,
        }
    }
}

impl Sum for DirStats {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(DirStats::default(), |prev, current| current + prev)
    }
}

impl DirStats {
    pub fn new(path: PathBuf) -> Self {
        let files_iter = WalkDir::new(path)
            .skip_hidden(false)
            .follow_links(false)
            .try_into_iter()
            .ok()
            .map(|list| list.filter_map(|ele| ele.ok()).filter(|f| f.metadata().is_ok_and(|f| !f.is_symlink())));

        if let Some(files_iter) = files_iter {
            let mut max_value: Option<SystemTime> = None;
            let mut sum_value: Option<u64> = None;
            let mut visited: HashSet<u64> = HashSet::default();
            for file in files_iter {
                let last_mod = file.metadata().ok().and_then(|m| m.modified().ok());
                let size = file.metadata().ok().map(|m| m.len());
                if visited.insert(file.metadata().unwrap().st_ino()) {
                    // inode ids needs to be unique
                    if let Some(size) = size {
                        sum_value = Some(if let Some(v) = sum_value { v + size } else { size })
                    }
                }

                max_value = if max_value < last_mod { last_mod } else { max_value };
            }

            Self {
                size: sum_value.map(Size::from_bytes),
                last_mod: max_value,
            }
        } else {
            Self {
                size: None,
                last_mod: None,
            }
        }
    }

    pub fn last_mod_days(&self) -> Option<u64> {
        let timestamp = self.last_mod?;
        let now = SystemTime::now();
        let dur = now.duration_since(timestamp).ok()?;
        Some(dur.as_secs() / 86400)
    }
}

pub fn dir_stats_parallel(data: Vec<(usize, PathBuf)>, tx: Sender<(usize, DirStats)>) -> Vec<JoinHandle<()>> {
    let thread_count = available_parallelism().map(|x| x.get()).unwrap_or(4);
    info!("Running dir stats with {} threads.", thread_count);
    let chunks: Vec<_> = data.chunks(thread_count).map(|s| s.to_vec()).collect();

    chunks
        .into_iter()
        .map(|chunk| {
            let tx = tx.clone();
            thread::spawn(move || {
                for (i, ele) in chunk {
                    let res = tx.send((i, DirStats::new(ele)));
                    if res.is_err() {
                        error!("Failed to send");
                    }
                }
            })
        })
        .collect()
}
