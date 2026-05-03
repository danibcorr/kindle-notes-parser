use crate::utils::processing;
use crate::utils::terminal;
use console::style;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const NUM_LINES_CLIPPING_FORMAT: usize = 5;
const STARTING_INDEX_CONTENT: usize = 3;
const MAX_TITLE_LENGTH: usize = 50;

pub fn read_file_notes(notes_path: &PathBuf) -> String {
    return match fs::read_to_string(notes_path) {
        Ok(content) => content,
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    };
}

pub fn extract_book_titles(notes_content: &str) -> Vec<String> {
    let iterable_doc_content = notes_content.lines();

    // We create an iterable from the contents of the notes, which allows us to iterate
    // through each line. When we use `enumerate`, for each line iterated, we get its
    // index and value. If the index, when divided by 5 (which represents each block of
    // a note for a book), yields an exact result, then we have the title of the book
    // for that note
    let mut available_titles: Vec<String> = iterable_doc_content
        .enumerate()
        .filter_map(|(index, content)| {
            if index % NUM_LINES_CLIPPING_FORMAT == 0 {
                Some(processing::clean_content(content))
            } else {
                None
            }
        })
        .collect();

    // We can remove duplicate titles that appear in different sections of the
    // notes by using a HashSet. We create an empty one and insert the values; it
    // only keeps unique values
    let mut available_titles_filtered: HashSet<String> = HashSet::new();
    available_titles
        .retain(|titles| available_titles_filtered.insert(titles.to_string()));

    // Convert the HashSet to a string to iterate through the available titles
    // and sort them so that they are always displayed in the same order on the terminal
    let mut available_titles_filtered: Vec<String> =
        available_titles_filtered.into_iter().collect();
    available_titles_filtered.sort();

    return available_titles_filtered;
}

pub fn select_book_title_index(available_titles: Vec<String>) -> String {
    // Book titles can be very long, so we can count the number of characters in each
    // title and, if it exceeds a certain limit, add an ellipsis
    let display_labels: Vec<String> = available_titles
        .iter()
        .map(|content| {
            if content.chars().count() > MAX_TITLE_LENGTH {
                format!(
                    "{}...",
                    content
                        .chars()
                        .take(MAX_TITLE_LENGTH as usize)
                        .collect::<String>()
                )
            } else {
                content.clone()
            }
        })
        .collect();

    let (terminal, selection, selection_confirmation) =
        terminal::terminal_processing(&display_labels);

    // The selection retains the title's index, we need to select the entire title
    if selection_confirmation {
        terminal.clear_last_lines(1).unwrap();

        match selection {
            Some(index) => {
                let selected_full = available_titles[index].clone();

                terminal.clear_last_lines(1).unwrap();

                println!(
                    "{} {} {}",
                    style("📖"),
                    style("Selected:").bold(),
                    style(&display_labels[index]).yellow()
                );

                selected_full
            }
            None => std::process::exit(0),
        }
    } else {
        terminal.clear_last_lines(2).unwrap();
        select_book_title_index(available_titles);
        std::process::exit(0);
    }
}

pub fn get_content(notes_content: &str, selected_title: &str) -> Vec<String> {
    // We convert an &str to a Vector to iterate over it
    let notes_content_vector: Vec<&str> = notes_content.lines().collect();

    let mut results = Vec::new();

    // We iterate every 5 lines, which correspond to each block of a note in a book.
    // The content is always 3 positions away from the block's starting position.
    for index in (0..notes_content_vector.len()).step_by(NUM_LINES_CLIPPING_FORMAT) {
        let book_title = processing::clean_content(notes_content_vector[index]);
        if book_title == selected_title {
            if let Some(book_content) =
                notes_content_vector.get(index + STARTING_INDEX_CONTENT)
            {
                results.push(book_content.to_string());
            }
        }
    }

    return processing::delete_duplicates(results);
}

pub fn save_content(selected_title_content: Vec<String>, output_path_notes: &str) {
    // We need to extract from the output format, i.e., output/output.txt,
    // the initial part without the file name, i.e., output, to create a folder with that
    // name
    let path_directory: Vec<&str> = output_path_notes.split('/').collect();
    let path_directory_without_file =
        path_directory.get(0..path_directory.len() - 1).unwrap();
    let path_directory_without_file = path_directory_without_file.join("/");

    // Once we have parsed the expecte output path we create this folder
    fs::create_dir_all(path_directory_without_file)
        .expect("Error creating the output folder");

    // Now we can create the file in this folder
    let mut file = File::create(output_path_notes).expect("Error creating file");

    let unified_content = selected_title_content.join("\n");

    match file.write_all(unified_content.as_bytes()) {
        Ok(_) => {
            println!(
                "🟢 {}",
                style("The file has been saved successfully").bold()
            );
        }
        Err(_) => {
            println!("🔴 {}", style("Error saving the file").bold());
        }
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
        let book_title = processing::clean_content(notes_content_vector[index]);
        if book_title != selected_title {
            if let Some(book_content) =
                notes_content_vector.get(index..index + NUM_LINES_CLIPPING_FORMAT - 1)
            {
                results.push(book_content.join("\n"));
                results.push("==========".to_string());
            }
        }
    }

    return results;
}
