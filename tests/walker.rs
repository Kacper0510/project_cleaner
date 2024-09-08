use project_cleaner::Scanner;

#[test]
fn current_project_directory_walk() {
    let directory = std::env::current_dir().unwrap();
    println!("Root: {}", directory.display());
    let (sender, receiver) = std::sync::mpsc::channel();

    let collector = move || {
        let mut counter = 0;
        while let Ok(data) = receiver.recv() {
            counter += 1;
            println!("Found match: {:#?}", data);
        }
        assert!(counter > 0, "target directory wasn't detected");
    };
    let handle = std::thread::spawn(collector);

    Scanner::new(&directory, sender).scan_with_progress().for_each(|path| {
        let path = path.unwrap();
        println!("Scanning {}", path.display());
        assert!(!path.ends_with("target/debug"), "target directory wasn't skipped");
    });
    handle.join().unwrap();
}
