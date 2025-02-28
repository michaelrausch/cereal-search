// Helper function to extract bang and search term from query
pub fn extract_bang(query: &str) -> (Option<&str>, &str) {
    let trimmed = query.trim();
    
    // Check for bangs at the beginning of the query
    if trimmed.starts_with('!') {
        let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
        
        if parts.len() == 2 {
            // Bang is at the beginning followed by space
            return (Some(parts[0]), parts[1]);
        } else {
            // The entire query is just a bang
            return (Some(trimmed), "");
        }
    }
    
    // Check for bangs at the end of the query
    if trimmed.ends_with('!') && trimmed.contains(' ') {
        let last_space_pos: usize = trimmed.rfind(' ').unwrap();
        let search_term = &trimmed[..last_space_pos];
        let bang = &trimmed[last_space_pos + 1..];
        
        if bang.starts_with('!') {
            return (Some(bang), search_term);
        }
    } else if trimmed.contains(' ') {
        // Check if the last word is a bang
        let parts: Vec<&str> = trimmed.rsplitn(2, ' ').collect();
        if parts.len() == 2 && parts[0].starts_with('!') {
            return (Some(parts[0]), parts[1]);
        }
    }
    
    // No bang found
    (None, trimmed)
}
