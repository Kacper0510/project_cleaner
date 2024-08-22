use clap::Parser;
use project_cleaner::{args::{Args, AppResult}, log::initialize_logging, ui, simple};

fn main() -> AppResult<()> {
    let _ = initialize_logging();
    let args = Args::parse();
    if args.no_ui {
        simple::run(args);
    } else {
        ui::run(args)?;
    }
    Ok(())
}
