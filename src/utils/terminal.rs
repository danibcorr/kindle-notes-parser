use console::{Term, style};
use dialoguer::{Confirm, Select, theme::ColorfulTheme};

pub fn terminal_processing(
    display_labels: &[String],
    use_title_selector: bool,
) -> (Term, Option<usize>, bool) {
    let terminal: Term = Term::stdout();

    let terminal_theme: ColorfulTheme = ColorfulTheme {
        prompt_prefix: style("📚".to_string()),
        checked_item_prefix: style("✔".to_string()).for_stderr().blue(),
        ..ColorfulTheme::default()
    };

    if use_title_selector {
        let selection_menu_titles: Option<usize> = Select::with_theme(&terminal_theme)
            .with_prompt("Select one of the titles found")
            .items(display_labels)
            .default(0)
            .interact_on_opt(&terminal)
            .unwrap_or_else(|_| {
                let _ = terminal.show_cursor();
                std::process::exit(0);
            });

        terminal.clear_last_lines(1).unwrap();

        let selected_index =
            selection_menu_titles.unwrap_or_else(|| std::process::exit(0));

        println!(
            "{} {} · {}",
            style("📖"),
            style("Selected title").bold(),
            style(display_labels[selected_index].clone()).green()
        );

        let selection_menu_confirmation: bool =
            Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to continue?")
                .interact_on(&terminal)
                .unwrap_or_else(|_| {
                    let _ = terminal.show_cursor();
                    std::process::exit(0);
                });

        (terminal, selection_menu_titles, selection_menu_confirmation)
    } else {
        // This is to copy the same style as ColorfulTheme
        println!(
            "{} {} {}",
            style("📚".to_string()),
            style("Available titles").bold(),
            style("›".to_string()).for_stderr().black().bright(),
        );

        for titles in display_labels {
            println!("{} {titles}", style("❯".to_string()).for_stderr().green());
        }

        (terminal, None, false)
    }
}
