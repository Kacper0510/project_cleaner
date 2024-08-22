use std::{
    fs::{metadata, remove_dir_all, remove_file},
    path::PathBuf,
    thread::{self, available_parallelism, JoinHandle},
};

use tracing::{error, info};

pub fn dir_rm_parallel(data: Vec<PathBuf>) -> Vec<JoinHandle<()>> {
    let thread_count = available_parallelism().map(|x| x.get()).unwrap_or(4);
    info!("Running dir rm with {} threads.", thread_count);
    let chunks: Vec<_> = data.chunks(thread_count).map(|s| s.to_vec()).collect();

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
