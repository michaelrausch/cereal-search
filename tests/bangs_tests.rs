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

#[test]
fn test_extract_bang_beginning() {
    // Test bangs at the beginning
    assert_eq!(extract_bang("!g search term"), (Some("!g"), "search term"));
    assert_eq!(extract_bang("!ddg another search"), (Some("!ddg"), "another search"));
    assert_eq!(extract_bang("!w"), (Some("!w"), ""));
}

#[test]
fn test_extract_bang_end() {
    // Test bangs at the end
    assert_eq!(extract_bang("search term !g"), (Some("!g"), "search term"));
    assert_eq!(extract_bang("another search !ddg"), (Some("!ddg"), "another search"));
}

#[test]
fn test_extract_bang_with_whitespace() {
    // Test with extra whitespace
    assert_eq!(extract_bang("  !g search term  "), (Some("!g"), "search term"));
    assert_eq!(extract_bang("  search term !g  "), (Some("!g"), "search term"));
}

#[test]
fn test_no_bang() {
    // Test with no bang
    assert_eq!(extract_bang("just a search"), (None, "just a search"));
    assert_eq!(extract_bang(""), (None, ""));
    assert_eq!(extract_bang("   "), (None, ""));
}

#[test]
fn test_invalid_bang_patterns() {
    // Test with invalid bang patterns
    assert_eq!(extract_bang("search with ! in middle"), (None, "search with ! in middle"));
    assert_eq!(extract_bang("g!"), (None, "g!"));
}