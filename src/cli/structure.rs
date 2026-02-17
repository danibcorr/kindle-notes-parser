use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Kindle Notes Parser")]
#[command(version, about)]
pub struct KindleCLI {
    /// Parse the txt file given the directory of that file
    #[arg(short, long, value_name = "FILE PATH")]
    pub(crate) parser: Option<PathBuf>,
}
