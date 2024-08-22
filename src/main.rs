use clap::Parser;
use project_cleaner::{args::{Args, AppResult}, ui, simple};

fn main() -> AppResult<()> {
    let args = Args::parse();
    if args.no_ui {
        simple::run(args);
    } else {
        ui::run(args)?;
    }
    Ok(())
}
