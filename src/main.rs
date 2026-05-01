use clap::Parser;
mod cli;
mod utils;

fn main() {
    let cli_input = cli::structure::KindleCLI::parse();

    if let (Some(input_args), _) | (_, Some(input_args)) =
        (cli_input.delete.as_ref(), cli_input.parser.as_ref())
    {
        // The input consists of two arguments: the input path for the notes
        // and the output path where we want to save the result
        let input_notes_path = &input_args[0];
        let output_notes_path = &input_args[1];

        // We convert &PathBuf to a string using to_str(), but that returns an
        // Option<&str>, which we can unwrap using unwrap(), resulting in &str
        let output_notes_path = output_notes_path.to_str().unwrap();

        // We obtain the content of the notes from the notes input
        let notes_content: String = utils::utils::read_file_notes(input_notes_path);

        // Now we obtain the available titles from the notes
        let available_titles: Vec<String> =
            utils::utils::extract_book_titles(&notes_content);

        // Users can now choose from the available titles
        let selected_title = utils::utils::select_book_title_index(available_titles);

        // Now we apply andifferent process for each option
        if let Some(_) = cli_input.parser.as_ref() {
            // Select the title to extract the information from the notes
            let selected_title_content =
                utils::utils::get_content(&notes_content, &selected_title.as_str());

            // Save the selected content
            utils::utils::save_content(selected_title_content, output_notes_path);
        } else if let Some(_) = cli_input.delete.as_ref() {
            // Delete the notes for the selected title
            let cleaned_content: Vec<String> =
                utils::utils::delete_content(&notes_content, &selected_title.as_str());

            // Save the rest of the content that does not match the content of the
            // selected title
            utils::utils::save_content(cleaned_content, output_notes_path);
        }
    }
}
