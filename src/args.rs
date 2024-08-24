use clap::Parser;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Disable usage of Nerd Fonts icons in interactive mode
    #[arg(long)]
    pub no_icons: bool,
    /// Start scan at a different root path [default: current working directory]
    #[arg(short, long, value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    pub path: Option<std::path::PathBuf>,
    /// Run in simple, non-interactive mode (e.g. for scripting)
    #[arg(short, long)]
    pub no_ui: bool,
    /// Do not ask for confirmation when deleting directories
    #[arg(short = 'y')]
    pub delete_instantly: bool,
    /// Show dangerous paths
    #[arg(long)]
    pub dangerous: bool,
}
