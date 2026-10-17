use axum::{
    Json,
    extract::{Path, Request, State},
    http::{StatusCode, header},
    response::IntoResponse,
};
use nyxos_appstate::AppState;
use nyxos_common::normalize_name;
use serde::Serialize;
use std::collections::HashMap;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

/// Creates the registry routes
pub fn create_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_project))
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
    path = "/{project}/",
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
