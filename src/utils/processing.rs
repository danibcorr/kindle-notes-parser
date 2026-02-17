pub fn clean_content(content: &str) -> String {
    return content.trim().to_string().replace("\u{feff}", "");
}
