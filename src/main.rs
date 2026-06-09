mod cli;
mod handlers;
mod utils;
use clap::Parser;
use cli::structure::{Commands, KindleCLI};
use handlers::terminal::set_ctrlc_handler;
use std::path::PathBuf;
use utils::utils::{
    delete_content, extract_book_titles, get_content, read_file_notes, save_content,
    select_book_title_index, show_all_books,
};

fn process_input_file(input_path_notes: &PathBuf) -> (String, Vec<String>) {
    let notes_content: String = read_file_notes(input_path_notes);
    let available_titles: Vec<String> = extract_book_titles(&notes_content);
    (notes_content, available_titles)
}

fn load_and_select_note_title(input_path_notes: &PathBuf) -> (String, String) {
    let (notes_content, available_titles) = process_input_file(&input_path_notes);
    let selected_title = select_book_title_index(&available_titles);
    (notes_content, selected_title)
}

fn process_commands() {
    let cli = KindleCLI::parse();

    match cli.command {
        Commands::Delete { input_path_notes, output_path_notes } => {
            let (notes_content, selected_title): (String, String) =
                load_and_select_note_title(&input_path_notes);
            let cleaned_content: Vec<String> =
                delete_content(&notes_content, &selected_title.as_str());
            save_content(cleaned_content, output_path_notes.to_str().unwrap());
        },
        Commands::Parser { input_path_notes, output_path_notes } => {
            let (notes_content, selected_title): (String, String) =
                load_and_select_note_title(&input_path_notes);
            let selected_title_content: Vec<String> =
                get_content(&notes_content, &selected_title.as_str());
            save_content(selected_title_content, output_path_notes.to_str().unwrap());
        },
        Commands::Show { input_path_notes } => {
            let (_, available_titles): (_, Vec<String>) =
                process_input_file(&input_path_notes);
            show_all_books(&available_titles);
        },
    }
}

fn main() {
    set_ctrlc_handler();
    process_commands();
}
