use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use bang_search::{
    config::default_config,
    routes::create_router,
};
use tower::ServiceExt;

async fn make_request(query: Option<&str>) -> Response {
    let config = default_config();
    let app = create_router().with_state(config.bangs);
    
    let uri = match query {
        Some(q) => format!("/search?q={}", urlencoding::encode(q)),
        None => "/search".to_string(),
    };
    
    app.oneshot(Request::get(uri).body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
async fn test_search_with_no_query() {
    let response = make_request(None).await;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    
    let location = response.headers().get("location").unwrap();
    assert_eq!(location, "https://www.google.com");
}

#[tokio::test]
async fn test_search_with_google_bang() {
    let response = make_request(Some("!g rust programming")).await;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    
    let location = response.headers().get("location").unwrap();
    assert_eq!(location, "https://www.google.com/search?q=rust%20programming");
}

#[tokio::test]
async fn test_search_with_duckduckgo_bang() {
    let response = make_request(Some("!ddg rust programming")).await;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    
    let location = response.headers().get("location").unwrap();
    assert_eq!(location, "https://duckduckgo.com/?q=rust%20programming");
}

#[tokio::test]
async fn test_search_with_unknown_bang() {
    let response = make_request(Some("!unknown rust programming")).await;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    
    // Should default to Google
    let location = response.headers().get("location").unwrap();
    assert_eq!(location, "https://www.google.com/search?q=%21unknown%20rust%20programming");
}

#[tokio::test]
async fn test_search_without_bang() {
    let response = make_request(Some("rust programming")).await;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    
    // Should use Google
    let location = response.headers().get("location").unwrap();
    assert_eq!(location, "https://www.google.com/search?q=rust%20programming");
} 