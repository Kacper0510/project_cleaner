use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
    /// Disable usage of Nerd Fonts icons
    #[arg(long)]
    pub no_icons: bool,
}
