use axum::http::StatusCode;

pub(crate) async fn get_user_id_profile() -> std::result::Result<String, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
