use serde::Deserialize;

// Query parameters struct
#[derive(Deserialize)]
#[allow(dead_code)]  // Suppress the warning
pub struct SearchQuery {
    pub q: Option<String>,
} 