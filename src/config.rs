use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

// Bang details structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BangDetails {
    pub url: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub icon: String,
}

// Configuration structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub bangs: HashMap<String, BangDetails>,
    #[serde(default)]
    pub private_bangs: HashMap<String, BangDetails>,
    #[serde(default)]
    pub auth_token: String,
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
    let private_bangs = HashMap::new();
    
    // Add some common bangs with their search URL templates
    bangs.insert("!g".to_string(), BangDetails {
        url: "https://www.google.com/search?q={searchTerms}".to_string(),
        name: "Google".to_string(),
        icon: "google".to_string(),
    });
    
    bangs.insert("!ddg".to_string(), BangDetails {
        url: "https://duckduckgo.com/?q={searchTerms}".to_string(),
        name: "DuckDuckGo".to_string(),
        icon: "duck".to_string(),
    });
    
    bangs.insert("!yt".to_string(), BangDetails {
        url: "https://www.youtube.com/results?search_query={searchTerms}".to_string(),
        name: "YouTube".to_string(),
        icon: "youtube".to_string(),
    });
    
    bangs.insert("!gh".to_string(), BangDetails {
        url: "https://github.com/search?q={searchTerms}".to_string(),
        name: "GitHub".to_string(),
        icon: "github".to_string(),
    });
    
    bangs.insert("!w".to_string(), BangDetails {
        url: "https://en.wikipedia.org/wiki/Special:Search?search={searchTerms}".to_string(),
        name: "Wikipedia".to_string(),
        icon: "wikipedia".to_string(),
    });
    
    bangs.insert("!maps".to_string(), BangDetails {
        url: "https://www.google.com/maps?q={searchTerms}&source=web".to_string(),
        name: "Google Maps".to_string(),
        icon: "map".to_string(),
    });
    
    Config { 
        bangs,
        private_bangs,
        auth_token: "".to_string(),
    }
} 