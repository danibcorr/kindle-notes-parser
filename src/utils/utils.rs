use crate::utils::processing;
use console::{Term, style};
use dialoguer::{Select, theme::ColorfulTheme};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const MAX_TITLE_LENGTH: i32 = 50;

pub fn read_file_notes(path_notes: &PathBuf) -> String {
    return match fs::read_to_string(path_notes) {
        Ok(content) => content,
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    };
}

pub fn extract_book_titles(notes_content: &str) -> HashSet<String> {
    let iterable_doc_content = notes_content.lines();

    let mut available_titles: Vec<String> = iterable_doc_content
        .enumerate()
        .filter_map(|(index, content)| {
            if index % 5 == 0 {
                Some(processing::clean_content(content))
            } else {
                None
            }
        })
        .collect();

    let mut available_titles_filtered: HashSet<String> = HashSet::new();
    available_titles
        .retain(|titles| available_titles_filtered.insert(titles.to_string()));

    return available_titles_filtered;
}

pub fn select_book_title_index(available_titles: HashSet<String>) -> String {
    let term = Term::stdout();

    let titles: Vec<String> = available_titles.into_iter().collect();

    let display_labels: Vec<String> = titles
        .iter()
        .map(|content| {
            if content.chars().count() as i32 > MAX_TITLE_LENGTH {
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

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select one of the titles found")
        .items(&display_labels)
        .default(0)
        .interact_opt()
        .expect("Error in the terminal");

    match selection {
        Some(index) => {
            let selected_full = titles[index].clone();

            term.clear_last_lines(1).unwrap();

            println!(
                "{} Selected: {}",
                style("✔").green(),
                style(&display_labels[index]).yellow()
            );

            selected_full
        }
        None => std::process::exit(0),
    }
}

pub fn get_content(notes_content: &str, selected_title: &str) -> Vec<String> {
    let notes_content_vector: Vec<&str> = notes_content.lines().collect();

    let mut results = Vec::new();

    for index in (0..notes_content_vector.len()).step_by(5) {
        let titulo_libro = processing::clean_content(notes_content_vector[index]);
        if titulo_libro == selected_title {
            if let Some(book_content) = notes_content_vector.get(index + 3) {
                results.push(book_content.to_string());
            }
        }
    }

    return processing::delete_duplicates(results);
}

pub fn save_content(selected_title_content: Vec<String>, selected_title: &str) {
    fs::create_dir_all("outputs").expect("Error creating the ‘outputs’ folder");

    let file_path = format!("outputs/{}.txt", selected_title);
    let mut file = File::create(file_path).expect("Error creating file");

    let unified_content = selected_title_content.join("\n");

    match file.write_all(unified_content.as_bytes()) {
        Ok(_) => println!("The file has been saved successfully"),
        Err(_) => println!("Error saving the file"),
    };
}
