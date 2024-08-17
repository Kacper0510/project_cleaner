use clap::Parser;
use project_cleaner::{args::*, ui, simple};

fn main() -> AppResult<()> {
    let args = Args::parse();
    if args.no_ui {
        simple::run(args);
    } else {
        ui::run(args)?;
    }
    Ok(())
}
