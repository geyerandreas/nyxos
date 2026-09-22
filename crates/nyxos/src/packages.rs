use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct Meta {
    #[serde(rename = "api-version")]
    pub api_version: String,
}

#[derive(Serialize)]
pub struct ProjectListResponse {
    pub meta: Meta,
    pub projects: Vec<ProjectItem>,
}

#[derive(Serialize)]
pub struct ProjectItem {
    pub name: String,
}

#[derive(Serialize)]
pub struct ProjectDetailResponse {
    pub meta: Meta,
    pub name: String,
    pub files: Vec<FileItem>,
}

#[derive(Serialize)]
pub struct FileItem {
    pub filename: String,
    pub url: String,
    pub hashes: HashMap<String, String>,
}
