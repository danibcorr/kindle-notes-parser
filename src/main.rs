mod cli;
mod handlers;
mod utils;

use clap::Parser;
use cli::structure::{Commands, KindleCLI};
use handlers::terminal::set_ctrlc_handler;
use std::path::PathBuf;
use std::thread;
use utils::utils::{
    delete_content, extract_book_titles, get_all_content, get_content, read_file_notes,
    save_content, select_book_title_index, show_all_books,
};

fn main() {
    set_ctrlc_handler();
    let cli = KindleCLI::parse();

    match cli.command {
        Commands::Show { input_path_notes } => {
            let content = read_file_notes(&input_path_notes);
            show_all_books(&extract_book_titles(&content));
        },
        Commands::Parser { input_path_notes, output_path_notes, export_all_notes } => {
            let content = read_file_notes(&input_path_notes);
            let titles = extract_book_titles(&content);

            if export_all_notes {
                let all_notes = get_all_content(&content);

                let handles = all_notes
                    .into_iter()
                    .map(|(title, notes)| {
                        thread::spawn(move || {
                            save_content(
                                &title,
                                &notes,
                                &PathBuf::from(format!("outputs/{}.txt", title)),
                            );
                        })
                    })
                    .collect::<Vec<_>>();

                for handle in handles {
                    handle.join().unwrap();
                }
            } else {
                let title = select_book_title_index(&titles);
                let notes = get_content(&content, &title);
                save_content(&title, &notes, &output_path_notes.unwrap());
            }
        },
        Commands::Delete { input_path_notes, output_path_notes } => {
            let content = read_file_notes(&input_path_notes);
            let titles = extract_book_titles(&content);
            let title = select_book_title_index(&titles);
            save_content(&title, &delete_content(&content, &title), &output_path_notes);
        },
    }
}
