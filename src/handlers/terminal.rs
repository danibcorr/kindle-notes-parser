use console::Term;

pub fn set_ctrlc_handler() {
    ctrlc::set_handler(move || {
        // Restore cursor visibility before exiting
        let term: Term = Term::stdout();
        let _ = term.show_cursor();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl+C handler");
}
