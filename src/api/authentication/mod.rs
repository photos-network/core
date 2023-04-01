
use axum::{
    routing::{ get, post },
    Router,
};

use oxide_auth_axum::OAuthRequest;

/**
 * Autentication Manager is handling access requests via OAuth
 * by validating the identity of users.
 */
pub(crate) struct AuthManager {

}

impl AuthManager {
    pub fn routes<S>() -> Router<S>
    where
        S: Send + Sync + 'static + Clone,
    {
        Router::new()
            .route("/authorize", get( || async { "Create authorization uri and redirect" } ))
            .route("/authorize", post( || async { "Login request" } ))
            .route("/token", post( || async { "Access token request" } ))
            .route("/refresh", post( || async { "Access token request" } ))
            .route("/", post( || async { "Access token request" } ))
    }
}

pub struct Authorize(pub OAuthRequest);

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{http::{Request, StatusCode, self}, body::Body};
    use tower::{ServiceExt};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AuthorizationRequest {
        response_type: String,
        client_id: String,
        redirect_uri: Option<Vec<String>>,
        scope: Option<String>,
        state: Option<String>,
    }

    #[tokio::test]
    async fn authorization_uri_redirects() {
        // given
        let router = AuthManager::routes();
        let form_data = serde_urlencoded::to_string(
            AuthorizationRequest { 
                response_type: String::from("code"), 
                client_id: String::from("1234"),
                redirect_uri: None,
                scope: Some(String::from("s6BhdRkqt3")),
                state: Some(String::from("xyz")),
            }
        ).expect("parse authorization request failed!");

        // when
        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/authorize")
                    .header(http::header::CONTENT_TYPE, "application/x-www-form-urlencode")
                    .body(Body::from(form_data))
                    .unwrap(),
            )
            .await
            .unwrap();

        // then
        assert_eq!(
            response.status().as_u16(),
            StatusCode::FOUND,
            "authorization request should return redirect uri as Location header!"
        );
    }

    #[tokio::test]
    async fn it_works_now() {
        // given
        let router = AuthManager::routes();

        // when
        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/authorize")
                    .header(http::header::CONTENT_TYPE, "application/json; charset=utf-8")
                    .body(Body::from(Body::empty()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // then
        assert_eq!(
            response.status().as_u16(),
            422,
            "{} returns client error status",
            "no username"
        );
    }
}
