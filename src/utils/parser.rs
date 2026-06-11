use crate::utils::constants::{
    MAX_TITLE_LENGTH, NUM_LINES_CLIPPING_FORMAT, STARTING_INDEX_CONTENT,
};
use crate::utils::processing::{clean_content, delete_duplicates};
use std::collections::{HashMap, HashSet};

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

pub fn display_labels(available_titles: &[String]) -> Vec<String> {
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

pub fn get_all_content(notes_content: &str) -> HashMap<String, Vec<String>> {
    let notes_content_vector: Vec<&str> = notes_content.lines().collect();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for index in (0..notes_content_vector.len()).step_by(NUM_LINES_CLIPPING_FORMAT) {
        let title = clean_content(notes_content_vector[index]);
        if let Some(content) = notes_content_vector.get(index + STARTING_INDEX_CONTENT)
        {
            map.entry(title).or_default().push(content.to_string());
        }
    }

    map.into_iter().map(|(title, notes)| (title, delete_duplicates(notes))).collect()
}

pub fn delete_content(notes_content: &str, selected_title: &str) -> Vec<String> {
    // We convert an &str to a Vector to iterate over it
    let notes_content_vector: Vec<&str> = notes_content.lines().collect();

    let mut results = Vec::new();

    // We iterate every 5 lines, which correspond to each block of a note in a book.
    // The content is always 3 positions away from the block's starting position.
    // We only keep the content if the title does not match the title
    // of the book selected by the user, and to maintain the consistency of the format
    // of the notes, we must add "\n" between each line of the content and, at the end
    // of the block, "=========="
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
