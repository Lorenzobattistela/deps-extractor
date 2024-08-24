use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {

    pub lang: String,

    pub file: PathBuf,

    #[arg(short, long, default_value_t = true)]
    pub include_external: bool,

    #[arg(short, long)]
    pub output: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
