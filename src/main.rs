use clap::Parser;
use std::collections::HashSet;
mod cli;
mod utils;

fn main() {
    let cli_input = cli::structure::KindleCLI::parse();

    if let Some(path_notas) = cli_input.parser.as_ref() {
        let notes_content: String = utils::utils::read_file_notes(path_notas);
        let available_titles: HashSet<String> =
            utils::utils::extract_book_titles(&notes_content);
        let selected_title = utils::utils::select_book_title_index(available_titles);
        let selected_title_content =
            utils::utils::get_content(&notes_content, &selected_title.as_str());
        utils::utils::save_content(selected_title_content, &selected_title);
    }
}
