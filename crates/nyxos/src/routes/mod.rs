use crate::openapi::ApiDoc;
use crate::{
    auth::AuthUser,
    packages::{FileItem, Meta, ProjectDetailResponse, ProjectItem, ProjectListResponse},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use nyxos_common::normalize_name;
use std::collections::BTreeSet;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::AppStateData;
mod auth;
mod health;

pub fn create_router(state: AppStateData) -> Router {
    let (router, api) = OpenApiRouter::<AppStateData>::with_openapi(ApiDoc::openapi())
        .routes(routes!(say_hello))
        .nest("/api/v1", health::create_routes())
        .split_for_parts();
    router
        .route(
            "/api/v1/users",
            get(nyxos_db::crud::list_users).post(nyxos_db::crud::create_user),
        )
        .route(
            "/api/v1/users/{id}",
            get(nyxos_db::crud::get_user)
                .put(nyxos_db::crud::update_user)
                .delete(nyxos_db::crud::delete_user),
        )
        .route("/api/v1/protected", get(protected_endpoint))
        .route("/api/v1/auth/login", axum::routing::post(auth::login))
        .route("/api/v1/simple/", get(list_projects))
        .route("/api/v1/simple/{project}/", get(list_packages))
        .route("/packages/{project}/{filename}", get(download_package))
        .with_state(state)
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", api))
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin([
                    HeaderValue::from_static("http://localhost:3001"),
                    HeaderValue::from_static("http://127.0.0.1:3001"),
                ])
                .allow_methods([axum::http::Method::POST, axum::http::Method::GET])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ]),
        )
}

#[utoipa::path(get, path = "/", responses((status = 200, description = "say hello")))]
async fn say_hello() -> &'static str {
    return "Hello, World";
}

async fn protected_endpoint(AuthUser { user_id }: AuthUser) -> String {
    format!("Authenticated user: {user_id}")
}

async fn list_projects(State(state): State<AppStateData>) -> Response {
    let keys = match state.storage.list(None).await {
        Ok(keys) => keys,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let projects = keys
        .iter()
        .filter_map(|key| key.split_once('/').map(|(name, _)| normalize_name(name)))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|name| ProjectItem { name })
        .collect();

    let response_data = ProjectListResponse {
        meta: Meta {
            api_version: "1.0".to_string(),
        },
        projects,
    };

    json_response(response_data)
}

async fn list_packages(Path(project): Path<String>, State(state): State<AppStateData>) -> Response {
    let project = normalize_name(&project);
    let prefix = format!("{project}/");
    let keys = match state.storage.list(Some(&prefix)).await {
        Ok(keys) => keys,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if keys.is_empty() {
        return StatusCode::NOT_FOUND.into_response();
    }

    let files = keys
        .into_iter()
        .filter_map(|key| key.strip_prefix(&prefix).map(str::to_owned))
        .filter(|filename| !filename.is_empty() && !filename.contains('/'))
        .map(|filename| FileItem {
            url: format!("/packages/{project}/{filename}"),
            filename,
            hashes: Default::default(),
        })
        .collect();

    json_response(ProjectDetailResponse {
        meta: Meta {
            api_version: "1.0".to_string(),
        },
        name: project,
        files,
    })
}

async fn download_package(
    Path((project, filename)): Path<(String, String)>,
    State(state): State<AppStateData>,
) -> Response {
    if filename.is_empty() || filename.contains('/') {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let project = normalize_name(&project);
    let key = format!("{project}/{filename}");
    let data = match state.storage.get(&key).await {
        Ok(data) => data,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    let mut response = data.into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    response
}

fn json_response<T: serde::Serialize>(value: T) -> Response {
    let mut response = Json(value).into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/vnd.pypi.simple.v1+json"),
    );
    response
}
