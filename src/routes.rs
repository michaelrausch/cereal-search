use axum::{
    extract::Query,
    response::{Redirect, IntoResponse, Response, Json},
    routing::get,
    Router,
    http::StatusCode,
};
use std::collections::HashMap;
use crate::models::SearchQuery;
use crate::bangs::extract_bang;
use serde::Serialize;

// New struct for the JSON response
#[derive(Serialize)]
struct BangInfo {
    query: String,
    bang: Option<String>,
}

pub fn create_router() -> Router<HashMap<String, String>> {
    Router::new()
        .route("/search", get(search_handler))
        .route("/health", get(health_check))
        .route("/live", get(live_handler))
}

// Health check endpoint
async fn health_check() -> Response {
    StatusCode::OK.into_response()
}

// Handler for the search endpoint
async fn search_handler(
    Query(params): Query<SearchQuery>,
    bangs: axum::extract::State<HashMap<String, String>>,
) -> impl IntoResponse {
    let query = match params.q {
        Some(q) => q,
        None => return Redirect::to("https://www.google.com"),
    };
    
    // Default search engine
    let default_search = "https://www.google.com/search?q={searchTerms}";
    
    // Extract bang if present
    let (bang, search_term) = extract_bang(&query);
    
    // Debug print
    println!("Query: '{}', Bang: '{:?}', Search Term: '{}'", 
             query, bang, search_term);
    
    if let Some(bang_key) = bang {
        // Check if this bang exists in our configuration
        if let Some(url_template) = bangs.get(bang_key) {
            // Replace the placeholder with the encoded search term
            let redirect_url = url_template.replace("{searchTerms}", &urlencoding::encode(search_term));
            println!("Redirecting to: {}", redirect_url);
            return Redirect::to(&redirect_url);
        } else {
            println!("Bang '{}' not found in configuration", bang_key);
        }
    }
    
    // No bang found or bang not recognized, redirect to default search engine
    let redirect_url = default_search.replace("{searchTerms}", &urlencoding::encode(&query));
    println!("Redirecting to default: {}", redirect_url);
    Redirect::to(&redirect_url)
}

// New handler for the live endpoint
async fn live_handler(
    Query(params): Query<SearchQuery>,
    bangs: axum::extract::State<HashMap<String, String>>,
) -> impl IntoResponse {
    let query = match params.q {
        Some(q) => q,
        None => String::new(),
    };
    
    // Extract bang if present
    let (bang, search_term) = extract_bang(&query);
    
    // Only include valid bangs in the response
    let valid_bang = bang.and_then(|b| {
        if bangs.contains_key(b) {
            Some(format!("{}", b))
        } else {
            None
        }
    });
    
    // Create the response with the search term (without the bang)
    let bang_info = BangInfo {
        query: search_term.to_string(),
        bang: valid_bang,
    };
    
    Json(bang_info)
} 