use console::style;
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

pub fn save_content(
    title: &str,
    selected_title_content: &[String],
    output_path_notes: &Path,
) {
    if let Some(parent) = output_path_notes.parent() {
        create_dir_all(parent).expect("Error creating the output folder");
    }

    let unified_content: String = selected_title_content.join("\n");

    match write(output_path_notes, unified_content) {
        Ok(_) => {
            println!(
                "🟢 {}",
                style(format!(
                    "The file '{}' has been saved successfully",
                    style(&title).italic()
                ))
                .bold()
            );
        },
        Err(_) => {
            println!("🔴 {}", style("Error saving the file").bold());
        },
    };
}
