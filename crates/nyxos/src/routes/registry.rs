use axum::{
    Json,
    extract::{Multipart, Path, Request, State},
    http::{
        HeaderMap, StatusCode,
        header::{self, CONTENT_TYPE},
    },
    response::{Html, IntoResponse, Response},
};
use nyxos_appstate::AppState;
use nyxos_common::normalize_name;
use nyxos_db::registry::Package;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::registry::ResponseFormat;

/// Creates the registry routes
pub fn create_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_project))
        .routes(routes!(list_projects, upload_package))
}

#[derive(Serialize)]
pub struct ProjectListJson {
    pub meta: VersionMeta,
    pub projects: Vec<ProjectItem>,
}

#[derive(Serialize)]
pub struct VersionMeta {
    #[serde(rename = "_version")]
    pub version: String,
}

#[derive(Serialize)]
pub struct ProjectItem {
    pub name: String,
}

#[derive(Serialize, ToSchema)]
struct Meta {
    #[serde(rename = "api-version")]
    api_version: String,
}

#[derive(Serialize, ToSchema)]
struct PackageFileJson {
    filename: String,
    url: String,
    hashes: std::collections::HashMap<String, String>,
    #[serde(rename = "requires-python", skip_serializing_if = "Option::is_none")]
    requires_python: Option<String>,
}

impl PackageFileJson {
    fn from_package_file(value: nyxos_db::registry::PackageFile, package_name: &str) -> Self {
        let filename = value.filename;
        Self {
            url: format!("/packages/{package_name}/{filename}"),
            filename,
            hashes: HashMap::from([("sha256".to_owned(), value.sha256_hash)]),
            requires_python: value.requires_python,
        }
    }
}

#[derive(serde::Serialize, ToSchema)]
struct ProjectDetailsJson {
    meta: Meta,
    files: Vec<PackageFileJson>,
}

#[utoipa::path(
    get,
    path = "/simple/{project}/",
    tag = "registry",
    responses(
        (
            status = 200,
            description = "HTML or JSON Index of the Packages",
            content(
                (String = "text/html"),
                (ProjectDetailsJson = "application/vnd.pypi.simple.v1+json")
            )
        )
    )
)]
async fn get_project(
    State(state): State<AppState>,
    Path(project): Path<String>,
    request: Request,
) -> impl IntoResponse {
    let normalized = normalize_name(&project);

    let package = match nyxos_db::registry::get_package(&state.pool, &normalized).await {
        Ok(package) => package,
        Err(sqlx::Error::RowNotFound) => return StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            tracing::error!(%error, package = %normalized, "failed to load package");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    let files = match nyxos_db::registry::get_package_files(&state.pool, package.id).await {
        Ok(files) => files,
        Err(error) => {
            tracing::error!(%error, package = %normalized, "failed to load package files");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let accept_header = request
        .headers()
        .get(header::ACCEPT)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    if accept_header.contains("application/vnd.pypi.simple.v1+json") {
        // PEP 691 JSON
        let response_json = ProjectDetailsJson {
            meta: Meta {
                api_version: "1.0".to_owned(),
            },
            files: files
                .into_iter()
                .map(|file| PackageFileJson::from_package_file(file, &normalized))
                .collect(),
        };
        (
            [(header::CONTENT_TYPE, "application/vnd.pypi.simple.v1+json")],
            Json(response_json),
        )
            .into_response()
    } else {
        let links = files
            .into_iter()
            .map(|file| {
                let url = format!("/packages/{normalized}/{}", file.filename);
                let hash = file.sha256_hash;
                let requires_python = file
                    .requires_python
                    .map(|value| format!(r#" data-requires-python="{value}""#))
                    .unwrap_or_default();
                format!(
                    r#"<a href="{url}#sha256={hash}"{requires_python}>{}</a>"#,
                    file.filename
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let html_content = format!(
            r#"<!DOCTYPE html><html><head><meta name="pypi:repository-version" content="1.0"></head><body><h1>Links for {normalized}</h1>{links}</body></html>"#
        );
        (
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            html_content,
        )
            .into_response()
    }
}

/// List all projects
#[utoipa::path(
    get,
    path = "/simple/",
    tag = "registry",
    responses(
        (status = 200, description = "List of all projects"),
        (status = 500, description = "Internal server error"),
    ),
)]
async fn list_projects(
    State(state): State<AppState>,
    format: ResponseFormat,
) -> Result<Response, StatusCode> {
    let projects: Vec<String> = nyxos_db::registry::get_packages(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match format {
        ResponseFormat::Json => {
            let body = ProjectListJson {
                meta: VersionMeta {
                    version: "1.0".to_owned(),
                },
                projects: projects
                    .into_iter()
                    .map(|name| ProjectItem { name })
                    .collect(),
            };
            let mut headers = HeaderMap::new();
            headers.insert(
                CONTENT_TYPE,
                "application/vnd.pypi.simple.v1+json".parse().unwrap(),
            );
            Ok((headers, Json(body)).into_response())
        }
        ResponseFormat::Html => {
            let mut html = String::from("<!DOCTYPE html><html><body>\n");
            for project in projects {
                html.push_str(&format!(
                    "<a href=\"/simple/{}/\">{}</a><br>\n",
                    project, project
                ));
            }
            html.push_str("</body></html>");
            Ok(Html(html).into_response())
        }
    }
}

#[derive(ToSchema)]
pub struct UploadPayload {
    /// The package archive to upload
    #[schema(value_type = String, format = Binary)]
    pub content: Vec<u8>,
    /// The package name
    pub name: String,
    /// The package version
    pub version: String,
}

/// Upload a package
#[utoipa::path(
    post,
    path = "/simple/",
    tag = "registry",
    request_body(
        content = UploadPayload,
        description = "The package file and associated metadata",
        content_type = "multipart/form-data"
    ),
    responses(
        (status = 201, description = "Package upload successful"),
        (status = 400, description = "Bad request"),
    ),
)]
async fn upload_package(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Package>), StatusCode> {
    let mut file_bytes = Vec::new();
    let mut filename = String::new();
    let mut name = String::new();
    let mut version = String::new();

    // Consume multipart stream safely
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "content" => {
                filename = field.file_name().unwrap_or("artifact.whl").to_string();
                file_bytes = field
                    .bytes()
                    .await
                    .map_err(|_| StatusCode::BAD_REQUEST)?
                    .to_vec();
            }
            "name" => {
                name = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            }
            "version" => {
                version = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            }
            _ => {}
        }
    }

    if file_bytes.is_empty() || name.is_empty() || version.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Compute cryptographic SHA256 integrity hash required by client tools

    let mut hasher = Sha256::new();
    hasher.update(&file_bytes);
    let hash_result = hex::encode(hasher.finalize());

    // Write metadata to state layer, stream bytes to backend local storage folder path
    // Production note: Store package metadata & files on target block paths or S3
    let storage_key = format!("{}/{}/{filename}", "storage", normalize_name(&name));
    state
        .storage
        .put(&storage_key, file_bytes)
        .await
        .map_err(|error| {
            tracing::error!(%error, key = %storage_key, "failed to store package");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    nyxos_db::registry::add_package(&state.pool, &name)
        .await
        .map(|package| (StatusCode::CREATED, Json(package)))
        .map_err(|error| {
            tracing::error!(%error, package = %name, "failed to add package");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
