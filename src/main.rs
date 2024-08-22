use clap::Parser;
use project_cleaner::{args::Args, log::initialize_logging, ui::run};

fn main() {
    let _ = initialize_logging();
    let args = Args::parse();
    let _ = run(args);
}
