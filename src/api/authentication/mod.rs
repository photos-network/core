use axum::{
    routing::{get, post},
    Router,
};


mod requests {
    pub mod authorization_requests;
}

use requests::authorization_requests::authorization_endpoint_get;

/**
 * Autentication Manager is handling access requests via OAuth
 * by validating the identity of users.
 */
pub(crate) struct AutenticationManager {}

impl AutenticationManager {
    pub fn routes<S>() -> Router<S>
    where
        S: Send + Sync + 'static + Clone,
    {
        Router::new()
            .route("/authorize", get(authorization_endpoint_get))
            .route("/token", post(|| async { "Access token request" }))
            .route("/refresh", post(|| async { "Access token request" }))
            .route("/", post(|| async { "Access token request" }))
    }
}

