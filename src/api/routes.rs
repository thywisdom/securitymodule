use axum::{
    routing::{get, post},
    Router,
};
use crate::api::handlers;
use tower_http::trace::TraceLayer;

pub fn app() -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/keygen", post(handlers::keygen_handler))
        .route("/encrypt", post(handlers::encrypt_handler))
        .route("/decrypt", post(handlers::decrypt_handler))
        .layer(TraceLayer::new_for_http())
}
