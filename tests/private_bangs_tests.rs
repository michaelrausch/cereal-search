use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use bang_search::{
    config::{Config, BangDetails},
    routes::create_router,
};
use std::collections::HashMap;
use tower::ServiceExt;

async fn make_request_with_auth(query: &str, auth_token: &str, config_token: &str) -> Response {
    // Create a custom config with private bangs
    let mut bangs = HashMap::new();
    let mut private_bangs = HashMap::new();
    
    // Add a public bang
    bangs.insert("!g".to_string(), BangDetails {
        url: "https://www.google.com/search?q={searchTerms}".to_string(),
        name: "Google".to_string(),
        icon: "google".to_string(),
    });
    
    // Add a private bang
    private_bangs.insert("!private".to_string(), BangDetails {
        url: "https://private.example.com/search?q={searchTerms}".to_string(),
        name: "Private Search".to_string(),
        icon: "private".to_string(),
    });
    
    let config = Config {
        bangs,
        private_bangs,
        auth_token: config_token.to_string(),
        host_url: Some("http://localhost:3000".to_string()),
    };
    
    let app = create_router().with_state(config);
    
    // Create the request with the query and auth token
    let uri = format!("/search?q={}&login={}", urlencoding::encode(query), urlencoding::encode(auth_token));
    
    app.oneshot(Request::get(uri).body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
async fn test_private_bang_with_auth() {
    // Both tokens match - should work
    let response = make_request_with_auth("!private test search", "secret_token", "secret_token").await;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    
    let location = response.headers().get("location").unwrap();
    assert_eq!(location, "https://private.example.com/search?q=test%20search");
}

#[tokio::test]
async fn test_private_bang_without_auth() {
    // Tokens don't match - should not access private bang
    let response = make_request_with_auth("!private test search", "wrong_token", "secret_token").await;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    
    // Should redirect to default search engine with the full query
    let location = response.headers().get("location").unwrap();
    assert_eq!(location, "https://www.google.com/search?q=%21private%20test%20search");
}

#[tokio::test]
async fn test_public_bang_with_auth() {
    // Public bang with matching tokens
    let response = make_request_with_auth("!g test search", "secret_token", "secret_token").await;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    
    let location = response.headers().get("location").unwrap();
    assert_eq!(location, "https://www.google.com/search?q=test%20search");
}

#[tokio::test]
async fn test_public_bang_without_auth() {
    // Public bangs should work without auth
    let response = make_request_with_auth("!g test search", "wrong_token", "secret_token").await;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    
    let location = response.headers().get("location").unwrap();
    assert_eq!(location, "https://www.google.com/search?q=test%20search");
}
