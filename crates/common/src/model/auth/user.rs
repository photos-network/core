use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use http::request::Parts;
use std::fmt;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    uuid: Uuid,
    email: String,
    password: Option<String>,
    lastname: Option<String>,
    firstname: Option<String>,
    is_locked: bool,
    created_at: OffsetDateTime,
    updated_at: Option<OffsetDateTime>,
    last_login: Option<OffsetDateTime>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}), locked:{})",
            self.email, self.uuid, self.is_locked
        )
    }
}

impl User {
    fn new(email: String) -> User {
        User {
            uuid: Uuid::new_v4(),
            email,
            password: Option::None,
            lastname: Option::None,
            firstname: Option::None,
            is_locked: false,
            created_at: OffsetDateTime::now_utc(),
            updated_at: Option::None,
            last_login: Option::None,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let _auth_token = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        // TODO: get user for Authtoken
        Ok(User::new("info@photos.network".to_string()))
        // TODO: verify Token
        //        verify_auth_token(auth_header)
        //            .await
        //            .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized"))
        //Err((StatusCode::UNAUTHORIZED, "Unauthorized"))
    }
}
