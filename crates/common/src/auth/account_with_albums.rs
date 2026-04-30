use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumRef {
    pub album_id: String,
    pub album_name: String,
    pub role: String, // "owner" or "viewer"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountWithAlbums {
    pub account_id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub is_admin: bool,
    pub albums: Vec<AlbumRef>,
}
