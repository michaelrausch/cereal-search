use bang_search::config::{load_config, default_config};
use std::io::Write;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_default_config() {
    let config = default_config();
    
    // Test that default config has expected bangs
    assert!(config.bangs.contains_key("!g"));
    assert!(config.bangs.contains_key("!ddg"));
    assert!(config.bangs.contains_key("!yt"));
    
    // Test URL format
    for (_, url) in config.bangs.iter() {
        assert!(url.contains("{searchTerms}"), "URL should contain placeholder");
    }
}

#[tokio::test]
async fn test_load_config_nonexistent_file() {
    // Test loading from a file that doesn't exist
    let result = load_config("nonexistent_file.yml").await;
    
    // Should return default config
    assert!(result.is_ok());
    let config = result.unwrap();
    assert!(config.bangs.contains_key("!g"));
}

#[tokio::test]
async fn test_load_config_valid_yaml() {
    // Create a temporary YAML file
    let mut temp_file = NamedTempFile::new().unwrap();
    
    // Write test config to the file
    let yaml_content = r#"
bangs:
  "!test": "https://example.com/search?q={searchTerms}"
  "!custom": "https://custom.example.com/?query={searchTerms}&other=param"
"#;
    
    temp_file.write_all(yaml_content.as_bytes()).unwrap();
    
    // Load the config from the temp file
    let config_path = temp_file.path().to_str().unwrap();
    let result = load_config(config_path).await;
    
    // Verify the loaded config
    assert!(result.is_ok());
    let config = result.unwrap();
    
    assert_eq!(config.bangs.len(), 2);
    assert!(config.bangs.contains_key("!test"));
    assert!(config.bangs.contains_key("!custom"));
    
    assert_eq!(
        config.bangs.get("!test").unwrap(),
        "https://example.com/search?q={searchTerms}"
    );
}

#[tokio::test]
async fn test_load_config_invalid_yaml() {
    // Create a temporary file with invalid YAML
    let mut temp_file = NamedTempFile::new().unwrap();
    
    // Write invalid YAML to the file
    let invalid_yaml = r#"
bangs:
  - this is not valid YAML for our structure
  - missing key-value pairs
"#;
    
    temp_file.write_all(invalid_yaml.as_bytes()).unwrap();
    
    // Try to load the config
    let config_path = temp_file.path().to_str().unwrap();
    let result = load_config(config_path).await;
    
    // Should return an error
    assert!(result.is_err());
} 