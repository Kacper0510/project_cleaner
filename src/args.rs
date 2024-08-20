use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
    /// Disable usage of Nerd Fonts icons
    #[arg(long)]
    pub no_icons: bool,

    #[arg(short,long,value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    pub path: Option<std::path::PathBuf>,
}
