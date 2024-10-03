use file_id::FileId;
use jwalk::WalkDir;
use size::Size;
use std::{
    cmp::{max, Ordering},
    collections::HashSet,
    iter::Sum,
    ops::Add,
    path::PathBuf,
    sync::mpsc::Sender,
    thread::{self, available_parallelism, JoinHandle},
    time::SystemTime,
};
use tracing::{debug, error, info, trace};

use crate::core::{DEFAULT_THREAD_COUNT, _CORE_MULTIPLIER};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DirStats {
    pub size: Option<Size>,
    pub last_mod: Option<SystemTime>,
}

impl Ord for DirStats {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.size.cmp(&other.size) {
            Ordering::Equal => self.last_mod.cmp(&other.last_mod),
            x => x.reverse(),
        }
    }
}

impl PartialOrd for DirStats {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for DirStats {
    type Output = DirStats;

    fn add(self, rhs: Self) -> Self::Output {
        let size = match (self.size, rhs.size) {
            (Some(a), Some(b)) => Some(a + b),
            (a, b) => a.or(b),
        };

        let last_mod = [self.last_mod, rhs.last_mod].iter().flatten().max().copied();
        DirStats { size, last_mod }
    }
}

impl Sum for DirStats {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(DirStats::default(), |prev, current| current + prev)
    }
}

impl DirStats {
    pub fn new(path: PathBuf) -> Self {
        debug!("Calculating stats for {:?}", path);
        let files_iter = WalkDir::new(path.clone())
            .parallelism(jwalk::Parallelism::Serial)
            .skip_hidden(false)
            .follow_links(false)
            .try_into_iter()
            .map_err(|err| error!("WalkDir into iter error: {:?}", err))
            .ok()
            .map(|list| list.filter_map(|ele| ele.ok()).filter(|f| f.metadata().is_ok_and(|f| !f.is_symlink())));

        if let Some(files_iter) = files_iter {
            let mut max_value: Option<SystemTime> = None;
            let mut sum_value: Option<u64> = None;
            let mut visited: HashSet<FileId> = HashSet::default();
            for file in files_iter {
                let last_mod = file.metadata().ok().and_then(|m| m.modified().ok());
                let size = file.metadata().ok().map(|m| m.len());
                if visited.insert(file_id::get_file_id(file.path()).unwrap()) {
                    // inode ids needs to be unique
                    if let Some(size) = size {
                        sum_value = Some(if let Some(v) = sum_value { v + size } else { size })
                    }
                }

                max_value = if max_value < last_mod { last_mod } else { max_value };
            }

            debug!("Stats for {:?}: size: {:?}, last_mod: {:?}", path, sum_value, max_value);
            Self { size: sum_value.map(Size::from_bytes), last_mod: max_value }
        } else {
            error!("Got empty iterator for {:?}", path);
            Self { size: None, last_mod: None }
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
    let thread_count = available_parallelism().map(|x| x.get()).unwrap_or(DEFAULT_THREAD_COUNT) / _CORE_MULTIPLIER;
    info!("Running dir stats with {} threads.", thread_count);
    let chunk_size = max(data.len() / thread_count, data.len());
    let chunks: Vec<_> = data.chunks(chunk_size).map(|s| s.to_vec()).collect();
    trace!("Chunks: {:?}", chunks.iter().map(|c| c.len()).collect::<Vec<_>>());

    chunks
        .into_iter()
        .map(|chunk| {
            let tx = tx.clone();
            thread::spawn(move || {
                for (i, ele) in chunk {
                    let stats = DirStats::new(ele);
                    let res = tx.send((i, stats));
                    if res.is_err() {
                        error!("Failed to send");
                    }
                }
            })
        })
        .collect()
}
