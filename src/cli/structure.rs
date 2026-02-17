use clap::{ColorChoice, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Kindle Notes Parser")]
#[command(version, about, author)]
#[command(color = ColorChoice::Auto)]
#[command(arg_required_else_help = true)]
pub struct KindleCLI {
    /// Parse the txt file given the directory of that file
    #[arg(short, long, value_name = "FILE PATH")]
    pub(crate) parser: Option<PathBuf>,
}
