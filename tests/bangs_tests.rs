use bang_search::bangs::extract_bang;
use bang_search::config::default_config;

#[test]
fn test_extract_bang() {
    // Test with bang at beginning
    let (bang, term) = extract_bang("!ddg search term");
    assert_eq!(bang, Some("!ddg"));
    assert_eq!(term, "search term");

    // Test with no bang
    let (bang, term) = extract_bang("just a search");
    assert_eq!(bang, None);
    assert_eq!(term, "just a search");

    // Test with just a bang
    let (bang, term) = extract_bang("!g");
    assert_eq!(bang, Some("!g"));
    assert_eq!(term, "");
    
    // Test with bang but no space
    let (bang, term) = extract_bang("!ddgsearchterm");
    assert_eq!(bang, Some("!ddgsearchterm"));
    assert_eq!(term, "");
    
    // Test with multiple spaces
    let (bang, term) = extract_bang("!ddg  multiple  spaces");
    assert_eq!(bang, Some("!ddg"));
    assert_eq!(term, " multiple  spaces");
}

#[test]
fn test_config_bangs() {
    let config = default_config();
    
    // Test that we have the expected bangs
    assert!(config.bangs.contains_key("!g"));
    assert!(config.bangs.contains_key("!ddg"));
    assert!(config.bangs.contains_key("!yt"));
    assert!(config.bangs.contains_key("!gh"));
    assert!(config.bangs.contains_key("!w"));
    assert!(config.bangs.contains_key("!maps"));
    
    // Test the URL templates
    assert_eq!(
        config.bangs.get("!g").unwrap(),
        "https://www.google.com/search?q={searchTerms}"
    );
    assert_eq!(
        config.bangs.get("!ddg").unwrap(),
        "https://duckduckgo.com/?q={searchTerms}"
    );
    
    // Test that the URL templates contain the placeholder
    for (_, url) in config.bangs.iter() {
        assert!(url.contains("{searchTerms}"), "URL template should contain {{searchTerms}} placeholder");
    }
} 