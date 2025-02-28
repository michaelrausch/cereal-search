// Helper function to extract bang and search term from query
pub fn extract_bang(query: &str) -> (Option<&str>, &str) {
    // Check for bangs at the beginning of the query
    let parts: Vec<&str> = query.splitn(2, ' ').collect();
    
    if parts.len() == 2 && parts[0].starts_with('!') {
        // Bang is at the beginning followed by space
        (Some(parts[0]), parts[1])
    } else if query.starts_with('!') {
        // Check if the entire query is just a bang
        if !query.contains(' ') {
            (Some(query), "")
        } else {
            // This shouldn't happen with the splitn above, but just in case
            (None, query)
        }
    } else {
        // No bang found
        (None, query)
    }
}
