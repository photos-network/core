//! Returns a list of owned media items for current user
//! 
use axum::{http::StatusCode, extract::Query, Json};
use serde::{Deserialize, Serialize};
use std::result::Result;

#[derive(Serialize, Deserialize)]
pub(crate) struct MediaListQuery {
    offset: Option<i32>,
    limit: Option<i32>,
}

pub(crate) async fn get_media(Query(query): Query<MediaListQuery>) -> Result<Json<String>, StatusCode> {
    // Err(StatusCode::UNAUTHORIZED)
    Ok(Json(format!("list media items. limit={}, offset={}", query.limit.unwrap(), query.offset.unwrap()).to_owned()))
}
