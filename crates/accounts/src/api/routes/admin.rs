use std::sync::Arc;

use axum::{extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse, Json};
use common::auth::auth_manager::AuthManager;
use common::database::ArcDynDatabase;
use serde::Deserialize;
use super::customer::extract_session;

pub async fn list_users(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let (account_id, role) = match extract_session(&headers) {
        Ok(pair) => pair,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" || !db.is_account_admin(&account_id).await.unwrap_or(false) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Admin access required"}))).into_response();
    }
    match db.list_all_accounts().await {
        Ok(accounts) => (StatusCode::OK, Json(accounts)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn list_albums(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let (account_id, role) = match extract_session(&headers) {
        Ok(pair) => pair,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" || !db.is_account_admin(&account_id).await.unwrap_or(false) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Admin access required"}))).into_response();
    }
    match db.list_all_albums().await {
        Ok(albums) => (StatusCode::OK, Json(albums)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn list_users_detailed(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let (account_id, role) = match extract_session(&headers) {
        Ok(pair) => pair,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" || !db.is_account_admin(&account_id).await.unwrap_or(false) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Admin access required"}))).into_response();
    }
    match db.get_accounts_with_albums().await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

pub async fn create_user(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Json(req): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let (account_id, role) = match extract_session(&headers) {
        Ok(pair) => pair,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" || !db.is_account_admin(&account_id).await.unwrap_or(false) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Admin access required"}))).into_response();
    }
    let auth_manager = AuthManager::new(Arc::clone(&db));
    let display_name = req.display_name.unwrap_or_default();
    match auth_manager.create_account(req.email, req.password, display_name, None).await {
        Ok(new_account_id) => (StatusCode::CREATED, Json(serde_json::json!({"account_id": new_account_id}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn list_customers(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let (account_id, role) = match extract_session(&headers) {
        Ok(pair) => pair,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" || !db.is_account_admin(&account_id).await.unwrap_or(false) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Admin access required"}))).into_response();
    }
    match db.list_customers().await {
        Ok(customers) => (StatusCode::OK, Json(customers)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCustomerCodeRequest {
    pub display_name: String,
}

pub async fn create_customer_code(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Json(req): Json<CreateCustomerCodeRequest>,
) -> impl IntoResponse {
    let (account_id, role) = match extract_session(&headers) {
        Ok(pair) => pair,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" || !db.is_account_admin(&account_id).await.unwrap_or(false) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Admin access required"}))).into_response();
    }
    match db.generate_code(&req.display_name).await {
        Ok(access_code) => (StatusCode::CREATED, Json(serde_json::json!({"access_code": access_code}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}
