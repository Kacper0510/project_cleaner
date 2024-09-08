use crate::{core::MatchData, Scanner};
use std::{
    env::current_dir,
    io::{stdin, stdout, Write},
};

pub fn run(args: super::args::Args) {
    let directory = args.path.unwrap_or_else(|| {
        current_dir().expect("Cannot access current directory, run with -p <DIR> to select another")
    });
    let (sender, receiver) = std::sync::mpsc::channel::<MatchData>();

    let collector = move || {
        let mut collected = vec![];
        while let Ok(data) = receiver.recv() {
            println!("{}{}", if data.dangerous() { "(Dangerous!) " } else { "" }, data.path.display());
            for language in data.languages() {
                println!("\t-> {}", language);
            }
            collected.push(data.path);
        }
        collected
    };
    let handle = std::thread::spawn(collector);

    println!("Searching for files and directories to delete...");
    let mut scanner = Scanner::new(&directory, sender);
    scanner.dangerous = args.dangerous;
    scanner.scan_with_progress().for_each(|progress| {
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
        let form = if results.len() == 1 { "path" } else { "paths" };
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
