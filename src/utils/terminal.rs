use console::{Term, style};
use dialoguer::{Select, theme::ColorfulTheme};

pub fn terminal_processing(
    display_labels: &Vec<String>,
) -> (Term, Option<usize>, bool) {
    let terminal = Term::stdout();

    // Terminal theme
    let terminal_theme = ColorfulTheme {
        prompt_prefix: style("📚".to_string()),
        checked_item_prefix: style("✔".to_string()).for_stderr().blue(),
        ..ColorfulTheme::default()
    };

    // This is the selection menu for the titles
    let selection = Select::with_theme(&terminal_theme)
        .with_prompt("Select one of the titles found")
        .items(display_labels)
        .default(0)
        .interact_opt()
        .expect("Error in the terminal");

    // This is another menu for the confirmation, to display a y/n option
    let selection_confirmation =
        dialoguer::Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to continue?")
            .interact_on(&terminal)
            .unwrap();

    return (terminal, selection, selection_confirmation);
}
