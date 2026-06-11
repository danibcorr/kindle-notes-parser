use crate::utils::parser::display_labels;
use crate::utils::terminal::terminal_processing;

pub fn show_all_books(available_titles: &[String]) {
    let display_labels: Vec<String> = display_labels(available_titles);
    terminal_processing(&display_labels, false);
}

pub fn select_book_title_index(available_titles: &[String]) -> String {
    // Book titles can be very long, so we can count the number of characters in each
    // title and, if it exceeds a certain limit, add an ellipsis
    let display_labels: Vec<String> = display_labels(available_titles);

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
