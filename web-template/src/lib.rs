#![allow(async_fn_in_trait)]

pub mod api;
pub mod application;
pub mod config;
pub mod di;
pub mod domain;
pub mod infra;
pub mod utils;

/// Generate the same document served at `/api-docs/openapi.json`.
#[must_use]
pub fn openapi_yaml() -> String {
    use utoipa::OpenApi as _;

    serde_yaml::to_string(&api::controllers::ApiDoc::openapi())
        .expect("OpenAPI document is always serializable")
}
