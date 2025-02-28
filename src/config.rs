use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

// Configuration structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub bangs: HashMap<String, String>,
}

// Load configuration from a YAML file
pub async fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // Check if file exists
    if !Path::new(path).exists() {
        return Ok(default_config());
    }
    
    // Open the file
    let mut file = File::open(path).await?;
    
    // Read the file contents
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    
    // Parse the YAML into our Config structure
    let config: Config = serde_yaml::from_str(&contents)?;
    
    Ok(config)
}

// Create a default configuration
pub fn default_config() -> Config {
    let mut bangs = HashMap::new();
    
    // Add some common bangs with their search URL templates
    bangs.insert("!g".to_string(), "https://www.google.com/search?q={searchTerms}".to_string());
    bangs.insert("!ddg".to_string(), "https://duckduckgo.com/?q={searchTerms}".to_string());
    bangs.insert("!yt".to_string(), "https://www.youtube.com/results?search_query={searchTerms}".to_string());
    bangs.insert("!gh".to_string(), "https://github.com/search?q={searchTerms}".to_string());
    bangs.insert("!w".to_string(), "https://en.wikipedia.org/wiki/Special:Search?search={searchTerms}".to_string());
    bangs.insert("!maps".to_string(), "https://www.google.com/maps?q={searchTerms}&source=web".to_string());
    
    Config { bangs }
} 