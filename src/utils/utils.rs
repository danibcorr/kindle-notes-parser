use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::utils::processing;

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
    let index_titles: HashMap<usize, String> =
        available_titles.into_iter().enumerate().collect();

    let mut index_titles_sorted: Vec<(&usize, &String)> = index_titles.iter().collect();
    index_titles_sorted.sort_by_key(|(index, _)| *index);

    println!("Titles found:\n");

    index_titles_sorted.iter().for_each(|(id, title)| {
        println!("{} - {}", id, title);
    });

    println!("\nEnter the index of the title to select:");

    loop {
        let mut text_input: String = String::new();

        io::stdin()
            .read_line(&mut text_input)
            .expect("Error reading input");

        let input_user: usize = match text_input.trim().parse() {
            Ok(num) => {
                if index_titles.contains_key(&num) {
                    num
                } else {
                    println!("The index does not exist, please try again");
                    continue;
                }
            }
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        let selected_title: String = index_titles
            .get(&input_user)
            .expect("Error retrieving title")
            .to_string();

        println!("Selected title: {}", selected_title);

        return selected_title;
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

    return results;
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
