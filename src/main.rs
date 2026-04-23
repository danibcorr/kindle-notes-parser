use clap::Parser;
use std::collections::HashSet;
mod cli;
mod utils;

fn main() {
    let cli_input = cli::structure::KindleCLI::parse();

    if let Some(paths) = cli_input.parser.as_ref() {
        let input_path_notes = &paths[0];
        let output_path_notes = &paths[1];
        let output_path_notes = output_path_notes.to_str().unwrap();

        let notes_content: String = utils::utils::read_file_notes(input_path_notes);
        let available_titles: HashSet<String> =
            utils::utils::extract_book_titles(&notes_content);
        let selected_title = utils::utils::select_book_title_index(available_titles);
        let selected_title_content =
            utils::utils::get_content(&notes_content, &selected_title.as_str());
        utils::utils::save_content(selected_title_content, output_path_notes);
    } else if let Some(paths) = cli_input.delete.as_ref() {
        let input_path_notes = &paths[0];
        let output_path_notes = &paths[1];
        let output_path_notes = output_path_notes.to_str().unwrap();

        let notes_content: String = utils::utils::read_file_notes(input_path_notes);
        let available_titles: HashSet<String> =
            utils::utils::extract_book_titles(&notes_content);
        let selected_title = utils::utils::select_book_title_index(available_titles);
        let cleaned_content: Vec<String> =
            utils::utils::delete_content(&notes_content, &selected_title.as_str());
        println!("{}", input_path_notes.to_str().unwrap());
        utils::utils::save_content(cleaned_content, output_path_notes);
    }
}
