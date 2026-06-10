use clap::builder::styling::{AnsiColor, Styles};
use clap::{ColorChoice, Parser, Subcommand};
use std::path::PathBuf;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().bold())
    .usage(AnsiColor::Green.on_default().bold())
    .literal(AnsiColor::White.on_default().bold())
    .placeholder(AnsiColor::White.on_default().bold());

#[derive(Parser)]
#[command(name = "Kindle Notes Parser")]
#[command(version, about, author)]
#[command(color = ColorChoice::Auto)]
#[command(styles = STYLES)]
#[command(arg_required_else_help = true)]
pub struct KindleCLI {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show all available books in the file
    #[command(
        short_flag = 's',
        override_usage = "knp {show|-s} {--input-path-notes|-i} <INPUT_PATH_NOTES>"
    )]
    Show {
        /// Path to the Kindle 'My Clippings.txt' file
        #[arg(short = 'i', long)]
        input_path_notes: PathBuf,
    },

    /// Parse the txt file given the directory of that file
    #[command(
        short_flag = 'p',
        override_usage = "knp {parser|-p} {--input-path-notes|-i} <INPUT_PATH_NOTES> [{--output-path-notes|-o} <OUTPUT_PATH_NOTES>] [{--export-all-notes|-a}]"
    )]
    Parser {
        /// Path to the Kindle 'My Clippings.txt' file
        #[arg(short = 'i', long)]
        input_path_notes: PathBuf,

        /// Path where the parsed notes will be saved
        #[arg(short = 'o', long, required_unless_present = "export_all_notes")]
        output_path_notes: Option<PathBuf>,

        /// Export all notes to individual files in an outputs/ directory
        #[arg(short = 'a', long)]
        export_all_notes: bool,
    },

    /// Delete all notes for a given title from the directory containing that file
    #[command(
        short_flag = 'd',
        override_usage = "knp {delete|-d} {--input-path-notes|-i} <INPUT_PATH_NOTES> {--output-path-notes|-o} <OUTPUT_PATH_NOTES>"
    )]
    Delete {
        /// Path to the Kindle 'My Clippings.txt' file
        #[arg(short = 'i', long)]
        input_path_notes: PathBuf,

        /// Path to the output directory/file
        #[arg(short = 'o', long)]
        output_path_notes: PathBuf,
    },
}
