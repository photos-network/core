use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::task::JoinHandle;

use crate::database::{media_item::MediaItem, ArcDynDatabase};

const CACHE_BASE: &str = "./data/cache/albums";
const DEBOUNCE_SECS: u64 = 300;

pub struct ZipCacheManager {
    pending: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
}

impl ZipCacheManager {
    pub fn new() -> Self {
        Self {
            pending: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn all_zip_path(album_id: &str) -> PathBuf {
        PathBuf::from(CACHE_BASE).join(album_id).join("all.zip")
    }

    pub fn customer_zip_path(album_id: &str, caller_id: &str) -> PathBuf {
        PathBuf::from(CACHE_BASE)
            .join(album_id)
            .join(format!("{}.zip", caller_id))
    }

    /// Delete all cached ZIPs for an album and cancel any pending generation task.
    pub async fn invalidate(&self, album_id: &str) {
        let handle = self.pending.lock().unwrap().remove(album_id);
        if let Some(h) = handle {
            h.abort();
        }
        let _ = tokio::fs::remove_dir_all(PathBuf::from(CACHE_BASE).join(album_id)).await;
    }

    /// Schedule eager "all items" ZIP generation after a debounce delay.
    /// Cancels any previously scheduled task for the same album.
    pub async fn schedule_generation(&self, album_id: String, db: ArcDynDatabase) {
        let prev = self.pending.lock().unwrap().remove(&album_id);
        if let Some(h) = prev {
            h.abort();
        }

        let pending = Arc::clone(&self.pending);
        let album_id_clone = album_id.clone();

        let handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(DEBOUNCE_SECS)).await;
            match generate_and_write_all_zip(&album_id_clone, &db).await {
                Ok(_) => tracing::info!("Eager ZIP cached for album {}", album_id_clone),
                Err(e) => tracing::warn!(
                    "Eager ZIP pre-generation failed for album {}: {:?}",
                    album_id_clone, e
                ),
            }
            pending.lock().unwrap().remove(&album_id_clone);
        });

        self.pending.lock().unwrap().insert(album_id, handle);
    }
}

/// Build a ZIP and write it atomically to `path`.
/// Each photo is read and compressed individually — peak RAM use is one photo, not the whole album.
pub async fn build_zip_to_file(
    items: &[MediaItem],
    path: &PathBuf,
    db: &ArcDynDatabase,
) -> anyhow::Result<()> {
    use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

    // Collect (entry_name, file_path) pairs via async DB calls.
    let mut entries: Vec<(String, String)> = Vec::new();
    for (i, item) in items.iter().enumerate() {
        if let Ok(Some((filepath, filename))) = db.get_media_file_path(&item.uuid).await {
            entries.push((
                format!("{:03}_{}", i + 1, filename),
                format!("{}/{}", filepath, filename),
            ));
        }
    }

    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let tmp = tmp_path(path);
    let tmp_clone = tmp.clone();
    let path_clone = path.clone();

    // Run sync zip I/O on a blocking thread — Deflate compression is CPU-bound.
    tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
        let file = std::fs::File::create(&tmp_clone)?;
        let mut zip = ZipWriter::new(file);
        let options =
            SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

        for (entry_name, full_path) in entries {
            if let Ok(bytes) = std::fs::read(&full_path) {
                if zip.start_file(entry_name, options).is_ok() {
                    let _ = zip.write_all(&bytes);
                }
            }
        }

        zip.finish()?;
        std::fs::rename(&tmp_clone, &path_clone)?;
        Ok(())
    })
    .await??;

    Ok(())
}

/// Write bytes to a cache path atomically (temp file → rename).
pub async fn write_cache_file(path: &PathBuf, bytes: Vec<u8>) {
    if let Some(parent) = path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    let tmp = tmp_path(path);
    if let Err(e) = tokio::fs::write(&tmp, &bytes).await {
        tracing::warn!("Failed to write ZIP cache tmp {:?}: {:?}", tmp, e);
        return;
    }
    if let Err(e) = tokio::fs::rename(&tmp, path).await {
        tracing::warn!("Failed to rename ZIP cache {:?}: {:?}", path, e);
        let _ = tokio::fs::remove_file(&tmp).await;
    }
}

/// Generate and cache `all.zip` without loading the whole album into RAM.
pub async fn generate_and_write_all_zip(
    album_id: &str,
    db: &ArcDynDatabase,
) -> anyhow::Result<()> {
    let items = db.get_media_for_album(album_id).await?;
    let path = ZipCacheManager::all_zip_path(album_id);
    build_zip_to_file(&items, &path, db).await
}

pub fn zip_tmp_path(path: &PathBuf) -> PathBuf {
    tmp_path(path)
}

fn tmp_path(path: &PathBuf) -> PathBuf {
    let name = path
        .file_name()
        .map(|n| format!("{}.tmp", n.to_string_lossy()))
        .unwrap_or_else(|| "archive.zip.tmp".to_string());
    path.with_file_name(name)
}
