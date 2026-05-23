use clap::Parser;
use std::path::PathBuf;
mod cli;
mod utils;

fn process_input_args(input_args: &Vec<PathBuf>) -> (PathBuf, Option<&str>) {
    // The input could have one or two arguments
    // The input path for the notes
    let input_notes_path: &PathBuf = &input_args[0];

    // The output path (Optional -> Some) where we want to save the result
    let output_notes_path: Option<&str> = {
        if input_args.len() == 2 {
            let output_notes_path = &input_args[1];

            // We convert &PathBuf to a string using to_str(), but that returns an
            // Option<&str>, which we can unwrap using unwrap(), resulting in &str
            output_notes_path.to_str()
        } else {
            None
        }
    };

    return (input_notes_path.to_owned(), output_notes_path);
}

fn process_input_file(input_notes_path: &PathBuf) -> (String, Vec<String>) {
    // We obtain the content of the notes from the notes input
    let notes_content: String = utils::utils::read_file_notes(input_notes_path);

    // Now we obtain the available titles from the notes
    let available_titles: Vec<String> =
        utils::utils::extract_book_titles(&notes_content);

    return (notes_content, available_titles);
}

fn process_parser_option(
    notes_content: &String,
    selected_title: &String,
    output_notes_path: &str,
) {
    // Select the title to extract the information from the notes
    let selected_title_content =
        utils::utils::get_content(&notes_content, &selected_title.as_str());

    // Save the selected content
    utils::utils::save_content(selected_title_content, output_notes_path);
}

fn process_delete_option(
    notes_content: &String,
    selected_title: &String,
    output_notes_path: &str,
) {
    // Delete the notes for the selected title
    let cleaned_content: Vec<String> =
        utils::utils::delete_content(&notes_content, &selected_title.as_str());

    // Save the rest of the content that does not match the content of the
    // selected title
    utils::utils::save_content(cleaned_content, output_notes_path);
}

fn process_show_option(available_titles: &Vec<String>) {
    utils::utils::show_all_books(&available_titles);
}

fn main() {
    let cli_input = cli::structure::KindleCLI::parse();

    if let Some(input_args) = cli_input
        .delete
        .as_ref()
        .or(cli_input.parser.as_ref())
        .or(cli_input.show.as_ref())
    {
        let (input_notes_path, output_notes_path) = process_input_args(&input_args);

        let (notes_content, available_titles) = process_input_file(&input_notes_path);

        if input_args.len() == 2 {
            // Here we need the `output_notes_path`
            let output_notes_path = output_notes_path.unwrap();

            // Users can now choose from the available titles
            let selected_title =
                utils::utils::select_book_title_index(&available_titles);

            // Now we apply andifferent process for each option
            if let Some(_) = cli_input.parser.as_ref() {
                process_parser_option(
                    &notes_content,
                    &selected_title,
                    &output_notes_path,
                );
            } else if let Some(_) = cli_input.delete.as_ref() {
                process_delete_option(
                    &notes_content,
                    &selected_title,
                    &output_notes_path,
                );
            }
        } else if let Some(_) = cli_input.show.as_ref() {
            process_show_option(&available_titles);
        }
    }
}
