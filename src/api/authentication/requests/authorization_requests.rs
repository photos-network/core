use abi_stable::std_types::string;

use serde::{Serialize, Deserialize};
use tracing::debug;


use axum::{
    http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
    extract::{FromRef, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Router, Json,
};
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    message: String,
}

///!
/// Authorization Request
/// See 4.1.1: https://tools.ietf.org/html/rfc6749#section-4.1.1
///
pub async fn authorization_endpoint_get() -> impl IntoResponse {
    debug!("GET /oauth/authorize from ");

    // TODO: change return
    let error = Error { message: String::from("Not implemented!") };
    (
        StatusCode::FOUND, 
        [
            (header::LOCATION, "https://client.example.com/cb?code=SplxlOBeZQQYbYS6WxSbIA&state=xyz")
        ],
        Json(error)
    )
}

#[cfg(test)]
mod tests {
    use crate::api::authentication::AutenticationManager;
    use axum::{http::{Request, StatusCode, header, self}, body::Body};
    use tower::ServiceExt;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AuthorizationRequest {
        response_type: String,
        client_id: String,
        redirect_uri: Option<Vec<String>>,
        scope: Option<String>,
        state: Option<String>,
    }

    ///!
    /// Validate the request to ensure that all required parameters are present and valid.
    /// See Section 4.1.1: https://tools.ietf.org/html/rfc6749#section-4.1.1
    ///
    #[tokio::test]
    async fn valid_authorization_should_be_validated_and_redirected() {
        // given
        let router = AutenticationManager::routes();
        let form_data = serde_urlencoded::to_string(
            AuthorizationRequest { 
                response_type: String::from("code"), 
                client_id: String::from("1234"),
                redirect_uri: None,//Some(vec!(String::from("https://client.example.com/cb"))),
                scope: Some(String::from("s6BhdRkqt3")),
                state: Some(String::from("xyz")),
            }
        ).expect("parse authorization request failed!");

        // when
        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
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
        assert_eq!(
            response.headers()[header::LOCATION],
            "https://client.example.com/cb?code=SplxlOBeZQQYbYS6WxSbIA&state=xyz",
            "authorization request should contain the redirect_uri and authorization code!"
        );
    }
}
