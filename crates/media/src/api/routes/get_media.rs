//! Returns a list of owned media items for current user
//!
use axum::{extract::Query, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::result::Result;

#[derive(Serialize, Deserialize)]
pub(crate) struct MediaListQuery {
    offset: Option<i32>,
    limit: Option<i32>,
}

pub(crate) async fn get_media(
    Query(query): Query<MediaListQuery>,
) -> Result<Json<String>, StatusCode> {
    // TODO: check auth header
    // TODO: read list from persistency
    // TODO: return list
    Ok(Json(
        format!(
            "list media items. limit={}, offset={}",
            query.limit.unwrap_or_else(|| 1000),
            query.offset.unwrap_or_else(|| 0)
        )
        .to_owned(),
    ))
}
