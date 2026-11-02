use axum::Router;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

mod default;
mod fallback;
mod healthcheck;
mod v1;

#[derive(OpenApi)]
#[openapi(
    info(title = "API"),
    tags(
        (name = "health", description = "Process liveness"),
        (name = "users", description = "User example"),
    ),
    nest(
        (path = "/", api = healthcheck::Doc, tags = ["health"]),
        (path = "/api/v1", api = v1::users::Doc, tags = ["users"]),
    ),
)]
pub struct ApiDoc;

pub fn router() -> Router {
    Router::new()
        .merge(default::router())
        .merge(healthcheck::router())
        .nest("/api/v1", v1::router())
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/docs"))
        .fallback(fallback::not_found)
}
