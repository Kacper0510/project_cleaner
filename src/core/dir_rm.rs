use std::{
    fs::remove_dir_all,
    path::PathBuf,
    thread::{self, available_parallelism, JoinHandle},
};

pub fn dir_rm_parallel(data: Vec<PathBuf>) -> Vec<JoinHandle<()>> {
    let thread_count = available_parallelism().map(|x| x.get()).unwrap_or(4);
    let chunks: Vec<_> = data.chunks(thread_count).map(|s| s.to_vec()).collect();

    chunks
        .into_iter()
        .map(|chunk| {
            thread::spawn(move || {
                for ele in chunk {
                    let _ = remove_dir_all(ele);
                }
            })
        })
        .collect()
}
