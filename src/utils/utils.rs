use crate::utils::constants::{
    MAX_TITLE_LENGTH, NUM_LINES_CLIPPING_FORMAT, STARTING_INDEX_CONTENT,
};
use crate::utils::processing::{clean_content, delete_duplicates};
use crate::utils::terminal::terminal_processing;
use console::style;
use std::collections::HashSet;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::Path;

pub fn read_file_notes(notes_path: &Path) -> String {
    match read_to_string(notes_path) {
        Ok(content) => content,
        Err(e) => {
            panic!("Error reading file: {}", e);
        },
    }
}

pub fn extract_book_titles(notes_content: &str) -> Vec<String> {
    let mut titles_seen = HashSet::new();

    // We create an iterable from the contents of the notes, which allows us to iterate
    // through each line. When we use `enumerate`, for each line iterated, we get its
    // index and value. If the index, when divided by 5 (which represents each block of
    // a note for a book), yields an exact result, then we have the title of the book
    // for that note
    let mut available_titles: Vec<String> = notes_content
        .lines()
        .enumerate()
        .filter_map(|(index, content)| {
            if index % NUM_LINES_CLIPPING_FORMAT == 0 {
                let title = clean_content(content);
                if titles_seen.insert(title.clone()) { Some(title) } else { None }
            } else {
                None
            }
        })
        .collect();

    available_titles.sort();

    available_titles
}

pub fn display_labels(available_titles: &Vec<String>) -> Vec<String> {
    // Book titles can be very long, so we can count the number of characters in each
    // title and, if it exceeds a certain limit, add an ellipsis
    available_titles
        .iter()
        .map(|content| {
            if content.chars().count() > MAX_TITLE_LENGTH {
                format!(
                    "{}...",
                    content.chars().take(MAX_TITLE_LENGTH as usize).collect::<String>()
                )
            } else {
                content.clone()
            }
        })
        .collect()
}

pub fn show_all_books(available_titles: &Vec<String>) {
    let display_labels: Vec<String> = display_labels(&available_titles);
    terminal_processing(&display_labels, false);
}

pub fn select_book_title_index(available_titles: &Vec<String>) -> String {
    // Book titles can be very long, so we can count the number of characters in each
    // title and, if it exceeds a certain limit, add an ellipsis
    let display_labels: Vec<String> = display_labels(&available_titles);

    loop {
        let (terminal, selection, confirmed) =
            terminal_processing(&display_labels, true);

        // The selection retains the title's index, we need to select the entire title
        if confirmed {
            match selection {
                Some(index) => {
                    terminal.clear_last_lines(1).unwrap();
                    return available_titles[index].clone();
                },
                None => std::process::exit(0),
            }
        } else {
            terminal.clear_last_lines(2).unwrap();
        }
    }
}

pub fn get_content(notes_content: &str, selected_title: &str) -> Vec<String> {
    // We convert an &str to a Vector to iterate over it
    let notes_content_vector: Vec<&str> = notes_content.lines().collect();

    // We iterate every 5 lines, which correspond to each block of a note in a book.
    // The content is always 3 positions away from the block's starting position.
    let mut results = Vec::new();

    for index in (0..notes_content_vector.len()).step_by(NUM_LINES_CLIPPING_FORMAT) {
        let book_title = clean_content(notes_content_vector[index]);
        if book_title == selected_title {
            if let Some(book_content) =
                notes_content_vector.get(index + STARTING_INDEX_CONTENT)
            {
                results.push(book_content.to_string());
            }
        }
    }

    delete_duplicates(results)
}

pub fn save_content(selected_title_content: Vec<String>, output_path_notes: &Path) {
    if let Some(parent) = output_path_notes.parent() {
        create_dir_all(parent).expect("Error creating the output folder");
    }

    let unified_content: String = selected_title_content.join("\n");

    match write(output_path_notes, unified_content) {
        Ok(_) => {
            println!("🟢 {}", style("The file has been saved successfully").bold());
        },
        Err(_) => {
            println!("🔴 {}", style("Error saving the file").bold());
        },
    };
}

pub fn delete_content(notes_content: &str, selected_title: &str) -> Vec<String> {
    // We convert an &str to a Vector to iterate over it
    let notes_content_vector: Vec<&str> = notes_content.lines().collect();

    let mut results = Vec::new();

    // We iterate every 5 lines, which correspond to each block of a note in a book.
    // The content is always 3 positions away from the block's starting position.
    // We only keep the content if the title does not match the title
    // of the book selected by the user, and to maintain the consistency of the format
    // of the notes, we must add “\n” between each line of the content and, at the end
    // of the block, “==========”
    for index in (0..notes_content_vector.len()).step_by(NUM_LINES_CLIPPING_FORMAT) {
        let book_title = clean_content(notes_content_vector[index]);
        if book_title != selected_title {
            if let Some(book_content) =
                notes_content_vector.get(index..index + NUM_LINES_CLIPPING_FORMAT - 1)
            {
                results.push(book_content.join("\n"));
                results.push("==========".to_string());
            }
        }
    }

    results
}
