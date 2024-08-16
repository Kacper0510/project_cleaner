use std::{
    os::unix::fs::MetadataExt,
    path::PathBuf,
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
    time::SystemTime,
};

use jwalk::WalkDir;
use size::Size;

#[derive(Debug, Clone, Copy, Default)]
pub struct DirStats {
    pub size: Option<Size>,
    pub last_mod: Option<SystemTime>,
}

impl DirStats {
    pub fn new(path: PathBuf) -> Self {
        let files_iter = WalkDir::new(path)
            .try_into_iter()
            .ok()
            .map(|list| list.filter_map(|ele| ele.ok()).filter(|f| f.metadata().is_ok_and(|f| f.is_file())));

        if let Some(files_iter) = files_iter {
            let mut max_value: Option<SystemTime> = None;
            let mut sum_value: Option<u64> = None;
            for file in files_iter {
                let last_mod = file.metadata().ok().and_then(|m| m.modified().ok());
                let size = file.metadata().ok().map(|m| m.size()); // TODO: fix to be correct size

                if let Some(size) = size {
                    sum_value = Some(if let Some(v) = sum_value { v + size } else { size })
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
    const THREADS_COUNT: usize = 4; // TODO: Maybe calculate this number?
    let chunks: Vec<_> = data.chunks(THREADS_COUNT).map(|s| s.to_vec()).collect();

    chunks
        .into_iter()
        .map(|chunk| {
            let tx = tx.clone();
            thread::spawn(move || {
                for (i, ele) in chunk {
                    let _ = tx.send((i, DirStats::new(ele)));
                }
            })
        })
        .collect()
}
