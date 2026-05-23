use clap::{ColorChoice, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Kindle Notes Parser")]
#[command(version, about, author)]
#[command(color = ColorChoice::Auto)]
#[command(arg_required_else_help = true)]
pub struct KindleCLI {
    /// Show all available books in the file
    #[arg(short, long, num_args = 1, value_names = ["INPUT_FILE_PATH"])]
    pub(crate) show: Option<Vec<PathBuf>>,

    /// Parse the txt file given the directory of that file
    #[arg(short, long, num_args = 2, value_names = ["INPUT_FILE_PATH", "OUTPUT_FILE_PATH"])]
    pub(crate) parser: Option<Vec<PathBuf>>,

    /// Delete all notes for a given title from the directory containing that file
    #[arg(short, long, num_args = 2, value_names = ["INPUT_FILE_PATH", "OUTPUT_FILE_PATH"])]
    pub(crate) delete: Option<Vec<PathBuf>>,
}
