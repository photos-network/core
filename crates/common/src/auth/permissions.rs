use crate::database::ArcDynDatabase;
use anyhow::Result;

pub enum AlbumPermission {
    Read,
    Write,
    Owner,
}

/// Returns true if the account has at least the required permission on the album.
/// Admins always pass. Owners pass for all levels. Viewers pass for Read only.
pub async fn has_album_permission(
    db: &ArcDynDatabase,
    account_id: &str,
    album_id: &str,
    required: AlbumPermission,
) -> Result<bool> {
    if db.is_account_admin(account_id).await.unwrap_or(false) {
        return Ok(true);
    }
    if let Ok(album) = db.get_album(album_id).await {
        if album.owner == account_id {
            return Ok(true);
        }
    }
    let role = db
        .get_album_account_role(account_id, album_id)
        .await
        .unwrap_or(None);
    Ok(match (role.as_deref(), required) {
        (Some("owner"), _) => true,
        (Some("viewer"), AlbumPermission::Read) => true,
        _ => false,
    })
}
