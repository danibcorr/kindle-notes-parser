use clap::{ColorChoice, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Kindle Notes Parser")]
#[command(version, about, author)]
#[command(color = ColorChoice::Auto)]
#[command(arg_required_else_help = true)]
pub struct KindleCLI {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show all available books in the file
    Show {
        /// Path to the Kindle 'My Clippings.txt' file
        #[arg(short, long)]
        input_path_notes: PathBuf,
    },

    /// Parse the txt file given the directory of that file
    Parser {
        /// Path to the Kindle 'My Clippings.txt' file
        #[arg(short, long)]
        input_path_notes: PathBuf,

        /// Path where the parsed notes will be saved
        #[arg(short, long)]
        output_path_notes: PathBuf,
    },

    /// Delete all notes for a given title from the directory containing that file
    Delete {
        /// Path to the Kindle 'My Clippings.txt' file
        #[arg(short, long)]
        input_path_notes: PathBuf,

        /// Path to the output directory/file
        #[arg(short, long)]
        output_path_notes: PathBuf,
    },
}
