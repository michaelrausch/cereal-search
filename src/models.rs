use serde::{Deserialize, Serialize};

// Query parameters struct
#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]  // Suppress the warning
pub struct SearchQuery {
    pub q: Option<String>,
    pub login: Option<String>,
} 