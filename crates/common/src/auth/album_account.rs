use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct AlbumAccountEntry {
    pub account_id: String,
    pub album_id: String,
    pub role: String,
}
