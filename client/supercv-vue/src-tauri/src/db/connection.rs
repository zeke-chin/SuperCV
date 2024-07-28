use std::path::Path;

use log::info;
use migration::Migrator;
use migration::MigratorTrait;
use sea_orm::{Database, DatabaseConnection, DbErr};

use crate::utils::config::CONFIG;

pub async fn init_db_connection(path: Option<&str>) -> Result<DatabaseConnection, DbErr> {
    let db_path = if let Some(p) = path {
        Path::new(p).to_path_buf()
    } else {
        CONFIG.read().unwrap().db_path.join("db.sqlite")
    };

    let db_url = format!("sqlite:{}", db_path.display());
    info!("{}", &db_url);
    if !db_path.exists() {
        if let Some(parent) = db_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| DbErr::Custom(format!("Failed to create directory: {}", e)))?;
        }
        info!("Creating new database file: {}", db_path.display());
        tokio::fs::File::create(db_path)
            .await
            .map_err(|e| DbErr::Custom(format!("Failed to create file: {}", e)))?;
    }
    establish_connection(&db_url).await
}

async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(database_url).await;
    match db {
        Ok(conn) => {
            Migrator::up(&conn, None).await?;
            Ok(conn)
        }
        Err(e) => Err(e),
    }
}
