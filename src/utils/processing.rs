pub fn clean_content(content: &str) -> String {
    // Remove whitespace, convert to a string, and replace a character that appears
    // in the notes
    content.trim().to_string().replace("\u{feff}", "")
}

pub fn delete_duplicates(content: Vec<String>) -> Vec<String> {
    // Enumerate the vector content to create tuples of (index, content)
    let mut indexed_content: Vec<(usize, String)> =
        content.into_iter().enumerate().collect();

    // Sort the content by string length in descending order
    indexed_content.sort_by(|vect_a, vect_b| vect_b.1.len().cmp(&vect_a.1.len()));

    // Create a new vector to store the filtered results
    let mut results: Vec<(usize, String)> = Vec::new();

    // Iterate to check if the indexed content is a substring of an existing result;
    // push it if it's not already contained
    for (idx, content) in indexed_content {
        if !results.iter().any(|(_, result_content)| result_content.contains(&content))
        {
            results.push((idx, content));
        }
    }

    // Sort the results by their original keys (the indices) to restore chronological order
    results.sort_by_key(|(result_idx, _)| *result_idx);

    // Extract only the text content for the final output
    results.into_iter().map(|(_, result_content)| result_content).collect()
}
