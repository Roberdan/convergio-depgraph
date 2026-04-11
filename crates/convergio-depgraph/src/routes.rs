//! HTTP API routes for convergio-depgraph.

use axum::Router;

/// Returns the router for this crate's API endpoints.
pub fn routes() -> Router {
    Router::new()
    // .route("/api/depgraph/health", get(health))
}
