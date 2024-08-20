use std::{
    env::current_dir,
    io::{stdin, stdout, Write},
};

use crate::core::MatchData;

pub fn run(args: super::args::Args) {
    let directory = args.path.unwrap_or_else(|| {
        current_dir().expect("Cannot access current directory, run with -p <DIR> to select another")
    });
    let (sender, receiver) = std::sync::mpsc::channel::<MatchData>();

    let collector = move || {
        let mut collected = vec![];
        while let Ok(data) = receiver.recv() {
            println!("{}", data.path.display());
            for reason in data.languages() {
                println!("-> {}", reason);
            }
            collected.push(data.path);
        }
        collected
    };
    let handle = std::thread::spawn(collector);

    println!("Searching for files and directories to delete...");
    crate::walk_directories(&directory, sender, |progress| {
        if let Err(error) = progress {
            if let Some(path) = error.path() {
                println!("Failed to read {} ({})", path.display(), error);
            }
        }
    });
    let results = handle.join().unwrap();
    if results.is_empty() {
        println!("Found nothing, exiting...");
        return;
    }

    let confirmed = args.delete_instantly || {
        let form = if results.len() == 1 { "directory" } else { "directories" };
        println!("Do you want to permanently delete the {} {} listed above?", results.len(), form);
        print!("WARNING: this action is irreversible! [y/N] ");
        let _ = stdout().flush();
        let mut buffer = String::new();
        if stdin().read_line(&mut buffer).is_err() {
            false
        } else {
            buffer.trim().to_lowercase() == "y"
        }
    };
    if !confirmed {
        println!("Aborting...");
        return;
    }

    for path in results {
        let result = if path.is_dir() { std::fs::remove_dir_all(&path) } else { std::fs::remove_file(&path) };
        if let Err(error) = result {
            println!("Failed to delete {} ({})", path.display(), error);
        } else {
            println!("Deleted {}", path.display());
        }
    }
}
