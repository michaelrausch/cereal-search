use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Redirect, Response},
    routing::get,
    Router,
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    bangs::extract_bang,
    config::Config,
    models::SearchQuery,
};

/// Response structure for the /live endpoint
#[derive(Serialize)]
struct BangInfo {
    query: String,
    bang: Option<String>,
    bang_name: Option<String>,
    bang_icon: Option<String>,
}

/// Details of a bang for the /bangs endpoint
#[derive(Serialize)]
struct BangDetails {
    key: String,
    name: String,
    icon: String,
    url: String,
}

/// Response structure for the /bangs endpoint
#[derive(Serialize)]
struct BangsList {
    bangs: Vec<BangDetails>,
}

/// Creates the application router with all routes and middleware
pub fn create_router() -> Router<Config> {
    // Create a CORS layer that allows any origin
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/search", get(search_handler))
        .route("/health", get(health_check))
        .route("/live", get(live_handler))
        .route("/bangs", get(bangs_list_handler))
        .layer(cors)
}

/// Health check endpoint that returns 200 OK
async fn health_check() -> Response {
    StatusCode::OK.into_response()
}

/// Handler for the search endpoint that redirects based on bang commands
async fn search_handler(
    Query(params): Query<SearchQuery>,
    State(config): State<Config>,
) -> impl IntoResponse {
    let query = match params.q {
        Some(q) => q,
        None => return Redirect::to("https://www.google.com"),
    };
    
    // Default search engine
    let default_search = "https://www.google.com/search?q={searchTerms}";
    
    // Extract bang if present
    let (bang, search_term) = extract_bang(&query);
    
    // Check if this is an authenticated request
    let is_authenticated = params.login.as_deref() == Some(&config.auth_token) && !config.auth_token.is_empty();
    
    if let Some(bang_key) = bang {
        // First check public bangs
        if let Some(bang_details) = config.bangs.get(bang_key) {
            let redirect_url = bang_details.url.replace("{searchTerms}", &urlencoding::encode(search_term));
            return Redirect::to(&redirect_url);
        }
        
        // Then check private bangs if authenticated
        if is_authenticated {
            if let Some(bang_details) = config.private_bangs.get(bang_key) {
                let redirect_url = bang_details.url.replace("{searchTerms}", &urlencoding::encode(search_term));
                return Redirect::to(&redirect_url);
            }
        }
        
        println!("Bang '{}' not found in configuration", bang_key);
    }
    
    // No bang found or bang not recognized, redirect to default search engine
    let redirect_url = default_search.replace("{searchTerms}", &urlencoding::encode(&query));
    println!("Redirecting to default: {}", redirect_url);
    Redirect::to(&redirect_url)
}

/// Handler for the live endpoint that returns information about the current query
async fn live_handler(
    Query(params): Query<SearchQuery>,
    State(config): State<Config>,
) -> impl IntoResponse {
    let query = params.q.unwrap_or_default();
    
    // Extract bang if present
    let (bang, search_term) = extract_bang(&query);
    
    // Check if this is an authenticated request
    let is_authenticated = params.login.as_deref() == Some(&config.auth_token) && !config.auth_token.is_empty();
    
    // Only include valid bangs in the response
    let (valid_bang, bang_name, bang_icon) = match bang {
        Some(b) => {
            if let Some(details) = config.bangs.get(b) {
                (Some(b.to_string()), Some(details.name.clone()), Some(details.icon.clone()))
            } else if is_authenticated {
                if let Some(details) = config.private_bangs.get(b) {
                    (Some(b.to_string()), Some(details.name.clone()), Some(details.icon.clone()))
                } else {
                    (None, None, None)
                }
            } else {
                (None, None, None)
            }
        },
        None => (None, None, None)
    };
    
    Json(BangInfo {
        query: search_term.to_string(),
        bang: valid_bang,
        bang_name,
        bang_icon,
    })
}

/// Handler for the bangs list endpoint that returns all available bangs
async fn bangs_list_handler(
    Query(params): Query<SearchQuery>,
    State(config): State<Config>,
) -> impl IntoResponse {
    // Check if this is an authenticated request
    let is_authenticated = params.login.as_deref() == Some(&config.auth_token) && !config.auth_token.is_empty();
    
    let mut bangs_list = Vec::new();
    
    // Add public bangs
    for (key, details) in &config.bangs {
        bangs_list.push(BangDetails {
            key: key.clone(),
            name: details.name.clone(),
            icon: details.icon.clone(),
            url: details.url.clone(),
        });
    }
    
    // Add private bangs if authenticated
    if is_authenticated {
        for (key, details) in &config.private_bangs {
            bangs_list.push(BangDetails {
                key: key.clone(),
                name: details.name.clone(),
                icon: details.icon.clone(),
                url: details.url.clone(),
            });
        }
    }
    
    // Sort the list by key for consistent output
    bangs_list.sort_by(|a, b| a.key.cmp(&b.key));
    
    Json(BangsList { bangs: bangs_list })
} 