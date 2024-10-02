use std::{
    cmp::max,
    fs::{metadata, remove_dir_all, remove_file},
    path::PathBuf,
    thread::{self, available_parallelism, JoinHandle},
};

use tracing::{error, info, trace};

use crate::core::{DEFAULT_THREAD_COUNT, _CORE_MULTIPLIER};

pub fn dir_rm_parallel(data: Vec<PathBuf>) -> Vec<JoinHandle<()>> {
    let thread_count = available_parallelism().map(|x| x.get()).unwrap_or(DEFAULT_THREAD_COUNT) / _CORE_MULTIPLIER;
    info!("Running dir rm with {} threads.", thread_count);
    let chunk_size = max(data.len() / thread_count, data.len());
    let chunks: Vec<_> = data.chunks(chunk_size).map(|s| s.to_vec()).collect();
    trace!("Chunks: {:?}", chunks.iter().map(|c| c.len()).collect::<Vec<_>>());

    chunks
        .into_iter()
        .map(|chunk| {
            thread::spawn(move || {
                for ele in chunk {
                    if let Ok(data) = metadata(ele.clone()) {
                        if data.is_dir() {
                            if remove_dir_all(&ele).is_err() {
                                error!("Failed to remove {:?} as dir.", ele);
                            }
                        } else if remove_file(&ele).is_err() {
                            error!("Failed to remove {:?} as file.", ele);
                        };
                    }
                }
            })
        })
        .collect()
}
