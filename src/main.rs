use clap::Parser;
use project_cleaner::{args::Args, ui::run};

fn main() {
    let args = Args::parse();
    let _ = run(args);
}
