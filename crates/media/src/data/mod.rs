use std::time::Duration;

use sea_orm::{Database, DatabaseConnection, ConnectOptions, DbErr};
use log::LevelFilter;

pub mod error;
pub mod file;
pub mod location;
pub mod media_item;
pub mod exif_info;

pub async fn open_db_conn(db_url: String) -> std::result::Result<DatabaseConnection, DbErr> {

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Info);
        // .set_schema_search_path("my_schema".into()); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await?;

    Ok(db)
}
