use crate::error::Result;
use nyxos_common::normalize_name;
use serde::Serialize;
use sqlx::{Error, SqlitePool, prelude::FromRow};

#[derive(Debug, Serialize, FromRow)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub normalized_name: String,
}

#[derive(Debug, FromRow)]
pub struct PackageFile {
    pub id: i32,
    pub package_id: i32,
    pub filename: String,
    pub version: String,
    pub requires_python: Option<String>,
    pub sha256_hash: String,
    pub file_path: String,
    pub uploaded_at: String,
}

pub async fn get_package(pool: &SqlitePool, package_name: &str) -> Result<Package, Error> {
    sqlx::query_as::<_, Package>("SELECT * FROM Packages WHERE normalized_name = $1")
        .bind(package_name)
        .fetch_one(pool)
        .await
}

pub async fn get_package_files(
    pool: &SqlitePool,
    package_id: i32,
) -> Result<Vec<PackageFile>, Error> {
    sqlx::query_as("SELECT * FROM PackageFiles WHERE package_id = $1 ORDER BY filename")
        .bind(package_id)
        .fetch_all(pool)
        .await
}

pub async fn get_packages(pool: &SqlitePool) -> Result<Vec<String>, Error> {
    sqlx::query_scalar("SELECT name FROM Packages ORDER BY name ASC")
        .fetch_all(pool)
        .await
}

pub async fn add_package(pool: &SqlitePool, package_name: &str) -> Result<Package> {
    let result = sqlx::query_as(
        "INSERT INTO Packages (name, normalized_name)
        VALUES ($1, $2)
        RETURNING *",
    )
    .bind(package_name)
    .bind(normalize_name(package_name))
    .fetch_one(pool)
    .await?;

    Ok(result)
}
