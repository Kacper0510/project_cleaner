use clap::Parser;
use project_cleaner::args::Args;

fn main() {
    let args = Args::parse();
    let _ = project_cleaner::ui::run(args);
}
